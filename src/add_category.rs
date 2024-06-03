use crate::CardKanban::CardKanban;

pub fn add_category(p0: CardKanban, p1: &str) -> CardKanban {
    return CardKanban { name: p0.name, category: p1.to_string(), quantity_stock: 0 };
}
