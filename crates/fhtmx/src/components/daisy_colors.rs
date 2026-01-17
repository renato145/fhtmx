/// Semantic colors used by DaisyUI components.
/// These colors adapt to the current theme automatically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DaisyColor {
    /// Primary brand color
    Primary,
    /// Secondary brand color
    Secondary,
    /// Accent color for highlights
    Accent,
    /// Neutral color for general content
    Neutral,
    /// Informational messages and elements
    Info,
    /// Success states and confirmations
    Success,
    /// Warning states and cautions
    Warning,
    /// Error states and destructive actions
    Error,
    /// Base background color (lightest)
    Base100,
    /// Secondary background color
    Base200,
    /// Tertiary background color (darkest)
    Base300,
}

impl DaisyColor {
    /// Returns the DaisyUI class name for this color
    pub fn as_class(&self) -> &'static str {
        match self {
            DaisyColor::Primary => "primary",
            DaisyColor::Secondary => "secondary",
            DaisyColor::Accent => "accent",
            DaisyColor::Neutral => "neutral",
            DaisyColor::Info => "info",
            DaisyColor::Success => "success",
            DaisyColor::Warning => "warning",
            DaisyColor::Error => "error",
            DaisyColor::Base100 => "base-100",
            DaisyColor::Base200 => "base-200",
            DaisyColor::Base300 => "base-300",
        }
    }

    /// Returns the background color class (e.g. `bg-primary`)
    pub fn bg(&self) -> String {
        format!("bg-{}", self.as_class())
    }

    /// Returns the text color class (e.g. `text-primary`)
    pub fn text(&self) -> String {
        format!("text-{}", self.as_class())
    }

    /// Returns the content color class for contrasting text (e.g. `text-primary-content`)
    /// Use this for text on top of a background of this color.
    pub fn content(&self) -> String {
        match self {
            DaisyColor::Base100 | DaisyColor::Base200 | DaisyColor::Base300 => {
                "text-base-content".to_string()
            }
            _ => format!("text-{}-content", self.as_class()),
        }
    }

    /// Returns both the background and content color classes (e.g. `bg-primary text-primary-content`)
    /// Useful for elements where you want the background and matching contrasting text.
    pub fn bg_content(&self) -> String {
        format!("{} {}", self.bg(), self.content())
    }

    /// Returns the border color class (e.g. `border-primary`)
    pub fn border(&self) -> String {
        format!("border-{}", self.as_class())
    }

    /// Returns the outline color class (e.g. `outline-primary`)
    pub fn outline(&self) -> String {
        format!("outline-{}", self.as_class())
    }

    /// Returns the ring color class (e.g. `ring-primary`)
    pub fn ring(&self) -> String {
        format!("ring-{}", self.as_class())
    }

    /// Returns the fill color class for SVGs (e.g. `fill-primary`)
    pub fn fill(&self) -> String {
        format!("fill-{}", self.as_class())
    }

    /// Returns the stroke color class for SVGs (e.g. `stroke-primary`)
    pub fn stroke(&self) -> String {
        format!("stroke-{}", self.as_class())
    }
}
