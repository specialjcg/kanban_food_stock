use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ItemStockProps {
    pub quantity_stock: i32,
}

#[function_component(ItemStock)]
pub fn item_stock(props: &ItemStockProps) -> Html {
    html! {
        <div class="flex-none text-sm text-gray-600 mx-4">
            { format!("Stock: {}", props.quantity_stock) }
        </div>
    }
}
