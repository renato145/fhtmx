use fhtmx::prelude::*;

#[test]
#[allow(dead_code)]
fn macro_works() {
    #[derive(Debug)]
    struct Details {
        active: bool,
        years: u8,
    }

    #[derive(HtmlView)]
    #[html_view(title = "User info")]
    struct User {
        name: String,
        age: usize,
        #[html_view(skip)]
        password: String,
        #[html_view(value_debug_pretty)]
        details: Details,
        contract: Option<String>,
    }

    let x = User {
        name: "Karls".to_string(),
        age: 20,
        password: "xxxx".to_string(),
        details: Details {
            active: false,
            years: 5,
        },
        contract: None,
    };
    let res = x.html_view().render();
    insta::assert_snapshot!(res, @r#"
    <div class="card">
      <div class="card-body">
        <h2 class="card-title">User info</h2>
        <ul class="list">
          <li class="list-row p-1">
            <div class="font-bold">name</div>
            <div>Karls</div>
          </li>
          <li class="list-row p-1">
            <div class="font-bold">age</div>
            <div>20</div>
          </li>
          <li class="list-row p-1">
            <div class="font-bold">details</div>
            <div>
              <pre class="text-wrap">Details {
        active: false,
        years: 5,
    }</pre>
            </div>
          </li>
          <li class="list-row p-1">
            <div class="font-bold">contract</div>
            <div>-</div>
          </li>
        </ul>
      </div>
    </div>
    "#);
}
