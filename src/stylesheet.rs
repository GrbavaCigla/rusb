use std::collections::HashMap;

#[derive(Debug)]
pub struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Declarations,
}

#[derive(Debug)]
pub struct Selector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let a = self.id.iter().count();
        let b = self.class.len();
        let c = self.tag_name.iter().count();
        (a, b, c)
    }
}

#[derive(Debug)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
}

#[derive(Debug)]
pub enum Unit {
    Px,
    Pt,
    Percent,
    Wh,
    Vh,
    Em,
    Rem,
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub type Declarations = HashMap<String, Value>;
