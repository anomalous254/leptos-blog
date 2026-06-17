use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_location};
use leptos::web_sys::MouseEvent;

/// custom link component that navigates when the mouse button is pressed [mousedown]
/// instead of when released [click]
#[component]
pub fn FastA(
    #[prop(into)] href: String, 
    children: Children,
) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();

    let path = href.clone();
    let path_cloned = path.clone();

    // check if current route matches this link
    let is_active = move || location.pathname.get() == path_cloned;

    view! {
        <a
            href=href
            class=move || if is_active() { "nav-link active" } else { "nav-link" }
            on:mousedown=move |ev: MouseEvent| {
                if ev.button() == 0 {
                    ev.prevent_default();
                    navigate(&path, Default::default());
                }
            }
        >
            {children()}
        </a>
    }
}


/// Card component, used to warap other components
#[component]
pub fn Card(children: Children, #[prop(into)] title: String  ) -> impl  IntoView{
    view! {
        <div class="card">
            <div class="card-title">{title}</div>
            <div class="card-content">{children()}</div>
        </div>
    }
}
