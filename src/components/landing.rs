use yew::prelude::*;

use crate::components::svg::emojis::Handshake;

#[function_component(Landing)]
pub fn landing() -> Html {
    html!{
        <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-28">
            { "Hi! I'm Robby & " }
            <Handshake class="relative -top-3 w-[3.5rem] h-[3.5rem] animate-handwave origin-[70%_70%]" />
            <br />
            { "I'm a full" } <wbr /> { "stack de" } <wbr /> { "velo" } <wbr /> { "per" }
        </h1>
    }
}
