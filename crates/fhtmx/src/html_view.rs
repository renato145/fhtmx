use crate::{
    element::Element,
    html_element::*,
    node::{HtmlNode, IntoNode},
    prelude::dc_list_row,
    render::Render,
};

pub fn html_list_row<K, V>(key: K, value: V) -> HtmlElement
where
    K: IntoNode,
    V: IntoNode,
{
    dc_list_row().add(key).add(value)
}

pub trait HtmlView {
    fn html_content(&self) -> HtmlNode;

    fn html_view(&self) -> HtmlNode {
        self.html_content()
    }

    fn render(&self) -> String {
        self.html_view().render()
    }
}

impl<T: HtmlView> HtmlView for Option<T> {
    fn html_content(&self) -> HtmlNode {
        match self {
            Some(x) => x.html_view(),
            None => "-".into_node(),
        }
    }
}

macro_rules! implement_for_display {
    ($($t:ty),* $(,)?) => {
        $(
            impl HtmlView for $t {
                fn html_content(&self) -> HtmlNode {
                    HtmlNode::Text(self.to_string())
                }
            }
        )*
    };
}

implement_for_display!(
    bool, char, &str, &String, String, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128,
    usize, f32, f64
);

#[cfg(feature = "chrono_0_4")]
impl HtmlView for chrono::NaiveDate {
    fn html_content(&self) -> HtmlNode {
        HtmlNode::Text(self.to_string())
    }
}

// TODO: also implement into node

#[cfg(feature = "chrono_0_4")]
impl HtmlView for chrono::DateTime<chrono::Utc> {
    fn html_content(&self) -> HtmlNode {
        HtmlNode::Text(self.to_string())
    }
}

#[cfg(feature = "jiff_0_2")]
impl HtmlView for jiff::civil::Date {
    fn html_content(&self) -> HtmlNode {
        HtmlNode::Text(self.to_string())
    }
}

#[cfg(feature = "jiff_0_2")]
impl HtmlView for jiff::Timestamp {
    fn html_content(&self) -> HtmlNode {
        HtmlNode::Text(self.to_string())
    }
}
