use crate::domain_core::add_card_kanban_to_list_without_duplicate::add_card_kanban_to_list_without_duplicate;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_field;
use crate::domain_core::CardKanban::CardKanban;
use crate::shell::output::{console_output, Output};
use crate::shell::storage::{memory_store, Storage};
use std::io;

#[test]
fn it_should_save() {
    let card_kanban_carotte = create_card_kanban_with_all_field("carotte", "legume", 10);
    let card_kanban_concombre = create_card_kanban_with_all_field("cucumber", "legume", 10);
    let mut listCard = Vec::new();
    listCard = add_card_kanban_to_list_without_duplicate(card_kanban_carotte, listCard);
    listCard = add_card_kanban_to_list_without_duplicate(card_kanban_concombre, listCard);
    let storage = memory_store::create_MemoryStore();
    save_list_kanban(&storage, listCard.clone());

    assert_eq!(listCard, load_list_kanban(&storage).unwrap())
}
#[test]
fn it_should_console_output() {
    let card_kanban_carotte = create_card_kanban_with_all_field("carotte", "legume", 10);
    let card_kanban_concombre = create_card_kanban_with_all_field("cucumber", "legume", 10);
    let mut listCard = Vec::new();
    listCard = add_card_kanban_to_list_without_duplicate(card_kanban_carotte.clone(), listCard);
    listCard = add_card_kanban_to_list_without_duplicate(card_kanban_concombre, listCard);
    let mut output = console_output::create_console_output();
    display_card(&mut output, &card_kanban_carotte);

    assert_eq!(
        output.printed,
        vec!["Card: CardKanban { name: \"carotte\", category: \"legume\", quantity_stock: 10 }"]
    );
}

fn load_list_kanban<S: Storage>(storage: &S) -> io::Result<Vec<CardKanban>> {
    storage.load()
}

fn save_list_kanban<S: Storage>(storage: &S, cards: Vec<CardKanban>) {
    storage.save(cards).expect("TODO: panic message");
}

fn display_card<O: Output>(output: &mut O, card: &CardKanban) {
    output.print(card);
}
