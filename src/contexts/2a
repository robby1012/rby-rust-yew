use std::rc::Rc;
use yew::{UseReducerHandle, Reducible, Hook, use_reducer};
use gloo::console;
use gloo::storage::{
    LocalStorage,
    Storage,
};
use gloo::history::{BrowserHistory, History};
use web_sys::{
    window,
    Url,
    Window,
    Navigator, UrlSearchParams,
};

#[derive(Clone)]
pub enum LanguageAction {
    English,
    German,
    Japanese,
    Korean,
}

impl LanguageAction {
    pub fn as_str(&self) -> &str {
         match self {
            LanguageAction::German => "de",
            LanguageAction::Japanese => "jp",
            LanguageAction::Korean => "kr",
            LanguageAction::English => "en",
        }
    }
}

impl Into<String> for LanguageAction {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl From<String> for LanguageAction {
    fn from(string: String) -> Self {
        match string.as_str() {
            "de" => LanguageAction::German,
            "jp" => LanguageAction::Japanese,
            "kr" => LanguageAction::Korean,
            "en" | _ => LanguageAction::English,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LanguageState {
    pub current: String,
}

pub fn use_language_context() -> impl Hook<Output = UseReducerHandle<LanguageState>> + 'static {
    use_reducer(LanguageState::default)
}

impl LanguageState {
    fn set_url_param(lang: LanguageAction) {
        let lang: String = lang.into();
        let history = BrowserHistory::new();
        let window: Window = window().expect("No Window Object!");
        let href = window.location().href().unwrap(); 
        history.push_with_query(href, serde_json::json!({
            "language": lang,
        })).unwrap();
    }

    fn get_from_url() -> Option<LanguageAction> {
        let href: String = window().expect("No Window Object found!").location().href().unwrap();
        let url: UrlSearchParams = Url::new(&href).unwrap().search_params();
        let lang_param: Option<String> = url.get("language");
        match lang_param {
            Some(param) => Some(LanguageAction::from(param)),
            None => None,
        }
    }

    fn set_storage(lang: LanguageAction) {
        let lang: String = lang.into();
        match LocalStorage::set("language", &lang) {
	    Ok(()) => console::log!(format!("Language set to {}", &lang)),
	    _ => console::error!("Couldn't set LocalStorage. Please turn the feature in your Browser on if possible."),
	};
    }

    fn get_from_storage() -> Option<LanguageAction> {
	let lang: Option<String> = LocalStorage::get("language").ok();
        match lang {
            Some(lang) => Some(LanguageAction::from(lang)),
            None => None,
        }
    }

    fn get_from_browser() -> Option<LanguageAction> {
 	let window: Window = window().expect("No Window Object!");
	let navigator: Navigator = window.navigator();
	let browser_lang: Option<String> = navigator.language();
	let lang: Option<String> = match browser_lang {
	    Some(bl) => {
		match bl.as_str() {
		    "ja" => Some("jp".to_string()),
		    "ko-KR" | "ko-kp" => Some("kr".to_string()),
		    "de" | "de-de" | "de-at" | "de-ch" | "de-li" | "de-lu"  => Some("de".to_string()),
		    "en-US" | "en" => Some("en".to_string()),
                    _ => None,
		}
	    },
	    None => None,
	};

        match lang {
            Some(lang) => Some(LanguageAction::from(lang)),
            None => None,
        }
    }
}

impl Default for LanguageState {
    fn default() -> Self {
        // Hierachy: Url > Storage > Browser
        let lang: LanguageAction = Self::get_from_url().unwrap_or_else(|| {
            Self::get_from_storage().unwrap_or_else(|| {
                Self::get_from_browser().unwrap_or(LanguageAction::English)
            })
        });


        Self::set_url_param(lang.clone());
        Self::set_storage(lang.clone());

	Self { current: lang.into() }
    }
}

impl Reducible for LanguageState {
    type Action = LanguageAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self::set_url_param(action.clone());
        Self::set_storage(action.clone());

	Self { current : action.into() }.into()
    }
}
