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
pub fn create_card_kanban(category: &str, items: Vec<KanbanItem>) -> CardKanban {
    CardKanban {
        category: category.to_string(),
        items,
        on_delete: Callback::default(),
        on_delete_item: Callback::default(),

    }
}
// Function to delete an item from a CardKanban
pub fn delete_item_from_card_kanban(mut card_kanban: CardKanban, item_to_delete: KanbanItem) -> CardKanban {
    card_kanban.items.retain(|item| item != &item_to_delete);
    card_kanban
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_core::add_card_kanban_to_list_without_duplicate::add_card_kanban_to_list_without_duplicate;
    use crate::domain_core::add_category::add_category;
    use crate::domain_core::add_name;
    use crate::domain_core::add_name::add_name;
    use crate::domain_core::card_kanban::KanbanItem;

    // Utility function to create a new KanbanItem


    #[test]
    fn it_should_create_a_list_of_card_kanban() {
        // Create KanbanItems
        let kanban_item_carotte = create_kanban_item("carotte", 20);
        let kanban_item_concomber = create_kanban_item("concomber", 10);

        // Create CardKanban
        let card_kanban_1 = create_card_kanban("legume", vec![kanban_item_carotte.clone()]);
        let card_kanban_2 = create_card_kanban("legume", vec![kanban_item_concomber.clone()]);

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
    #[test]
    fn it_should_delete_an_item_from_a_card_kanban() {
        // Create KanbanItems
        let kanban_item_carotte = create_kanban_item("carotte", 20);
        let kanban_item_concomber = create_kanban_item("concomber", 10);

        // Create a CardKanban with two items
        let mut card_kanban = create_card_kanban("legume", vec![kanban_item_carotte.clone(), kanban_item_concomber.clone()]);

        // Assert initial state
        assert_eq!(card_kanban.items.len(), 2);

        // Delete one item
        card_kanban = delete_item_from_card_kanban(card_kanban, kanban_item_carotte.clone());

        // Assert final state
        assert_eq!(card_kanban.items.len(), 1);
        assert_eq!(card_kanban.items[0].name, "concomber");
    }
}