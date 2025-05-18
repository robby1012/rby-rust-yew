use yew::prelude::*;
use crate::ui::TextLink;

use crate::AppContext;

#[function_component(Footer)]
pub fn footer() -> Html {

    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_footer_content(app_context: AppContext) -> Html {
	let yew_link: Html = html!{
            <TextLink
                link="https://yew.rs/"
                open_in_tab={true}
                class="underline text-gray-500 dark:text-gray-400"
            >
                { "Yew Framework" }
            </TextLink>
        };
	let wasm_link: Html = html!{
            <TextLink
                link="https://yew.rs/"
                open_in_tab={true}
                class="underline text-gray-500 dark:text-gray-400"
            >
                { "WebAssembly" }
            </TextLink>
        };

	match app_context.language.current.as_str() {
	    "de" => html!{
                <>
		    {  "Diese Website wurde im " } { yew_link } { " programmiert und zu " } { wasm_link } { " kompiliert." }
                </>
	    },
	    "jp" => html!{
		<>
		    {  "このサイトは" } { yew_link } { "で作られて" } { wasm_link } { "にコンパイルされました。" }
                </>
	    },
	    "kr" => html!{
		<>
		    {  "이 사이트는 " } { yew_link } { "에서 제작되어 " } { wasm_link } { "에 편집되었습니다." }
                </>
	    },
	    "eng" | _ => html!{
		<>
		    {  "This website was created with the " } { yew_link } { " and is compiled to " } { wasm_link } { "." }
                </>
	    },
	}
    }

    html!{
	<footer class="text-center text-gray-500 dark:text-gray-400 text-sm">
	    { handle_footer_content(app_context) }
	</footer>
    }

}
