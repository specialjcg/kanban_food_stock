use crate::domain_core::add_category::add_category;
use crate::domain_core::add_name::add_name;
use crate::domain_core::create_card_kanban::create_card_kanban;
use crate::domain_core::CardKanban::CardKanban;

pub fn add_card_kanban_to_list_without_duplicate(
    new_card: CardKanban,
    mut cards: Vec<CardKanban>,
) -> Vec<CardKanban> {
    if !cards.iter().any(|card| card.name == new_card.name) {
        cards.push(new_card);
    }
    cards
}
#[test]
fn it_should_create_a_list_of_card_kanban_with_no_doublon() {
    let mut card_kanban_carotte = create_card_kanban();
    let mut card_kanban_concomber = create_card_kanban();

    card_kanban_carotte = add_category(card_kanban_carotte, "legume");
    card_kanban_carotte = add_name(card_kanban_carotte, "carotte");
    card_kanban_concomber = add_category(card_kanban_concomber, "legume");
    card_kanban_concomber = add_name(card_kanban_concomber, "carotte");

    let mut cards_kanban = Vec::new();
    cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_carotte, cards_kanban);
    cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_concomber, cards_kanban);

    assert_eq!(cards_kanban.len(), 1);
}
