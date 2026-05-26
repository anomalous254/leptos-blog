use crate::components::markdown::Markdown;
use crate::layouts::dashboard::Dashboard;
use crate::pages::home::HomePage;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <div class="page-not-found"> <p>"Page not found"</p></div> }>
                <ParentRoute path=path!("/") view=Dashboard>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/about") view=Markdown />
                    <Route path=path!("/blog/:slug") view=Markdown />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
