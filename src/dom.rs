use std::collections::HashMap;
pub mod tree;
pub mod format;

#[derive(Debug, Clone)]
pub enum Node {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub is_paired: bool,
    pub attributes: AttrMap,
    pub children: Vec<Node>,
}

impl ElementData {
    fn _get_elements_by_tag_name(&self, tags: &mut Vec<ElementData>, tag_name: &str) {
        if self.tag_name == tag_name {
            tags.push((*self).clone());
        } else {
            for i in self.children.iter() {
                match i {
                    Node::Element(ed) => {
                        ed._get_elements_by_tag_name(tags, tag_name);
                    }
                    _ => continue
                }
            }
        }
    }

    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<ElementData> {
        let mut elements: Vec<ElementData> = Vec::new();

        self._get_elements_by_tag_name(&mut elements, tag_name);

        elements
    }
}

pub type AttrMap = HashMap<String, String>;
