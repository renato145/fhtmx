use darling::FromMeta;
use quote::quote;
use syn::Expr;

pub struct ExprOrString(pub Expr);

impl darling::FromMeta for ExprOrString {
    fn from_expr(expr: &Expr) -> darling::Result<Self> {
        Ok(Self(expr.clone()))
    }
}

#[derive(Default)]
pub enum Mode {
    #[default]
    List,
    Table,
    TableRight,
}

impl darling::FromMeta for Mode {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) = expr
        {
            match s.value().to_lowercase().as_str() {
                "list" => Ok(Self::List),
                "table" => Ok(Self::Table),
                "table_right" => Ok(Self::TableRight),
                _ => Err(darling::Error::custom(format!(
                    "unknown mode '{}', expected one of: list, table, table_right",
                    s.value()
                ))
                .with_span(s)),
            }
        } else {
            Err(darling::Error::unexpected_expr_type(expr))
        }
    }
}

#[derive(Default)]
pub enum PostProc {
    #[default]
    None,
    Flag,
    Custom(syn::Expr),
}

impl darling::FromMeta for PostProc {
    fn from_none() -> Option<Self> {
        Some(Self::None)
    }

    fn from_word() -> darling::Result<Self> {
        Ok(Self::Flag)
    }

    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        Ok(Self::Custom(expr.clone()))
    }
}

#[derive(FromMeta)]
pub enum DaisyColorAttr {
    Primary,
    Secondary,
    Accent,
    Neutral,
    Info,
    Success,
    Warning,
    Error,
    Base100,
    Base200,
    Base300,
}

impl DaisyColorAttr {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Primary => quote! { fhtmx::prelude::DaisyColor::Primary.bg_content() },
            Self::Secondary => quote! { fhtmx::prelude::DaisyColor::Secondary.bg_content() },
            Self::Accent => quote! { fhtmx::prelude::DaisyColor::Accent.bg_content() },
            Self::Neutral => quote! { fhtmx::prelude::DaisyColor::Neutral.bg_content() },
            Self::Info => quote! { fhtmx::prelude::DaisyColor::Info.bg_content() },
            Self::Success => quote! { fhtmx::prelude::DaisyColor::Success.bg_content() },
            Self::Warning => quote! { fhtmx::prelude::DaisyColor::Warning.bg_content() },
            Self::Error => quote! { fhtmx::prelude::DaisyColor::Error.bg_content() },
            Self::Base100 => quote! { fhtmx::prelude::DaisyColor::Base100.bg_content() },
            Self::Base200 => quote! { fhtmx::prelude::DaisyColor::Base200.bg_content() },
            Self::Base300 => quote! { fhtmx::prelude::DaisyColor::Base300.bg_content() },
        }
    }
}
