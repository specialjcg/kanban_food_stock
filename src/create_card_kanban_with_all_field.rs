use crate::CardKanban::CardKanban;

pub fn create_card_kanban_with_all_field(name: &str, category: &str, quantity_stock: i32) -> CardKanban {
    CardKanban { name: name.to_string(), category: category.to_string(), quantity_stock }
}
