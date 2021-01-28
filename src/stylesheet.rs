use std::collections::HashMap;

struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

struct Selector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(Color),
}

enum Unit {
    Px,
    Pt,
    Percent,
    Wh,
    Vh,
    Em,
    Rem,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

type Declaration = HashMap<String, Value>;
