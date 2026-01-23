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
    insta::assert_snapshot!(res, @r#"
    <div class="card">
      <div class="card-body">
        <h2 class="card-title">User info</h2>
        <ul class="list">
          <li class="list-row p-1">
            <div class="font-bold">name</div>
            <div>
              <p>Karls</p>
            </div>
          </li>
          <li class="list-row p-1">
            <div class="font-bold">age</div>
            <div>
              <p>20</p>
            </div>
          </li>
        </ul>
      </div>
    </div>
    "#);
}
