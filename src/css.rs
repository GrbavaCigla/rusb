use crate::parser;
use crate::stylesheet;

pub struct Parser {
    parser: parser::Parser,
}

impl Parser {
    pub fn parse_identifier(&mut self) -> String {
        self.parser.consume_while(|c| match c {
            'a'..='z' | '-' | '_' | 'A'..='Z' => true,
            _ => false,
        })
    }

    pub fn is_valid_identifier_char(&self, ch: char) -> bool {
        match ch {
            'a'..='z' | '-' | '_' | 'A'..='Z' => true,
            _ => false,
        }
    }

    // Parse a comma-separated list of selectors.
    fn parse_selectors(&mut self) -> Vec<stylesheet::Selector> {
        let mut selectors = Vec::new();

        loop {
            selectors.push(self.parse_selector());

            self.parser.consume_whitespace();

            match self.parser.next_char() {
                ',' => {
                    self.parser.consume_char();
                    self.parser.consume_whitespace();
                }
                '{' => break,
                _ => panic!(),
            }
        }

        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        return selectors;
    }

    pub fn parse_selector(&mut self) -> stylesheet::Selector {
        let mut selector = stylesheet::Selector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        while !self.parser.eof() {
            match self.parser.next_char() {
                '#' => {
                    self.parser.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.parser.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.parser.consume_char();
                }
                c if self.is_valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }

        selector
    }

    pub fn parse_value(&mut self) -> stylesheet::Value {
        match self.parser.next_char() {
            'a'..='z' => {
                let keyword = self.parser.consume_while(|c| match c {
                    'a' ..= 'z' => true,
                    _ => false
                });

                stylesheet::Value::Keyword(keyword)
            },
            '0' ..= '9' => {
                let amount = self.parser.consume_while(|c| match c {
                    '0' ..= '9' | '.' => true,
                    _ => false
                });

                let amount: f32 = amount.parse().unwrap();

                stylesheet::Value::Length(amount, stylesheet::Unit::Px)
            }
            _ => panic!(),
        }
    }

    pub fn parse_declaration(&mut self) -> (String, stylesheet::Value) {
        let property = self.parser.consume_while(|c| match c {
            'a'..='z' | '-' => true,
            _ => false,
        });

        assert_eq!(self.parser.consume_char(), ':');

        self.parser.consume_whitespace();

        let value = self.parse_value();

        assert_eq!(self.parser.consume_char(), ';');

        (property, value)
    }

    // pub fn parse_declarations(&mut self) -> Vec<stylesheet::Declaration> {
    //     let declarations = Vec::new();

    //     assert_eq!(self.parser.consume_char(), '{');

    //     loop {
    //         self.parser.consume_whitespace();

    //         match self.parser.next_char() {
    //             '}' => break,
    //             _ => panic!()
    //         };
    //     }

    //     declarations
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_selector() {
        let mut pars = Parser {
            parser: parser::Parser {
                input: String::from("p.class"),
                pos: 0,
            },
        };

        pars.parse_selector();
    }

    #[test]
    fn parse_selectors() {
        let mut pars = Parser {
            parser: parser::Parser {
                input: String::from("p.class p.another-class {"),
                pos: 0,
            },
        };

        pars.parse_selector();
    }
}
