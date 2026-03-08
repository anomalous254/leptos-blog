use leptos::prelude::*;
use comrak::{markdown_to_html, Options};


/// Markdown component to convert markdown to html
#[component]
pub fn Markdown() -> impl IntoView {
    let markdown = r#"
# Hello, Markdown!

This is **Markdown** content rendered as HTML.

- Item 1
- Item 2
- Nested Item

```rust
  fn main() -> () {
    println!("Hello rust");
  }
```
"#;

    // Convert markdown to HTML
    let html_content = markdown_to_html(markdown, &Options::default());

    view! { <div class="md" inner_html=html_content></div> }
}
