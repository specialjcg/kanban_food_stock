use crate::core::card_kanban::CardKanban;

pub fn display_card(card: &CardKanban) {
    println!("Card: {:?}", card);
}

pub fn display_list(list: &Vec<CardKanban>) {
    for card in list {
        println!("Card: {:?}", card);
    }
}
