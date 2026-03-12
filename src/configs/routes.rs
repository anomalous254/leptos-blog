use leptos_router::components::*;
use leptos_router::path;
use leptos::prelude::*;
use crate::layouts::dashboard::Dashboard;
use crate::pages::{home::HomePage};
use crate::components::markdown::Markdown;

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"Not found"</p> }>
                <ParentRoute path=path!("/") view=Dashboard>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/about") view=Markdown />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
