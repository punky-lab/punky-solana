use bolt_lang::*;

declare_id!("CZuB5tTqJgu8ngWUSBtn5vFFSBin4VXbB8Yk56HUwtNc");

#[component]
#[derive(Default)]
pub struct PunkyStatus {
    pub loyalty: i64,
    pub fitness: i64,
    pub health: i64,
    #[max_len(20)]
    pub name: String,
}
