use crate::dom::Node;

pub fn node_tree(node: &Node) -> String {
    let mut new_str = String::new();

    _node_tree(node, &mut new_str, 0);

    new_str
}

// Get pretty printed structure of html
// TODO: Not very memory efficient, fix this
fn _node_tree(node: &Node, string: &mut String, _space: usize) {
    match node {
        Node::Element(ed) => {
            let root = ed;
            string.push_str(&format!("{}{}\n", "  ".repeat(_space), root.tag_name));

            for element in &root.children {
                _node_tree(element, string, _space + 1);
            }
        }
        Node::Text(text) => {
            string.push_str(&format!("{}\"{}\"\n", "  ".repeat(_space), text));
        }
        Node::Comment(comm) => {
            string.push_str(&format!("{}//{}\n", "  ".repeat(_space), comm));
        }
    };
}