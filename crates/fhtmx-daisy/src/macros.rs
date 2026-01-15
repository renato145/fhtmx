macro_rules! daisy_component {
    ($name:ident = $tag:expr; $cls:literal; $desc:literal) => {
        paste::paste! {
            #[doc = "Daisy " $name " component.\n" $desc]
            pub struct $name {
                pub classes: indexmap::IndexSet<std::borrow::Cow<'static, str>>,
            }

            impl $name {
                #[doc = "Creates a new Daisy " $name " component.\n" $desc]
                pub fn new() -> Self {
                    Self {
                        classes: indexmap::indexset! {$cls.into()},
                    }
                }

                pub fn add_class(mut self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
                    self.classes.insert(value.into());
                    self
                }

                pub fn remove_class(mut self, value: &str) -> Self {
                    self.classes.shift_remove(value);
                    self
                }

                pub fn class(&self) -> String {
                    self.classes.iter().fold(String::new(), |mut acc, s| {
                        if !acc.is_empty() {
                            acc.push(' ');
                        }
                        acc.push_str(s);
                        acc
                    })
                }

                pub fn html(self) -> fhtmx::element::HtmlElement {
                    $tag.class(self.class())
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
            pub fn [< ds_ $name:snake >]() -> $name {
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
