use crate::prelude::{HtmlElement, HtmlScriptElement, script};

/// IIFE, or Immediately Invoked Function Expression, is a JavaScript pattern where a function runs
/// as soon as it is defined.
pub fn iife(code: impl std::fmt::Display) -> HtmlElement<&'static str, HtmlScriptElement> {
    let code = format!("(async () => {{\n{code}\n}})();");
    script().inner_unsafe(code)
}

#[cfg(test)]
mod test {
    use crate::{js::iife, prelude::HtmlRender};

    #[test]
    fn iife_works() {
        let res = iife("console.log(\"Hello\");").render_sorted();
        insta::assert_snapshot!(res, @r#"
        <script>(async () => {
        console.log("Hello");
        })();</script>
        "#);
    }
}
