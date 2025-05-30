use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: &'static str,
    pub children: Html,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &Props) -> Html {
    html! {
        <div class="relative inline-flex justify-center items-center [&>span]:hover:block">
            <div>
                { props.children.clone() }
            </div>

            <span class="hidden animate-fadein absolute top-[-1.5rem] py-1 px-2 bg-black/50 backdrop-blur-sm text-white text-sm rounded-lg select-none">
                { &props.value }
            </span>
        </div>
    }
}
