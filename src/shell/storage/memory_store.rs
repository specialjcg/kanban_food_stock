use std::collections::HashSet;
use crate::domain_core::card_kanban::CardKanban;
use crate::shell::storage::Storage;
use std::io;
use std::sync::{Arc, Mutex};

#[derive(Clone,Debug)]
pub struct MemoryStore {
    pub list_card: Arc<Mutex<Vec<CardKanban>>>,
}

pub fn create_memory_store() -> MemoryStore {
    MemoryStore {
        list_card: Arc::new(Mutex::new(Vec::new())),
    }
}

// Implement the Storage trait for MemoryStore
impl Storage for MemoryStore {
    // Save function with category uniqueness check
    fn save(&self, cards: Vec<CardKanban>) -> io::Result<()> {
        let mut stored_cards = self.list_card.lock().unwrap();

        // Check for duplicate categories
        let mut categories = HashSet::new();
        for card in &cards {
            if !categories.insert(card.category.clone()) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Duplicate category found: {}", card.category),
                ));
            }
        }

        // Save cards if no duplicates
        *stored_cards = cards;
        Ok(())
    }

    fn load(&self) -> io::Result<Vec<CardKanban>> {
        let stored_cards = self.list_card.lock().unwrap();
        Ok(stored_cards.clone())
    }

}
impl PartialEq for MemoryStore {
    fn eq(&self, other: &Self) -> bool {
        let self_cards = self.list_card.lock().unwrap();
        let other_cards = other.list_card.lock().unwrap();
        *self_cards == *other_cards
    }
}
#[cfg(test)]
mod tests {
    use yew::Callback;
    use super::*;
    use crate::domain_core::create_card_kanban_item::create_kanban_item;

    #[test]
    fn it_should_save_and_load_cards() {
        // Create a memory store
        let store = create_memory_store();

        // Create some KanbanItems
        let item1 = create_kanban_item("carotte", 10);
        let item2 = create_kanban_item("cucumber", 5);

        // Create some CardKanban
        let card1 = CardKanban {
            category: "vegetable".to_string(),
            items: vec![item1.clone()],
            on_delete: Callback::default(),
            on_delete_item: Default::default(),
        };

        let card2 = CardKanban {
            category: "fruit".to_string(),
            items: vec![item2.clone()],
            on_delete: Callback::default(),
            on_delete_item: Default::default(),
        };

        // Save cards
        assert!(store.save(vec![card1.clone(), card2.clone()]).is_ok());
        // Load cards
        let loaded_cards = store.load().unwrap();
        assert_eq!(loaded_cards.len(), 2);
        assert_eq!(loaded_cards[0].category, "vegetable");
        assert_eq!(loaded_cards[1].category, "fruit");
    }

    #[test]
    fn it_should_not_allow_duplicate_categories() {
        // Create a memory store
        let store = create_memory_store();

        // Create a KanbanItem
        let item1 = create_kanban_item("carotte", 10);

        // Create two CardKanban with the same category
        let card1 = CardKanban {
            category: "vegetable".to_string(),
            items: vec![item1.clone()],
            on_delete: Callback::default(),
            on_delete_item: Default::default(),
        };

        let card2 = CardKanban {
            category: "vegetable".to_string(),
            items: vec![item1.clone()],
            on_delete: Callback::default(),
            on_delete_item: Default::default(),
        };

        // Attempt to save cards with duplicate categories
        assert!(store.save(vec![card1.clone(), card2.clone()]).is_err());
    }
}
