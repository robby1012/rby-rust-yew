use yew::prelude::{Html, UseStateHandle, UseReducerHandle, Callback, MouseEvent, html, use_context, use_state, use_effect_with, function_component};
use gloo::events::EventListener;
use crate::ui::{Button, TextLink};
use crate::AppContext;
use crate::contexts::language::{LanguageState, LanguageAction};
use crate::contexts::theme::ThemeAction;
use crate::components::svg::{
    flags::{ Globe, Germany, England, Japan, Korea },
    themes::{ Light, Dark },
};

#[function_component]
pub fn Nav() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");
    let show_lang_dropdown: UseStateHandle<bool> = use_state(|| false);

    let cycle_theme = {
	let app_context = app_context.clone();
	let current_theme: &str = app_context.theme.current;
	let current_theme_index: usize = match app_context.theme_cycle.iter().position(|x: &&str| x == &current_theme) {
	    Some(i) => i,
	    None => 0,
	};
	let next_theme: &str = match app_context.theme_cycle.iter().nth(current_theme_index + 1) {
	    Some(nt) => nt,
	    None => "light",
	};
	Callback::from(move|_| {
	    match next_theme {
		"dark" => app_context.theme.dispatch(ThemeAction::Dark),
		"light" | _ => app_context.theme.dispatch(ThemeAction::Light)
	    }
	})
    };

    fn handle_theme_icon(app_context: AppContext) -> Html {
	match app_context.theme.current {
	    "dark" => html!{<Dark class={Some("h-[1.5rem] w-[1.5rem] fill-slate-300")} />},
	    "light" | _ => html!{<Light class={Some("h-[1.5rem] w-[1.5rem] fill-orange-400")} />}
	}
    }

    let toggle_lang_dropdown = {
	let show_lang_dropdown: UseStateHandle<bool> = show_lang_dropdown.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            show_lang_dropdown.set(!*show_lang_dropdown);
        })
    };

    {
        let show_lang_dropdown = show_lang_dropdown.clone();
        use_effect_with(
            show_lang_dropdown.clone(),
            move |_| {
                let window = gloo::utils::window();
                let callback = Callback::from(move |_| {
                    if *show_lang_dropdown {
                        show_lang_dropdown.set(false);
                    }
                });
                let listener = EventListener::new(
                    &window,
                    "click",
                    move |e| callback.emit(e.clone()),
                );

                || drop(listener)
            }
        );
    }

    fn change_language(next_lang: &'static str, lang_state: UseReducerHandle<LanguageState>) -> Callback<MouseEvent> {
    	Callback::from(move |_| {
	    match next_lang {
		"eng" => lang_state.dispatch(LanguageAction::English),
		"de" => lang_state.dispatch(LanguageAction::German),
		"jp" => lang_state.dispatch(LanguageAction::Japanese),
		"kr" => lang_state.dispatch(LanguageAction::Korean),
		_ => lang_state.dispatch(LanguageAction::English),
	    }
	})
    }

    fn translate_projects(app_context: AppContext) -> &'static str {
	match app_context.language.current.as_str() {
	    "eng" => "Projects",
	    "de" => "Projekte",
	    "jp" => "制作",
	    "kr" => "제작물",
	    _ => "Projects"
	}
    }

    fn translate_contact(app_context: AppContext) -> &'static str {
	match app_context.language.current.as_str() {
	    "eng" => "Get in touch",
	    "de" => "Kontakt",
	    "jp" => "お問い合わせ",
	    "kr" => "문의처",
	    _ => "Get in touch"
	}
    }

    html!{
	<header class="flex flex-wrap gap-2 justify-between items-center select-none">
	    /* Logo */
	    <TextLink
                link="/"
                class="flex gap-6 items-center"
            >
                <span class={"font-Handwriting text-3xl"}>{ "Robby Sitanala" }</span>
	    </TextLink>

	    /* Navigation */
	    <nav>
		<ul class="flex flex-wrap gap-4 items-center [&>li]:cursor-pointer">
	            /* Theme Switcher */
                    <li onclick={ cycle_theme }>
                        { handle_theme_icon(app_context.clone()) }
                    </li>

	            /* Language Dropdown-Menu */
                    <li
                        class="flex gap-1 items-center justify-center relative z-10"
                        onclick={ toggle_lang_dropdown }
                    >
                        <Globe class="w-[1.5rem] h-[1.5rem] fill-slate-700" />
                        <span>{ &app_context.language.current }</span>
			<ul
                            class="absolute p-1 flex flex-col gap-2 bg-white border border-gray-400 shadow rounded-md top-10 [&>li]:flex [&>li]:gap-2 [&>li]:items-center [&>li]:p-1 [&>li]:rounded-md [&>li:hover]:bg-gray-100 [&>li:hover]:transition [&>li>svg]:w-[1.8rem] [&>li>svg]:h-[1.8rem] dark:bg-slate-800 dark:border-slate-700 [&>li:hover]:dark:bg-slate-700"
                            style={ if *show_lang_dropdown { "display:flex;" } else { "display:none;" } }
                        >
                            <li onclick={ change_language("eng", app_context.language.clone()) }>
                                <England />{ "English" }
                            </li>
                            <li onclick={ change_language("de", app_context.language.clone()) }>
                                <Germany />{ "Deutsch" }
                            </li>
                            <li onclick={ change_language("jp", app_context.language.clone()) }>
                                <Japan />{ "日本語" }
                            </li>
                            <li onclick={ change_language("kr", app_context.language.clone()) }>
                                <Korea />{ "한국어" }
                            </li>
			</ul>
		    </li>

	            /* Link to Projects */
                    <li>
                        <a href="#projects">
                            <Button>
                                {translate_projects(app_context.clone())}
                            </Button>
                        </a>
                    </li>

	            /* Link to Contact */
                    <li>
                        <a href="#contact">
                            <Button is_primary={true}>
                                {translate_contact(app_context.clone())}
                            </Button>
                        </a>
                    </li>
		</ul>
	    </nav>
	</header>
    }
}
