use leptos::prelude::*;
use super::ui::FastA;
use icons::{Github};
use leptos_router::components::A;


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
                     <li>
                        <FastA href="/philosophy">"Philosophy"</FastA>
                    </li>
                </ul>

                 <div class="icons_github">
                  <A  href="https://github.com/anomalous254" target="_blank"> <Github /></A>
               </div>
            </nav> 
        </header>
    }
   
}
