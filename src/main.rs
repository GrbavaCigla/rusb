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
<style>p.class {
    propery: value;
    another-property: 10px;
}

p2.class{propery: value;another-property: 10px;}
</style>
</html>";

fn main() {
    let node = html::Parser::parse(String::from(HTML_TO_PARSE));

    match node {
        dom::Node::Element(ed) => {
            let css =  ed.get_elements_by_tag_name("style");

            if !css.is_empty() {
                if !css[0].children.is_empty() {
                    match &css[0].children[0] {
                        dom::Node::Text(text) => {
                            let css = css::Parser::parse(String::from(text));

                            println!("{:#?}", css);
                        }
                        _ => {}
                    }
                }
            }
        },
        _ => {}
    };
}
