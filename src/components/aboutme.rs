use yew::prelude::*;
use crate::components::svg::{
    flags::*,
    emojis::Heart,
    logos::Youtube,
};
use crate::ui::{Card, TextLink, Tooltip};
use crate::AppContext;

#[function_component(Aboutme)]
pub fn aboutme() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    let programming_languages: [Html; 3] = [
        html! {
            <Tooltip value="JavaScript">
                <img
                    src="./assets/images/tech-icons/js.png" alt="js"
                    class="inline-block relative -top-[0.1rem] h-[1.2rem] w-[1.2rem] rounded ml-[0.2rem]"
                />
            </Tooltip>
        },
         html! {
            <Tooltip value="Rust">
                <img
                    src="./assets/images/tech-icons/rust.png" alt="rust"
                    class="inline-block relative -top-[0.1rem] h-[1.2rem] w-[1.2rem] rounded ml-[0.2rem]"
                />
            </Tooltip>
        },               
        html! {
            <Tooltip value="Lisp">
                <img
                    src="./assets/images/tech-icons/lisp.png" alt="lisp"
                    class="inline-block relative -top-[0.1rem] h-[1.2rem] w-[1.2rem] rounded ml-[0.2rem]"
                />
            </Tooltip>
        },
    ];

    let human_languages: [Html; 8] = [
        html! { <Tooltip value="Indonesia"><Indonesia class="h-[1.5rem] w-[1.5rem]" /></Tooltip> },
        html! { <Tooltip value="German"><Germany class="h-[1.5rem] w-[1.5rem]" /></Tooltip> },
        html! { <Tooltip value="English"><England class="h-[1.5rem] w-[1.5rem]" /></Tooltip> },
        html! { <Tooltip value="Japanese"><Japan class="h-[1.5rem] w-[1.5rem]" /></Tooltip> },
        html! { <Tooltip value="Korean"><Korea class="h-[1.5rem] w-[1.5rem]" /></Tooltip> },
        html! { <Tooltip value="Polish"><Poland class="h-[1.5rem] w-[1.5rem]" /></Tooltip> },
        html! { <Tooltip value="Arabic"><Egypt class="h-[1.5rem] w-[1.5rem]" /></Tooltip> },
        html! { <Tooltip value="Ainu"><Ainu class="h-[1.7rem] w-[1.7rem]" /></Tooltip> },
        html! { <Tooltip value="Uchinaaguchi"><Okinawa class="h-[1.7rem] w-[1.7rem]" /></Tooltip> },

    ];

    fn translate_aboutme_content(app_context: AppContext) -> Html {
	match app_context.language.current.as_str() {
	    "de" => html!{
		<>
		    <p>{ "Hallo ich heiße Robby." }</p>
		    <p>{ "Ich bin ein Full-Stack-Ingenieur aus Indonesien" }
			{ " auch Student der Indonesia Open University in Jakarta" } <Indonesia /> { " und habe eine Leidenschaft zum" }
			{ " Programmieren " } <Heart /> { " Haupstächlich arbeite ich mit " }
			<strong> { "JavaScript" } </strong>
			{ " (Node.js & React) und " } <strong> { "Rust" } </strong>
			{ " (Actix & Yew) um coole, " }
			{ "schnelle Webapps zu programmieren." }</p>
		    <p>{ "Hast Du noch Fragen zu Webentwicklung oder zu mir? Dann "}
                        <TextLink link="#contact">{"kontaktiere"}</TextLink>
                        {" mich doch." }
                    </p>
		</>
	    },
	    "jp" => html!{
		<>
		    <p>{ "はじめまして。マウラ・マークと申します。" }</p>
		    <p>{ "フランクフルト" } <Germany /> { "の大学に通って日本学と韓国学を勉強しています。" }
			{ "プログラミングに情熱を注いでいます" } <Heart /> { "普段は速くて素晴らしいウェブアプリを" }
			{ "作る為に " } <strong> { "JavaScript"} </strong> { "(Node.jsやReact)と" }
			<strong> { "Rust" } </strong> { "(ActixやYew)を使っています。" }</p>
		    <p>{ "興味があれば、"}
                        <TextLink link="#contact">{"連絡してください"}</TextLink>
                        {"。"}
                    </p>
		</>
	    },
	    "kr" => html!{
		<>
		    <p>{ "안녕 하세요? 저는 머이라 마크입니다." }</p>
		    <p>{ "프랑크푸르트" } <Germany /> { " 대학에 다니고 일본학과 한국학을 공부합니다. " }
			{ "그리고 프로그래밍에도 푹 빠졌습니다" } <Heart /> { " 평소에 빠르고 좋은 웹앱을 " }
			{ "프로그래밍하기 위해서 " } <strong> { "JavaScript" } </strong> { " (Node.js와 React)와 " }
			<strong> { "Rust" } </strong> { "(Actix와 Yew)를 사용하고 있습니다. " }</p>
		    <p>{ "저와 함께 이야기를 나누고 싶거나, 같이 일해 보고 싶다면 언제든지 "}
                        <TextLink link="#contact">{"연락해주세요"}</TextLink>
                        {"."}
                    </p>
		</>
	    },
	    "eng" | _ => html! {
		<>
		    <p>{ "Hi! My name is Robby" }</p>
		    <p>{ "I'm a Full Stack Engineer From Indonesia" }
			{ " also Indonesia Open University Student in Jakarta" } <Indonesia />
			{ " with a passion for programming" } <Heart />
			{ " I mostly work with " } <strong> { "JavaScript" } </strong>
			{ " (Node.js & React) and " } <strong> { "Rust" } </strong>
			{ " (Actix & Yew) to develop cool and fast webapps." }</p>
		    <p>{ "If you have any web development needs, feel free to "}
                        <TextLink link="#contact">{"contact"}</TextLink>
                        {" or get to know me a little better." }</p>
		</>
	    }
	}
    }

    fn translate_aboutme_title(app_context: AppContext) -> &'static str {
        match app_context.language.current.as_str() {
            "de" => "... und ich bin auch:",
            "jp" => "…そして私は",
            "kr" => "...그리고 저는",
            "en" | _ => "... and I'm also:",
        }
    }

    html!{
        <Card>
            <div class="flex gap-4 justify-between items-center max-[650px]:flex-col max-[650px]:gap-8">
                <section class="flex flex-col gap-4 justify-center">
                    <h3 class="font-bold text-xl">{translate_aboutme_title(app_context.clone())}</h3>
                    <ul class="pl-4 list-disc flex flex-col gap-4 items-between [&>li>a>svg]:w-[1.8rem] [&>li>a>svg]:h-[1.8rem] [&>li>a>svg]:relative [&>li>a>svg]:-top-[0.1rem] [&>li>a>svg]:dark:fill-white [&>li>a>svg]:dark:stroke-white">
                        <li>{"I use arch and emacs, btw"}</li>
                    </ul>
                </section>

                <img
                    src="./assets/images/me.jpg"
                    alt="robby"
                    class="max-h-[250px] shadow-md rounded-full"
                />
            </div>

            // Seperator
            <div class="p-14">
                <div class="h-[2px] rounded bg-gray-200 dark:bg-slate-700" />
            </div>

            <div class="flex flex-col gap-4 [&>p>svg]:w-[1.5rem] [&>p>svg]:h-[1.5rem]">
                { translate_aboutme_content(app_context) }
            </div>
        </Card>
    }
}
