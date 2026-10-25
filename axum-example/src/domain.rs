use fhtmx::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TodoListItem {
    pub id: Uuid,
    pub value: String,
}

impl TodoListItem {
    pub fn new(value: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            value,
        }
    }

    pub fn html_form(&self) -> HtmlElement {
        li().class("my-2").id(self.id.to_string()).add(
            form()
                .class("contents")
                .add(
                    dc_input()
                        .typ("text")
                        .name("content")
                        .value(&self.value)
                        .set_attr("onfocus", "this.select()")
                        .autofocus()
                        .required(),
                )
                .add(
                    dc_btn()
                        .add_class("ml-2 btn-primary")
                        .hx_put(format!("/todo/{}", self.id))
                        .hx_target(HXTarget::Closest("li"))
                        .hx_swap(HXSwap::OuterHTML)
                        .add("ok"),
                )
                .add(
                    dc_btn()
                        .add_class("ml-1 btn-error")
                        .hx_get(format!("/todo/{}", self.id))
                        .hx_target(HXTarget::Closest("li"))
                        .hx_swap(HXSwap::OuterHTML)
                        .add("cancel"),
                ),
        )
    }
}

impl IntoHtmlElement for TodoListItem {
    fn into_element(self) -> HtmlElement {
        li().id(self.id.to_string()).add_children(children![
            self.value,
            dc_link()
                .add_class("ml-2 link-info text-xs")
                .hx_get(format!("/todo/{}/form", self.id))
                .hx_target(HXTarget::Closest("li"))
                .hx_swap(HXSwap::OuterHTML)
                .add("modify"),
            dc_link()
                .add_class("ml-1 link-warning text-xs")
                .hx_get(format!("/todo/invalid-{}/form", self.id))
                .hx_target(HXTarget::Closest("li"))
                .hx_swap(HXSwap::OuterHTML)
                .add("invalid modify"),
            dc_link()
                .add_class("ml-1 link-error text-xs")
                .hx_delete(format!("/todo/{}", self.id))
                .hx_target(HXTarget::Closest("li"))
                .hx_swap(HXSwap::Delete)
                .hx_confirm("Are you sure?")
                .add("remove"),
        ])
    }
}
