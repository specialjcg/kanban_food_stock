use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq, Clone)]
pub struct ItemStockProps {
    pub quantity_stock: i32,
    pub on_update_stock: Callback<i32>,
}

#[function_component(ItemStock)]
pub fn item_stock(props: &ItemStockProps) -> Html {
    let on_input = {
        let on_update_stock = props.on_update_stock.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target().unwrap().unchecked_into();
            if let Ok(stock) = input.value().parse::<i32>() {
                on_update_stock.emit(stock);
            }
        })
    };

    html! {
        <div class="flex-none text-sm text-gray-600 mx-4">
            <input type="number" min="0" value={props.quantity_stock.to_string()}
                class="w-16 px-2 py-1 border border-gray-300 rounded"
                oninput={on_input} />
        </div>
    }
}
