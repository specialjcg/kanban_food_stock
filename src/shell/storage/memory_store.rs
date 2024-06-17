use crate::domain_core::CardKanban::CardKanban;
use crate::shell::storage::Storage;
use std::io;
use std::sync::Mutex;

#[derive()]
pub struct MemoryStore {
    pub listCard: Mutex<Vec<CardKanban>>,
}

pub fn create_MemoryStore() -> MemoryStore {
    MemoryStore {
        listCard: Mutex::new(Vec::new()),
    }
}

impl Storage for MemoryStore {
    fn save(&self, cards: Vec<CardKanban>) -> io::Result<()> {
        let mut stored_cards = self.listCard.lock().unwrap();
        *stored_cards = cards.to_vec();
        Ok(())
    }

    fn load(&self) -> io::Result<Vec<CardKanban>> {
        let stored_cards = self.listCard.lock().unwrap();
        Ok(stored_cards.clone())
    }
}
