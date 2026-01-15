use crate::{element::Element, node::HtmlNode, utils::escape_html_to_with_indent};

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

impl<T: Element> Render for T {
    fn render_to(&self, buf: &mut String, indent: usize) {
        // TODO: Convert recursion to iteration using an explicit stack:

        let tag = self.tag();
        let pad = "  ".repeat(indent);

        buf.push_str(&pad);
        buf.push('<');
        buf.push_str(tag);
        for (k, v) in self.attrs() {
            buf.push(' ');
            buf.push_str(k);
            v.render_to(buf);
        }

        if self.is_void_tag() {
            buf.push_str(" />");
            return;
        }

        if self.children().is_empty() {
            buf.push_str("></");
            buf.push_str(tag);
            buf.push('>');
            return;
        }

        if self.has_inline_content() {
            buf.push('>');
            self.children().iter().for_each(|c| c.render_to(buf, 0));
            buf.push_str("</");
            buf.push_str(tag);
            buf.push('>');
        } else {
            buf.push_str(">\n");
            for c in self.children() {
                c.render_to(buf, indent + 1);
                buf.push('\n');
            }
            buf.push_str(&pad);
            buf.push_str("</");
            buf.push_str(tag);
            buf.push('>');
        }
    }

    fn size_hint(&self) -> usize {
        let tag_len = self.tag().len() * 2 + 5;
        let attrs_len = self
            .attrs()
            .iter()
            .map(|(k, v)| k.len() + v.size_hint())
            .sum::<usize>();
        let children_len = self.children().iter().map(|c| c.size_hint()).sum::<usize>();
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
            HtmlNode::SvgElement(el) => el.render_to(buf, indent),
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
            HtmlNode::SvgElement(el) => el.size_hint(),
            HtmlNode::Fragment(nodes) => nodes.iter().map(|n| n.size_hint()).sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::*;

    #[test]
    fn simple_render() {
        let res = p().class("bg-red-500").add("some text").render();
        insta::assert_snapshot!(res, @r#"<p class="bg-red-500">some text</p>"#);
    }

    #[test]
    fn inline_content() {
        let res = div()
            .add("Some intro text")
            .add(p().add("A paragraph"))
            .render();
        insta::assert_snapshot!(res, @r"
        <div>
          Some intro text
          <p>A paragraph</p>
        </div>
        ");
    }

    #[test]
    fn inline_content_no_text() {
        let res = div().add(p().add(span())).add(span()).render();
        insta::assert_snapshot!(res, @r"
        <div>
          <p><span></span></p>
          <span></span>
        </div>
        ");
    }

    #[test]
    fn inline_content_multiline_text() {
        let res = div()
            .add("Some intro text\nwith multiple\nlines")
            .add(p().add("A paragraph"))
            .render();
        insta::assert_snapshot!(res, @r"
        <div>
          Some intro text
          with multiple
          lines
          <p>A paragraph</p>
        </div>
        ");
    }

    #[test]
    fn inline_content_raw() {
        let res = div()
            .add_raw("<p>one</p>\n<p>two</p>\n<p>three</p>")
            .add(p().add("A paragraph"))
            .render();
        insta::assert_snapshot!(res, @r"
        <div>
          <p>one</p>
          <p>two</p>
          <p>three</p>
          <p>A paragraph</p>
        </div>
        ");
    }
}
