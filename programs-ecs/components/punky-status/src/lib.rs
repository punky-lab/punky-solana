use bolt_lang::*;

declare_id!("CZuB5tTqJgu8ngWUSBtn5vFFSBin4VXbB8Yk56HUwtNc");

#[component]
#[derive(Default)]
pub struct PunkyStatus {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    #[max_len(20)]
    pub description: String,
}
