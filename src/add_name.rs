use crate::CardKanban::CardKanban;

pub fn add_name(p0: CardKanban, p1: &str) -> CardKanban {
    CardKanban { name: p1.to_string(), category: p0.category, quantity_stock: 0 }
}
