use crate::layouts::dashboard::Dashboard;
use crate::pages::{ArticlesPage, NotFound};
use leptos::prelude::*;
use crate::components::WelcomeCard;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFound>
                <ParentRoute path=path!("/") view=Dashboard>
                    <Route path=path!("/") view=WelcomeCard />
                    <Route path=path!("/articles") view=ArticlesPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
