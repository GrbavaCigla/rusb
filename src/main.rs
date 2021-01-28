mod html;
mod dom;
mod stylesheet;
mod parser;
mod css;

const HTML_TO_PARSE: &str = "
<html>
<head>
<title>I am the title</title>
</head>
<body>
<!--I am the comment-->
<h1 key=\"value\" key2=\"value\">I am the heading.</h1>
<p>I am the paragraph.</p>
<br/>
</body>
<script>I am the script</script>
</html>";

const CSS_TO_PARSE: &str = "p.class {propery: value;another-property: 10px;}p2.class{propery: value;another-property: 10px;}";

fn main() {
    let _node = html::Parser::parse(String::from(HTML_TO_PARSE));

    // println!("{}", dom::tree::node_tree(&node));
    // println!("{}", dom::format::node_format(&node));

    let mut node2 = css::Parser {
        parser: parser::Parser {
            // input: String::from("property: #00bfff; property2: red"),
            input: String::from(CSS_TO_PARSE),
            pos: 0
        }
    };

    println!("{:?}", node2.parse_selectors());
    println!("{:?}", node2.parse_declarations());
    println!("{:?}", node2.parse_selectors());
    println!("{:?}", node2.parse_declarations());
}
