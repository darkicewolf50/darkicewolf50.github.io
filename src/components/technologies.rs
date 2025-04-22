use leptos::prelude::*;

#[component]
pub fn technology_cat(
    category: ReadSignal<&'static str>,
    tech_vec: Vec<TechDescription>,
) -> impl IntoView {
    view! {
        <div>
            <div>
                <h3>{category}</h3>
            </div>
            <div>
                {tech_vec
                    .into_iter()
                    .map(move |TechDes| {

                        view! { <h1>{format!("{:#?}", TechDes)}</h1> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn technoolgies_card(
    lang_name: ReadSignal<&'static str>,
    lang_logo: ReadSignal<&'static str>,
    lang_logo_alt: ReadSignal<&'static str>,
    project_site_link: ReadSignal<&'static str>,
    skill_level: ReadSignal<u8>,
) -> impl IntoView {
    view! {
        <a rel="external" href=project_site_link>
            <img src=lang_logo alt=lang_logo_alt />
            <h5>{lang_name}</h5>
            <progress value=skill_level max="100" />
        </a>
    }
}

#[derive(Debug, Clone)]
pub struct TechDescription {
    pub lang_name: &'static str,
    pub lang_logo: &'static str,
    pub project_site_link: &'static str,
    pub skill_level: u8,
}

// <technologies_card
//                                 lang_name=TechDescription.lang_name
//                                 lang_logo=TechDescription.lang_logo
//                                 lang_logo_alt=format!("{}'s logo", TechDescription.lang_name)
//                                 project_site_link=TechDescription.project_site_link
//                                 skill_level=TechDescription.skill_level
//                             />
