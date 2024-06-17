use crate::domain_core::add_category::add_category;
use crate::domain_core::add_name::add_name;
use crate::domain_core::create_card_kanban::create_card_kanban;
use crate::domain_core::CardKanban::CardKanban;

pub fn add_quantity_stock(card_kanban: CardKanban, quantityStock: i32) -> CardKanban {
    CardKanban {
        name: card_kanban.name,
        category: card_kanban.category,
        quantity_stock: quantityStock,
    }
}
#[test]
fn it_should_card_kanban_have_a_quantity_stock() {
    let mut card_kanban_carotte = create_card_kanban();

    card_kanban_carotte = add_category(card_kanban_carotte, "legume");
    card_kanban_carotte = add_name(card_kanban_carotte, "carotte");
    card_kanban_carotte = add_quantity_stock(card_kanban_carotte, 10);

    assert_eq!(card_kanban_carotte.quantity_stock, 10);
}
