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
        CardKanban {name: p1.to_string(), category: p0.category }
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
    #[test]
    fn it_should_create_a_card_kanban_with_category_name_and_name() {
        let mut card_kanban = create_card_kanban();
        card_kanban = add_category(card_kanban, "legume");
        card_kanban = add_name(card_kanban, "carotte");

        assert_eq!(card_kanban, CardKanban { name: "carotte".to_string(), category: "legume".to_string() });
    }
    #[test]
    fn it_should_create_a_list_of_card_kanban() {
        let mut card_kanban_carotte = create_card_kanban();
        let mut card_kanban_concomber = create_card_kanban();

        card_kanban_carotte = add_category(card_kanban_carotte, "legume");
        card_kanban_carotte = add_name(card_kanban_carotte, "carotte");
        card_kanban_concomber = add_category(card_kanban_concomber, "legume");
        card_kanban_concomber = add_name(card_kanban_concomber, "concomber");

        let mut cards_kanban = Vec::new();
        cards_kanban.push(card_kanban_carotte);
        cards_kanban.push(card_kanban_concomber);

        assert_eq!(cards_kanban.len(), 2);
    }
    #[test]
    fn it_should_create_a_list_of_card_kanban_with_no_doublon() {
        let mut card_kanban_carotte = create_card_kanban();
        let mut card_kanban_concomber = create_card_kanban();

        card_kanban_carotte = add_category(card_kanban_carotte, "legume");
        card_kanban_carotte = add_name(card_kanban_carotte, "carotte");
        card_kanban_concomber = add_category(card_kanban_concomber, "legume");
        card_kanban_concomber = add_name(card_kanban_concomber, "carotte");

        let mut cards_kanban = Vec::new();
        cards_kanban=add_card_kanban_to_list_without_duplicate(card_kanban_carotte, cards_kanban);
        cards_kanban=add_card_kanban_to_list_without_duplicate(card_kanban_concomber, cards_kanban);


        assert_eq!(cards_kanban.len(), 1);
    }

    fn add_card_kanban_to_list_without_duplicate(card_kanban: CardKanban, mut list_Kanban: Vec<CardKanban>) -> Vec<CardKanban> {
        for x in &list_Kanban {
            if x.name == card_kanban.name && x.category == card_kanban.category {
                // Return  list if duplicate is found
                return list_Kanban;
            }
        }
        list_Kanban.push(card_kanban);
        list_Kanban
    }
}
