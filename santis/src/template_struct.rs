use askama::Template;

#[derive(Template)]
#[template(path="index.html")]
pub struct RootTemplate<'a> {
    pub cats: Vec<&'a str>,
    pub items: Vec<&'a str>,
    pub sizes: Vec<&'a str>
}


