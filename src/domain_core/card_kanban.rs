use yew::{Callback, Properties};
use serde::{Deserialize, Serialize};
//
// #[derive(PartialEq, Debug, Clone,Properties)]
// pub struct CardKanban {
//     pub(crate) name: String,
//     pub(crate) category: String,
//     pub quantity_stock: i32,
//     #[prop_or_default]
//     pub on_delete: Callback<()>,
//
// }

#[derive(PartialEq, Debug, Clone, Properties, Serialize, Deserialize)]
pub struct KanbanItem {
    pub(crate) name: String,
    pub quantity_stock: i32,
    #[serde(skip)] // Skip this field during serialization
    #[prop_or_default]
    pub on_delete: Callback<()>,
}

#[derive(PartialEq, Debug, Clone, Properties, Serialize, Deserialize)]
pub struct CardKanban {
    pub(crate) category: String,
    pub items: Vec<KanbanItem>,
    #[serde(skip)] // Skip this field during serialization
    #[prop_or_default]
    pub on_delete: Callback<()>,
}