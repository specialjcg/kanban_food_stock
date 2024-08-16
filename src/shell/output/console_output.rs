use crate::domain_core::card_kanban::CardKanban;
use crate::shell::output::Output;

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
        // Serialize card to JSON, skipping the `on_delete` field
        let json_str = serde_json::to_string(card)
            .unwrap_or_else(|err| format!("Serialization failed: {}", err));

        // Store and print the serialized JSON
        self.printed.push(json_str.clone());
        println!("{}", json_str);
    }
}
