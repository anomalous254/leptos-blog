use leptos::prelude::*;
use leptos_router::components::Outlet;


#[component]
pub fn Dashboard() -> impl IntoView{ 
    view! {
        <div class="dashboard">
            <main class="outlet-container">
                <Outlet />
            </main>
        </div>
    }
}
