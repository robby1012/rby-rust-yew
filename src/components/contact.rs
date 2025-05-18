use yew::prelude::*;
use crate::components::svg::{
    flags::*,
    emojis::{ Blushing, Mail },
    undraw::QuickChat,
};
use crate::components::Socials;
use crate::ui::{Title, Card, TextLink};
use crate::AppContext;

#[function_component(Contact)]
pub fn contact() -> Html {

    let app_context = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_title(app_context: AppContext) -> &'static str {
	match app_context.language.current.as_str() {
	    "kr" => "편하게 연락해주세요!",
	    "eng" | "de" | "jp" | _ => "Let's chat",
	}
    }

    fn handle_contact_description(app_context: AppContext) -> Html {
	match app_context.language.current.as_str() {
	    "de" => html!{
		<>
		    <p>{ "Egal ob Du geschäftlich mit mir in Kontakt treten " }
			{ "möchtest oder einfach mit mir abhängen möchtest, ich freue mich mit Dir zu quatschen." }</p>
		    <p>{ "Meine Muttersprache ist Deutsch" } <Indonesia />
			{ ", aber ich kann auch Englisch" } <England />
			{ ", Japanisch" } <Japan />
			{ ", Koreanisch" } <Korea />
			{ " und lerne momentan Polnisch" } <Poland /> { " und Arabisch" } <Egypt />
			{ ". Du kannst mich gerne in einer dieser Sprachen anschreiben." }</p>
		    <p>{ "Am liebsten bevorzuge ich " }
                        <TextLink link="mailto:robby.sitanala@gmail.com">{ "email" }<Mail /></TextLink>
                    </p>
		</>
	    },
	    "jp" => html!{
		<>
		    <p>{ "母国語はドイツ語" } <Indonesia />
			{ "ですが、英語" } <England />
			{ "と日本語" } <Japan />
			{ "と韓国語" } <Korea />
			{ "も喋れます。そして、今ポーランド語" } <Poland /> { "とアラビア語" } <Egypt />
			{ "を勉強しています。" }
			{ "私と仕事をしてみたい、または楽しく会話してみたい、と思った方はどんな言語でも気軽に連絡して下さい。" }</p>
		    <p>
                        <TextLink link="mailto:robby.sitanala@gmail.com">{ "お問い合わせは" }<Mail /></TextLink>
                    </p>
		</>

	    },
	    "kr" => html!{
		<>
		    <p>{ "저와 함께 이야기를 나누고 싶거나, 같이 일해 보고 싶다면 언제든지 연락해주세요. " }
			{ "제 모국어는 독일어" } <Indonesia />
			{ "이지만 저는 영어" } <England />
			{ "와 일본어" } <Japan />
			{ ",  한국어" } <Korea />
			{ "도 가능합니다. 그뿐만 아니라 현재 폴란드어" } <Poland /> { "와 아랍어" } <Egypt />
			{ "도 배우고 있습니다. 어떤 언어로든 저에게 부담없이 연락하주시면 됩니다." }</p>
		    <p>
                        <TextLink link="mailto:robby.sitanala@gmail.com">{ "문의처" }<Mail /></TextLink>
                    </p>
		</>

	    },
	    "eng" | _ => html!{
		<>
		    <p>{ "Whether you are interested to do business with me, " }
			{ " want to chat about some of my content, or just want to hang out, " }
			{ " I am happy to talk to you. " }</p>
		    <p>{ "My mother tongue is Indonesia" } <Indonesia />
			{ ", but I can speak English" } <England />
			{ ", Japanese" } <Japan />
			{ ", Korean" } <Korea />
			{ " as well and I'm currently learning Polish" } <Poland /> { " and Arabic" } <Egypt />
			{ " Feel free to contact me in any of those languages." }</p>
		    <p>{ "My preferred way of contact is via " }
                        <TextLink link="mailto:robby.sitanala@gmail.com">{ "email" }<Mail /></TextLink>
		    </p>
		</>

	    },
	}
    } 


    html!{
	<>
	    <Title id="contact">
                { handle_title(app_context.clone()) } { " " } <Blushing />
            </Title>

	    <Card>
            <div class="flex flex-col gap-10">
                <div class="flex flex-wrap gap-8 items-center justify-center">
                    <QuickChat class="min-w-[200px] max-w-[400px] h-auto max-[300px]:min-w-full" />
                    <div class="flex-1 grow min-w-[300px] [&>p>svg]:h-[1.5rem] [&>p>a>svg]:h-[1.5rem] [&>p>a]:text-blue-700 [&>p>a]:underline [&>p>a]:dark:text-blue-400 flex flex-col gap-4 max-[850px]:min-w-full">
                        { handle_contact_description(app_context) }
                    </div>
                </div>

                <Socials />
            </div>
	    </Card>
        </>
    }
}
