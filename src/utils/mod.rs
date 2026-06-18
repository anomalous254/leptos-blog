pub fn match_markdown_content(slug: &str) -> &str {
    match slug{
        "rust" => include_str!("../../contents/leptos.md"),
        "borrowing" => include_str!("../../contents/leptos.md"),
        _ => include_str!("../../contents/leptos.md")
    }

}
