use crate::CardKanban::CardKanban;

pub fn add_card_kanban_to_list_without_duplicate(card_kanban: CardKanban, mut list_Kanban: Vec<CardKanban>) -> Vec<CardKanban> {
    for x in &list_Kanban {
        if x.name == card_kanban.name && x.category == card_kanban.category {
            // Return  list if duplicate is found
            return list_Kanban;
        }
    }
    list_Kanban.push(card_kanban);
    list_Kanban
}
