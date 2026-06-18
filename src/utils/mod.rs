pub fn match_markdown_content(slug: &str) -> &str {
    match slug{ 
        "actix-web-rest-api" => include_str!("../../contents/actix-web-rest-api.md"),
        "custom-django-middleware" => include_str!("../../contents/custom-middleware-in-django.md"),
        "fast-spa-navigation-with-mousedown" => include_str!("../../contents/fast-spa-navigation-with-mousedown.md"),
        _ => include_str!("../../contents/leptos.md")
    }

}
