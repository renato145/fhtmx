use fake::{
    Dummy, Fake, Faker,
    faker::{lorem::en::Sentence, name::en::Name},
    rand::{SeedableRng, rngs::StdRng},
};
use fhtmx::prelude::*;

#[derive(HtmlView, Dummy)]
#[html_view(mode="table", title=self.custom_title(), class=self.custom_class(), mode_class="table-sm", postproc)]
struct UserInfo {
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "Sentence(4..10)")]
    description: String,
    #[dummy(faker = "-5000.0..5000.0")]
    #[html_view(value=self.salary())]
    salary: f32,
    #[html_view(alias = "is active?", value=self.active())]
    active: bool,
    #[dummy(faker = "18..60")]
    #[html_view(value_class = "italic")]
    age: usize,
    some_opt_value: Option<u8>,
}

impl UserInfo {
    fn custom_title(&self) -> String {
        format!("User info - {}", self.name)
    }

    fn custom_class(&self) -> String {
        if self.salary < 0.0 {
            DaisyColor::Error.bg_content()
        } else {
            DaisyColor::Info.bg_content()
        }
    }

    fn postproc(&self, card: HtmlElement) -> HtmlElement {
        card.update_html_element(0, |body| {
            body.insert_child(
                1,
                p().class("opacity-70 font-semibold text-sm")
                    .add("Some description can go here..."),
            )
            .add(
                dc_card_actions()
                    .add(dc_btn().add("Edit"))
                    .add(dc_btn().add_class("btn-error btn-soft").add("Remove")),
            )
        })
    }

    fn salary(&self) -> String {
        format!("S/ {:.2}", self.salary)
    }

    fn active(&self) -> HtmlElement {
        dc_toggle()
            .add_class("toggle-sm")
            .set_attr("onclick", "return false;")
            .set_attr("checked", self.active)
    }
}

fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    let users = (0..6)
        .map(|_| Faker.fake_with_rng(&mut rng))
        .collect::<Vec<UserInfo>>();

    let body = main_container()
        .add_class("mt-4")
        .add(
            h1().add("Html view example")
                .class("text-2xl font-bold text-center"),
        )
        .add(
            div()
                .class("mt-4 p-2 grid grid-cols-2 lg:grid-cols-3 gap-4")
                .add_children(users.iter().map(|o| o.html_view())),
        );
    let page = HtmlPage::new()
        .custom_html_node(html().set_attr("data-theme", "dark").lang("en"))
        .title("Html view")
        .add_header_node(daisy_link())
        .add_header_node(source_tailwind())
        .add_body_node(body)
        .render();
    std::fs::write("examples/html_view_table.html", page).unwrap();
    println!("Done!");
}
