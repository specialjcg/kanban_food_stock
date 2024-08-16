use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::cards::Cards;
use crate::components::save_button::SaveButton;
use crate::domain_core::card_kanban::{CardKanban, KanbanItem};
use crate::domain_core::create_card_kanban_item::create_kanban_item;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_fields;
use crate::shell::storage::memory_store::create_memory_store;
use crate::shell::storage::Storage;

mod domain_core;
mod shell;
mod components;

#[derive(Deserialize, Debug)]
struct Item {
    id: u32,
    name: String,
    quantity_stock: u32,
    card_category: String,
}

#[derive(Serialize)]
struct LoginPayload<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
    user: User,
}

#[derive(Deserialize, Debug, Clone)]
struct User {
    id: String,
    email: String,
}

#[function_component]
pub fn App() -> Html {
    let list_card = use_state(Vec::<CardKanban>::new);
    let fetch_failed = use_state(|| false);
    let memory_store = create_memory_store();
    let jwt_token = Rc::new(RefCell::new(String::new()));

    // Form input states
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let supabase_url = "https://vszdcvfviuevtugmgtyw.supabase.co/rest/v1/items";
    let api_public_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZzemRjdmZ2aXVldnR1Z21ndHl3Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MjMyODUyMTEsImV4cCI6MjAzODg2MTIxMX0.bkAPJfOw_kM2tEIugjAEejg9R1p8Ryn8hry5rBfZmiw";

    // Handle form input for email
    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            email.set(input);
        })
    };

    // Handle form input for password
    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            password.set(input);
        })
    };

    // Handle login form submission
    let on_login_click = {
        let email = email.clone();
        let password = password.clone();
        let jwt_token = jwt_token.clone();
        let supabase_url = supabase_url.to_string();
        let api_key = api_public_key.to_string();
        let list_card = list_card.clone();
        let fetch_failed = fetch_failed.clone();

        Callback::from(move |_| {
            let email = email.clone();
            let password = password.clone();
            let jwt_token = jwt_token.clone();
            let supabase_url = supabase_url.clone();
            let api_key = api_key.clone();
            let list_card = list_card.clone();
            let fetch_failed = fetch_failed.clone();

            spawn_local(async move {
                let client = Client::new();
                let login_data = LoginPayload {
                    email: &email,
                    password: &password,
                };
                let login_url = "https://vszdcvfviuevtugmgtyw.supabase.co/auth/v1/token?grant_type=password";

                let response = client
                    .post(login_url)
                    .header("apikey", api_key.clone())
                    .json(&login_data)
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            let auth_response: AuthResponse = resp.json().await.unwrap();
                            {
                                *jwt_token.borrow_mut() = auth_response.access_token.clone();
                            }

                            web_sys::console::log_1(&format!("Logged in, JWT: {}", auth_response.access_token).into());

                            // Now use the JWT to fetch data from Supabase
                            let response = client
                                .get(&supabase_url)
                                .header("Authorization", format!("Bearer {}", jwt_token.borrow()))
                                .header("apikey", api_key.clone())
                                .send()
                                .await;

                            match response {
                                Ok(res) => {
                                    match res.json::<Vec<Item>>().await {
                                        Ok(items) => {
                                            web_sys::console::log_1(&format!("Fetched items: {:?}", items).into());

                                            let mut card_map: HashMap<String, Vec<KanbanItem>> = HashMap::new();
                                            for item in items {
                                                let kanban_item = create_kanban_item(&item.name, item.quantity_stock as i32);
                                                card_map.entry(item.card_category).or_default().push(kanban_item);
                                            }

                                            let new_list_card: Vec<CardKanban> = card_map.into_iter()
                                                .map(|(category, items)| create_card_kanban_with_all_fields(&category, items))
                                                .collect();

                                            list_card.set(new_list_card);
                                        }
                                        Err(_) => {
                                            web_sys::console::log_1(&"Failed to parse JSON".into());
                                            fetch_failed.set(true);
                                        }
                                    }
                                }
                                Err(_) => {
                                    web_sys::console::log_1(&"Failed to fetch items from Supabase".into());
                                    fetch_failed.set(true);
                                }
                            }
                        } else {
                            web_sys::console::log_1(&"Login failed".into());
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&format!("Error: {:?}", err).into());
                    }
                }
            });
        })
    };

    let list_card_borrow: Vec<CardKanban> = (*list_card).clone();

    if memory_store.save(list_card_borrow.clone()).is_ok() {
        memory_store.save(list_card_borrow).unwrap();

        html! {
            <>
                <h1>{ "Login" }</h1>
                <form>
                    <label for="email">{ "Email: " }</label>
                    <input type="email" id="email" value={(*email).clone()} oninput={on_email_input} />

                    <label for="password">{ "Password: " }</label>
                    <input type="password" id="password" value={(*password).clone()} oninput={on_password_input} />

                    <button type="button" onclick={on_login_click}>{ "Log In" }</button>
                </form>

                <Cards memory_store={memory_store.clone()} />
                <SaveButton memory_store={memory_store} />
            </>
        }
    } else {
        html! {<p>{ "Failed to save initial data to MemoryStore" }</p>}
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
