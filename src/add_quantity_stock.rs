use crate::CardKanban::CardKanban;

pub fn add_quantity_stock(card_kanban: CardKanban, quantityStock: i32) -> CardKanban {
    CardKanban {
        name: card_kanban.name,
        category: card_kanban.category,
        quantity_stock: quantityStock
    }
}
