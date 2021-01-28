use crate::dom;

pub struct Parser {
    pub pos: usize,
    pub input: String,
}

impl Parser {
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

    pub fn consume_whitespace(&mut self) {
        self.consume_while(|c| match c {
            ' ' => true,
            '\n' => true,
            _ => false,
        });
    }

    pub fn consume_sequence(&mut self, seq: &str) {
        for ch in seq.chars() {
            assert_eq!(self.consume_char(), ch);
        }
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
        let pars = Parser {
            input: String::from("Hello, World"),
            pos: 3,
        };

        assert_eq!(pars.next_char(), 'l');
    }

    #[test]
    fn consume_while() {
        let mut pars = Parser{input: String::from("Hello, World"), pos: 7};

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
        let mut pars = Parser{input: String::from("Hello, World"), pos: 7};

        assert_eq!(pars.consume_char(), 'W');
        assert_eq!(pars.consume_char(), 'o');
    }
}
