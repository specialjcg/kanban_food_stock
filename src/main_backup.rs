use web_sys::{EventTarget, HtmlInputElement};
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::{props, Renderer};

use crate::domain_core::add_card_kanban_to_list_without_duplicate::add_card_kanban_to_list_without_duplicate;
use crate::domain_core::CardKanban::_CardKanban::category;
use crate::domain_core::CardKanban::CardKanban;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_field;

mod domain_core;
mod shell;

#[function_component(Card)]
fn card(props: &CardKanban) -> Html {
    // State for managing the edit mode
    let edit_mode = use_state(|| false);
    // State for managing the quantity stock
    let quantity_stock = use_state(|| props.quantity_stock);
    let name_stock = use_state(|| props.name.clone());
    let category_stock = use_state(|| props.category.clone());

    // Callback for toggling the edit mode
    let toggle_edit = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            edit_mode.set(!*edit_mode);
        })
    };

    // Node reference for the input element
    let input_node_ref = use_node_ref();
    let input_node_ref_category = use_node_ref();
    let input_node_ref_name = use_node_ref();

    // Callback for handling changes in the input element
    let on_change = {
        let input_node_ref = input_node_ref.clone();
        let quantity_stock = quantity_stock.clone();
        Callback::from(move |_: Event| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let value = input.value().parse::<i32>().unwrap_or(0);
                quantity_stock.set(value); // Using the state handle to update the state
            }
        })

    };
    let on_change_category = {
        let input_node_ref_category = input_node_ref_category.clone();
        let category_stock = category_stock.clone();
        Callback::from(move |_: Event| {
            if let Some(input) = input_node_ref_category.cast::<HtmlInputElement>() {
                let value = input.value();
                category_stock.set(value); // Using the state handle to update the state
            }
        })

    };
    let on_change_name = {
        let input_node_ref_name = input_node_ref_name.clone();
        let name_stock = name_stock.clone();
        Callback::from(move |_: Event| {
            if let Some(input) = input_node_ref_name.cast::<HtmlInputElement>() {
                let value = input.value();
                name_stock.set(value); // Using the state handle to update the state
            }
        })

    };
    // Conditional content based on the edit mode
    let content = if *edit_mode {
        html! {
            <div>
                <input type="text" value={props.name.clone()} ref={input_node_ref_name}
                    onchange={on_change_name} class="input input-bordered w-full max-w-xs" />
                <input type="text" value={props.category.clone()} ref={input_node_ref_category}
                    onchange={on_change_category} class="input input-bordered w-full max-w-xs" />
                <input type="number" value={quantity_stock.to_string()} ref={input_node_ref}
                    onchange={on_change} class="input input-bordered w-full max-w-xs" />
                <button onclick={toggle_edit} class="bg-green-500 text-white px-4 py-2 rounded shadow hover:bg-green-600">
                    { "Save" }
                </button>
            </div>
        }
    } else {
        html! {
            <div class="kanban-card-header bg-gradient-to-r from-pink-200 to-blue-200 rounded-xl text-white px-4 py-2 text-lg font-bold flex justify-between items-center">
                <h1 class="text-2xl font-bold">{<String as Clone>::clone(&*category_stock) }</h1>
                <div class="icon text-xl cursor-pointer" onclick={toggle_edit}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 3.487a1.125 1.125 0 011.588 0l2.062 2.062a1.125 1.125 0 010 1.588l-9.244 9.244a1.125 1.125 0 01-.488.29l-3.222 1.078a1.125 1.125 0 01-1.414-1.414l1.078-3.222a1.125 1.125 0 01.29-.488l9.244-9.244zM8.25 13.5l2.25 2.25m1.5-1.5L16.5 9.75m-6.75 6.75L9 15.75m-.75.75h-1.5v-1.5"/>
                    </svg>
                </div>
            </div>
        }
    };

    html! {
        <div class="card max-w-xs bg-white bg-opacity-30 backdrop-blur-lg rounded-xl shadow-lg p-6 m-4 transition-transform transform hover:scale-105 hover:shadow-2xl">
            { content }
            <div class="kanban-card-body p-4">
                <div class="kanban-card-title text-base font-bold mb-2">
                    { format!("Name: {}", *name_stock) }
                </div>
                <div class="kanban-card-description text-sm text-gray-600 mb-4">
                    { format!("Quantity Stock: {}", *quantity_stock) }
                </div>
            </div>
            <div class="kanban-card-footer flex justify-between items-center text-sm">
                <div class="tag bg-gray-200 rounded px-3 py-1 text-gray-700">{"High Priority"}</div>
                <div class="due-date text-gray-500">{"Due: 25th June"}</div>
            </div>
        </div>
    }
}

#[function_component(Cards)]
fn cards() -> Html {
    let initial_cards = vec![
        create_card_kanban_with_all_field("carotte", "legume", 10),
        create_card_kanban_with_all_field("cucumber", "legume", 10),
    ];

    let cards = use_state(|| initial_cards);

    let new_card = create_card_kanban_with_all_field("New Card", "New Category", 0);

    let add_card = {
        let cards = cards.clone();
        Callback::from(move |_| {
            let mut current_cards = (*cards).clone();
            current_cards = add_card_kanban_to_list_without_duplicate(new_card.clone(), current_cards);
            cards.set(current_cards);
        })
    };

    html! {
        <div class="container mx-auto p-4">
            <div class="flex justify-end mb-4">
                <button
                    class="bg-blue-500 text-white px-4 py-2 rounded shadow hover:bg-blue-600"
                    onclick={add_card}
                >
                    { "Add Card" }
                </button>
            </div>
            <div class="flex flex-wrap">
                {
                    for (*cards).iter().map(|card| html! {
                        <Card
                            category={card.category.clone()}
                            quantity_stock={card.quantity_stock}
                            name={card.name.clone()}
                        />
                    })
                }
            </div>
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <Cards />
    }
}

fn main() {
    Renderer::<App>::new().render();
}
