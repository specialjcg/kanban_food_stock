use std::io;

use crate::domain_core::add_card_kanban_to_list_without_duplicate::add_card_kanban_to_list_without_duplicate;
use crate::domain_core::card_kanban::CardKanban;
use crate::domain_core::create_card_kanban_item::create_kanban_item;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_fields;
use crate::shell::output::{console_output, Output};
use crate::shell::storage::{memory_store, Storage};

#[test]
fn it_should_save() {
    let item1 = create_kanban_item("carotte", 10);
    let card_kanban_carotte = create_card_kanban_with_all_fields("legume", vec![item1.clone()]);
    let item2 =create_kanban_item("cucumber", 10);
    let card_kanban_concombre = create_card_kanban_with_all_fields("legume1", vec![item2.clone()]);
    let mut list_card = Vec::new();
    list_card = add_card_kanban_to_list_without_duplicate(card_kanban_carotte, list_card);
    list_card = add_card_kanban_to_list_without_duplicate(card_kanban_concombre, list_card);
    let storage = memory_store::create_memory_store();
    save_list_kanban(&storage, list_card.clone()).expect("TODO: panic message");

    assert_eq!(list_card, load_list_kanban(&storage).unwrap())
}
#[test]
fn it_should_console_output() {
    let item1 = create_kanban_item("carotte", 10);
    let card_kanban_carotte = create_card_kanban_with_all_fields("legume", vec![item1.clone()]);
    let item2 = create_kanban_item("cucumber", 10);
    let card_kanban_concombre = create_card_kanban_with_all_fields("legume", vec![item2.clone()]);
    let mut list_card = Vec::new();
    list_card = add_card_kanban_to_list_without_duplicate(card_kanban_carotte.clone(), list_card);
    list_card = add_card_kanban_to_list_without_duplicate(card_kanban_concombre, list_card);
    let mut output = console_output::create_console_output();
    display_card(&mut output, &card_kanban_carotte);

    assert_eq!(
        output.printed,
        vec!["{\"category\":\"legume\",\"items\":[{\"name\":\"carotte\",\"quantity_stock\":10}]}"]
    );
}

pub(crate) fn load_list_kanban<S: Storage>(storage: &S) -> io::Result<Vec<CardKanban>> {
    storage.load()
}

pub(crate) fn save_list_kanban<S: Storage>(storage: &S, cards: Vec<CardKanban>) -> Result<(), Box<dyn std::error::Error>> {
    storage.save(cards).expect("TODO: panic message");
    Ok(())
}

fn display_card<O: Output>(output: &mut O, card: &CardKanban) {
    output.print(card);
}
