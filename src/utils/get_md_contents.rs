///Reads the markdown contents based on the slug fron the contents dir
pub fn get_markdown_contents(slug: &str) -> &'static str {
    match slug {
        "rust" => include_str!("../../contents/rust.md"),
        "leptos" => include_str!("../../contents/leptos.md"),
        "me" => include_str!("../../contents/me.md"),
        _ => "# 404",
    }
}
