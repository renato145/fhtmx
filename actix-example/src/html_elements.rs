use fhtmx::prelude::*;

pub fn todo_list_description(n: usize) -> HtmlElement {
    let s = match n {
        0 => "The list is empty, try adding some items...".to_string(),
        1 => "1 item".to_string(),
        n => format!("{} items:", n),
    };
    p().id("todo-list-description").add(s)
}

pub fn sort_list_btn(n: usize) -> HtmlElement {
    if n == 0 {
        div().id("sort-btn").hidden()
    } else {
        div()
            .id("sort-btn")
            .class("flex gap-x-2")
            .add(
                dc_btn()
                    .add_class("self-start btn-primary btn-sm")
                    .hx_post("/todo/sort")
                    .hx_target("#todo-list")
                    .add("Sort items"),
            )
            .add(
                dc_btn()
                    .add_class("self-start btn-error btn-sm")
                    .hx_post("/todo/clear")
                    .hx_target("#todo-list")
                    .hx_swap("innerHTML swapEmpty:true")
                    .hx_confirm("Are you sure?")
                    .add("Clear todo"),
            )
            .add(
                dc_btn()
                    .add_class("self-start btn-info btn-sm")
                    .hx_post("/js_invoke")
                    .hx_target("body")
                    .hx_swap(HXSwap::BeforeEnd)
                    .add("Call js"),
            )
    }
}
