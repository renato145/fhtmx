use crate::utils::escape_html_to;

// TODO: use Cow here when rust gets the specialization feature
#[derive(Clone, Debug)]
pub enum AttributeValue {
    Empty,
    Raw(String),
    Value(String),
}

impl AttributeValue {
    pub fn size_hint(&self) -> usize {
        match self {
            AttributeValue::Empty => 0,
            AttributeValue::Raw(x) | AttributeValue::Value(x) => x.len() + 4,
        }
    }

    pub fn into_raw(self) -> Self {
        match self {
            AttributeValue::Value(s) => AttributeValue::Raw(s),
            x => x,
        }
    }

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

pub trait IntoAttributeValue: Sized {
    /// Transforms into a html attribute string
    fn into_attr(self) -> Option<AttributeValue>;

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
