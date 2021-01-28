use std::u8;

use crate::parser;
use crate::stylesheet;

pub struct Parser {
    pub parser: parser::Parser,
}

impl Parser {
    pub fn parse(source: String) -> stylesheet::Stylesheet {
        Self {
            parser: parser::Parser{
                pos: 0,
                input: source
            }
        }.parse_stylesheet()
    }

    pub fn parse_stylesheet(&mut self) -> stylesheet::Stylesheet {
        let mut rules = Vec::<stylesheet::Rule>::new();

        while !self.parser.eof() {
            rules.push(self.parse_rule());

            self.parser.consume_whitespace();
        }

        stylesheet::Stylesheet{
            rules: rules
        }
    }

    pub fn parse_rule(&mut self) -> stylesheet::Rule {
        let selectors = self.parse_selectors();
        let declarations = self.parse_declarations();

        stylesheet::Rule{
            selectors: selectors,
            declarations: declarations
        }
    }

    pub fn parse_identifier(&mut self) -> String {
        self.parser.consume_while(|c| match c {
            'a'..='z' | '-' | '_' | 'A'..='Z' | '0' ..= '9' => true,
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
    pub fn parse_selectors(&mut self) -> Vec<stylesheet::Selector> {
        let mut selectors = Vec::new();

        loop {
            self.parser.consume_whitespace();

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

    pub fn parse_length(&mut self) -> stylesheet::Value {
        let amount = self.parser.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });

        let amount: f32 = amount.parse().unwrap();

        let unit = self.parser.consume_while(|c| match c {
            'a'..='z' | '%' => true,
            _ => false,
        });

        let unit = match &unit[..] {
            "%" => stylesheet::Unit::Percent,
            "pt" => stylesheet::Unit::Pt,
            "px" => stylesheet::Unit::Px,
            "wh" => stylesheet::Unit::Wh,
            "vh" => stylesheet::Unit::Vh,
            "em" => stylesheet::Unit::Em,
            "rem" => stylesheet::Unit::Rem,
            _ => panic!(),
        };

        stylesheet::Value::Length(amount, unit)
    }

    pub fn parse_color(&mut self) -> stylesheet::Value {
        self.parser.consume_char();

        let hex = self.parser.consume_while(|c| match c {
            '0'..='9' | 'a'..='f' | 'A'..='F' => true,
            _ => false,
        });

        let r = u8::from_str_radix(&hex[..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..], 16).unwrap();

        stylesheet::Value::Color(stylesheet::Color {
            r: r,
            g: g,
            b: b,
            a: 255,
        })
    }

    pub fn parse_keyword(&mut self) -> stylesheet::Value {
        let keyword = self.parser.consume_while(|c| match c {
            'a'..='z' => true,
            _ => false,
        });

        stylesheet::Value::Keyword(keyword)
    }

    pub fn parse_value(&mut self) -> stylesheet::Value {
        match self.parser.next_char() {
            'a'..='z' => self.parse_keyword(),
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
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

    pub fn parse_declarations(&mut self) -> stylesheet::Declarations {
        let mut declarations = stylesheet::Declarations::new();

        assert_eq!(self.parser.consume_char(), '{');

        loop {
            self.parser.consume_whitespace();

            let declaration = match self.parser.next_char() {
                ';' => {
                    self.parser.consume_char();
                    break
                },
                '}' => {
                    self.parser.consume_char();
                    break
                },
                _ => self.parse_declaration(),
            };

            declarations.insert(declaration.0, declaration.1);
        }

        declarations
    }
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
