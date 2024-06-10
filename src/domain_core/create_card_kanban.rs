use crate::domain_core::CardKanban::CardKanban;

pub fn create_card_kanban() -> CardKanban {
    CardKanban {
        name: "".to_string(),
        category: "".to_string(),
        quantity_stock: 0,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_core::add_card_kanban_to_list_without_duplicate::add_card_kanban_to_list_without_duplicate;
    use crate::domain_core::add_category::add_category;
    use crate::domain_core::add_name;
    use crate::domain_core::add_name::add_name;

    #[test]
    fn it_should_create_a_card_kanban() {
        let card_kanban = create_card_kanban();
        assert_eq!(
            card_kanban,
            CardKanban {
                name: "".to_string(),
                category: "".to_string(),
                quantity_stock: 0
            }
        );
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
}
