mod CardKanban;
mod add_card_kanban_to_list_without_duplicate;
mod create_card_kanban;
mod add_name;
mod add_category;
mod delete_card_kanban;
mod add_quantity_stock;

#[cfg(test)]
mod tests {
    use crate::{add_name, add_quantity_stock};
    use crate::add_card_kanban_to_list_without_duplicate::add_card_kanban_to_list_without_duplicate;
    use crate::add_category::add_category;
    use crate::add_name::add_name;
    use crate::add_quantity_stock::add_quantity_stock;
    use crate::CardKanban::CardKanban;
    use crate::create_card_kanban::create_card_kanban;
    use crate::delete_card_kanban::delete_card_kanban;

    #[test]
    fn it_should_create_a_card_kanban() {
        let card_kanban = create_card_kanban();
        assert_eq!(card_kanban, CardKanban { name: "".to_string(), category: "".to_string(), quantity_stock: 0 });
    }

    #[test]
    fn it_should_create_a_card_kanban_with_name() {
        let mut card_kanban = create_card_kanban();
        card_kanban = add_name::add_name(card_kanban, "test");
        assert_eq!(card_kanban, CardKanban { name: "test".to_string(), category: "".to_string(), quantity_stock: 0 });
    }

    #[test]
    fn it_should_create_a_card_kanban_with_category() {
        let mut card_kanban = create_card_kanban();
        card_kanban = add_category(card_kanban, "test");
        assert_eq!(card_kanban, CardKanban { name: "".to_string(), category: "test".to_string(), quantity_stock: 0 });
    }

    #[test]
    fn it_should_create_a_card_kanban_with_category_name_and_name() {
        let mut card_kanban = create_card_kanban();
        card_kanban = add_category(card_kanban, "legume");
        card_kanban = add_name(card_kanban, "carotte");

        assert_eq!(card_kanban, CardKanban { name: "carotte".to_string(), category: "legume".to_string(), quantity_stock: 0 });
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
        cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_carotte, cards_kanban);
        cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_concomber, cards_kanban);


        assert_eq!(cards_kanban.len(), 1);
    }

    #[test]
    fn it_should_delete_a_card_kanban_from_a_list_of_card() {
        let mut card_kanban_carotte = create_card_kanban();
        let mut card_kanban_concomber = create_card_kanban();

        card_kanban_carotte = add_category(card_kanban_carotte, "legume");
        card_kanban_carotte = add_name(card_kanban_carotte, "carotte");
        card_kanban_concomber = add_category(card_kanban_concomber, "legume");
        card_kanban_concomber = add_name(card_kanban_concomber, "cucumber");

        let mut cards_kanban = Vec::new();
        cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_carotte.clone(), cards_kanban);
        cards_kanban = add_card_kanban_to_list_without_duplicate(card_kanban_concomber, cards_kanban);
        cards_kanban = delete_card_kanban(
            card_kanban_carotte.clone(),
            cards_kanban
        );


        assert_eq!(cards_kanban.len(), 1);
    }

    #[test]
    fn it_should_card_kanban_have_a_quantity_stock() {
        let mut card_kanban_carotte = create_card_kanban();

        card_kanban_carotte = add_category(card_kanban_carotte, "legume");
        card_kanban_carotte = add_name(card_kanban_carotte, "carotte");
        card_kanban_carotte = add_quantity_stock(card_kanban_carotte, 10);

        assert_eq!(card_kanban_carotte.quantity_stock, 10);
    }
}
