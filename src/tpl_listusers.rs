use askama::Template;

#[derive(Template)]
#[template(path = "tpl_listusers.html")]
pub struct ListUsersTemplate {
    pub users: Vec<Vec<String>>,
}
