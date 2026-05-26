use crate::utils::get_md_contents::get_markdown_contents;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use comrak::{Options, markdown_to_html};

#[derive(Params, PartialEq)]
struct BlogParams {
    slug: Option<String>,
}

#[component]
pub fn Markdown() -> impl IntoView {
    let params = use_params::<BlogParams>();

    // Reactive slug getter
    let slug = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.slug.clone())
            .unwrap_or_else(|| "me".to_string())
    };

    // Generate HTML from slug
    let html_content =
        move || markdown_to_html(get_markdown_contents(&slug()), &Options::default());

    view! {
        <div
            class="md"
            inner_html=html_content
        />
    }
}
