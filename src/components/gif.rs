use leptos::prelude::*;

#[component]
pub fn GifHeader() -> impl IntoView {
    view! {
        <div class="gif-header">
            <img src="../../assets/img/hermies-gift.gif" alt="Nyando Blog" class="hero-gif" />

            <div class="hero-text">
                <p>"'If you don't encrypt, you're unequipped'"</p>
            </div>

            <img src="../../assets/img/hermies-gift.gif" alt="Nyando Blog" class="hero-gif" />

        </div>
    }
}
