use crate::layouts::dashboard::Dashboard;
use crate::pages::{ArticlesPage, NotFound, ProjectsPage, AboutPage, MarkdownContentPage};
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
                    <Route path=path!("/projects") view=ProjectsPage />
                    <Route path=path!("/about") view=AboutPage />
                    <Route path=path!("/articles/:slug") view=MarkdownContentPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
