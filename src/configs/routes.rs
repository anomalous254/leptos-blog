use leptos_router::components::*;
use leptos_router::path;
use leptos::prelude::*;
use crate::layouts::dashboard::Dashboard;
use crate::pages::{home::HomePage};
use crate::components::markdown::Markdown;



use leptos_router::hooks::use_params;
use leptos_router::params::Params;

/// Define the params structure to match the `:slug` in your route
#[derive(Params, PartialEq)]
struct BlogParams {
    slug: Option<String>,
}

#[component]
pub fn TestSlug() -> impl IntoView {
    // Grab the params from the route
    let params = use_params::<BlogParams>();

    // Create a memo that reads the slug safely
    let slug = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.slug.clone())
            .unwrap_or_else(|| "no-slug".to_string())
    };

    // Display the slug
    view! {
        <div>
            <h1>"Testing slug parameter"</h1>
            <p>"Slug: " {slug}</p>
        </div>
    }
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <div class="page-not-found"> <p>"Page not found"</p></div> }>
                <ParentRoute path=path!("/") view=Dashboard>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/about") view=Markdown />
                    <Route path=path!("/blog/:slug") view=TestSlug />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
