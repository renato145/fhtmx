//! Proc-macros for fhtmx.
//!
//! Currently provides the [`HtmlView`](crate::derive_html_view) derive macro.

#![warn(missing_docs)]

mod utils;

use crate::utils::{DaisyColorAttr, ExprOrString, Mode, PostProc};
use darling::{FromDeriveInput, FromField, ast::Data};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

#[derive(FromField)]
#[darling(attributes(html_view))]
struct HtmlViewField {
    ident: Option<Ident>,
    #[darling(default)]
    alias: Option<String>,
    #[darling(default)]
    value: Option<ExprOrString>,
    #[darling(default)]
    value_display: bool,
    #[darling(default)]
    value_debug: bool,
    #[darling(default)]
    value_debug_pretty: bool,
    #[darling(default)]
    row_class: Option<String>,
    #[darling(default)]
    value_class: Option<String>,
    #[darling(default)]
    skip: bool,
}

impl HtmlViewField {
    fn validate(self) -> darling::Result<Self> {
        let count = self.value.is_some() as u8
            + self.value_display as u8
            + self.value_debug as u8
            + self.value_debug_pretty as u8;
        if count > 1 {
            return Err(darling::Error::custom(
                "only one of `value`, `value_display`, `value_debug`, or `value_debug_pretty` can be set",
            ));
        }
        Ok(self)
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(html_view), supports(struct_named))]
struct HtmlViewInput {
    ident: Ident,
    data: Data<(), HtmlViewField>,
    #[darling(default)]
    mode: Mode,
    #[darling(default)]
    title: Option<ExprOrString>,
    #[darling(default)]
    color: Option<DaisyColorAttr>,
    #[darling(default)]
    class: Option<ExprOrString>,
    #[darling(default)]
    mode_class: Option<ExprOrString>,
    #[darling(default)]
    postproc: PostProc,
}

/// Derive macro that implements `HtmlView` for a named struct.
///
/// # Attributes
///
/// On the struct:
/// - `title = "..."` or `title = expr`: Sets the card title.
/// - `mode = "list" | "table" | "table_right"`: Layout mode (default: list).
/// - `color = "primary" | ...`: `DaisyUI` color for the card.
/// - `class = expr`: Extra CSS classes.
/// - `postproc = expr`: Custom post-processing function.
///
/// On fields:
/// - `skip`: Omits the field from the view.
/// - `alias = "..."`: Custom label instead of the field name.
/// - `value = expr`: Custom value expression.
/// - `value_display`: Uses `format!("{}", ...)` for the value.
/// - `value_debug`: Uses `format!("{:?}", ...)` for the value.
/// - `value_debug_pretty`: Uses `format!("{:#?}", ...)` inside a `<pre>`.
/// - `row_class = "..."`: CSS class for the list row.
/// - `value_class = "..."`: CSS class for the value cell.
#[proc_macro_derive(HtmlView, attributes(html_view))]
pub fn derive_html_view(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let parsed = match HtmlViewInput::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };
    let mode = parsed.mode;

    // Extract fields from the parsed data
    let fields = parsed.data.take_struct().expect("expected named struct");
    let field_items = fields
        .into_iter()
        .map(|o| o.validate().unwrap())
        .filter(|o| !o.skip)
        .map(|o| {
            let field_ident = o.ident.unwrap();
            let key = o.alias.unwrap_or_else(|| field_ident.to_string());
            let value = match (
                o.value,
                o.value_display,
                o.value_debug,
                o.value_debug_pretty,
            ) {
                (Some(ExprOrString(expr)), false, false, false) => quote! { #expr },
                (None, true, false, false) => quote! { format!("{}", self.#field_ident) },
                (None, false, true, false) => quote! { format!("{:?}", self.#field_ident) },
                (None, false, false, true) => {
                    quote! { pre().add(format!("{:#?}", self.#field_ident)).class("text-wrap") }
                }
                (None, false, false, false) => {
                    quote! { (&self.#field_ident).html_content() }
                }
                _ => unreachable!(),
            };
            let row_class = o.row_class.unwrap_or_else(|| "p-1".to_string());
            let value_class_call = o.value_class.map(|x| quote! { .class(#x) });
            match mode {
                Mode::List => quote! {
                    .add(
                        html_list_row(
                            div().class("font-bold").add(#key),
                            div()#value_class_call.add(#value)
                        )
                        .add_class(#row_class)
                    )
                },
                Mode::Table => quote! {
                    .add(
                        tr()
                        .add(th().add(#key))
                        .add(td().add(#value))
                    )
                },
                Mode::TableRight => quote! {
                    .add(
                        tr()
                        .add(th().class("text-right").add(#key))
                        .add(td().add(#value))
                    )
                },
            }
        });

    let struct_name = parsed.ident;

    let mode_class_call = parsed
        .mode_class
        .map(|ExprOrString(expr)| quote! { .add_class(#expr) });
    let content = match mode {
        Mode::List => quote! {
            dc_list() #mode_class_call #(#field_items)*
        },
        Mode::Table | Mode::TableRight => quote! {
            div()
            .class("overflow-x-auto")
            .add(dc_table() #mode_class_call .add(tbody() #(#field_items)*))
        },
    };

    let title = match parsed.title {
        Some(ExprOrString(expr)) => quote! { Some(#expr.as_ref()) },
        None => quote! { None },
    };
    let color_call = parsed
        .color
        .map(|x| x.to_tokens())
        .map(|x| quote! { .add_class(#x) });
    let class_call = parsed
        .class
        .map(|ExprOrString(expr)| quote! { .add_class(#expr) });
    let card = quote! { mk_card(#title, self.html_content()) #color_call #class_call };
    let postproc_card = match parsed.postproc {
        PostProc::None => quote! { #card },
        PostProc::Flag => quote! { self.postproc(#card) },
        PostProc::Custom(expr) => quote! { #expr(#card) },
    };

    quote! {
        impl HtmlView for #struct_name {
            fn html_content(&self) -> HtmlNode {
                #content .into_node()
            }

            fn html_view(&self) -> HtmlNode {
                #postproc_card .into_node()
            }
        }
    }
    .into()
}
