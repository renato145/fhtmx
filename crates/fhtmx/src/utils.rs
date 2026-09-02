use uuid::Uuid;

/// Escapes HTML special characters and returns a new string.
///
/// Escapes `&`, `<`, `>`, `"` and `'`.
///
/// # Examples
///
/// ```
/// use fhtmx::prelude::escape_html;
///
/// assert_eq!(
///     escape_html(r#"<a>"x" & 'y'</a>"#),
///     "&lt;a&gt;&quot;x&quot; &amp; &#x27;y&#x27;&lt;/a&gt;"
/// );
/// ```
pub fn escape_html(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    escape_html_to(s, &mut buf);
    buf
}

/// Escapes HTML special characters appending to `buf`.
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

/// Escapes HTML special characters appending to `buf` with indentation.
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

/// Generates a random id with the given prefix (e.g. `prefix-uuid`).
///
/// # Examples
///
/// ```
/// use fhtmx::prelude::random_id;
///
/// let id = random_id("toast");
/// assert!(id.starts_with("toast-"));
/// ```
pub fn random_id(prefix: &str) -> String {
    let uuid = Uuid::new_v4();
    // Convert it to a hyphenated string format
    format!("{}-{}", prefix, uuid.as_hyphenated())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn escape_html_works() {
        let res = escape_html("<a>\"hello\"</a>");
        insta::assert_snapshot!(res, @"&lt;a&gt;&quot;hello&quot;&lt;/a&gt;");
        let res = escape_html("<a>'he \\& llo'</a>");
        insta::assert_snapshot!(res, @r"&lt;a&gt;&#x27;he \&amp; llo&#x27;&lt;/a&gt;");
    }
}
