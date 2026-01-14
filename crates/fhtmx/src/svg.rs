use crate::{
    attribute::{AttributeValue, IntoAttributeValue},
    element::{Element, set_attr},
    node::{HtmlNode, IntoNode, raw_node},
};
use indexmap::IndexMap;
use paste::paste;
use std::borrow::Cow;

pub const SVG_INLINE_ELEMENTS: &[&str] = &["text", "tspan"];

/// Represents a HTML element
#[derive(Clone, Debug)]
pub struct SvgElement {
    pub tag: &'static str,
    pub attrs: IndexMap<Cow<'static, str>, AttributeValue>,
    pub children: Vec<HtmlNode>,
}

impl SvgElement {
    pub fn new(tag: &'static str) -> Self {
        Self {
            tag,
            attrs: IndexMap::new(),
            children: Vec::new(),
        }
    }
}

impl Element for SvgElement {
    #[inline]
    fn tag(&self) -> &'static str {
        self.tag
    }

    #[inline]
    fn attrs(&self) -> &IndexMap<Cow<'static, str>, AttributeValue> {
        &self.attrs
    }

    #[inline]
    fn children(&self) -> &[HtmlNode] {
        &self.children
    }

    #[inline]
    fn is_void_tag(&self) -> bool {
        false
    }

    #[inline]
    fn is_inline_tag(&self) -> bool {
        SVG_INLINE_ELEMENTS.contains(&self.tag())
    }

    fn add_raw(mut self, raw: impl ToString) -> Self {
        self.children.push(raw_node(raw));
        self
    }

    fn add_child(mut self, node: impl IntoNode) -> Self {
        let node = node.into_node();
        match node {
            HtmlNode::Fragment(mut x) => {
                self.children.append(&mut x);
            }
            x => self.children.push(x),
        }
        self
    }

    fn add_children(mut self, nodes: impl IntoIterator<Item = impl IntoNode>) -> Self {
        self.children
            .extend(nodes.into_iter().map(|n| n.into_node()));
        self
    }

    fn insert_attr(mut self, attr: impl Into<Cow<'static, str>>, value: AttributeValue) -> Self {
        self.attrs.insert(attr.into(), value);
        self
    }
}

macro_rules! create_svg_fn {
    ($name:ident) => {
        paste! {
            #[doc = "Creates a `" $name "` svg element."]
            pub fn [<svg_ $name>]() -> SvgElement {
                SvgElement::new(stringify!($name))
            }
        }
    };

    ($name:ident; $eg:expr) => {
        paste! {
            #[doc = "Creates a `" $name "` svg element.\n"$eg]
            pub fn [<svg_ $name>]() -> SvgElement {
                SvgElement::new(stringify!($name))
            }
        }
    };

    ($name:ident = $tag:expr; $eg:expr) => {
        paste! {
            #[doc = "Creates a `" $tag "` svg element.\n"$eg]
            pub fn [<svg_ $name>]() -> SvgElement {
                SvgElement::new($tag)
            }
        }
    };

    ($name:ident$(=$tag:expr)?$(;$eg:expr)?, $($rest:ident$(=$tag_rest:expr)?$(;$eg_rest:expr)?),+) => {
        create_svg_fn!($name$(=$tag)?$(;$eg)?);
        create_svg_fn!($($rest$(=$tag_rest)?$(;$eg_rest)?),+);
    };
}
/// Creates a `svg` html element.
/// Defines a container for SVG graphics
pub fn svg() -> SvgElement {
    SvgElement::new("svg").set_attr("xmlns", "http://www.w3.org/2000/svg")
}

create_svg_fn!(
    a; "Creates a hyperlink around an element",
    animate; "Animates an attribute of an element over time",
    animate_motion="animateMotion"; "Sets how an element moves along a motion path",
    animate_transform="animateTransform"; "Animates a transformation attribute on the target element",
    circle; "Defines a circle",
    clip_path="clipPath"; "Specifies a clipping path, to be used by the clip-path property",
    defs; "A container for referenced elements",
    desc; "A text-only description for container elements or graphic elements",
    ellipse; "Defines an ellipse",
    fe_blend="feBlend"; "SVG filter. Combines two graphics together by a certain blending mode",
    fe_color_matrix="feColorMatrix"; "SVG filter. Changes colors based on a transformation matrix",
    fe_component_transfer="feComponentTransfer"; "SVG filter. Performs component-wise remapping of data for each pixel. Can adjust brightness, contrast, color balance, etc",
    fe_composite="feComposite"; "SVG filter. Performs combination of two input images pixel-wise in image space using a compositing operation",
    fe_convolve_matrix="feConvolveMatrix"; "SVG filter. Applies a matrix convolution filter effect (this includes blurring, edge detection, sharpening, embossing and beveling)",
    fe_diffuse_lighting="feDiffuseLighting"; "SVG filter. Lights a graphic by using the alpha channel as a bump map",
    fe_displacement_map="feDisplacementMap"; "SVG filter. Uses pixels values from the graphic from in2 attribute to displace the image from the in attribute",
    fe_distant_light="feDistantLight"; "SVG filter. Specifies a distant light source to be used inside a lighting filter primitive: <feDiffuseLighting> or <feSpecularLighting>",
    fe_drop_shadow="feDropShadow"; "SVG filter. Creates a drop shadow of the graphic",
    fe_flood="feFlood"; "SVG filter. Fills the filter subregion with the color and opacity defined by flood-color and flood-opacity attributes",
    fe_func_a="feFuncA"; "SVG filter. Sub-element to feComponentTransfer",
    fe_func_b="feFuncB"; "SVG filter. Sub-element to feComponentTransfer",
    fe_func_g="feFuncG"; "SVG filter. Sub-element to feComponentTransfer",
    fe_func_r="feFuncR"; "SVG filter. Sub-element to feComponentTransfer",
    fe_gaussian_blur="feGaussianBlur"; "SVG filter. Blurs the graphic",
    fe_image="feImage"; "SVG filter. Gets graphic data from an external source and provides the pixel data as output",
    fe_merge="feMerge"; "SVG filter. Blends input graphic layers (applies filter effects concurrently instead of sequentially)",
    fe_merge_node="feMergeNode"; "SVG filter. Takes the result of another filter to be processed by its parent <feMerge>",
    fe_morphology="feMorphology"; "SVG filter. Erodes or dilates the graphic (for fattening or thinning effects)",
    fe_offset="feOffset"; "SVG filter. Offsets the input graphic",
    fe_point_light="fePointLight"; "SVG filter. Specifies a light source that allows creating a point light effect",
    fe_specular_lighting="feSpecularLighting"; "SVG filter. Lights a source graphic by using the alpha channel as a bump map",
    fe_spot_light="feSpotLight"; "SVG filter. Specifies a light source that allows creating a spotlight effect",
    fe_tile="feTile"; "SVG filter. Fills a target rectangle with a repeated pattern of an input graphic",
    fe_turbulence="feTurbulence"; "SVG filter. Creates a graphic with the Perlin turbulence function",
    filter; "A container for filter effects",
    foreign_object="foreignObject"; "Includes elements from a different XML namespace",
    g; "A container for grouping other SVG elements",
    image; "Includes an image in SVG (must be .jpeg, .png, or other SVG files)",
    line; "Creates a line",
    linear_gradient="linearGradient"; "Defines a linear gradient",
    marker; "Defines a graphic that is used to draw arrowheads or polymarkers on a specific <path>, <line>, <polyline> or <polygon> element",
    mask; "Defines an alpha mask for compositing the current object into the background. Masking is a combination of opacity values and clipping. Like clipping you can use shapes, text or paths to define sections of the mask. The default state of a mask is fully transparent which is the opposite of clipping plane. The graphics in a mask sets how opaque portions of the mask are",
    metadata; "Applies metadata to SVG content",
    mpath; "A sub-element for the <animateMotion> element which provides the ability to reference an external <path> element as the definition of a motion path",
    path; "Defines a shape",
    pattern; "Defines an object that can be redrawn at repeated x- and y-coordinate intervals",
    polygon; r#"Creates a graphic that contains at least three sides. Polygons are made of straight lines, and the shape is "closed""#,
    polyline; "Defines any shape that consists of only straight lines. The shape is open",
    radial_gradient="radialGradient"; "Defines a radial gradient",
    rect; "Defines a rectangle",
    script; "Container for scripts in SVG",
    set; "Sets the value of an attribute for a specified duration",
    stop; "The stops for a linear or radial gradient",
    style; "Allows style sheets to be embedded directly within SVG",
    switch; "None",
    symbol; "Define graphical template objects which can be instantiated by a <use> element",
    text; "Defines a text",
    text_path="textPath"; "Renders text along the shape of a path",
    title; "A text description for elements in SVG - not displayed as part of the graphics. Browsers usually display the text as a tooltip",
    tspan; "Defines a subtext within a <text> element",
    use; "Takes a node within the SVG document, and duplicates it somewhere else.",
    view; "How to view the graphic (zoom level or detail view)"
);

impl SvgElement {
    set_attr!(
        alignment_baseline = "alignment-baseline",
        baseline_shift = "baseline-shift",
        class,
        clip,
        clip_path = "clip-path",
        clip_rule = "clip-rule",
        color,
        color_interpolation = "color-interpolation",
        color_interpolation_filters = "color-interpolation-filters",
        cursor,
        cx,
        cy,
        d,
        dx,
        dy,
        direction,
        display,
        dominant_baseline = "dominant-baseline",
        fill,
        fill_opacity = "fill-opacity",
        fill_rule = "fill-rule",
        filter,
        flood_color = "flood-color",
        flood_opacity = "flood-opacity",
        font_family = "font-family",
        font_size = "font-size",
        font_size_adjust = "font-size-adjust",
        font_stretch = "font-stretch",
        font_style = "font-style",
        font_variant = "font-variant",
        font_weight = "font-weight",
        height,
        id,
        image_rendering = "image-rendering",
        kerning,
        lang,
        letter_spacing = "letter-spacing",
        marker_end = "marker-end",
        marker_mid = "marker-mid",
        marker_start = "marker-start",
        mask,
        opacity,
        path,
        points,
        preserve_aspect_ratio = "preserveAspectRatio",
        r,
        radius,
        rotate,
        rx,
        ry,
        scale,
        stop_color = "stop-color",
        stop_opacity = "stop-opacity",
        stroke,
        stroke_dasharray = "stroke-dasharray",
        stroke_dashoffset = "stroke-dashoffset",
        stroke_linecap = "stroke-linecap",
        stroke_linejoin = "stroke-linejoin",
        stroke_opacity = "stroke-opacity",
        stroke_width = "stroke-width",
        style,
        tabindex,
        text_anchor = "text-anchor",
        text_decoration = "text-decoration",
        transform,
        view_box = "viewBox",
        visibility,
        width,
        x,
        x1,
        x2,
        xlink_actuate = "xlink:actuate",
        xlink_arcrole = "xlink:arcrole",
        xlink_href = "xlink:href",
        xlink_role = "xlink:role",
        xlink_show = "xlink:show",
        xlink_title = "xlink:title",
        xlink_type = "xlink:type",
        xml_base = "xml:base",
        xml_lang = "xml:lang",
        xml_space = "xml:space",
        y,
        y1,
        y2,
        z
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::Render;

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
        insta::assert_snapshot!(res, @r#"
        <svg xmlns="http://www.w3.org/2000/svg">
          <rect width="100" height="20" fill="green" stroke="black" stroke-width="2"></rect>
        </svg>
        "#);
    }

    #[test]
    fn svg_inline_tags() {
        let res = svg()
            .add_child(
                svg_text()
                    .add_child("Hello")
                    .add_child(svg_tspan().add_child("world")),
            )
            .render();
        insta::assert_snapshot!(res, @r#"<svg xmlns="http://www.w3.org/2000/svg"><text>Hello<tspan>world</tspan></text></svg>"#);
    }
}
