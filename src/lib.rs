#[cfg(test)]
mod tests {
    #[derive(PartialEq, Debug)]
    struct CardKanban {
        name: String,
        category: String,
    }

    pub fn create_card_kanban() -> CardKanban {
        CardKanban { name: "".to_string(), category: "".to_string() }
    }

    pub fn add_name(p0: CardKanban, p1: &str) -> CardKanban {
        CardKanban {name: p1.to_string(), category: "".to_string() }
    }
    pub fn add_category(p0: CardKanban, p1: &str) -> CardKanban {
        return CardKanban { name: p0.name, category: p1.to_string()}
    }
    #[test]
    fn it_should_create_a_card_kanban() {
        let card_kanban = create_card_kanban();
        assert_eq!(card_kanban, CardKanban { name: "".to_string(), category: "".to_string() });
    }

    #[test]
    fn it_should_create_a_card_kanban_with_name() {
        let mut card_kanban = create_card_kanban();
        card_kanban = add_name(card_kanban, "test");
        assert_eq!(card_kanban, CardKanban { name: "test".to_string(), category: "".to_string() });
    }
    #[test]
    fn it_should_create_a_card_kanban_with_category() {
        let mut card_kanban = create_card_kanban();
        card_kanban = add_category(card_kanban, "test");
        assert_eq!(card_kanban, CardKanban { name: "".to_string(), category: "test".to_string() });
    }


}
