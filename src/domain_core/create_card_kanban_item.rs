use yew::Callback;

use crate::domain_core::card_kanban::{CardKanban, KanbanItem};

pub fn create_kanban_item(name: &str, quantity_stock: i32) -> KanbanItem {
    KanbanItem {
        name: name.to_string(),
        quantity_stock,
        on_delete: Callback::default(), // Default callback for testing
    }
}

// Utility function to create a new CardKanban with items


#[cfg(test)]
mod tests {
    use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_fields;
    use super::*;

    // Utility function to create a new KanbanItem


    #[test]
    fn it_should_create_a_list_of_card_kanban() {
        // Create KanbanItems
        let kanban_item_carotte = create_kanban_item("carotte", 20);
        let kanban_item_concomber = create_kanban_item("concomber", 10);

        // Create CardKanban
        let card_kanban_1 = create_card_kanban_with_all_fields("legume", vec![kanban_item_carotte.clone()]);
        let card_kanban_2 = create_card_kanban_with_all_fields("legume", vec![kanban_item_concomber.clone()]);

        // Create a list of CardKanban
        let mut cards_kanban = Vec::new();
        cards_kanban.push(card_kanban_1);
        cards_kanban.push(card_kanban_2);

        // Assertions
        assert_eq!(cards_kanban.len(), 2);

        // Check the first card's category and items
        assert_eq!(cards_kanban[0].category, "legume");
        assert_eq!(cards_kanban[0].items.len(), 1);
        assert_eq!(cards_kanban[0].items[0].name, "carotte");
        assert_eq!(cards_kanban[0].items[0].quantity_stock, 20);

        // Check the second card's category and items
        assert_eq!(cards_kanban[1].category, "legume");
        assert_eq!(cards_kanban[1].items.len(), 1);
        assert_eq!(cards_kanban[1].items[0].name, "concomber");
        assert_eq!(cards_kanban[1].items[0].quantity_stock, 10);
    }

}