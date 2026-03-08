use leptos_router::components::*;
use leptos_router::path;
use leptos::prelude::*;
use crate::layouts::dashboard::Dashboard;
use crate::pages::{home::HomePage};

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"Not found"</p> }>
                <ParentRoute path=path!("/") view=Dashboard>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/about") view=|| view! { <p>"About Page"</p> } />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
