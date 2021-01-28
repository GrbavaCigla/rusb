use crate::dom;
use crate::parser;

pub struct Parser {
    parser: parser::Parser,
}

impl Parser {
    pub fn parse(source: String) -> dom::Node {
        let mut parser = Parser {
            parser: parser::Parser {
                pos: 0,
                input: source,
            },
        };

        let mut nodes = parser.parse_nodes();

        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::Node::Element(dom::ElementData {
                children: nodes,
                is_paired: true,
                attributes: dom::AttrMap::new(),
                tag_name: String::from("html"),
            })
        }
    }

    pub fn parse_tag_name(&mut self) -> String {
        self.parser.consume_while(|c| match c {
            'a'..='z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.parser.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.parser.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.parser.consume_while(|c| c != open_quote);
        assert_eq!(self.parser.consume_char(), open_quote);
        return value;
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = dom::AttrMap::new();
        loop {
            self.parser.consume_whitespace();
            if self.parser.next_char() == '>' || self.parser.next_char() == '/' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    pub fn parse_comment(&mut self) -> dom::Node {
        self.parser.consume_sequence("!--");

        let to_return = dom::Node::Comment(self.parser.consume_while(|c| c != '-'));

        self.parser.consume_sequence("-->");

        to_return
    }

    pub fn parse_element(&mut self) -> dom::Node {
        assert_eq!(self.parser.consume_char(), '<');

        if self.parser.starts_with("!--") {
            return self.parse_comment();
        }

        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();

        let mut children = vec![];
        let mut is_paired = true;

        match self.parser.consume_char() {
            '>' => {
                children = self.parse_nodes();

                self.parser.consume_sequence("</");
                assert_eq!(self.parse_tag_name(), tag_name);
                assert_eq!(self.parser.consume_char(), '>');
            }
            '/' => {
                assert_eq!(self.parser.consume_char(), '>');

                is_paired = false;
            }
            _ => assert!(false),
        }

        let elem_data = dom::ElementData {
            attributes: attrs,
            is_paired: is_paired,
            children: children,
            tag_name: tag_name,
        };

        dom::Node::Element(elem_data)
    }

    pub fn parse_text(&mut self) -> dom::Node {
        dom::Node::Text(self.parser.consume_while(|c| c != '<'))
    }

    pub fn parse_node(&mut self) -> dom::Node {
        match self.parser.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.parser.consume_whitespace();
            if self.parser.eof() || self.parser.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let mut pars = Parser::parse(String::from("<tag attrib=\"value\">bla</tag>"));
    }
}
