use crate::domain_core::CardKanban::CardKanban;

pub fn create_card_kanban_with_all_field(
    name: &str,
    category: &str,
    quantity_stock: i32,
) -> CardKanban {
    CardKanban {
        name: name.to_string(),
        category: category.to_string(),
        quantity_stock,
    }
}
#[test]
fn it_should_create_card_kanban_with_all_field() {
    let mut card_kanban_carotte = create_card_kanban_with_all_field("carotte", "legume", 10);

    assert_eq!(card_kanban_carotte.name, "carotte");
    assert_eq!(card_kanban_carotte.category, "legume");
    assert_eq!(card_kanban_carotte.quantity_stock, 10);
}