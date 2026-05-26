use crate::components::ui::Card;
use crate::components::ui::FastA;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn HomePage() -> impl IntoView {
    // Map of blog title -> Markdown file path
    let mut blog_posts: HashMap<String, &str> = HashMap::new();
    blog_posts.insert(
        "How to create a middleware in Actix-Web".to_string(),
        "/blog/rust",
    );
    blog_posts.insert("Getting started with Leptos".to_string(), "/blog/leptos");
    blog_posts.insert(
        "Deploying Rust apps to Vercel".to_string(),
        "/blog/vercel_deploy.md",
    );

    let blog_posts_vec: Vec<String> = blog_posts.keys().cloned().collect();

    view! {
        <Card title="All Articles">
            <ul class="blog-lists">
                <For
                    each=move || blog_posts_vec.clone()
                    key=|post: &String| post.clone()
                    let(post)
                >
                    <li>
                        <FastA href=blog_posts.get(&post).unwrap().to_string()>
                            {post.clone()}
                        </FastA>
                    </li>
                </For>
            </ul>
        </Card>
    }
}
