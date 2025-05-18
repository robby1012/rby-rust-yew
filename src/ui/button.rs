use yew::{function_component, html, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(false)]
    pub is_primary: bool,
    #[prop_or(false)]
    pub is_secondary: bool,
    pub children: Html,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    fn handle_button_classes(props: &Props) -> String {
        let default_classes: String = "bg-white shadow border border-gray-400 rounded-md py-1 px-2 hover:bg-gray-100 transition dark:bg-slate-700 dark:border-slate-600 dark:text-slate-200 dark:hover:bg-slate-600".to_string();
        let primary_classes: String = "bg-slate-700 text-white shadow border border-slate-600 rounded-md py-1 px-2 hover:bg-slate-600 transition dark:bg-indigo-500 dark:border-indigo-400 dark:text-slate-200 dark:hover:bg-indigo-400".to_string();
        let secondary_classes: String = "".to_string();

        if props.is_primary {
            return primary_classes;
        } else if props.is_secondary {
            return secondary_classes;
        } else {
            return default_classes;
        }
    }

    html! {
        <button
            class={handle_button_classes(props)}
        >
            {props.children.clone()}
        </button>
    }
}
