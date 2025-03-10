use askama::Template;
use axum::response::Html;


#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate<'a> {
    title: &'a str,
    name: &'a str,
}


pub async fn contact() -> Html<String> {
    Html(
        ContactTemplate {
            title: "Contact",
            name: "contact template",
        }
        .render()
        .expect("error"),
    )
}
