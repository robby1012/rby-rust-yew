use yew::prelude::{Html, Properties, html, function_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default(false)]
    pub open_in_tab: bool,
    #[prop_or_default("")]
    pub link: &'static str,
    #[prop_or_default("")]
    pub class: &'static str,
    pub children: Html,
}

#[function_component]
pub fn TextLink(props: &Props) -> Html {
    let classes: &'static str = match props.class.is_empty() {
        true => "underline text-blue-700 dark:text-blue-400", 
        false => props.class,
    };

    if props.open_in_tab {
        return html! {
            <a
                href={props.link}
                target="_blank"
                rel="noopener noreferrer"
                class={classes}
            >
                {props.children.clone()}
            </a>           
        }
    }

    html! {
        <a
            href={props.link}
            class={classes}
        >
            {props.children.clone()}
        </a>
    }
}
