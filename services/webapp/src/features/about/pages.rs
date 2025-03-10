use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate<'a> {
    title: &'a str,
    name: &'a str,
}


pub async fn about() -> Html<String> {
    Html(
        AboutTemplate {
            title: "About",
            name: "about template",
        }
        .render()
        .expect("error"),
    )
}