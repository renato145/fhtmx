// use crate::icons;
// use fhtmx::prelude::*;
// use paste::paste;
//
// /// An alert message
// #[derive(Clone, Debug)]
// pub struct Alert<C: HtmlRender + 'static> {
//     color: &'static str,
//     no_bg: bool,
//     icon: HtmlElement<&'static str, HtmlSvgElement>,
//     child: C,
// }
//
// macro_rules! new_alert {
//     ($color:expr) => {
//         paste! {
//             #[doc = "Creates a `Alert::" $color "` `Alert`."]
//             pub fn $color(child: C) -> Self {
//                 Self::new(stringify!($color), child, icons::$color())
//             }
//         }
//     };
//
//     ($color:expr, $($rest:expr), +) => {
//         new_alert!($color);
//         new_alert!($($rest), +);
//     };
// }
//
// impl<C: HtmlRender + 'static> Alert<C> {
//     pub fn new(
//         color: &'static str,
//         child: C,
//         icon: HtmlElement<&'static str, HtmlSvgElement>,
//     ) -> Self {
//         Self {
//             color,
//             no_bg: false,
//             icon,
//             child,
//         }
//     }
//
//     new_alert!(info, success, warning, error);
//
//     pub fn no_bg(mut self) -> Self {
//         self.no_bg = true;
//         self
//     }
//
//     pub fn html(self) -> HtmlElement<&'static str, HtmlGenericElement> {
//         let mut class = "alert".to_string();
//         if !self.no_bg {
//             class.push_str(" alert-");
//             class.push_str(self.color);
//         }
//         let mut icon_class = "h-6 w-6".to_string();
//         if self.no_bg {
//             icon_class.push_str(" text-");
//             icon_class.push_str(self.color);
//         }
//         div()
//             .class(class)
//             .set_attr("role", "alert")
//             .add_child(self.icon.class(icon_class))
//             .add_child(self.child)
//     }
// }
//
// macro_rules! new_alert_from_str {
//     ($color:expr) => {
//         paste! {
//             #[doc = "Creates a `Alert::" $color "` `Alert`."]
//             pub fn [<alert_ $color>](s: impl AsRef<str>) -> Alert<HtmlElement<&'static str, HtmlGenericElement>> {
//                 let child = span().class("whitespace-pre-wrap").inner(s);
//                 Alert::new(stringify!($color), child, icons::$color())
//             }
//         }
//     };
//
//     ($color:expr, $($rest:expr), +) => {
//         new_alert_from_str!($color);
//         new_alert_from_str!($($rest), +);
//     };
// }
//
// new_alert_from_str!(info, success, warning, error);
