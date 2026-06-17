use leptos::prelude::*;
use crate::components::WelcomeCard;


#[component]
pub fn Dashboard() -> impl IntoView{ 
    view! {
        <div class="dashboard">
            <main class="hero">

                <WelcomeCard />

            </main>
        </div>
    }
}
