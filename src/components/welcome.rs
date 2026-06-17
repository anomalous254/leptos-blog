use leptos::prelude::*;
use crate::components::Header;

#[component]
pub fn WelcomeCard() -> impl IntoView {
    view! {
        <section class="welcome-card">

            <Header />

            <div class="welcome-header">
                <p class="welcome-tagline">"you've landed on @nyando blog site"</p>

                <img
                    class="profile-img"
                    src="https://nyando.vercel.app/assets/photo-Tj8_3dZl.jpg"
                    alt="Nyando profile"
                />
            </div>

            <div class="welcome-content">
                <h1>"Software Engineer • Rust Developer"</h1>

                <p class="subtitle">"If you don't encrypt, you're unequipped"</p>

                <a
                    class="download-btn"
                    href="https://www.rust-lang.org"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    "🦀 Proud Rustacean"
                </a>

                <p class="version">
                    "Powered by "
                    <a href="https://leptos.dev" target="_blank" rel="noopener noreferrer">
                        "Leptos v0.8.16"
                    </a>
                </p>
            </div>

        </section>
    }
}
