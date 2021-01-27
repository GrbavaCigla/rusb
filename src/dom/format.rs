use crate::dom::Node;

pub fn node_format(node: &Node) -> String {
    let mut new_str = String::new();

    _node_format(node, &mut new_str, 0);

    new_str
}

// Get pretty printed structure of html
// TODO: Not very memory efficient, fix this
fn _node_format(node: &Node, string: &mut String, _space: usize) {
    match node {
        Node::Element(ed) => {
            let root = ed;

            let mut attrs = String::new();

            for (k, v) in ed.attributes.iter() {
                attrs.push_str(&format!(" {}=\"{}\"", k, v));
            }

            string.push_str(&format!(
                "{}<{}{}>\n",
                "    ".repeat(_space),
                root.tag_name,
                attrs
            ));

            for element in &root.children {
                _node_format(element, string, _space + 1);
            }

            string.push_str(&format!("{}</{}>\n", "    ".repeat(_space), root.tag_name));
        }
        Node::Text(text) => {
            string.push_str(&format!("{}{}\n", "    ".repeat(_space), text));
        }
        Node::Comment(comm) => {
            string.push_str(&format!("{}<!--{}-->\n", "    ".repeat(_space), comm));
        }
    };
}
