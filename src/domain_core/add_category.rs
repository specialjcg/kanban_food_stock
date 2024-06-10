use crate::domain_core::add_name::add_name;
use crate::domain_core::create_card_kanban::create_card_kanban;
use crate::domain_core::CardKanban::CardKanban;

pub fn add_category(p0: CardKanban, p1: &str) -> CardKanban {
    return CardKanban {
        name: p0.name,
        category: p1.to_string(),
        quantity_stock: 0,
    };
}

#[test]
fn it_should_create_a_card_kanban_with_category() {
    let mut card_kanban = create_card_kanban();
    card_kanban = add_category(card_kanban, "test");
    assert_eq!(
        card_kanban,
        CardKanban {
            name: "".to_string(),
            category: "test".to_string(),
            quantity_stock: 0
        }
    );
}
#[test]
fn it_should_create_a_card_kanban_with_category_name_and_name() {
    let mut card_kanban = create_card_kanban();
    card_kanban = add_category(card_kanban, "legume");
    card_kanban = add_name(card_kanban, "carotte");

    assert_eq!(
        card_kanban,
        CardKanban {
            name: "carotte".to_string(),
            category: "legume".to_string(),
            quantity_stock: 0
        }
    );
}
