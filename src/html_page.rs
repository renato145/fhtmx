use crate::elements::*;

#[derive(Clone, Debug)]
pub struct HtmlPage {
    doctype: bool,
    custom_html_node: Option<HtmlElement<&'static str, HtmlGenericElement>>,
    title: Option<String>,
    description: Option<String>,
    meta_charset: bool,
    meta_viewport: bool,
    header_children: Option<HtmlElements>,
    body_children: Option<HtmlElements>,
}

impl HtmlPage {
    pub fn new() -> Self {
        Self {
            doctype: true,
            custom_html_node: None,
            title: None,
            description: None,
            meta_charset: true,
            meta_viewport: true,
            header_children: None,
            body_children: None,
        }
    }

    pub fn custom_html_node(mut self, node: HtmlElement<&'static str, HtmlGenericElement>) -> Self {
        self.custom_html_node = Some(node);
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

    pub fn add_header_child<T: HtmlRender + 'static>(mut self, child: T) -> Self {
        match self.header_children.as_mut() {
            Some(header) => {
                header.push(Box::new(child));
            }
            None => {
                self.header_children = Some(vec![Box::new(child)]);
            }
        }
        self
    }

    pub fn set_body(mut self, children: HtmlElements) -> Self {
        self.body_children = Some(children);
        self
    }

    pub fn add_body_child<T: HtmlRender + 'static>(mut self, child: T) -> Self {
        match self.body_children.as_mut() {
            Some(header) => {
                header.push(Box::new(child));
            }
            None => {
                self.body_children = Some(vec![Box::new(child)]);
            }
        }
        self
    }

    pub fn html_elements(self) -> HtmlElements {
        let mut elements = Vec::new();
        if self.doctype {
            elements.push(doctype_html().boxed());
        }
        let html_node = self.custom_html_node.unwrap_or_else(html);
        let mut header = head();
        if let Some(page_title) = self.title {
            header = header.add_child(title().inner(&page_title));
        }
        if self.meta_charset {
            header = header.add_child(meta().set_attr("charset", "UTF-8"));
        }
        if self.meta_viewport {
            header = header.add_child(meta().set_attr("name", "viewport").set_attr(
                "content",
                "width=device-width, initial-scale=1.0, maximum-scale=1.0",
            ));
        }
        if let Some(description) = self.description {
            header = header.add_child(
                meta()
                    .set_attr("name", "description")
                    .set_attr("content", description),
            );
        }
        if let Some(children) = self.header_children {
            header = header.add_children(children);
        }
        let mut body = body();
        if let Some(children) = self.body_children {
            body = body.add_children(children);
        }
        elements.push(html_node.add_child(header).add_child(body).boxed());
        elements
    }

    pub fn render(self) -> String {
        self.html_elements().render()
    }

    pub fn render_sorted(self) -> String {
        self.html_elements().render_sorted()
    }
}

impl Default for HtmlPage {
    fn default() -> Self {
        Self::new()
    }
}
