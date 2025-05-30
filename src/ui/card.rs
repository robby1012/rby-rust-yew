use yew::{function_component, html, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <div class="py-8">
            <div
                class="p-6 bg-white border border-gray-200 rounded-lg dark:bg-slate-800 dark:border-slate-700"
            >
                { props.children.clone() }
            </div>
        </div>
    }
}
