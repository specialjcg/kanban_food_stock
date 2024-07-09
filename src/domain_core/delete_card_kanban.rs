use yew::prelude::*;
use crate::domain_core::card_kanban::CardKanban;
use crate::domain_core::create_card_kanban::create_kanban_item;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_fields;
use crate::domain_core::list_kanban::{load_list_kanban, save_list_kanban};
use crate::shell::storage::memory_store::create_memory_store;
use crate::shell::storage::Storage;

pub fn delete_card_kanban_by_index(mut cards: &mut Vec<CardKanban>, index: usize) -> &mut Vec<CardKanban> {
    if index < cards.len() {
        cards.remove(index); // Remove the card at the specified index
    }
    cards
}

#[test]
fn it_should_delete_a_card_kanban_from_a_list_of_card() {
    let storage = create_memory_store();

    let item1 = create_kanban_item("carotte", 10);
    let item2 = create_kanban_item("concombre", 5);
    let card_kanban_carotte = create_card_kanban_with_all_fields("legume", vec![item1.clone(), item2.clone()]);

    let item3 = create_kanban_item("cucumber", 10);
    let card_kanban_concombre = create_card_kanban_with_all_fields("legume1", vec![item3.clone()]);

    let mut initial_cards = vec![
        card_kanban_carotte.clone(),
        card_kanban_concombre.clone(),
    ];




    initial_cards.remove(0);
    save_list_kanban(&storage, initial_cards.clone());
    let cards=load_list_kanban(&storage);
    // Simulate loading cards from storage


    // Now assert on the updated cards length
    assert_eq!(cards.expect("REASON").len(),  1);
}
