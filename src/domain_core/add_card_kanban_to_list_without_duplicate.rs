use crate::domain_core::card_kanban::CardKanban;
use crate::domain_core::create_card_kanban::{create_card_kanban, create_kanban_item};

pub fn add_card_kanban_to_list_without_duplicate(
    new_card: CardKanban,
    mut cards: Vec<CardKanban>,
) -> Vec<CardKanban> {
    let new_card_names: Vec<String> = new_card.items.iter().map(|item| item.name.clone()).collect();

    if !cards.iter().any(|card| {
        card.category == new_card.category &&
            card.items.iter().any(|item| new_card_names.contains(&item.name))
    }) {
        cards.push(new_card);
    }
    cards
}

#[test]
fn it_should_create_a_list_of_card_kanban_with_no_duplicates() {
    // Create KanbanItems
    let kanban_item_carotte_1 = create_kanban_item("carotte", 20);
    let kanban_item_carotte_2 = create_kanban_item("carotte", 15);

    // Create CardKanban with a single item
    let card_kanban_1 = create_card_kanban("legume", vec![kanban_item_carotte_1.clone()]);
    let card_kanban_2 = create_card_kanban("legume", vec![kanban_item_carotte_2.clone()]);

    // Create a list of CardKanban
    let mut cards_kanban = Vec::new();
    cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_1, cards_kanban);
    cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_2, cards_kanban);

    // There should only be one card in the list because the second has the same name
    assert_eq!(cards_kanban.len(), 1);

    // Verify the card's details
    assert_eq!(cards_kanban[0].category, "legume");
    assert_eq!(cards_kanban[0].items.len(), 1);
    assert_eq!(cards_kanban[0].items[0].name, "carotte");
    assert_eq!(cards_kanban[0].items[0].quantity_stock, 20); // Or 15 depending on which one you want to keep
}
