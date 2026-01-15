macro_rules! daisy_component {
    ($name:ident = $tag:expr; $desc:literal) => {
        paste::paste! {
            #[doc = "Daisy " $name " component.\n" $desc]
            pub struct $name {
                pub classes: indexmap::IndexSet<std::borrow::Cow<'static, str>>,
            }

            impl $name {
                #[doc = "Creates a new Daisy " $name " component.\n" $desc]
                pub fn new() -> Self {
                    Self {
                        classes: indexmap::IndexSet::new(),
                    }
                }

                #[doc = "Converts into a `HtmlElement`"]
                pub fn html(mut self) -> fhtmx::html_element::HtmlElement {
                    let mut x = $tag;
                    x.classes.append(&mut self.classes);
                    x
                }

                pub fn render(self) -> String {
                    pub use fhtmx::render::Render;
                    self.html().render()
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl fhtmx::node::IntoNode for $name {
                fn into_node(self) -> fhtmx::node::HtmlNode {
                    self.html().into_node()
                }
            }

            #[doc = "Creates a new Daisy " $name " component.\n" $desc]
            pub fn [< dc_ $name:snake >]() -> $name {
                $name::new()
            }
        }
    };
}

pub(crate) use daisy_component;

macro_rules! daisy_class {
    ($($method:ident = $class:literal; $doc:literal),* $(,)?) => {
        $(
            #[doc = $doc]
            pub fn $method(mut self) -> Self {
                self.classes.insert($class.into());
                self
            }
        )*
    };
}

pub(crate) use daisy_class;
