// use yew::prelude::*;
// use reqwest::Client;
// use serde_json::to_string;
// use crate::components::card::CardsProps;
// use crate::shell::storage::Storage;
//
// #[function_component(SaveButton)]
// pub fn save_button(props: &CardsProps) -> Html {
//     let onclick = {
//         let memory_store = props.memory_store.clone();
//         Callback::from(move |_| {
//             let memory_store = memory_store.clone();
//             wasm_bindgen_futures::spawn_local(async move {
//                 let cards = memory_store.load().unwrap();
//                 // Log cards to the console
//                 match to_string(&cards) {
//                     Ok(cards_json) => web_sys::console::log_1(&cards_json.into()),
//                     Err(err) => web_sys::console::log_1(&format!("Failed to serialize cards: {}", err).into()),
//                 };
//                 let client = Client::new();
//                 let url = "http://127.0.0.1:8080/save_card";
//                 let res = client.post(url)
//                     .json(&cards)
//                     .send()
//                     .await;
//
//                 match res {
//                     Ok(_) => web_sys::console::log_1(&"Successfully saved cards to database".into()),
//                     Err(_) => web_sys::console::log_1(&"Failed to save cards to database".into()),
//                 }
//             });
//         })
//     };
//
//     html! {
//         <button {onclick}>{ "Save to Database" }</button>
//     }
// }
