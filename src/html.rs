use crate::dom;

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser {
            input: input,
            pos: 0,
        }
    }

    pub fn parse(source: String) -> dom::Node {
        let mut nodes = Parser {
            pos: 0,
            input: source,
        }
        .parse_nodes();

        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::Node::Element(dom::ElementData {
                children: nodes,
                attributes: dom::AttrMap::new(),
                tag_name: String::from("html"),
            })
        }
    }

    // Return next char and increment pos
    pub fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();

        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));

        self.pos += next_pos;
        cur_char
    }

    // Consume characters until `test` returns false.
    pub fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
        let mut result = String::new();

        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    pub fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | '0' ..= '9' => true,
            _ => false,
        })
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        return value;
    }
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| match c {
            ' ' => true,
            '\n' => true,
            _ => false,
        });
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = dom::AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    pub fn parse_comment(&mut self) -> dom::Node {
        assert_eq!(self.consume_char(), '!');
        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '-');

        let to_return = dom::Node::Comment(self.consume_while(|c| c != '-'));

        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '>');

        to_return
    }

    pub fn parse_element(&mut self) -> dom::Node {
        assert_eq!(self.consume_char(), '<');

        if self.starts_with("!--") {
            return self.parse_comment();
        }

        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        let children = self.parse_nodes();

        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        let elem_data = dom::ElementData {
            attributes: attrs,
            children: children,
            tag_name: tag_name,
        };

        dom::Node::Element(elem_data)
    }

    pub fn parse_text(&mut self) -> dom::Node {
        dom::Node::Text(self.consume_while(|c| c != '<'))
    }

    pub fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    // Read the current character without consuming it.
    pub fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Check if starts with string.
    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // Return true if all input is consumed.
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_char() {
        let pars = Parser::new(String::from("Hello, World"), 3);

        assert_eq!(pars.next_char(), 'l');
    }

    #[test]
    fn consume_while() {
        let mut pars = Parser::new(String::from("Hello, World"), 7);

        pars.consume_while(|c| match c {
            'A'..='Z' | 'a'..='z' | ',' | ' ' => true,
            _ => {
                assert!(false);
                false
            }
        });
    }

    #[test]
    fn consume_char() {
        let mut pars = Parser::new(String::from("Hello, World"), 7);

        assert_eq!(pars.consume_char(), 'W');
        assert_eq!(pars.consume_char(), 'o');
    }

    #[test]
    fn parse_tag_name() {
        let mut pars = Parser::new(String::from("<tag>"), 1);

        assert_eq!(pars.parse_tag_name(), "tag");
    }

    #[test]
    fn parse_node() {
        let mut pars = Parser::new(String::from("<tag attrib=\"value\">bla</tag>"), 0);

        pars.parse_node();
    }
}
