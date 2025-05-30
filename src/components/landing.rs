use yew::prelude::*;

use crate::components::svg::emojis::Handshake;
use crate::AppContext;

#[function_component(Landing)]
pub fn landing() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");
    fn translate_landing_content(app_context: AppContext) -> Html {
	match app_context.language.current.as_str() {
	    "de" => html!{
		<>
		    <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-28">
            { "Hallo, ich bin Robby & " }
            <Handshake class="relative -top-3 w-[3.5rem] h-[3.5rem] animate-handwave origin-[70%_70%]" />
            <br />
            { "Ich bin Softwareentwickler." }
        </h1>
		</>
	    },
	    "jp" => html!{
		<>
		    <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-28">
            { "こんにちは、ロビーです。 & " }
            <Handshake class="relative -top-3 w-[3.5rem] h-[3.5rem] animate-handwave origin-[70%_70%]" />
            <br />
            { "ソフトウェア開発者です" }
        </h1>
		</>
	    },
	    "kr" => html!{
		<>
		    <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-28">
            { "안녕하세요, 저는 로비입니다. & " }
            <Handshake class="relative -top-3 w-[3.5rem] h-[3.5rem] animate-handwave origin-[70%_70%]" />
            <br />
            { "저는 소프트웨어 개발자입니다." }
        </h1>
		</>
	    },
	    "eng" | _ => html! {
		<>
		    <h1 class="text-center text-4xl leading-relaxed font-mono font-bold py-28">
            { "Hi, I'm Robby & " }
            <Handshake class="relative -top-3 w-[3.5rem] h-[3.5rem] animate-handwave origin-[70%_70%]" />
            <br />
            { "I'm a Software Engineer" }</h1>
		</>
	    }
	}
    }
    html!{
        translate_landing_content(app_context)
    }
}
