use yew::Callback;
use crate::domain_core::add_name::add_name;
use crate::domain_core::create_card_kanban::{create_card_kanban, create_kanban_item};
use crate::domain_core::card_kanban::{CardKanban, KanbanItem};

pub fn add_category(card: CardKanban, category: &str) -> CardKanban {
    CardKanban {
        category: category.to_string(),
        items: card.items,
        on_delete: card.on_delete,
        on_delete_item: Callback::default(),
        // Preserve existing items
    }
}
pub fn add_name_and_quantity(card: CardKanban, name: &str, quantity_stock: i32) -> CardKanban {
    let mut items = card.items;
    items.push(create_kanban_item(name, quantity_stock));
    CardKanban {
        category: card.category,
        items,
        on_delete: card.on_delete,
        on_delete_item: Callback::default(),

    }
}

#[test]#[test]
fn it_should_create_a_card_kanban_with_category() {
    let kanban_item_carotte = create_kanban_item("carotte", 20);
    let mut card_kanban =create_card_kanban("legume", vec![kanban_item_carotte.clone()]);


    // Create CardKanban
    let card_kanban = add_category(card_kanban, "test");

    assert_eq!(card_kanban.category, "test");

}
#[test]
fn it_should_create_a_card_kanban_with_category_and_items() {
    let card_kanban = create_card_kanban("", vec![]);
    let card_kanban = add_category(card_kanban, "legume");
    let card_kanban = add_name_and_quantity(card_kanban, "carotte", 0);

  assert_eq!(card_kanban.category, "legume");
  assert_eq!(card_kanban.items.len(), 1);
  assert_eq!(card_kanban.items[0].name, "carotte");
  assert_eq!(card_kanban.items[0].quantity_stock, 0);
}
