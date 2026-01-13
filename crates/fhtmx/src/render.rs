use crate::{
    html::{HtmlAttribute, HtmlElement, HtmlNode, VOID_ELEMENTS},
    utils::{escape_html_to, escape_html_to_with_indent},
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

impl Render for HtmlElement {
    fn render_to(&self, buf: &mut String, indent: usize) {
        // TODO: Convert recursion to iteration using an explicit stack:

        let pad = "  ".repeat(indent);

        buf.push_str(&pad);
        buf.push('<');
        buf.push_str(self.tag);
        for (k, v) in &self.attrs {
            buf.push(' ');
            buf.push_str(k);
            if let HtmlAttribute::Value(v) = v {
                buf.push_str("=\"");
                escape_html_to(v, buf);
                buf.push('"');
            }
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

        if self.has_inline_content() {
            buf.push('>');
            self.children.iter().for_each(|c| c.render_to(buf, 0));
            buf.push_str("</");
            buf.push_str(self.tag);
            buf.push('>');
        } else {
            buf.push_str(">\n");
            for c in &self.children {
                c.render_to(buf, indent + 1);
                buf.push('\n');
            }
            buf.push_str(&pad);
            buf.push_str("</");
            buf.push_str(self.tag);
            buf.push('>');
        }
    }

    fn size_hint(&self) -> usize {
        let tag_len = self.tag.len() * 2 + 5;
        let attrs_len = self
            .attrs
            .iter()
            .map(|(k, v)| k.len() + v.size_hint())
            .sum::<usize>();
        let children_len = self.children.iter().map(|c| c.size_hint()).sum::<usize>();
        tag_len + attrs_len + children_len
    }
}

#[inline]
fn push_str_with_indent(s: &str, buf: &mut String, indent: usize) {
    if indent == 0 {
        buf.push_str(s);
    } else {
        let pad = "  ".repeat(indent);
        s.lines().for_each(|o| {
            buf.push_str(&pad);
            buf.push_str(o);
            buf.push('\n');
        });
        buf.pop();
    }
}

impl Render for HtmlNode {
    fn render_to(&self, buf: &mut String, indent: usize) {
        match self {
            HtmlNode::Doctype => push_str_with_indent("<!DOCTYPE html>", buf, indent),
            HtmlNode::Raw(s) => push_str_with_indent(s, buf, indent),
            HtmlNode::Text(s) => escape_html_to_with_indent(s, buf, indent),
            HtmlNode::Element(el) => el.render_to(buf, indent),
            HtmlNode::Fragment(nodes) => {
                for node in nodes {
                    node.render_to(buf, indent);
                    buf.push('\n');
                }
                buf.pop();
            }
        }
    }

    fn size_hint(&self) -> usize {
        match self {
            HtmlNode::Doctype => 15,
            HtmlNode::Raw(s) | HtmlNode::Text(s) => s.len(),
            HtmlNode::Element(el) => el.size_hint(),
            HtmlNode::Fragment(nodes) => nodes.iter().map(|n| n.size_hint()).sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::*;
    use googletest::prelude::*;

    #[test]
    fn simple_render() {
        let x = p().add_child("some text").render();
        assert_that!(x, eq("<p>some text</p>"));
    }

    #[gtest]
    fn inline_content() {
        let x = div()
            .add_child("Some intro text")
            .add_child(p().add_child("A paragraph"))
            .render();
        expect_that!(
            x,
            eq("<div>\n  Some intro text\n  <p>A paragraph</p>\n</div>")
        );

        let x = div()
            .add_child("Some intro text\nwith multiple\nlines")
            .add_child(p().add_child("A paragraph"))
            .render();
        expect_that!(
            x,
            eq("<div>\n  Some intro text\n  with multiple\n  lines\n  <p>A paragraph</p>\n</div>")
        );

        let x = div()
            .add_raw("<p>one</p>\n<p>two</p>\n<p>three</p>")
            .add_child(p().add_child("A paragraph"))
            .render();
        expect_that!(
            x,
            eq("<div>\n  <p>one</p>\n  <p>two</p>\n  <p>three</p>\n  <p>A paragraph</p>\n</div>")
        );
    }
}
