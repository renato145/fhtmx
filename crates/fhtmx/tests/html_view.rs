use fhtmx::prelude::*;

#[test]
fn macro_works() {
    #[derive(HtmlView)]
    #[html_view(title = "User info")]
    struct User {
        name: &'static str,
        age: usize,
    }
    let x = User {
        name: "Karls",
        age: 20,
    };
    let res = x.html_view().render();
    println!("{res}");
    // insta::assert_snapshot!(res, @"");
}
