use leptos::prelude::*;
use super::ui::FastA;


/// Header component for navigation between pages
#[component]
pub fn Header() -> impl IntoView{
    view! {
        <header>
            <nav>
                <h3>@nyando</h3>
                <ul>
                    <li>
                        <FastA href="/">"Home"</FastA>
                    </li>
                    <li>
                        <FastA href="/projects">"Projects"</FastA>
                    </li>
                    <li>
                        <FastA href="/about">"About"</FastA>
                    </li>
                    <li>
                        <FastA href="/contacts">"Contacts"</FastA>
                    </li>
                </ul>
            </nav>
        </header>
    }
   
}
