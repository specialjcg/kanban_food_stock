use web_sys::{InputEvent, MouseEvent};
use yew::{Callback, function_component, Html, html, Properties, TargetCast, use_state};

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub on_submit: Callback<String>,
    pub on_cancel: Callback<()>,
    pub error_message: Option<String>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let category_name = use_state(|| "".to_string());

    let on_input = {
        let category_name = category_name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                category_name.set(input.value());
            }
        })
    };

    let on_submit = {
        let category_name = (*category_name).clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_submit.emit(category_name.clone());
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_cancel.emit(());
        })
    };

    html! {
        <div class="fixed inset-0 flex items-center justify-center bg-gray-900 bg-opacity-50">
            <div class="bg-white p-6 rounded-lg shadow-lg">
                <h2 class="text-lg font-bold mb-4">{ "Add New Card" }</h2>
                <input
                    type="text"
                    class="border rounded p-2 mb-4 w-full"
                    placeholder="Enter category name"
                    value={(*category_name).clone()}
                    oninput={on_input}
                />
                { if let Some(ref error_message) = props.error_message {
                    html! { <p class="text-red-500 mb-4">{ error_message }</p> }
                } else {
                    html! {}
                }}
                <div class="flex justify-end">
                    <button
                        class="bg-blue-500 text-white px-4 py-2 rounded shadow hover:bg-blue-600 mr-2"
                        onclick={on_submit.clone()}
                    >
                        { "Add" }
                    </button>
                    <button
                        class="bg-gray-500 text-white px-4 py-2 rounded shadow hover:bg-gray-600"
                        onclick={on_cancel.clone()}
                    >
                        { "Cancel" }
                    </button>
                </div>
            </div>
        </div>
    }
}
