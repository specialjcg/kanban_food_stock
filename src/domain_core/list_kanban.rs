use std::io;
use crate::domain_core::add_card_kanban_to_list_without_duplicate::add_card_kanban_to_list_without_duplicate;
use crate::domain_core::CardKanban::CardKanban;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_field;
use crate::shell::storage::{memory_store, Storage};


#[test]
fn it_should_save() {
    let card_kanban_carotte = create_card_kanban_with_all_field("carotte", "legume", 10);
    let card_kanban_concomber = create_card_kanban_with_all_field("cucumber", "legume", 10);
    let mut listCard = Vec::new();
    listCard = add_card_kanban_to_list_without_duplicate(card_kanban_carotte, listCard);
    listCard = add_card_kanban_to_list_without_duplicate(card_kanban_concomber, listCard);
    let storage = memory_store::create_MemoryStore();
   save_list_kanban( &storage,listCard.clone());
    ;
    assert_eq!(listCard,load_list_kanban(&storage).unwrap())
}

fn load_list_kanban<S: Storage>(storage:&S) -> io::Result<Vec<CardKanban>> {
storage.load()
}

fn save_list_kanban<S: Storage>(storage:&S, cards: Vec<CardKanban>) {
    storage.save(cards).expect("TODO: panic message");
}