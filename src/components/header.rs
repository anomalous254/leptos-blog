use leptos::prelude::*;
use crate::components::FastA;


#[component]
pub fn Header() -> impl IntoView{
    view! {
        <header class="main-header">

            <nav class="header-nav">
                <FastA href="/articles">"Articles"</FastA>
                <FastA href="/projects">"Projects"</FastA>
                // <FastA href="/philosophy">"Philosophy"</FastA>
                <FastA href="/about">"About"</FastA>

            </nav>

        </header>
    }
}
