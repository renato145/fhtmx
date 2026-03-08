use url::form_urlencoded;

pub struct UrlBuilder {
    pub base: String,
    query_encoder: Option<form_urlencoded::Serializer<'static, String>>,
}

impl UrlBuilder {
    pub fn new(base: impl ToString) -> Self {
        Self {
            base: base.to_string(),
            query_encoder: None,
        }
    }

    pub fn push_query_mut(&mut self, name: &str, value: &str) {
        self.query_encoder
            .get_or_insert(form_urlencoded::Serializer::new(String::new()))
            .append_pair(name, value);
    }

    pub fn push_query(mut self, name: &str, value: &str) -> Self {
        self.push_query_mut(name, value);
        self
    }

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
