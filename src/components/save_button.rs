use reqwest::Client;
use serde_json::to_string;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::card::CardsProps;
use crate::shell::storage::Storage;
use web_sys::console;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;

#[derive(Deserialize)]
struct SupabaseCard {
    category: String,
}

#[derive(Deserialize)]
struct SupabaseItem {
    card_category: String,
}

#[function_component(SaveButton)]
pub fn save_button(props: &CardsProps) -> Html {
    let onclick = {
        let memory_store = props.memory_store.clone();
        Callback::from(move |_| {
            let memory_store = memory_store.clone();
            spawn_local(async move {
                let cards = memory_store.load().unwrap();

                // Log cards to the console
                match to_string(&cards) {
                    Ok(cards_json) => console::log_1(&cards_json.into()),
                    Err(err) => {
                        console::log_1(&format!("Failed to serialize cards: {}", err).into());
                        return;
                    }
                };

                let client = Client::new();
                let supabase_url = "https://vszdcvfviuevtugmgtyw.supabase.co/rest/v1";
                let api_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZzemRjdmZ2aXVldnR1Z21ndHl3Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MjMyODUyMTEsImV4cCI6MjAzODg2MTIxMX0.bkAPJfOw_kM2tEIugjAEejg9R1p8Ryn8hry5rBfZmiw";
                let auth_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InZzemRjdmZ2aXVldnR1Z21ndHl3Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MjMyODUyMTEsImV4cCI6MjAzODg2MTIxMX0.bkAPJfOw_kM2tEIugjAEejg9R1p8Ryn8hry5rBfZmiw";
                let authorization_header = format!("Bearer {}", auth_token);
                // Step 1: Fetch existing categories and card categories from Supabase
                let fetch_cards_url = format!("{}/cards", supabase_url);
                let fetch_items_url = format!("{}/items", supabase_url);

                let supabase_cards_res = client
                    .get(&fetch_cards_url)
                    .header("apikey", api_key)
                    .header("Authorization", &authorization_header)
                    .send()
                    .await;

                let supabase_items_res = client
                    .get(&fetch_items_url)
                    .header("apikey", api_key)
                    .header("Authorization", &authorization_header)
                    .send()
                    .await;
                // Step 1: Delete existing items related to the cards
                // Deserialize the fetched data
                let supabase_cards: Vec<SupabaseCard> = match supabase_cards_res {
                    Ok(res) => match res.json().await {
                        Ok(data) => data,
                        Err(_) => {
                            console::log_1(&"Failed to parse Supabase cards data".into());
                            return;
                        }
                    },
                    Err(err) => {
                        console::log_1(&format!("Failed to fetch Supabase cards: {}", err).into());
                        return;
                    }
                };

                let supabase_items: Vec<SupabaseItem> = match supabase_items_res {
                    Ok(res) => match res.json().await {
                        Ok(data) => data,
                        Err(_) => {
                            console::log_1(&"Failed to parse Supabase items data".into());
                            return;
                        }
                    },
                    Err(err) => {
                        console::log_1(&format!("Failed to fetch Supabase items: {}", err).into());
                        return;
                    }
                };

                // Step 2: Delete existing items related to the fetched categories
                for item in &supabase_items {
                    let encoded_category = utf8_percent_encode(&item.card_category, NON_ALPHANUMERIC).to_string();
                    let delete_items_url = format!(
                        "{}/items?card_category=eq.{}",
                        supabase_url,
                        encoded_category
                    );

                    let delete_items_res = client
                        .delete(&delete_items_url)
                        .header("apikey", api_key)
                        .header("Authorization", &authorization_header)
                        .send()
                        .await;

                    if let Err(err) = delete_items_res {
                        console::log_1(&format!("Failed to delete items for category {}: {}", &item.card_category, err).into());
                        return;
                    }
                }

                // Step 3: Delete the cards themselves using the fetched categories
                for card in &supabase_cards {
                    let encoded_category = utf8_percent_encode(&card.category, NON_ALPHANUMERIC).to_string();
                    let delete_cards_url = format!(
                        "{}/cards?category=eq.{}",
                        supabase_url,
                        encoded_category
                    );

                    let delete_cards_res = client
                        .delete(&delete_cards_url)
                        .header("apikey", api_key)
                        .header("Authorization", &authorization_header)
                        .send()
                        .await;

                    if let Err(err) = delete_cards_res {
                        console::log_1(&format!("Failed to delete card category {}: {}", &card.category, err).into());
                        return;
                    }
                }

                // Step 3: Insert new cards and their items
                for card in &cards {
                    // Insert the card
                    let card_insert_res = client
                        .post(&format!("{}/cards", supabase_url))
                        .header("apikey", api_key)
                        .header("Authorization", &authorization_header)
                        .json(&serde_json::json!({ "category": card.category }))
                        .send()
                        .await;

                    if let Err(err) = card_insert_res {
                        console::log_1(&format!("Failed to insert card: {}", err).into());
                        return;
                    }

                    // Insert items associated with the card
                    for item in &card.items {
                        let item_insert_res = client
                            .post(&format!("{}/items", supabase_url))
                            .header("apikey", api_key)
                            .header("Authorization", &authorization_header)
                            .json(&serde_json::json!({
                                "name": item.name,
                                "quantity_stock": item.quantity_stock,
                                "card_category": card.category
                            }))
                            .send()
                            .await;

                        if let Err(err) = item_insert_res {
                            console::log_1(&format!("Failed to insert item: {}", err).into());
                            return;
                        }
                    }
                }

                console::log_1(&"Successfully saved cards and items to Supabase".into());
            });
        })
    };

    html! {
        <button {onclick}>{ "Save to Supabase" }</button>
    }
}
