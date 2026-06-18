mod configs;
mod components;
mod layouts;
mod pages;
mod utils;


use configs::app::App;

pub struct MyBlog;


impl MyBlog{
    pub fn build() {
        leptos::mount::mount_to_body(App);

    }
}
