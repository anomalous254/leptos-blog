mod components;
mod layouts;
mod configs;
mod pages;

use configs::app::App;

fn main() {
    leptos::mount::mount_to_body(App);
}

