use crate::CardKanban::CardKanban;

pub fn delete_card_kanban(deletedCard: CardKanban, listCard: Vec<CardKanban>) -> Vec<CardKanban> {
    listCard.into_iter().filter(|card| card != &deletedCard).collect()
}
