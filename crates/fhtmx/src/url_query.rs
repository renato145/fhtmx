use url::form_urlencoded;

/// Builds a URL with optional query parameters.
///
/// Values are appended with `form_urlencoded`, so names and values are properly escaped.
///
/// # Examples
///
/// ```
/// use fhtmx::prelude::UrlBuilder;
///
/// let url = UrlBuilder::new("/items")
///     .push_query("page", "2")
///     .push_query("q", "rust & htmx")
///     .finish();
/// assert_eq!(url, "/items?page=2&q=rust+%26+htmx");
/// ```
pub struct UrlBuilder {
    /// The base URL path.
    pub base: String,
    query_encoder: Option<form_urlencoded::Serializer<'static, String>>,
}

impl UrlBuilder {
    /// Creates a new builder with the given base URL.
    pub fn new(base: impl ToString) -> Self {
        Self {
            base: base.to_string(),
            query_encoder: None,
        }
    }

    /// Appends a query parameter mutably.
    pub fn push_query_mut(&mut self, name: &str, value: &str) {
        self.query_encoder
            .get_or_insert(form_urlencoded::Serializer::new(String::new()))
            .append_pair(name, value);
    }

    /// Appends a query parameter and returns `self`.
    pub fn push_query(mut self, name: &str, value: &str) -> Self {
        self.push_query_mut(name, value);
        self
    }

    /// Builds and returns the final URL string.
    pub fn finish(self) -> String {
        let mut url = self.base;
        if let Some(mut encoder) = self.query_encoder {
            let query = encoder.finish();
            if !query.is_empty() {
                url.push('?');
                url.push_str(&query);
            }
        }
        url
    }
}

impl From<UrlBuilder> for String {
    fn from(x: UrlBuilder) -> Self {
        x.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn url_builder_works() {
        let res = UrlBuilder::new("/tst").push_query("x", "10").finish();
        expect_that!(res, eq("/tst?x=10"));

        let res = UrlBuilder::new("/tst").push_query("x", "x-_&1").finish();
        expect_that!(res, eq("/tst?x=x-_%261"));

        let res = UrlBuilder::new("/tst")
            .push_query("some key", "xyz")
            .push_query("x", "1")
            .push_query("y", "2")
            .finish();
        expect_that!(res, eq("/tst?some+key=xyz&x=1&y=2"));
    }
}
