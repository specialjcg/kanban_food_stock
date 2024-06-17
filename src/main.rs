
mod domain_core;
mod shell;

use yew::prelude::*;
use yew::Renderer;
use crate::domain_core::CardKanban::CardKanban;
use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_field;
#[function_component(Card)]
fn card(props: &CardKanban) -> Html {
    html! {
       <div class="card max-w-xs bg-white bg-opacity-30 backdrop-blur-lg rounded-xl shadow-lg p-6 m-4 transition-transform transform hover:scale-105 hover:shadow-2xl">
    <div class="card-header bg-gradient-to-r from-pink-200 to-blue-200 p-4 rounded-t-xl text-center">
        <h1 class="text-2xl font-bold text-gray-900">{ &props.category }</h1>
    </div>
    <div class="card-content text-center mt-4">
        <p class="text-lg text-gray-700">{ format!( "Name:{}",  props.name) }</p>
        <p class="text-lg text-gray-700">{ format!("Quantity Stock:{}", props.quantity_stock) }</p>
    </div>
</div>

    }
}

#[function_component(Cards)]
fn cards() -> Html {
    use crate::domain_core::create_card_kanban_with_all_field::create_card_kanban_with_all_field;

    let card_kanban_carotte = create_card_kanban_with_all_field("carotte", "legume", 10);

    html! {
        <Card category={card_kanban_carotte.category} quantity_stock={card_kanban_carotte.quantity_stock} name={card_kanban_carotte.name} />
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



