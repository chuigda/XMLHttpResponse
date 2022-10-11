use std::collections::HashMap;
use minidom::{Element, Node};

pub fn instantiate(document_node: &Element, vars: HashMap<&str, String>) -> Element {
    instantiate_elem(document_node, &vars).unwrap()
}

fn lookup_var<'a>(vars: &'a HashMap<&str, String>, var_name: &str) -> &'a str {
    if let Some(var_value) = vars.get(var_name) {
        var_value
    } else {
        "undefined"
    }
}

fn instantiate_attr(value: &str, vars: &HashMap<&str, String>) -> String {
    let trimmed_value = value.trim();
    if trimmed_value.starts_with("$$") {
        value.replace("$$", "$").into()
    } else if trimmed_value.starts_with('$') {
        lookup_var(vars, trimmed_value).to_string()
    } else {
        value.to_string()
    }
}

fn instantiate_text(text: &str, vars: &HashMap<&str, String>) -> String {
    let mut ret = String::new();
    for piece in text.split_whitespace() {
        if piece.starts_with("$$") {
            ret.push_str(&piece.replace("$$", "$"));
        } else if piece.starts_with("$") {
            ret.push_str(lookup_var(vars, piece));
        } else {
            ret.push_str(piece);
        }
        ret.push(' ');
    }
    ret
}

fn is_falsy_value(s: &str) -> bool {
    s == "false"
    || s == "False"
    || s == "undefined"
    || s == "null"
    || s == "0"
    || s == "[]"
    || s == "\"\""
    || s == "''"
    || s == ""
}

fn instantiate_elem(
    node: &Element,
    vars: &HashMap<&str, String>
) -> Option<Element> {
    if let Some(condition) = node.attr("x-if") {
        if is_falsy_value(lookup_var(vars, condition)) {
            return None;
        }
    }

    let mut builder = Element::builder(node.name(), node.ns());
    for (attr_name, value) in node.attrs() {
        if attr_name.starts_with("x-") {
            continue;
        }

        let instantiated_value = instantiate_attr(value, vars);
        builder = builder.attr(attr_name, instantiated_value);
    }

    for node in node.nodes() {
        match node {
            Node::Element(elem) => {
                if let Some(instantiated_elem) = instantiate_elem(elem, vars) {
                    builder = builder.append(Node::Element(instantiated_elem));
                }
            },
            Node::Text(text) => {
                builder = builder.append(Node::Text(instantiate_text(text, vars)));
            }
        }
    }

    Some(builder.build())
}
