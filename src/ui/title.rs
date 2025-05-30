use yew::{function_component, html, Properties, Html, AttrValue};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub id: AttrValue,
}

#[function_component(Title)]
pub fn title(props: &Props) -> Html {
    html! {
        <h3
            id={props.id.clone()}
            class="text-3xl pt-4 pb-8 font-bold flex gap-2 justify-center items-center [&>svg]:w-[2rem] [&>svg]:h-[2rem]"
        >
            { props.children.clone() } 
        </h3>
    }
}
