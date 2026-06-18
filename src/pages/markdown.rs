use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use comrak::{markdown_to_html, Options};
use markdown_frontmatter::parse;
use serde::Deserialize;

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

    let (meta, content) = parse::<FrontMatter>(markdown_file).unwrap();

    Post {
        meta,
        content: content.to_string(),
    }
}


#[component]
pub fn MarkdownContentPage() -> impl IntoView {
    let params = use_params_map();

    let slug = params.read().get("slug").unwrap_or_default();
    let post = load_post(&slug);
    let html = markdown_to_html(&post.content, &Options::default());

    view! {
        <div class="blog-page">
            <article class="blog-article">
                <div class="article-header">
                    <h1>{post.meta.title}</h1>
                    <p>{post.meta.description}</p>
                    <div>{post.meta.author} " • " {post.meta.date}</div>
                    <img src=post.meta.image alt="cover" />

                </div>

                <div inner_html=html class="blog-content" />

            </article>

        </div>
    }
}
