//! Basic icons

use crate::{element::Element, svg::*};

pub fn menu() -> SvgElement {
    svg()
        .fill("none")
        .view_box("0 0 24 24")
        .stroke("currentColor")
        .add(
            svg_path()
                .stroke_linecap("round")
                .stroke_linejoin("round")
                .stroke_width(2)
                .d("M4 6h16M4 12h8m-8 6h16"),
        )
}

pub fn user() -> SvgElement {
    svg()
        .fill("none")
        .view_box("0 0 24 24")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_path().d("M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"))
        .add(svg_circle().cx(12).cy(7).r(4))
}

pub fn search() -> SvgElement {
    svg()
        .fill("none")
        .view_box("0 0 24 24")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_circle().cx(11).cy(11).r(8))
        .add(svg_path().d("m21 21-4.3-4.3"))
}

pub fn email() -> SvgElement {
    svg()
        .fill("none")
        .view_box("0 0 24 24")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_rect().width(20).height(16).x(2).y(4).rx(2))
        .add(svg_path().d("m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"))
}

pub fn password() -> SvgElement {
    svg()
        .fill("none")
        .view_box("0 0 24 24")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_path().d("M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z"))
        .add(svg_circle().cx(16.5).cy(7.5).r(0.5))
}

pub fn pin() -> SvgElement {
    svg()
        .fill("none")
        .view_box("0 0 24 24")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_path().d("M12 17v5"))
        .add(svg_path().d("M9 10.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H8a2 2 0 0 0 0 4 1 1 0 0 1 1 1z"))
}

pub fn refresh() -> SvgElement {
    svg()
        .fill("none")
        .view_box("0 0 24 24")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_path().d("M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"))
        .add(svg_path().d("M21 3v5h-5"))
}

pub fn sun() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("currentColor")
        .add(svg_path().d("M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"))
}

pub fn moon() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("currentColor")
        .add(svg_path().d("M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"))
}

pub fn edit() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_width(2)
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .add(svg_path().d("M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"))
        .add(svg_path().d("m15 5 4 4"))
}

pub fn delete() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_width(2)
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .add(svg_path().d("M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"))
        .add(svg_path().d("M3 6h18"))
        .add(svg_path().d("M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"))
}

pub fn save() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_width(2)
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .add(svg_path().d("M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"))
        .add(svg_path().d("M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"))
        .add(svg_path().d("M7 3v4a1 1 0 0 0 1 1h7"))
}

pub fn info() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_path().d("M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"))
}

pub fn note() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_circle().cx(12).cy(12).r(10))
        .add(svg_path().d("M12 16v-4"))
        .add(svg_path().d("M12 8h.01"))
}

pub fn important() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_circle().cx(12).cy(12).r(10))
        .add(svg_line().x1(12).x2(12).y1(8).y2(12))
        .add(svg_line().x1(12).x2(12.01).y1(16).y2(16))
}

pub fn tip() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(
            svg_path()
                .d("M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"),
        )
        .add(svg_path().d("M9 18h6"))
        .add(svg_path().d("M10 22h4"))
}

pub fn caution() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(svg_path().d("M12 16h.01"))
        .add(svg_path().d("M12 8v4"))
        .add(svg_path().d(
            "M15.312 2a2 2 0 0 1 1.414.586l4.688 4.688A2 2 0 0 1 22 8.688v6.624a2 2 0 0 1-.586 1.414l-4.688 4.688a2 2 0 0 1-1.414.586H8.688a2 2 0 0 1-1.414-.586l-4.688-4.688A2 2 0 0 1 2 15.312V8.688a2 2 0 0 1 .586-1.414l4.688-4.688A2 2 0 0 1 8.688 2z",
        ))
}

pub fn warning() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .stroke_linecap("round")
        .stroke_linejoin("round")
        .stroke_width(2)
        .add(
            svg_path()
                .d("m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"),
        )
        .add(svg_line().x1(12).x2(12).y1(8).y2(12))
        .add(svg_line().x1(12).x2(12.01).y1(16).y2(16))
}

pub fn error() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .add(
            svg_path()
                .stroke_linecap("round")
                .stroke_linejoin("round")
                .stroke_width(2)
                .d("M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"),
        )
}

pub fn success() -> SvgElement {
    svg()
        .view_box("0 0 24 24")
        .fill("none")
        .stroke("currentColor")
        .add(
            svg_path()
                .stroke_linecap("round")
                .stroke_linejoin("round")
                .stroke_width(2)
                .d("M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"),
        )
}
