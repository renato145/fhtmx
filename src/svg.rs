use crate::prelude::{HtmlElement, HtmlTagWrap, set_attr};
use paste::paste;

#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgCircleElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgEllipseElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgLineElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgPathElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgPolygonElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgRectElement;
#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgTextElement;

macro_rules! create_svg_web_element {
    ($wrap:expr => $name:ident:$group:expr) => {
        paste! {
            #[doc = "Creates a `" $name "` svg element."]
            pub fn [<svg_ $name>]() -> HtmlElement<&'static str, $group> {
                HtmlElement::new(stringify!($name), $wrap)
            }
        }
    };

    ($wrap:expr => $name:ident:$group:expr, $($rest:ident:$group_rest:expr),+) => {
        create_svg_web_element!($wrap => $name:$group);
        create_svg_web_element!($wrap => $($rest:$group_rest),+);
    };
}

pub fn svg() -> HtmlElement<&'static str, HtmlSvgElement> {
    HtmlElement::new("svg", HtmlTagWrap::Wrap).set_attr("xmlns", "http://www.w3.org/2000/svg")
}

create_svg_web_element!(
    HtmlTagWrap::NoWrap => circle:HtmlSvgCircleElement, ellipse:HtmlSvgEllipseElement,
    line:HtmlSvgLineElement, path:HtmlSvgPathElement, polygon:HtmlSvgPolygonElement,
    polyline:HtmlSvgPolygonElement, rect:HtmlSvgRectElement, text:HtmlSvgTextElement
);

// TODO: stroke_width should be set as stroke-width

impl<T> HtmlElement<T, HtmlSvgCircleElement> {
    set_attr!(r, cx, cy, fill, stroke, stroke_width);
}

impl<T> HtmlElement<T, HtmlSvgEllipseElement> {
    set_attr!(rx, ry, cx, cy, fill, stroke, stroke_width);
}

impl<T> HtmlElement<T, HtmlSvgLineElement> {
    set_attr!(x1, y1, x2, y2, stroke, w, stroke_width);
}

impl<T> HtmlElement<T, HtmlSvgPathElement> {
    set_attr!(d, fill, stroke, stroke_width);
}

impl<T> HtmlElement<T, HtmlSvgPolygonElement> {
    set_attr!(points, fill, stroke, stroke_width);
}

impl<T> HtmlElement<T, HtmlSvgRectElement> {
    set_attr!(width, height, x, y, fill, stroke, stroke_width, rx, ry);
}

impl<T> HtmlElement<T, HtmlSvgTextElement> {
    set_attr!(
        x,
        y,
        font_family,
        font_size,
        fill,
        text_anchor,
        dominant_baseline,
        font_weight,
        font_style,
        text_decoration
    );
}

#[cfg(test)]
mod test {
    use crate::{
        prelude::HtmlRender,
        svg::{svg, svg_rect},
    };

    #[test]
    fn svg_attrs_works() {
        let res = svg()
            .add_child(
                svg_rect()
                    .width(100)
                    .height(20)
                    .fill("green")
                    .stroke("black")
                    .stroke_width(2),
            )
            .render();
        println!("{}", res);
    }
}
