use crate::domain_core::add_name;
use crate::domain_core::create_card_kanban::create_card_kanban;
use crate::domain_core::CardKanban::CardKanban;

pub fn add_name(p0: CardKanban, p1: &str) -> CardKanban {
    CardKanban {
        name: p1.to_string(),
        category: p0.category,
        quantity_stock: 0,
    }
}
#[test]
fn it_should_create_a_card_kanban_with_name() {
    let mut card_kanban = create_card_kanban();
    card_kanban = add_name::add_name(card_kanban, "test");
    assert_eq!(
        card_kanban,
        CardKanban {
            name: "test".to_string(),
            category: "".to_string(),
            quantity_stock: 0
        }
    );
}
