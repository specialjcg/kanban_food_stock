use yew::Callback;
use crate::domain_core::add_category::add_category;
use crate::domain_core::add_name::add_name;
use crate::domain_core::create_card_kanban::{create_card_kanban, create_kanban_item};
use crate::domain_core::card_kanban::{CardKanban, KanbanItem};

pub fn add_quantity_stock(card_kanban: CardKanban, item_name: &str, quantity_stock: i32) -> CardKanban {
    let items: Vec<KanbanItem> = card_kanban.items.into_iter().map(|mut item| {
        if item.name == item_name {
            item.quantity_stock = quantity_stock;
        }
        item
    }).collect();

    CardKanban {
        category: card_kanban.category,
        items,
        on_delete: card_kanban.on_delete,
        on_delete_item: Callback::default(),

    }
}

#[test]
fn it_should_card_kanban_have_a_quantity_stock() {
    // Create an empty CardKanban and add category and item
    let kanban_item_carotte = create_kanban_item("carotte", 20);
    let mut card_kanban_carotte =create_card_kanban("legume", vec![kanban_item_carotte.clone()]);

    card_kanban_carotte = add_category(card_kanban_carotte, "legume");
    card_kanban_carotte = add_name(card_kanban_carotte, "carotte");

    // Add quantity stock to the item named "carotte"
    card_kanban_carotte = add_quantity_stock(card_kanban_carotte, "carotte", 10);

    // Find the item with the name "carotte" to check the quantity stock
    let kanban_item_carotte = card_kanban_carotte.items.into_iter()
        .find(|item| item.name == "carotte")
        .expect("Item 'carotte' should exist");

    // Assert the item has the correct quantity stock
    assert_eq!(kanban_item_carotte.quantity_stock, 10);
}
