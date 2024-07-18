use yew::Callback;
use crate::domain_core::card_kanban::{CardKanban, KanbanItem};
use crate::domain_core::create_card_kanban::create_kanban_item;

pub fn create_card_kanban_with_all_fields(
    category: &str,
    items: Vec<KanbanItem>,
) -> CardKanban {
    CardKanban {
        category: category.to_string(),
        items,
        on_delete: Callback::default(),
        on_delete_item: Callback::default(),
    }
}

#[test]
fn it_should_create_card_kanban_with_all_fields() {
    // Create a CardKanban with all specified fields
    let item1 = create_kanban_item("carotte", 10);
    let card_kanban = create_card_kanban_with_all_fields("legume", vec![item1.clone()]);

    // Expected KanbanItem


    // Assert that the CardKanban matches the expected structure
    assert_eq!(card_kanban.items[0].name, "carotte");
    assert_eq!(card_kanban.category, "legume");
    assert_eq!(card_kanban.items[0].quantity_stock, 10);
}

#[test]
fn it_should_create_card_kanban_with_multiple_items() {
    // Define multiple KanbanItems
    let item1 = create_kanban_item("carotte", 10);

    let item2 =create_kanban_item("cucumber", 5);
    // Create a CardKanban with multiple items
    let card_kanban = create_card_kanban_with_all_fields("legume", vec![item1.clone(), item2.clone()]);

    // Assert that the CardKanban has the correct category
    assert_eq!(card_kanban.category, "legume");

    // Assert that the CardKanban has the correct number of items
    assert_eq!(card_kanban.items.len(), 2);

    // Validate the first item
    assert_eq!(card_kanban.items[0].name, "carotte");
    assert_eq!(card_kanban.items[0].quantity_stock, 10);

    // Validate the second item
    assert_eq!(card_kanban.items[1].name, "cucumber");
    assert_eq!(card_kanban.items[1].quantity_stock, 5);
}
// Function to add an item to a CardKanban
pub fn add_item_to_card_kanban(mut card_kanban: CardKanban, item: KanbanItem) -> CardKanban {
    card_kanban.items.push(item);
    card_kanban
}

// Test for adding items to CardKanban
#[test]
fn it_should_add_item_to_card_kanban() {
    // Create an initial KanbanItem
    let item1 = create_kanban_item("carotte", 10);

    // Create an initial CardKanban with one item
    let mut card_kanban = create_card_kanban_with_all_fields("legume", vec![item1.clone()]);

    // Create another KanbanItem
    let item2 = create_kanban_item("cucumber", 5);

    // Add the new item to the existing CardKanban
    card_kanban.items.push(item2.clone());

    // Assert that the CardKanban has the correct category
    assert_eq!(card_kanban.category, "legume");

    // Assert that the CardKanban has the correct number of items
    assert_eq!(card_kanban.items.len(), 2);

    // Validate the first item
    assert_eq!(card_kanban.items[0].name, "carotte");
    assert_eq!(card_kanban.items[0].quantity_stock, 10);

    // Validate the second item
    assert_eq!(card_kanban.items[1].name, "cucumber");
    assert_eq!(card_kanban.items[1].quantity_stock, 5);
}