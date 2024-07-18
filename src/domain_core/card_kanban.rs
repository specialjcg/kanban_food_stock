use yew::{Callback, Properties};
use serde::{Deserialize, Serialize};


#[derive( Debug, Clone, Properties, Serialize, Deserialize)]
pub struct KanbanItem {
    pub(crate) name: String,
    pub quantity_stock: i32,
    #[serde(skip)] // Skip this field during serialization
    #[prop_or_default]
    pub on_delete: Callback<()>,
}
// Ensure KanbanItem also implements PartialEq
impl PartialEq for KanbanItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.quantity_stock == other.quantity_stock
    }
}
#[derive(PartialEq, Debug, Clone, Properties, Serialize, Deserialize)]
pub struct CardKanban {
    pub(crate) category: String,
    pub  items: Vec<KanbanItem>,
    #[serde(skip)] // Skip this field during serialization
    #[prop_or_default]
    pub on_delete: Callback<()>,
    #[serde(skip)]
    #[prop_or_default]
    pub on_delete_item: Callback<String>,
}