use crate::CardKanban::CardKanban;

pub fn create_card_kanban() -> CardKanban {
    CardKanban { name: "".to_string(), category: "".to_string() }
}
