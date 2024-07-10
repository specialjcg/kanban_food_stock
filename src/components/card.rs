use yew::{Callback, function_component, Html, html, Properties, use_state};
use crate::components::item_modal::ItemModal;
use crate::domain_core::card_kanban::CardKanban;
use crate::domain_core::create_card_kanban::create_kanban_item;
use crate::shell::storage::memory_store::MemoryStore;

#[derive(Properties, PartialEq, Clone)]
pub struct CardsProps {
    pub memory_store: MemoryStore,
}

#[function_component(Card)]
pub fn card(props: &CardKanban) -> Html {
    let edit_mode = use_state(|| false);
    let items = use_state(|| props.items.clone());
    let show_item_modal = use_state(|| false);
    let error_message = use_state(|| None as Option<String>);
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
    // Open item modal
    let open_item_modal = {
        let show_item_modal = show_item_modal.clone();
        Callback::from(move |_| {
            show_item_modal.set(true);
        })
    };

    // Close item modal
    let close_item_modal = {
        let show_item_modal = show_item_modal.clone();
        Callback::from(move |_| {
            show_item_modal.set(false);
        })
    };

    // Add item to the list
    let add_item = {
        let items = items.clone();
        let close_item_modal = close_item_modal.clone();
        let set_error_message = error_message.clone();
        Callback::from(move |item_name: String| {
            if item_name.is_empty() {
                set_error_message.set(Some("Item name cannot be empty".to_string()));
            } else {
                let mut current_items = (*items).clone();
                let new_item = create_kanban_item(&item_name, 0); // Or set initial stock as needed
                current_items.push(new_item);
                items.set(current_items);
                set_error_message.set(None);
                close_item_modal.emit(());
            }
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
         <div class="icon text-xl cursor-pointer" onclick={open_item_modal.clone()}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 5v14m-7-7h14" />
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
        if *show_item_modal {
                <ItemModal
                    on_submit={add_item.clone()}
                    on_cancel={close_item_modal.clone()}
                    error_message={(*error_message).clone()}
                />
            }
        </div>
    }
}