#[derive(PartialEq, Debug, Clone)]
pub struct CardKanban {
    pub(crate) name: String,
    pub(crate) category: String,
    pub quantity_stock: i32,
}
