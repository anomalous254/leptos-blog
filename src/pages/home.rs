use leptos::prelude::*;
use crate::components::ui::Card;


#[component]
pub fn HomePage() -> impl IntoView{
    view! {
        <Card  title="HomePage">

            <h1>"List of blog posts."</h1>

        </Card>
    }
}
