use crate::utils::escape_html_to;

/// Represents the value of an HTML attribute.
///
/// - [`Empty`](Self::Empty): boolean attribute with no value (`hidden`, `disabled`, etc.)
/// - [`Raw`](Self::Raw): inserted as-is (useful for JSON or unescaped values)
/// - [`Value`](Self::Value): HTML-escaped on render
// TODO: use Cow here when rust gets the specialization feature
#[derive(Clone, Debug)]
pub enum AttributeValue {
    /// Boolean attribute with no value.
    Empty,
    /// Raw string inserted without HTML escaping.
    Raw(String),
    /// Escaped string value.
    Value(String),
}

impl AttributeValue {
    /// Estimated byte size when rendered.
    pub fn size_hint(&self) -> usize {
        match self {
            AttributeValue::Empty => 0,
            AttributeValue::Raw(x) | AttributeValue::Value(x) => x.len() + 4,
        }
    }

    /// Converts a [`Value`](Self::Value) into [`Raw`](Self::Raw), leaving others unchanged.
    pub fn into_raw(self) -> Self {
        match self {
            AttributeValue::Value(s) => AttributeValue::Raw(s),
            x => x,
        }
    }

    /// Renders the attribute value into `buf`.
    pub fn render_to(&self, buf: &mut String) {
        match self {
            AttributeValue::Empty => {}
            AttributeValue::Raw(v) => {
                let has_double_quote = v.contains('\"');
                if has_double_quote {
                    buf.push_str("='");
                } else {
                    buf.push_str("=\"");
                }
                buf.push_str(v);
                if has_double_quote {
                    buf.push('\'');
                } else {
                    buf.push('"');
                }
            }
            AttributeValue::Value(v) => {
                buf.push_str("=\"");
                escape_html_to(v, buf);
                buf.push('"');
            }
        }
    }
}

/// Types that can be converted into an [`AttributeValue`].
pub trait IntoAttributeValue: Sized {
    /// Converts into an [`AttributeValue`]. Returns `None` to omit the attribute.
    fn into_attr(self) -> Option<AttributeValue>;

    /// Like [`into_attr`](Self::into_attr) but marks the result as [`Raw`](AttributeValue::Raw).
    fn into_raw_attr(self) -> Option<AttributeValue> {
        self.into_attr().map(|x| x.into_raw())
    }
}

impl IntoAttributeValue for AttributeValue {
    fn into_attr(self) -> Option<AttributeValue> {
        Some(self)
    }
}

impl IntoAttributeValue for bool {
    fn into_attr(self) -> Option<AttributeValue> {
        if self {
            Some(AttributeValue::Empty)
        } else {
            None
        }
    }
}

macro_rules! implement_for_display {
    ($($t:ty),* $(,)?) => {
        $(
            impl IntoAttributeValue for $t {
                fn into_attr(self) -> Option<AttributeValue> {
                    Some(AttributeValue::Value(self.to_string()))
                }
            }
        )*
    };
}

implement_for_display!(
    char, &str, &String, String, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
    f32, f64
);
