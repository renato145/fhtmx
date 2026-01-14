pub fn escape_html(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    escape_html_to(s, &mut buf);
    buf
}

pub fn escape_html_to(s: &str, buf: &mut String) {
    for c in s.chars() {
        match c {
            '&' => buf.push_str("&amp;"),
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '"' => buf.push_str("&quot;"),
            '\'' => buf.push_str("&#x27;"),
            _ => buf.push(c),
        }
    }
}

pub fn escape_html_to_with_indent(s: &str, buf: &mut String, indent: usize) {
    if indent == 0 {
        escape_html_to(s, buf);
    } else {
        let pad = "  ".repeat(indent);
        s.lines().for_each(|o| {
            buf.push_str(&pad);
            escape_html_to(o, buf);
            buf.push('\n');
        });
        buf.pop();
    }
}

#[cfg(test)]
mod test {
    use crate::utils::escape_html;
    use googletest::prelude::*;

    #[gtest]
    fn escape_html_works() {
        expect_that!(
            escape_html("<a>\"hello\"</a>"),
            eq("&lt;a&gt;&quot;hello&quot;&lt;/a&gt;")
        );
        expect_that!(
            escape_html("<a>'he \\& llo'</a>"),
            eq("&lt;a&gt;&#x27;he \\&amp; llo&#x27;&lt;/a&gt;")
        );
    }
}
