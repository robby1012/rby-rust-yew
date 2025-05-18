use yew::prelude::*;

use crate::components::svg::emojis::Nerd;
use crate::ui::{Title, Card};
use crate::AppContext;

struct Skill {
    pub lang: String,
    pub img_path: String,
}

#[function_component(Skills)]
pub fn skills() -> Html {
    let app_context = use_context::<AppContext>().expect("No AppContext not found!");

    let skills: Vec<Skill> = vec![
        Skill {
            lang: "JavaScript".into(),
            img_path: "./assets/images/tech-icons/js.png".into(),
        },
        Skill {
            lang: "TypeScript".into(),
            img_path: "./assets/images/tech-icons/ts.png".into(),
        },
        Skill {
            lang: "Node".into(),
            img_path: "./assets/images/tech-icons/node.png".into(),
        },
        Skill {
            lang: "React".into(),
            img_path: "./assets/images/tech-icons/react.png".into(),
        },
        Skill {
            lang: "Vue".into(),
            img_path: "./assets/images/tech-icons/vue.png".into(),
        },
        Skill {
            lang: "Sass".into(),
            img_path: "./assets/images/tech-icons/sass.png".into(),
        },
        Skill {
            lang: "Tailwind".into(),
            img_path: "./assets/images/tech-icons/tailwind.png".into(),
        },
        Skill {
            lang: "Flutter".into(),
            img_path: "./assets/images/tech-icons/flutter.png".into(),
        },
        Skill {
            lang: "Dart".into(),
            img_path: "./assets/images/tech-icons/dart.png".into(),
        },
        Skill {
            lang: "Rust".into(),
            img_path: "./assets/images/tech-icons/rust.png".into(),
        },
        Skill {
            lang: "Linux".into(),
            img_path: "./assets/images/tech-icons/linux.png".into(),
        },
        Skill {
            lang: "Git".into(),
            img_path: "./assets/images/tech-icons/git.png".into(),
        },
        Skill {
            lang: "Docker".into(),
            img_path: "./assets/images/tech-icons/docker.png".into(),
        },
        Skill {
            lang: "MySql".into(),
            img_path: "./assets/images/tech-icons/mysql.png".into(),
        },
        Skill {
            lang: "MongoDB".into(),
            img_path: "./assets/images/tech-icons/mongo.png".into(),
        },
    ];

    fn handle_title(app_context: AppContext) -> &'static str {
	match app_context.language.current.as_str() {
	    "de" => "Meine Skills",
	    "jp" => "スキル",
	    "kr" => "저의 스킬",
	    "eng" | _ => "My Skills",
	}
    }

    html!{
	<>
            <Title>
                { handle_title(app_context) } { " " } <Nerd />
            </Title>

            <Card>
                <div class="flex flex-wrap gap-12 items-center justify-center">
                    {
                        skills.iter().map(|skill: &Skill| html! {
                            <div class="relative flex justify-center [&>span]:hover:block">
                                <img
                                    class="h-[5rem] rounded-lg opacity-85 max-[615px]:h-[3rem] max-[255px]:h-[2rem]"
                                    src={skill.img_path.clone()}
                                    alt={skill.lang.clone()}
                                    loading="lazy"
                                />
                                <span class="hidden animate-fadein absolute top-[4rem] max-[615px]:top-[2.5rem] max-[255px]:top-[1.5rem] py-1 px-2 bg-black/50 backdrop-blur-sm text-white rounded-lg select-none">
                                    { skill.lang.clone() }
                                </span>
                            </div>
                        }).collect::<Html>()
                    }
                </div>
            </Card>
        </>
    }
}
