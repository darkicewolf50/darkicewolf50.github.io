use crate::components::technologies::{TechDescription, __technology_cat};
use leptos::prelude::*;

pub fn home() -> impl IntoView {
    let temp_vec = vec![TechDescription {
        lang_name: "Rust Language",
        project_site_link: "https://www.rust-lang.org",
        skill_level: 20,
        lang_logo: "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
    }];
    view! {
        <h1>Technology</h1>
        <p>What Technologies I prefer to use.</p>
        <technology_cat category="Languages" tech_vec=temp_vec />
    }
}
