use crate::{element::Element, html_element::*, prelude::dc_loading};
use pastey::paste;
use std::borrow::Cow;

/// A lazy load component
pub fn mk_lazy_load(label: Option<&str>, class: impl Into<Cow<'static, str>>) -> HtmlElement {
    let icon = dc_loading().add_class(class);
    match label {
        Some(lbl) => div().add(p().add(lbl).add(icon.add_class("ml-2"))),
        None => div().add(icon),
    }
}

macro_rules! new_lazy {
    ($icon: expr) => {
        paste! {
            #[doc = "Creates a lazy load component with " $icon "."]
            pub fn [<lazy_load_ $icon>](label: Option<&str>) -> HtmlElement {
                mk_lazy_load(label, concat!("loading-", stringify!($icon)))
            }
        }
    };

    ($icon: expr; $size:expr) => {
        paste! {
            #[doc = "Creates a lazy load component with " $icon " and size " $size "."]
            pub fn [<lazy_load_ $icon _ $size>](label: Option<&str>) -> HtmlElement {
                mk_lazy_load(label, concat!("loading-", stringify!($icon), " loading-", stringify!($size)))
                .add_class(concat!("text-", stringify!($size)))
            }
        }
    };

    ($($icon:expr; [$($size:expr),+]),* $(,)?) => {
        $(
            new_lazy!($icon);
            $(
                new_lazy!($icon; $size);
            )+
            // new_lazy!($icon $(; $size)?);
        )*
    };
}

new_lazy!(
    ball; [xs, sm, lg, xl],
    bars; [xs, sm, lg, xl],
    dots; [xs, sm, lg, xl],
    infinity; [xs, sm, lg, xl],
    ring; [xs, sm, lg, xl],
    spinner; [xs, sm, lg, xl],
);

/// Alias for `lazy_load_bars`
pub fn lazy_load(label: Option<&str>) -> HtmlElement {
    lazy_load_bars(label)
}
