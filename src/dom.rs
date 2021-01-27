use std::collections::HashMap;

#[derive(Debug)]
pub enum Node {
    Text(String),
    Element(ElementData),
    Comment(String),
}
#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
    pub children: Vec<Node>,
}

pub fn node_tree(node: &Node) -> String {
    let mut new_str = String::new();

    _node_tree(node, &mut new_str, 0);

    new_str
}

// Get pretty printed structure of html
// TODO: Not very memory efficient, fix this
fn _node_tree(node: &Node, string: &mut String, _spac: usize) {
    match node {
        Node::Element(ed) => {
            let root = ed;
            string.push_str(&format!("{}{}\n", "    ".repeat(_spac), root.tag_name));

            let mut tag_name = ed.tag_name.clone();

            for element in &root.children {
                _node_tree(element, string, _spac + 1);
            }
        }
        _ => {}
    };
}

pub type AttrMap = HashMap<String, String>;
