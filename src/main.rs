mod html;
mod dom;

#[allow(dead_code)]
const HTML_TO_PARSE: &str = "<html>
<body>

<h1>My First Heading</h1>
<p>My first paragraph.</p>

</body>

<script>js</script>

</html>";

fn main() {
    let mut parser = html::Parser::new(String::from(HTML_TO_PARSE));

    // println!("{:#?}", parser.parse_node());
    println!("{}", dom::node_tree(&parser.parse_node()));
}
