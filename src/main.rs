use yew::{UseReducerHandle, Html, ContextProvider, html, function_component};
use crate::contexts::theme::{ThemeState, use_theme_context};
use crate::contexts::language::{LanguageState, use_language_context};

mod components;
mod contexts;
mod ui;

use components::{
    nav::Nav,
    landing::Landing,
    aboutme::Aboutme,
    skills::Skills,
    contact::Contact,
    footer::Footer,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    theme: UseReducerHandle<ThemeState>,
    language: UseReducerHandle<LanguageState>,
    theme_cycle: Vec<&'static str>,
}

#[function_component]
fn App() -> Html {
    let theme: UseReducerHandle<ThemeState> = use_theme_context();
    let language: UseReducerHandle<LanguageState> = use_language_context();
    let theme_cycle: Vec<&str> = vec!["light", "dark"];

    html!{
        <ContextProvider<AppContext> context={AppContext {
            theme: theme.clone(),
            language: language,
            theme_cycle: theme_cycle
        }}>
            <main class={theme.current}>
                <div class="w-full h-full bg-gray-50 dark:bg-slate-900 text-black dark:text-slate-300 transition">
                    <div class="max-w-[1200px] m-auto p-4">
                        <Nav />
                        <Landing />
                        <Aboutme />
                        <Skills />
                        <Contact />
                        <Footer />
                    </div>
                </div>
            </main>
        </ContextProvider<AppContext>>
}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
