use yew::{function_component, Html, html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct ItemNameProps {
    pub name: String,
}

#[function_component(ItemName)]
pub fn item_name(props: &ItemNameProps) -> Html {
    html! {
        <div class="flex-1 text-base font-bold truncate">
            { &props.name }
        </div>
    }
}