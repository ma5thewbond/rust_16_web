use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
pub mod tpl_listusers;
pub struct HtmlTemplate<T>(T);

impl<T> HtmlTemplate<T>
where
    T: Template,
{
    pub fn new(tpl: T) -> HtmlTemplate<T> {
        HtmlTemplate(tpl)
    }
}

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
