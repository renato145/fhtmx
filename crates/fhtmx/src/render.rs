use crate::{
    html::{HtmlElement, HtmlNode, VOID_ELEMENTS},
    utils::escape_html_to,
};

/// Renders to HTML strings
pub trait Render {
    /// Render to a buffer
    fn render_to(&self, buf: &mut String, indent: usize);

    /// Size hint for more efficient rendering
    fn size_hint(&self) -> usize;

    fn render(&self) -> String {
        let mut buf = String::with_capacity(self.size_hint());
        self.render_to(&mut buf, 0);
        buf
    }
}

// impl Render for Node {
//     fn render_to(&self, buf: &mut String, indent: usize) {
//         match self {
//             Node::Text(s) => escape_html_to(s, buf),
//             Node::Raw(s) => buf.push_str(s),
//             Node::Fragment(nodes) => {
//                 for node in nodes {
//                     node.render_to(buf, indent);
//                     buf.push('\n');
//                 }
//             }
//             Node::Doctype => buf.push_str("<!DOCTYPE html>"),
//             Node::Element(el) => el.render_to(buf, indent),
//         }
//     }
//
//     fn size_hint(&self) -> usize {
//         match self {
//             Node::Text(s) => s.len(),
//             Node::Raw(s) => s.len(),
//             Node::Fragment(nodes) => nodes.iter().map(|n| n.size_hint()).sum(),
//             Node::Doctype => 15,
//             Node::Element(el) => el.size_hint(),
//         }
//     }
// }

impl Render for HtmlElement {
    fn render_to(&self, buf: &mut String, indent: usize) {
        let pad = "  ".repeat(indent);

        buf.push_str(&pad);
        buf.push('<');
        buf.push_str(self.tag);
        for (k, v) in &self.attrs {
            buf.push(' ');
            buf.push_str(k);
            buf.push_str("=\"");
            escape_html_to(v, buf);
            buf.push('"');
        }

        if VOID_ELEMENTS.contains(&self.tag) {
            buf.push_str(" />");
            return;
        }

        if self.children.is_empty() {
            buf.push_str("></");
            buf.push_str(self.tag);
            buf.push('>');
            return;
        }

        // if self.has_inline_content() {
        //     buf.push('>');
        //     self.children.iter().for_each(|c| c.render_to(buf, 0));
        //     buf.push_str("</");
        //     buf.push_str(self.tag);
        //     buf.push('>');
        // } else {
        //     buf.push_str(">\n");
        //     for c in &self.children {
        //         c.render_to(buf, indent + 1);
        //         buf.push('\n');
        //     }
        //     buf.push_str(&pad);
        //     buf.push_str("</");
        //     buf.push_str(self.tag);
        //     buf.push('>');
        // }
    }

    fn size_hint(&self) -> usize {
        let tag_len = self.tag.len() * 2 + 5;
        let attrs_len = self
            .attrs
            .iter()
            .map(|(k, v)| k.len() + v.len() + 4)
            .sum::<usize>();
        // let children_len = self.children.iter().map(|c| c.size_hint()).sum::<usize>();
        // tag_len + attrs_len + children_len
        tag_len + attrs_len
    }

    fn render(&self) -> String {
        let mut buf = String::with_capacity(self.size_hint());
        self.render_to(&mut buf, 0);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::*;
    use googletest::prelude::*;

    #[test]
    fn simple_render() {
        let x = p().render();
        assert_that!(x, eq("<p></p>"))
    }
}
