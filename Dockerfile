# Stage 1: Install and configure toolchains
FROM node:lts as solana-toolchain
RUN echo "installing yarn"
RUN npm install -g yarn

RUN echo "installing rust"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env"
RUN rustc --version
RUN cargo install cargo-chef --locked

RUN echo "installing solana cli"
RUN sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
RUN solana --version
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

RUN echo "installing anchor cli"
RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 anchor-cli
RUN anchor --version

# Stage 2: Install Node.js dependencies
FROM solana-toolchain as node-dependencies
WORKDIR /usr/src/myapp
COPY package.json .
COPY yarn.lock .
RUN yarn install

# Stage 3: Install Cargo dependencies
FROM solana-toolchain as cargo-chef
WORKDIR /usr/src/myapp
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM solana-toolchain as cargo-dependencies
WORKDIR /usr/src/myapp
COPY --from=cargo-chef /usr/src/myapp/recipe.json .
RUN cargo chef cook --recipe-path recipe.json

# Stage 4: Build and test the program
FROM solana-toolchain as build-and-test
WORKDIR /usr/src/myapp
COPY --from=cargo-dependencies /usr/src/myapp/target ./target
COPY --from=node-dependencies /usr/src/myapp/node_modules ./node_modules
COPY . .
CMD [ "anchor", "test" ]
