use crate::domain_core::card_kanban::CardKanban;
pub(crate) mod console_output;


pub trait Output {
    fn print(&mut self, card: &CardKanban);
}
