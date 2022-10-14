use minidom::{Element, Node};
use xjbutil::either::Either;

use crate::eval::{EvalContext, PyOutputValue};

pub fn instantiate(document_node: &Element, ctx: &EvalContext) -> Element {
    instantiate_one(document_node, ctx).unwrap()
}

fn instantiate_attr(value: &str, ctx: &EvalContext) -> String {
    let value = value.trim();
    if value.starts_with("$$") {
        value.replace("$$", "$").into()
    } else if value.starts_with('$') {
        ctx.eval_var_expr(value).unwrap_or("undefined").to_string()
    } else {
        value.to_string()
    }
}

fn instantiate_text(text: &str, ctx: &EvalContext) -> String {
    let mut ret = String::new();
    for piece in text.split_whitespace() {
        if piece.starts_with("$$") {
            ret.push_str(&piece.replace("$$", "$"));
        } else if piece.starts_with("$") {
            ret.push_str(ctx.eval_var_expr(piece).unwrap_or("undefined"));
        } else {
            ret.push_str(piece);
        }
        ret.push(' ');
    }
    ret
}

fn is_falsy_value(s: Option<&str>) -> bool {
    if let Some(s) = s {
        s == "false"
        || s == "False"
        || s == "undefined"
        || s == "null"
        || s == "[]"
        || s == ""
    } else {
        true
    }
}

fn instantiate_elem(
    node: &Element,
    ctx: &EvalContext,
) -> Option<Either<Vec<Element>, Element>> {
    if let Some(loop_var) = node.attr("x-for") {
        if let Some(PyOutputValue::Array(arr)) = ctx.lookup_var(loop_var) {
            let mut vec = Vec::new();
            for arr_value in arr {
                ctx.push_x_for(arr_value);
                if let Some(element) = instantiate_one(node, ctx) {
                    vec.push(element);
                }
                ctx.pop_x_for();
            }

            return Some(Either::Left(vec));
        }
    }

    instantiate_one(node, ctx).map(Either::Right)
}

fn instantiate_one(
    node: &Element,
    ctx: &EvalContext
) -> Option<Element> {
    if let Some(condition) = node.attr("x-if") {
        if is_falsy_value(ctx.eval_var_expr(condition)) {
            return None;
        }
    }

    let mut builder = Element::builder(node.name(), node.ns());
    for (attr_name, value) in node.attrs() {
        if attr_name.starts_with("x-") {
            continue;
        }

        let instantiated_value = instantiate_attr(value, ctx);
        builder = builder.attr(attr_name, instantiated_value);
    }

    for node in node.nodes() {
        match node {
            Node::Element(elem) => {
                if let Some(instantiated_elem) = instantiate_elem(elem, ctx) {
                    match instantiated_elem {
                        Either::Left(sub_elements) => {
                            builder = builder.append_all(sub_elements.into_iter());
                        },
                        Either::Right(element) => {
                            builder = builder.append(element);
                        }
                    }
                }
            },
            Node::Text(text) => {
                builder = builder.append(Node::Text(instantiate_text(text, ctx)));
            }
        }
    }

    Some(builder.build())
}
