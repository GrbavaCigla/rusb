use std::collections::HashMap;
pub mod tree;
pub mod format;

#[derive(Debug)]
pub enum Node {
    Text(String),
    Element(ElementData),
    Comment(String),
}
#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub is_paired: bool,
    pub attributes: AttrMap,
    pub children: Vec<Node>,
}

pub type AttrMap = HashMap<String, String>;
