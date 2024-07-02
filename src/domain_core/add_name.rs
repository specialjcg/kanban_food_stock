use yew::Callback;
use crate::domain_core::add_name;
use crate::domain_core::create_card_kanban::{create_card_kanban, create_kanban_item};
use crate::domain_core::card_kanban::{CardKanban, KanbanItem};

pub fn add_name(card: CardKanban, name: &str) -> CardKanban {
    let mut items = card.items;
    items.push(KanbanItem {
        name: name.to_string(),
        quantity_stock: 0,
        on_delete: Callback::from(|_| log::info!("Item deleted")),
    });

    CardKanban {
        category: card.category,
        items,
        on_delete: card.on_delete,
    }
}

#[test]
fn it_should_create_a_card_kanban_with_name() {
    // Create an empty CardKanban
    let kanban_item_carotte = create_kanban_item("carotte", 20);
    let card_kanban =create_card_kanban("legume", vec![kanban_item_carotte.clone()]);

    // Add a name to the CardKanban, which creates a new KanbanItem
    let updated_card_kanban = add_name(card_kanban, "test");

    // Expected CardKanban after adding the item


    // Assert the CardKanban contains the new item with the name "test"
    assert_eq!(updated_card_kanban.items[1].name, "test");
    assert_eq!(updated_card_kanban.items.len(), 2);

}
