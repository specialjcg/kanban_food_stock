use yew::Properties;
use crate::shell::storage::memory_store::MemoryStore;

#[derive(Properties, PartialEq, Clone)]
pub struct CardsProps {
    pub memory_store: MemoryStore,
}