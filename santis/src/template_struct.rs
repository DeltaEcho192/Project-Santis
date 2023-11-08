use askama::Template;

#[derive(Template)]
#[template(path="index.html")]
pub struct RootTemplate<'a> {
    pub cats: Vec<&'a str>,
    pub items: Vec<&'a str>,
    pub sizes: Vec<&'a str>,
    pub status_message: &'a str 
}

#[derive(Template)]
#[template(path="enter_message.html")]
pub struct EnterMessage<'a> {
    pub status_message: &'a str    
}

#[derive(Template)]
#[template(path="list.html")]
pub struct ListTemplate<'a> {
    pub items: Vec<&'a Items>
}

#[derive(Template)]
#[template(path="includes/table_edit.html")]
pub struct TableEditTemplate<'a> {
    pub cats: Vec<&'a str>,
    pub item: &'a Items,
}

pub struct Items {
    pub item_id: String,
    pub item_name: String,
    pub category: String
}
