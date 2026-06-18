use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use comrak::{markdown_to_html, Options};
use markdown_frontmatter::parse;
use serde::Deserialize;
use js_sys::Date;

use crate::utils::match_markdown_content;


#[derive(Debug, Deserialize, Clone)]
struct FrontMatter {
    title: String,
    image: String,
    date: String,
    author: String,
    description: String,
}


#[derive(Clone)]
struct Post {
    meta: FrontMatter,
    content: String,
}


fn load_post(slug: &str) -> Post {
    let markdown_file = match_markdown_content(slug);

    let (meta, content) = parse::<FrontMatter>(markdown_file)
        .expect("failed to parse frontmatter");

    Post {
        meta,
        content: content.to_string(),
    }
}


#[component]
pub fn MarkdownContentPage() -> impl IntoView {

    let year = Date::new_0().get_full_year();

    let copyright = format!(
        "© Copyright (c) {} @nyando. All Rights Reserved. ",
        year
    );


    let params = use_params_map();


    let slug = params
        .read()
        .get("slug")
        .unwrap_or_default();


    let post = load_post(&slug);


    let options = Options::default();


    let html = markdown_to_html(
        &post.content,
        &options
    );


    // Run highlight.js after the markdown HTML exists
    Effect::new(move |_| {

        let _ = js_sys::eval(
            r#"
            setTimeout(() => {
                if (window.hljs) {
                    hljs.highlightAll();
                }
            }, 50);
            "#
        );

    });



    view! {
        <div class="blog-page">

            <article class="blog-article">

                <div class="article-header">

                    <h1>{post.meta.title}</h1>

                    <p>{post.meta.description}</p>

                    <div>{post.meta.author} " • " {post.meta.date}</div>

                    <img src=post.meta.image alt="cover" />

                </div>

                <div class="blog-content" inner_html=html />

            </article>

            <p class="version">

                {copyright}
                <a href="https://leptos.dev" target="_blank" rel="noopener noreferrer">
                    "powered by Leptos v0.8.16"
                </a>

            </p>

        </div>
    }
}
