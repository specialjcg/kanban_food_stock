use web_sys::wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;
use crate::components::{Card, Modal};
use crate::components::card::CardsProps;


use crate::domain_core::create_card_kanban::create_kanban_item;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_fields;
use crate::domain_core::list_kanban::{load_list_kanban, save_list_kanban};
use crate::shell::storage::memory_store::create_memory_store;
use crate::shell::storage::Storage;

mod domain_core;
mod shell;
mod components;


#[function_component(Cards)]
fn cards(props: &CardsProps) -> Html {
    // State to hold the list of cards
    let cards = use_state(|| load_list_kanban(&props.memory_store).unwrap_or_default());
    let show_modal = use_state(|| false);
    let error_message = use_state(|| None as Option<String>);

    let open_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| {
            show_modal.set(true);
        })
    };

    let close_modal = {
        let show_modal = show_modal.clone();
        let error_message = error_message.clone();
        Callback::from(move |_| {
            show_modal.set(false);
            error_message.set(None);
        })
    };

    let add_card = {
        let cards = cards.clone();
        let close_modal = close_modal.clone();
        let set_error_message = error_message.clone();
        let memory_store = props.memory_store.clone();
        Callback::from(move |category_name: String| {
            if cards.iter().any(|card| card.category == category_name) {
                set_error_message.set(Some(format!("Category '{}' already exists!", category_name)));
            } else {
                let new_card = create_card_kanban_with_all_fields(&category_name, vec![create_kanban_item("New Item", 0)]);
                let mut new_cards = (*cards).clone();
                new_cards.push(new_card);
                match save_list_kanban(&memory_store, new_cards.clone()) {
                    Ok(_) => window().unwrap().alert_with_message("Successfully saved the new state to memory store.").unwrap(),
                    Err(e) => window().unwrap().alert_with_message(&format!("Failed to save the new state to memory store: {}", e)).unwrap(),
                }
                cards.set(new_cards.clone());
                set_error_message.set(None);
                close_modal.emit(());
            }
        })
    };

    let delete_card = {
        let cards = cards.clone();
        let memory_store = props.memory_store.clone();
        Callback::from(move |index: usize| {
            let mut new_cards = (*cards).clone();
            if index < new_cards.len() {
                new_cards.remove(index);
                cards.set(new_cards.clone());
                match save_list_kanban(&memory_store, new_cards.clone()) {
                    Ok(_) => window().unwrap().alert_with_message("Successfully saved the new state to memory store.").unwrap(),
                    Err(e) => window().unwrap().alert_with_message(&format!("Failed to save the new state to memory store: {}", e)).unwrap(),
                }
            }
        })
    };

    html! {
        <div class="container mx-auto p-4">
            <div class="flex justify-end mb-4">
                <button
                    class="bg-blue-500 text-white px-4 py-2 rounded shadow hover:bg-blue-600"
                    onclick={open_modal}
                >
                    { "Add Card" }
                </button>
            </div>
            <div class="flex flex-wrap">
                {
                    for cards.iter().enumerate().map(|(index, card)| {
                        html! {
                            <Card
                                key={index}
                                category={card.category.clone()}
                                items={card.items.clone()}
                                on_delete={delete_card.clone().reform(move |_| index)}
                            />
                        }
                    })
                }
            </div>
            if *show_modal {
                <Modal
                    on_submit={add_card.clone()}
                    on_cancel={close_modal.clone()}
                    error_message={(*error_message).clone()}
                />
            }
        </div>
    }
}
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
