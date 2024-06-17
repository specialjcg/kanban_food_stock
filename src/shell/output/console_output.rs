use crate::domain_core::CardKanban::CardKanban;
use crate::shell::output::{console_output, Output};

pub struct ConsoleOutput {
    pub printed: Vec<String>,
}

pub fn create_console_output() -> ConsoleOutput {
    ConsoleOutput {
        printed: Vec::new(),
    }
}

impl Output for ConsoleOutput {
    fn print(&mut self, card: &CardKanban) {
        let message = format!("Card: {:?}", card);
        self.printed.push(message.clone());
        println!("{}", message);
    }
}
