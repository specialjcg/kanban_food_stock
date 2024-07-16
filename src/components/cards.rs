use gloo_timers::callback::Timeout;
use web_sys::window;
use yew::{Callback, function_component, Html, html, use_state};

use crate::components::{Card, Modal};
use crate::components::card::CardsProps;
use crate::domain_core::card_kanban::{CardKanban, KanbanItem};
use crate::domain_core::card_kanban::_CardKanban::items;
use crate::domain_core::create_card_kanban::create_kanban_item;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_fields;
use crate::domain_core::list_kanban::{load_list_kanban, save_list_kanban};

#[function_component(Cards)]
pub fn cards(props: &CardsProps) -> Html {
    // State to hold the list of cards
    let cards = use_state(|| load_list_kanban(&props.memory_store).unwrap_or_default());
    let show_modal = use_state(|| false);
    let show_save_ok = use_state(|| false);
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
        let show_save_ok = show_save_ok.clone();

        Callback::from(move |category_name: String| {
            let show_save_ok = show_save_ok.clone();

            if cards.iter().any(|card| card.category == category_name) {
                set_error_message.set(Some(format!("Category '{}' already exists!", category_name)));
            } else {
                let new_card = create_card_kanban_with_all_fields(&category_name, vec![create_kanban_item("New Item", 0)]);
                let mut new_cards = (*cards).clone();
                new_cards.push(new_card);
                match save_list_kanban(&memory_store, new_cards.clone()) {
                    Ok(_) => {
                        show_save_ok.set(true);
                        Timeout::new(2000, move || show_save_ok.set(false)).forget();

                    },
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
        let set_error_message = error_message.clone();
        let show_save_ok = show_save_ok.clone();

        Callback::from(move |index: usize| {
            let mut new_cards: Vec<CardKanban> = (*cards).clone();
            let show_save_ok = show_save_ok.clone();

            if index < new_cards.len() {
                new_cards.remove(index);
                cards.set(new_cards.clone());
                match save_list_kanban(&memory_store, new_cards.clone()) {
                    Ok(_) => {
                        show_save_ok.set(true);
                        Timeout::new(2000, move || show_save_ok.set(false)).forget();

                    },
                    Err(e) => window().unwrap().alert_with_message(&format!("Failed to save the new state to memory store: {}", e)).unwrap(),
                }
                set_error_message.set(None);
            }
        })
    };

    let delete_item = {
        let cards = cards.clone();
        let memory_store = props.memory_store.clone();
        let show_save_ok = show_save_ok.clone();

        Callback::from(move |(card_index, item_name): (usize, String)| {
            let mut new_cards = (*cards).clone();
            let show_save_ok = show_save_ok.clone();

            if card_index < new_cards.len() {
                let card = &mut new_cards[card_index];
                card.items.retain(|item| item.name != item_name);
                cards.set(new_cards.clone());
                match save_list_kanban(&memory_store, new_cards.clone()) {
                    Ok(_) => {
                        show_save_ok.set(true);
                        Timeout::new(2000, move || show_save_ok.set(false)).forget();

                    },
                    Err(e) => window().unwrap().alert_with_message(&format!("Failed to save the new state to memory store: {}", e)).unwrap(),
                }
            }
        })
    };
    let add_item = {
        let cards = cards.clone();
        let memory_store = props.memory_store.clone();
        let show_save_ok = show_save_ok.clone();

        Callback::from(move |(card_index, item_name): (usize, String)| {
            let mut new_cards = (*cards).clone();
            let show_save_ok = show_save_ok.clone();

            if card_index < new_cards.len() {
                let card = &mut new_cards[card_index];

                // Check if the item already exists
                if !card.items.iter().any(|item| item.name == item_name) {
                    card.items.push(create_kanban_item(&item_name, 0));
                    cards.set(new_cards.clone());

                    match save_list_kanban(&memory_store, new_cards.clone()) {
                        Ok(_) => {
                            show_save_ok.set(true);
                            Timeout::new(2000, move || show_save_ok.set(false)).forget();

                        },
                        Err(e) => window().unwrap().alert_with_message(&format!("Failed to save the new state to memory store: {}", e)).unwrap(),
                    }
                } else {
                    window().unwrap().alert_with_message("Item already exists in the card.").unwrap();
                }
            }
        })
    };
    let update_stock = {
        let cards = cards.clone();
        let memory_store = props.memory_store.clone();
        let show_save_ok = show_save_ok.clone();
        Callback::from(move |(card_index, item_name, new_stock): (usize, String, i32)| {
            let mut new_cards = (*cards).clone();
            if card_index < new_cards.len() {
                let card = &mut new_cards[card_index];
                let show_save_ok = show_save_ok.clone();
                if let Some(item) = card.items.iter_mut().find(|item| item.name == item_name) {
                    item.quantity_stock = new_stock as i32;
                }
                cards.set(new_cards.clone());
                match save_list_kanban(&memory_store, new_cards.clone()) {
                    Ok(_) => {
                        show_save_ok.set(true);
                        Timeout::new(2000, move || show_save_ok.set(false)).forget();

                    },
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
                        let on_delete = delete_card.clone().reform(move |_| index);
                        let on_delete_item = delete_item.clone().reform(move |item_name| (index, item_name));
                        let on_add_item = add_item.clone().reform(move |item_name| (index, item_name));
                        let on_update_stock = update_stock.clone().reform(move |(item_name, new_stock)| (index, item_name, new_stock));

                        html! {
                            <Card
                                key={index}
                                category={card.category.clone()}
                                items={card.items.clone()}
                                on_delete={on_delete}
                                on_delete_item={on_delete_item}
                                on_add_item={on_add_item}
                        on_update_stock={on_update_stock}
                            />
                        }
                    })
                }
            </div>
            if *show_save_ok {

                <div class="text-green-500">{"Successfully saved the new state to memory store."}</div>

        }

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