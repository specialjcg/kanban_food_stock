use log::info;
use web_sys::console;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::domain_core::card_kanban::CardKanban;
use crate::domain_core::create_card_kanban::create_kanban_item;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_fields;
use crate::shell::storage::memory_store::{create_memory_store, MemoryStore};
use crate::shell::storage::Storage;

mod domain_core;
mod shell;

#[derive(Properties, PartialEq, Clone)]
struct CardsProps {
    pub memory_store: MemoryStore,
}

#[function_component(Card)]
fn card(props: &CardKanban) -> Html {
    let edit_mode = use_state(|| false);
    let items = use_state(|| props.items.clone());
    let toggle_edit = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            edit_mode.set(!*edit_mode);
        })
    };
    let on_delete = {
        let on_delete = props.on_delete.clone();
        Callback::from(move |_| on_delete.emit(()))
    };
    let delete_item = {
        let items = items.clone();
        Callback::from(move |item_name: String| {
            items.set(
                (*items).clone().into_iter()
                    .filter(|item| item.name != item_name)
                    .collect()
            );
        })
    };
    html! {
        <div class="card max-w-xs bg-white bg-opacity-30 backdrop-blur-lg rounded-xl shadow-lg p-6 m-4 transition-transform transform hover:scale-105 hover:shadow-2xl">
            <div class="kanban-card-header bg-gradient-to-r from-pink-200 to-blue-200 rounded-xl text-white px-4 py-2 text-lg font-bold flex justify-between items-center">
                <h1 class="text-2xl font-bold">{&props.category}</h1>
                <div class="flex space-x-2">
                    <div class="icon text-xl cursor-pointer" onclick={toggle_edit.clone()}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 3.487a1.125 1.125 0 011.588 0l2.062 2.062a1.125 1.125 0 010 1.588l-9.244 9.244a1.125 1.125 0 01-.488.29l-3.222 1.078a1.125 1.125 0 01-1.414-1.414l1.078-3.222a1.125 1.125 0 01.29-.488l9.244-9.244zM8.25 13.5l2.25 2.25m1.5-1.5L16.5 9.75m-6.75 6.75L9 15.75m-.75.75h-1.5v-1.5"/>
                        </svg>
                    </div>
                    <div class="icon text-xl cursor-pointer" onclick={on_delete.clone()}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </div>
                </div>
            </div>
            <div class="kanban-card-body p-4">
                { for (*items).iter().map(|item| {
                    let delete_item = delete_item.clone();
                    let item_name = item.name.clone();
                    html! {
                    <div class="flex items-center justify-between mb-2">
                        <div class="flex-1 text-base font-bold truncate">{ &item.name }</div>
                        <div class="flex-none text-sm text-gray-600 mx-4">{ format!("Stock: {}", item.quantity_stock) }</div>
                        <div class="flex-none icon text-xl cursor-pointer" onclick={Callback::from(move |_| delete_item.emit(item_name.clone()))}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </div>
                    </div>
                }
                }) }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ModalProps {
    pub on_submit: Callback<String>,
    pub on_cancel: Callback<()>,
    pub error_message: Option<String>,
}

#[function_component(Modal)]
fn modal(props: &ModalProps) -> Html {
    let category_name = use_state(|| "".to_string());

    let on_input = {
        let category_name = category_name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                category_name.set(input.value());
            }
        })
    };

    let on_submit = {
        let category_name = (*category_name).clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_submit.emit(category_name.clone());
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_cancel.emit(());
        })
    };

    html! {
        <div class="fixed inset-0 flex items-center justify-center bg-gray-900 bg-opacity-50">
            <div class="bg-white p-6 rounded-lg shadow-lg">
                <h2 class="text-lg font-bold mb-4">{ "Add New Card" }</h2>
                <input
                    type="text"
                    class="border rounded p-2 mb-4 w-full"
                    placeholder="Enter category name"
                    value={(*category_name).clone()}
                    oninput={on_input}
                />
                { if let Some(ref error_message) = props.error_message {
                    html! { <p class="text-red-500 mb-4">{ error_message }</p> }
                } else {
                    html! {}
                }}
                <div class="flex justify-end">
                    <button
                        class="bg-blue-500 text-white px-4 py-2 rounded shadow hover:bg-blue-600 mr-2"
                        onclick={on_submit.clone()}
                    >
                        { "Add" }
                    </button>
                    <button
                        class="bg-gray-500 text-white px-4 py-2 rounded shadow hover:bg-gray-600"
                        onclick={on_cancel.clone()}
                    >
                        { "Cancel" }
                    </button>
                </div>
            </div>
        </div>
    }
}

#[function_component(Cards)]
fn cards(props: &CardsProps) -> Html {
    let memory_store = props.memory_store.clone();
    let cards = {
        let store = memory_store.clone();
        use_state(move || store.load().unwrap_or_else(|_| Vec::new()))
    };

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
            error_message.set(None); // Clear error message when modal is closed
        })
    };

    let add_card = {
        let cards = cards.clone();
        let close_modal = close_modal.clone();
        let set_error_message = error_message.clone();
        let store = memory_store.clone();

        Callback::from(move |category_name: String| {
            let new_item = create_kanban_item("New Item", 0);
            let new_card = create_card_kanban_with_all_fields(&category_name, vec![new_item.clone()]);

            let mut current_cards = (*cards).clone(); // Access the inner Vec<CardKanban>
            let duplicate_found = {
                let stored_cards = store.list_card.lock().unwrap();
                stored_cards.iter().any(|card| card.category == new_card.category)
            };

            if duplicate_found {
                set_error_message.set(Some(format!("Category '{}' already exists!", category_name)));
            } else {
                current_cards.push(new_card);
                store.save(current_cards.clone()).unwrap();
                cards.set(current_cards);
                set_error_message.set(None);
                close_modal.emit(()); // Close the modal after successful addition
            }
        })
    };

    let delete_card = {
        let cards = cards.clone();
        let store = memory_store.clone();

        Callback::from(move |index: usize| {
            let mut current_cards = (*cards).clone(); // Access the inner Vec<CardKanban>
            if index < current_cards.len() {
                current_cards.remove(index);
                store.save(current_cards.clone()).unwrap();
                cards.set(current_cards);
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
                    for (*cards).iter().enumerate().map(|(index, card)| {
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
