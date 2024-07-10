use web_sys::wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;
use crate::components::{Card, Modal};
use crate::components::card::CardsProps;
use crate::components::cards::Cards;


use crate::domain_core::create_card_kanban::create_kanban_item;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_fields;
use crate::domain_core::list_kanban::{load_list_kanban, save_list_kanban};
use crate::shell::storage::memory_store::create_memory_store;
use crate::shell::storage::Storage;

mod domain_core;
mod shell;
mod components;


#[function_component]
pub fn App() -> Html {
    let memory_store = create_memory_store();

    let item1 = create_kanban_item("carotte", 10);
    let item2 = create_kanban_item("concombre", 5);
    let card_kanban_carotte = create_card_kanban_with_all_fields("legume", vec![item1.clone(), item2.clone()]);

    let item3 = create_kanban_item("cucumber", 10);
    let card_kanban_concombre = create_card_kanban_with_all_fields("legume1", vec![item3.clone()]);

    let initial_cards = vec![
        card_kanban_carotte,
        card_kanban_concombre,
    ];

    if memory_store.save(initial_cards.clone()).is_ok() {
        memory_store.save(initial_cards).unwrap();
        html! {
            <Cards memory_store={memory_store} />
        }
    } else {
        html! {<p>{ "Failed to save initial data to MemoryStore" }</p>}
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
