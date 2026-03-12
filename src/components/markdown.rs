use leptos::prelude::*;
use comrak::{markdown_to_html, Options};


/// Markdown component to convert markdown to html
#[component]
pub fn Markdown() -> impl IntoView { 
   let contents = include_str!("../../contents/README.md");
    let options = Options::default();
   
    // Convert markdown to HTML
    let html_content = markdown_to_html(&contents, &options);

    view! { <div class="md" inner_html=html_content></div> }
}
