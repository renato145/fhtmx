use crate::{
    element::Element,
    html_element::*,
    node::{HtmlNode, IntoNode},
    prelude::{AttributeValue, IntoAttributeValue},
    render::Render,
};

#[derive(Clone, Debug)]
pub struct HtmlPage {
    doctype: bool,
    custom_html_node: Option<HtmlElement>,
    title: Option<String>,
    description: Option<String>,
    meta_charset: Option<AttributeValue>,
    meta_viewport: Option<AttributeValue>,
    header_nodes: Vec<HtmlNode>,
    body_nodes: Vec<HtmlNode>,
}

impl HtmlPage {
    pub fn new() -> Self {
        Self {
            doctype: true,
            custom_html_node: None,
            title: None,
            description: None,
            meta_charset: "UTF-8".into_attr(),
            meta_viewport: "width=device-width, initial-scale=1.0, maximum-scale=1.0".into_attr(),
            header_nodes: Vec::new(),
            body_nodes: Vec::new(),
        }
    }

    pub fn custom_html_node(mut self, el: HtmlElement) -> Self {
        self.custom_html_node = Some(el);
        self
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Default value: "UTF-8"
    pub fn set_meta_charset(mut self, charset: impl IntoAttributeValue) -> Self {
        self.meta_charset = charset.into_attr();
        self
    }

    /// Default value: "width=device-width, initial-scale=1.0, maximum-scale=1.0"
    pub fn set_meta_viewport(mut self, viewport: impl IntoAttributeValue) -> Self {
        self.meta_viewport = viewport.into_attr();
        self
    }

    pub fn add_header_node(mut self, node: impl IntoNode) -> Self {
        self.header_nodes.push(node.into_node());
        self
    }

    pub fn add_body_node(mut self, node: impl IntoNode) -> Self {
        self.body_nodes.push(node.into_node());
        self
    }

    pub fn html_nodes(self) -> HtmlNode {
        let mut nodes = Vec::with_capacity(1);
        if self.doctype {
            nodes.push(HtmlNode::Doctype);
        }
        let html_node = self.custom_html_node.unwrap_or_else(html);
        let mut header = head();
        if let Some(page_title) = self.title {
            header = header.add(title().add(page_title));
        }
        if let Some(description) = self.description {
            header = header.add(meta().name("description").set_attr("content", description));
        }
        if let Some(charset) = self.meta_charset {
            header = header.add(meta().set_attr("charset", charset));
        }
        if let Some(viewport) = self.meta_viewport {
            header = header.add(
                meta()
                    .set_attr("name", "viewport")
                    .set_attr("content", viewport),
            );
        }
        nodes.push(
            html_node
                .add(header.add_children(self.header_nodes))
                .add(body().add_children(self.body_nodes))
                .into_node(),
        );
        HtmlNode::Fragment(nodes)
    }

    pub fn render(self) -> String {
        self.html_nodes().render()
    }
}

impl Default for HtmlPage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn render_page() {
        let page = HtmlPage::new()
            .title("My page title")
            .description("Some test page")
            .add_body_node(h1().add("A nice title"))
            .add_body_node(div().add(p().add("Some content...")))
            .render();
        insta::assert_snapshot!(page, @r#"
        <!DOCTYPE html>
        <html>
          <head>
            <title>My page title</title>
            <meta name="description" content="Some test page" />
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0" />
          </head>
          <body>
            <h1>A nice title</h1>
            <div>
              <p>Some content...</p>
            </div>
          </body>
        </html>
        "#);
    }
}
