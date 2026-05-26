mod components;
mod configs;
mod layouts;
mod pages;
mod utils;

use configs::app::App;

fn main() {
    leptos::mount::mount_to_body(App);
}
