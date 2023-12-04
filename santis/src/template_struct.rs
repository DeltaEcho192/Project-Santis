use askama::Template;

use crate::datastructs::ItemEdit;
use crate::datastructs::BoxItem;

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
pub struct ListTemplate<> {
    pub items: Vec<ItemEdit>,
    pub boxs: Vec<i64>
}

#[derive(Template)]
#[template(path="boxes.html")]
pub struct BoxesTemplate<> {
    pub items: Vec<BoxItem>
}

#[derive(Template)]
#[template(path="includes/search_template.html")]
pub struct SearchTemplate<> {
    pub items: Vec<ItemEdit>
}

#[derive(Template)]
#[template(path="includes/table_edit.html")]
pub struct TableEditTemplate<'a> {
    pub cats: Vec<&'a str>,
    pub item: &'a ItemEdit,
}

#[derive(Template)]
#[template(path="includes/table_row.html")]
pub struct ItemRowTemplate<'a> {
    pub item: &'a ItemEdit
}

#[derive(Template)]
#[template(path="includes/box_row.html")]
pub struct BoxRowTemplate<'a> {
    pub item: &'a BoxItem 
}
