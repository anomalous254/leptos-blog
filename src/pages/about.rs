use crate::components::ui::Card;
use crate::components::ui::FastA;
use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Card title="About Me">

            <div class="back-link">
                <FastA href="/">"← Go Back"</FastA>
            </div>

            <div class="about-content">

                <h2>"Hi, I'm a Software Developer"</h2>

                <p>
                    "I build fast, scalable, and modern applications using Rust and other backend technologies."
                </p>

                <p>
                    "My focus is on backend systems, APIs, and developer tools that are simple, reliable, and efficient."
                </p>

                <p>
                    "I enjoy working with async systems, clean architecture, and building tools that solve real problems."
                </p>

                <p>"Currently exploring Rust, Leptos, Django, and distributed systems."</p>

                <div class="about-contact">

                    <p>
                        <strong>"Email:"</strong>
                        " nyandopeter2@gmail.com"
                    </p>

                    <p>
                        <strong>"Phone:"</strong>
                        "..."
                    </p>

                </div>

                <div class="about-links">

                    <FastA href="https://github.com/anomalous254">"GitHub"</FastA>

                    <FastA href="https://www.linkedin.com">"LinkedIn"</FastA>

                    <FastA href="/projects">"View Projects"</FastA>

                </div>

            </div>

        </Card>
    }
}
