use bolt_lang::*;

declare_id!("3CFTHHqDdQaWNZ8pQ8X5GWmRXuyJWfP1yq94EmAAWqSk");

#[component]
#[derive(Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    #[max_len(20)]
    pub description: String,
}
