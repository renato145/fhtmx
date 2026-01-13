use crate::prelude::{HtmlElement, HtmlTagWrap, set_attr};
use paste::paste;

#[derive(Debug, Clone, Copy)]
pub struct HtmlSvgElement;

macro_rules! create_svg_web_element {
    ($wrap:expr => $name:ident) => {
        paste! {
            #[doc = "Creates a `" $name "` svg element."]
            pub fn [<svg_ $name>]() -> HtmlElement<&'static str, HtmlSvgElement> {
                HtmlElement::new(stringify!($name), $wrap)
            }
        }
    };

    ($wrap:expr => $name:ident, $($rest:ident),+) => {
        create_svg_web_element!($wrap => $name);
        create_svg_web_element!($wrap => $($rest),+);
    };
}

pub fn svg() -> HtmlElement<&'static str, HtmlSvgElement> {
    HtmlElement::new("svg", HtmlTagWrap::Wrap).set_attr("xmlns", "http://www.w3.org/2000/svg")
}

create_svg_web_element!(
    HtmlTagWrap::NoWrap => circle, ellipse, g, line, path, polygon, polyline, rect, text
);

impl<T> HtmlElement<T, HtmlSvgElement> {
    set_attr!(
        color,
        cx,
        cy,
        d,
        dominant_baseline = "dominant-baseline",
        dx,
        dy,
        fill,
        fill_opacity = "fill-opacity",
        font_family = "font-family",
        font_size = "font-size",
        font_style = "font-style",
        font_variant = "font-variant",
        font_weight = "font-weight",
        height,
        opacity,
        path,
        points,
        r,
        radius,
        rx,
        ry,
        rotate,
        scale,
        stroke,
        stroke_dasharray = "stroke-dasharray",
        stroke_dashoffset = "stroke-dashoffset",
        stroke_linecap = "stroke-linecap",
        stroke_linejoin = "stroke-linejoin",
        stroke_opacity = "stroke-opacity",
        stroke_width = "stroke-width",
        text_anchor = "text-anchor",
        text_decoration = "text-decoration",
        transform,
        view_box = "viewBox",
        width,
        x,
        x1,
        x2,
        y,
        y1,
        y2,
        z
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
            .render_sorted();
        insta::assert_snapshot!(res, @r#"
        <svg xmlns="http://www.w3.org/2000/svg">
          <rect fill="green" height="20" stroke-width="2" stroke="black" width="100" />
        </svg>
        "#);
    }
}
