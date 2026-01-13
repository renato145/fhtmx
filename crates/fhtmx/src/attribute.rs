#[derive(Clone, Debug)]
pub enum AttributeValue {
    Empty,
    Value(String),
}

impl AttributeValue {
    pub fn size_hint(&self) -> usize {
        match self {
            AttributeValue::Empty => 0,
            AttributeValue::Value(x) => x.len() + 4,
        }
    }
}

pub trait IntoAttributeValue {
    /// Transforms into a html attribute string
    fn into_attr(self) -> Option<AttributeValue>;
}

impl IntoAttributeValue for AttributeValue {
    fn into_attr(self) -> Option<AttributeValue> {
        Some(self)
    }
}

impl IntoAttributeValue for &str {
    fn into_attr(self) -> Option<AttributeValue> {
        Some(AttributeValue::Value(self.to_string()))
    }
}

impl IntoAttributeValue for String {
    fn into_attr(self) -> Option<AttributeValue> {
        Some(AttributeValue::Value(self))
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

// TODO: macro to implement for all numerics types
impl IntoAttributeValue for i32 {
    fn into_attr(self) -> Option<AttributeValue> {
        Some(AttributeValue::Value(self.to_string()))
    }
}
