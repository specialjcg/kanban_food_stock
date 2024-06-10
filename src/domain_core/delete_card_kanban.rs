use crate::domain_core::add_card_kanban_to_list_without_duplicate::add_card_kanban_to_list_without_duplicate;
use crate::domain_core::add_category::add_category;
use crate::domain_core::add_name::add_name;
use crate::domain_core::create_card_kanban::create_card_kanban;
use crate::domain_core::CardKanban::CardKanban;

pub fn delete_card_kanban(deletedCard: CardKanban, listCard: Vec<CardKanban>) -> Vec<CardKanban> {
    listCard
        .into_iter()
        .filter(|card| card != &deletedCard)
        .collect()
}
#[test]
fn it_should_delete_a_card_kanban_from_a_list_of_card() {
    let mut card_kanban_carotte = create_card_kanban();
    let mut card_kanban_concomber = create_card_kanban();

    card_kanban_carotte = add_category(card_kanban_carotte, "legume");
    card_kanban_carotte = add_name(card_kanban_carotte, "carotte");
    card_kanban_concomber = add_category(card_kanban_concomber, "legume");
    card_kanban_concomber = add_name(card_kanban_concomber, "cucumber");

    let mut cards_kanban = Vec::new();
    cards_kanban =
        add_card_kanban_to_list_without_duplicate(card_kanban_carotte.clone(), cards_kanban);
    cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_concomber, cards_kanban);
    cards_kanban = delete_card_kanban(card_kanban_carotte.clone(), cards_kanban);

    assert_eq!(cards_kanban.len(), 1);
}
