use leptos::prelude::*;
use crate::components::FastA;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="not-found">
            <img src="/assets/img/crab.png" alt="Crab" class="not-found__image" />

            <h1 class="not-found__code">"404"</h1>

            <div class="not-found__link">
                <FastA href="/">"Go Back Home"</FastA>
            </div>
        </section>
    }
}
