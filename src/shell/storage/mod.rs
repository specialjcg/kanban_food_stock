use crate::domain_core::card_kanban::CardKanban;
use std::io;

pub(crate) mod memory_store;

pub trait Storage {
    fn save(&self, cards: Vec<CardKanban>) -> io::Result<()>;
    fn load(&self) -> io::Result<Vec<CardKanban>>;
}
