use crate::components::ui::Card;
use crate::components::ui::FastA;
use leptos::prelude::*;

struct Article {
    title: &'static str,
    path: &'static str,
}

type Articles = Vec<Article>;

#[component]
pub fn ArticlesPage() -> impl IntoView {
    let articles: Articles = vec![
        Article { title: "Rust Ownership Explained", path: "/articles/ownership" },
        Article { title: "Borrowing in Rust", path: "/articles/borrowing" },
        Article { title: "Rust Lifetimes Guide", path: "/articles/lifetimes" },
        Article { title: "Structs and Enums", path: "/articles/structs-enums" },
        Article { title: "Error Handling in Rust", path: "/articles/error-handling" },
        Article { title: "Rust Traits Deep Dive", path: "/articles/traits" },
        Article { title: "Async Rust Basics", path: "/articles/async" },
        Article { title: "Cargo and Project Structure", path: "/articles/cargo" },
        Article { title: "Macros in Rust", path: "/articles/macros" },
        Article { title: "Smart Pointers Explained", path: "/articles/smart-pointers" },
    ];

    view! {
        <Card title="All Articles">

            // Go back link
            <div class="back-link">
                <FastA href="/">"← Go Back"</FastA>
            </div>

            <ul class="article-list">
                {articles
                    .into_iter()
                    .map(|article| {
                        view! {
                            <li class="article-item">
                                <FastA href=article.path>{article.title}</FastA>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>

        </Card>
    }
}
