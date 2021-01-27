mod html;
mod dom;

#[allow(dead_code)]
const HTML_TO_PARSE: &str = "<html>
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

fn main() {
    let mut parser = html::Parser::new(String::from(HTML_TO_PARSE));

    let node = parser.parse_node();

    println!("{}", dom::tree::node_tree(&node));
    println!("{}", dom::format::node_format(&node));
}
