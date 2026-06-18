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
         Article { 
        title: "Making SPA Navigation Faster with mousedown Instead of click", 
        path: "/articles/fast-spa-navigation-with-mousedown" 
    },
        Article { title: "Simple REST API using actix-web", path: "/articles/actix-web-rest-api" },
        Article { title: "Custom Middleware in Djandgo", path: "/articles/custom-django-middleware" },
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
