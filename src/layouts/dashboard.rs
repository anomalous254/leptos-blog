use leptos::prelude::*;
use leptos_router::components::Outlet;
use crate::components::header::Header;


#[component]
pub fn Dashboard() -> impl IntoView{
    let tags = vec!["#Rust", "#Go", "#actix-web", "#Python", "#JS","#Leptos" ,"#React"];
    let info = vec!["anomalous254", "nyando.vercel.app"];
    view! {
        <div class="dashboard">
            <Header />
            <main>
                <div class="main_user_info">
                    <div class="info_left">
                        <img src="/assets/img/me.jpg" alt="image" />
                        <div class="my_info">
                            <h2>"Peter Nyando"</h2>
                            <p>"If you don't encrypt, you're unequipped"</p>
                            <ul class="hashtags">
                                {tags
                                    .into_iter()
                                    .map(|tag| view! { <li>{tag}</li> })
                                    .collect::<Vec<_>>()}
                            </ul>

                        </div>
                    </div>
                    <div class="info_right">
                        <ul class="info_links">
                            {info
                                .into_iter()
                                .map(|tag| view! { <li>{tag}</li> })
                                .collect::<Vec<_>>()}
                        </ul>
                    </div>
                </div>
                <Outlet />
            </main>
        </div>
    }
}
