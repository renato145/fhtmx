use crate::element::*;

/// IIFE, or Immediately Invoked Function Expression, is a JavaScript pattern where a function runs
/// as soon as it is defined.
pub fn iife(code: impl std::fmt::Display) -> HtmlElement {
    let code = format!("(async () => {{\n{code}\n}})();");
    script().add_raw(code)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::Render;

    #[test]
    fn iife_works() {
        let res = iife("console.log(\"Hello\");").render();
        insta::assert_snapshot!(res, @r#"
        <script>(async () => {
        console.log("Hello");
        })();</script>
        "#);
    }
}
