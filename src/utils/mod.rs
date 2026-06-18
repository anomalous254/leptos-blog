pub fn match_markdown_content(slug: &str) -> &str {
    match slug{ 
        "api" => include_str!("../../contents/actix-web-rest-api.md"),
        "django-middleware" => include_str!("../../contents/custom-middleware-in-django.md"),
        _ => include_str!("../../contents/leptos.md")
    }

}
