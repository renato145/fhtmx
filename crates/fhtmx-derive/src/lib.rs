mod utils;

use crate::utils::{DaisyColorAttr, ExprOrString, PostProc};
use darling::{FromDeriveInput, FromField, ast::Data};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

#[derive(FromField)]
#[darling(attributes(html_view))]
struct HtmlViewField {
    ident: Option<Ident>,
    // ty: Type,
    #[darling(default)]
    alias: Option<String>,
    #[darling(default)]
    value: Option<ExprOrString>,
    #[darling(default)]
    row_class: Option<String>,
    #[darling(default)]
    value_class: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(html_view), supports(struct_named))]
struct HtmlViewInput {
    ident: Ident,
    data: Data<(), HtmlViewField>,
    #[darling(default)]
    title: Option<ExprOrString>,
    #[darling(default)]
    color: Option<DaisyColorAttr>,
    #[darling(default)]
    class: Option<ExprOrString>,
    #[darling(default)]
    postproc: PostProc,
}

#[proc_macro_derive(HtmlView, attributes(html_view))]
pub fn derive_html_view(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let parsed = match HtmlViewInput::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    // Extract fields from the parsed data
    let fields = parsed.data.take_struct().expect("expected named struct");
    let field_items = fields.into_iter().map(|o| {
        let field_ident = o.ident.unwrap();
        let key = o.alias.unwrap_or_else(|| field_ident.to_string());
        let value = match o.value {
            Some(ExprOrString(expr)) => quote! { #expr },
            None => quote! { self.#field_ident.html_content() },
        };
        let row_class = o.row_class.unwrap_or_else(|| "p-1".to_string());
        let value_class_call = o.value_class.map(|x| quote! { .class(#x) });
        quote! {
            .add(
                html_list_row(
                    div().class("font-bold").add(#key),
                    div()#value_class_call.add(#value)
                )
                .add_class(#row_class)
            )
        }
    });

    let struct_name = parsed.ident;
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
            fn html_content(&self) -> HtmlElement {
                dc_list()#(#field_items)*
            }

            fn html_view(&self) -> HtmlElement {
                #postproc_card
            }
        }
    }
    .into()
}
