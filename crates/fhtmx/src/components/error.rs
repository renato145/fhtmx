use crate::{
    element::Element,
    html_element::*,
    prelude::{FhtmxToast, mk_alert_error, mk_callout_error},
};
#[cfg(feature = "actix")]
use actix_web::{HttpResponse, ResponseError};
use std::fmt::{self, Write};

pub type FhtmxResult<T> = Result<T, FhtmxError>;

/// An error that can be rendered with fhtmx using `mk_callout_error`
pub struct FhtmxError {
    pub context: Option<String>,
    pub source: Option<Box<dyn std::error::Error>>,
    pub hx_retarget: Option<String>,
    pub hx_reswap: Option<String>,
    pub do_trace: bool,
    pub id: Option<String>,
    xtra_classes: Option<String>,
    pub as_toast: bool,
    pub hide_source: bool,
}

impl FhtmxError {
    /// Defines a new custom error as toast and with tracing
    pub fn custom_error(e: impl ToString) -> Self {
        Self {
            context: Some(e.to_string()),
            source: None,
            hx_retarget: None,
            hx_reswap: None,
            do_trace: true,
            id: None,
            xtra_classes: None,
            as_toast: true,
            hide_source: false,
        }
    }

    /// Creates a FhtmxError from `e` as toast and with tracing
    pub fn from_error<E>(e: E) -> Self
    where
        E: std::error::Error + 'static,
    {
        Self {
            context: None,
            source: Some(Box::new(e)),
            hx_retarget: None,
            hx_reswap: None,
            do_trace: true,
            id: None,
            xtra_classes: None,
            as_toast: true,
            hide_source: false,
        }
    }

    pub fn get_main_error(&self) -> String {
        match (&self.context, &self.source) {
            (Some(s), _) => s.to_string(),
            (None, Some(source)) => source.to_string(),
            _ => unreachable!(),
        }
    }

    pub fn get_source_error(&self) -> Option<String> {
        let mut current = match (&self.context, &self.source) {
            (Some(_), None) => return None,
            (Some(_), Some(source)) => Some(source.as_ref()),
            (None, Some(source)) => source.as_ref().source(),
            _ => unreachable!(),
        };
        current?;
        let mut res = String::new();
        let mut is_first = true;
        while let Some(cause) = current {
            if !is_first {
                writeln!(&mut res).unwrap();
            }
            write!(&mut res, "Caused by:").unwrap();
            for o in cause.to_string().lines() {
                write!(&mut res, "\n\t{o}").unwrap();
            }
            current = cause.source();
            is_first = false;
        }
        Some(res)
    }

    /// Sets the context of the error
    pub fn set_context(mut self, context: impl ToString) -> Self {
        self.context = Some(context.to_string());
        self
    }

    /// Retargets the error response to an error container id. When rendering as toast (default),
    /// default retarget is "#toast-container"
    pub fn hx_retarget(mut self, target: impl ToString) -> Self {
        self.hx_retarget = Some(target.to_string());
        self
    }

    /// Sets the swap strategy for the error response. When rendering as toast (default),
    /// default reswap is `HXSwap::AfterBegin`
    pub fn hx_reswap(mut self, swap: impl ToString) -> Self {
        self.hx_reswap = Some(swap.to_string());
        self
    }

    /// Do not log the error when rendering response
    pub fn skip_tracing(mut self) -> Self {
        self.do_trace = false;
        self
    }

    /// Add id to the rendered error
    pub fn set_id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Add extra classes to the rendered error
    pub fn set_xtra_classes(mut self, xtra_classes: impl ToString) -> Self {
        self.xtra_classes = Some(xtra_classes.to_string());
        self
    }

    /// Do not set the rendered error as a toast
    pub fn disable_toast(mut self) -> Self {
        self.as_toast = false;
        self
    }

    /// Hides source errors when renders the error
    pub fn hide_source(mut self) -> Self {
        self.hide_source = true;
        self
    }

    pub fn as_element(&self) -> HtmlElement {
        let main_error = self.get_main_error();
        let mut error_html = match (self.hide_source, self.get_source_error()) {
            (true, _) | (false, None) => {
                mk_alert_error(main_error).set_opt_attr("id", self.id.as_ref())
            }
            (false, Some(s)) => mk_callout_error(
                Some(&main_error),
                pre().class("text-wrap text-sm").add(s),
                true,
            ),
        };
        if let Some(xtra_classes) = &self.xtra_classes {
            error_html = error_html.add_class(xtra_classes.clone());
        }
        if self.as_toast {
            error_html.setup_toast(false)
        } else {
            error_html
        }
    }

    #[cfg(feature = "actix")]
    pub fn render_response(&self) -> HttpResponse {
        use crate::htmx::HXSwap;
        use crate::render::Render;
        use actix_web::http::header::ContentType;

        let html_body = self.as_element().render();
        let mut builder = HttpResponse::Ok();
        builder.content_type(ContentType::html());
        match (self.as_toast, &self.hx_retarget) {
            (_, Some(target)) => {
                builder.append_header(("HX-Retarget", target.as_str()));
            }
            (true, None) => {
                builder.append_header(("HX-Retarget", "#toast-container"));
            }
            _ => {}
        }
        match (self.as_toast, &self.hx_reswap) {
            (_, Some(hx_swap)) => {
                builder.append_header(("HX-Reswap", hx_swap.as_str()));
            }
            (true, None) => {
                builder.append_header(("HX-Reswap", HXSwap::AfterBegin.to_string()));
            }
            _ => {}
        }
        builder.body(html_body)
    }
}

impl fmt::Display for FhtmxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.context, &self.source) {
            (Some(custom), _) => write!(f, "{custom}"),
            (None, Some(source)) => write!(f, "{source}"),
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for FhtmxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_main_error())?;
        if let Some(e) = self.get_source_error() {
            write!(f, "\n\n{e}")?;
        }
        Ok(())
    }
}

impl std::error::Error for FhtmxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl IntoHtmlElement for FhtmxError {
    fn into_element(self) -> HtmlElement {
        let main_error = self.get_main_error();
        let mut error_html = match (self.hide_source, self.get_source_error()) {
            (true, _) | (false, None) => mk_alert_error(main_error).set_opt_attr("id", self.id),
            (false, Some(s)) => mk_callout_error(
                Some(&main_error),
                pre().class("text-wrap text-sm").add(s),
                true,
            ),
        };
        if let Some(xtra_classes) = self.xtra_classes {
            error_html = error_html.add_class(xtra_classes);
        }
        if self.as_toast {
            error_html.setup_toast(false)
        } else {
            error_html
        }
    }
}

#[cfg(feature = "actix")]
impl ResponseError for FhtmxError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if self.do_trace {
            tracing::error!(
                error.cause_chain=?self, error.message=%self,
                "Failed to render FhtmxError."
            );
        }
        self.render_response()
    }
}

pub trait FhtmxErrorExt {
    /// Sets the context of the error
    fn fhtmx_set_context(self, context: impl ToString) -> Self;

    /// Retargets the error response to an error container id. When rendering as toast (default),
    /// default retarget is "#toast-container"
    fn fhtmx_retarget(self, target: impl ToString) -> Self;

    /// Sets the swap strategy for the error response. When rendering as toast (default),
    /// default reswap is `HXSwap::AfterBegin`
    fn fhtmx_hx_reswap(self, swap: impl ToString) -> Self;

    /// Do not log the error when rendering response
    fn fhtmx_skip_tracing(self) -> Self;

    /// Add id to the rendered error
    fn fhtmx_add_id(self, id: impl ToString) -> Self;

    /// Add extra classes to the rendered error
    fn fhtmx_add_classes(self, xtra_classes: impl ToString) -> Self;

    /// Do not set the rendered error as a toast
    fn fhtmx_disable_toast(self) -> Self;

    /// Hides source errors when renders the error
    fn fhtmx_hide_source(self) -> Self;
}

impl FhtmxErrorExt for FhtmxError {
    fn fhtmx_set_context(self, context: impl ToString) -> Self {
        self.set_context(context)
    }

    fn fhtmx_retarget(self, target: impl ToString) -> Self {
        self.hx_retarget(target)
    }

    fn fhtmx_hx_reswap(self, swap: impl ToString) -> Self {
        self.hx_reswap(swap)
    }

    fn fhtmx_skip_tracing(self) -> Self {
        self.skip_tracing()
    }

    fn fhtmx_add_id(self, id: impl ToString) -> Self {
        self.set_id(id)
    }

    fn fhtmx_add_classes(self, xtra_classes: impl ToString) -> Self {
        self.set_xtra_classes(xtra_classes)
    }

    fn fhtmx_disable_toast(self) -> Self {
        self.disable_toast()
    }

    fn fhtmx_hide_source(self) -> Self {
        self.hide_source()
    }
}

impl<T> FhtmxErrorExt for Result<T, FhtmxError> {
    fn fhtmx_set_context(self, context: impl ToString) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.fhtmx_set_context(context)),
        }
    }

    fn fhtmx_retarget(self, target: impl ToString) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.fhtmx_retarget(target)),
        }
    }

    fn fhtmx_hx_reswap(self, swap: impl ToString) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.fhtmx_hx_reswap(swap)),
        }
    }

    fn fhtmx_skip_tracing(self) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.fhtmx_skip_tracing()),
        }
    }

    fn fhtmx_add_id(self, id: impl ToString) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.fhtmx_add_id(id)),
        }
    }

    fn fhtmx_add_classes(self, xtra_classes: impl ToString) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.fhtmx_add_classes(xtra_classes)),
        }
    }

    fn fhtmx_disable_toast(self) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.disable_toast()),
        }
    }

    fn fhtmx_hide_source(self) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.hide_source()),
        }
    }
}

/// Provides the `fhtmx_context` method for `Result`, similar to `context` in the anyhow crate.
#[allow(clippy::result_large_err)]
pub trait FhtmxContext<T> {
    /// Wrap the error as a `FhtmxError`.
    fn into_fhtmx_error(self) -> Result<T, FhtmxError>;

    /// Wrap the error value with additional context for `FhtmxError`.
    fn fhtmx_context<C>(self, context: C) -> Result<T, FhtmxError>
    where
        C: fmt::Display + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_fhtmx_context<C, F>(self, f: F) -> Result<T, FhtmxError>
    where
        C: fmt::Display + 'static,
        F: FnOnce() -> C;
}

impl<T, E> FhtmxContext<T> for Result<T, E>
where
    E: std::error::Error + 'static,
{
    fn into_fhtmx_error(self) -> Result<T, FhtmxError> {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(FhtmxError::from_error(e)),
        }
    }

    fn fhtmx_context<C>(self, context: C) -> Result<T, FhtmxError>
    where
        C: fmt::Display + 'static,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(FhtmxError::from_error(e).fhtmx_set_context(context)),
        }
    }

    fn with_fhtmx_context<C, F>(self, f: F) -> Result<T, FhtmxError>
    where
        C: fmt::Display + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(FhtmxError::from_error(e).fhtmx_set_context(f())),
        }
    }
}

impl<T> FhtmxContext<T> for Option<T> {
    fn into_fhtmx_error(self) -> Result<T, FhtmxError> {
        match self {
            Some(ok) => Ok(ok),
            None => Err(FhtmxError::custom_error("Expected a value, found None.")),
        }
    }

    fn fhtmx_context<C>(self, context: C) -> Result<T, FhtmxError>
    where
        C: fmt::Display + 'static,
    {
        match self {
            Some(ok) => Ok(ok),
            None => Err(FhtmxError::custom_error(context)),
        }
    }

    fn with_fhtmx_context<C, F>(self, f: F) -> Result<T, FhtmxError>
    where
        C: fmt::Display + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Some(ok) => Ok(ok),
            None => Err(FhtmxError::custom_error(f())),
        }
    }
}

#[cfg(feature = "anyhow")]
impl From<anyhow::Error> for FhtmxError {
    fn from(e: anyhow::Error) -> Self {
        Self {
            context: None,
            source: Some(e.into_boxed_dyn_error()),
            hx_retarget: None,
            hx_reswap: None,
            do_trace: true,
            id: None,
            xtra_classes: None,
            as_toast: true,
            hide_source: false,
        }
    }
}

#[cfg(feature = "anyhow")]
#[allow(clippy::result_large_err)]
pub trait FhtmxAnyhowExt<T> {
    /// Converts `anyhow::Error` into `FhtmxError`
    fn into_fhtmx_error(self) -> Result<T, FhtmxError>;
}

#[cfg(feature = "anyhow")]
impl<T> FhtmxAnyhowExt<T> for Result<T, anyhow::Error> {
    fn into_fhtmx_error(self) -> Result<T, FhtmxError> {
        match self {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Render;
    use googletest::prelude::*;

    #[test]
    fn fhtmx_error_debug_works() {
        let source_error = std::io::Error::from(std::io::ErrorKind::NotADirectory);
        let e = FhtmxError::from_error(source_error).fhtmx_set_context("Some context.");
        let s = format!("{e:?}");
        insta::assert_snapshot!(s, @r"
        Some context.

        Caused by:
        	not a directory
        ");
    }

    #[test]
    fn fhtmx_anyhow_ext_works() {
        let e: std::result::Result<(), anyhow::Error> = Err(anyhow::anyhow!("Some error"));
        let res = e.into_fhtmx_error();
        let s = format!("{res:?}");
        insta::assert_snapshot!(s, @"Err(Some error)");
    }

    #[test]
    fn fhtmx_multiline_works() {
        let source_error = anyhow::anyhow!("Some context\nA line\n- another line");
        let e = FhtmxError::from(source_error).set_context("Main error.");
        let s = format!("{e:?}");
        insta::assert_snapshot!(s, @r"
        Main error.

        Caused by:
        	Some context
        	A line
        	- another line
        ");
    }

    #[gtest]
    fn render_response_works() {
        let s = FhtmxError::custom_error("Some error")
            .into_element()
            .render();
        expect_that!(s, contains_substring(r#"x-data="toast""#));
        let s = FhtmxError::custom_error("Some error")
            .disable_toast()
            .into_element()
            .render();
        expect_that!(s, not(contains_substring(r#"x-data="toast""#)));
    }

    #[cfg(feature = "anyhow")]
    #[gtest]
    fn render_hide_source_works() {
        use anyhow::Context;

        let e = "not-a-number"
            .parse::<i32>()
            .context("Some context from anyhow")
            .into_fhtmx_error()
            .fhtmx_set_context("Some context from ksh-ui")
            .fhtmx_hide_source()
            .unwrap_err();
        let s = e.into_element().render();
        assert_that!(
            s,
            all!(
                contains_substring("Some context from ksh-ui"),
                not(contains_substring("Some context from anyhow")),
                not(contains_substring("invalid digit found in string")),
            )
        );
    }
}
