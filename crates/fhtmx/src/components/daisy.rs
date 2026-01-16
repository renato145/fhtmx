use crate::{element::Element, html_element::*};
use pastey::paste;

macro_rules! daisy_component {
    ($class:literal; $element:ident; $doc:literal) => {
        paste! {
            #[doc = "Creates a new Daisy " $class " component: `" $element "().class(\"" $class "\")`.\n" $doc]
            pub fn [<dc_ $class>]() -> HtmlElement {
                $element().class($class)
            }
        }
    };

    ($class:literal; $element:ident; $doc:literal; $xtra_k:ident=$xtra_v:literal) => {
        paste! {
            #[doc = "Creates a new Daisy " $class " component: `" $element "().class(\"" $class "\")." $xtra_k "(\"" $xtra_v "\")`.\n" $doc]
            pub fn [<dc_ $class>]() -> HtmlElement {
                $element().class($class).$xtra_k($xtra_v)
            }
        }
    };

    (
        $class:literal; $element:ident; $doc:literal $(;$xtra_k:ident=$xtra_v:literal)?,
        $($class_rest:literal; $element_rest:ident; $doc_rest:literal $(;$xtra_k_rest:ident=$xtra_v_rest:literal)?),+ $(,)?
    ) => {
        daisy_component!($class; $element; $doc $(;$xtra_k=$xtra_v)?);
        daisy_component!($($class_rest; $element_rest; $doc_rest $(;$xtra_k_rest=$xtra_v_rest)?),+);
    };
}

daisy_component!(
    // Actions

    "btn"; button; "Buttons allow the user to take actions",

    "dropdown"; details; "Dropdown can open a menu or any other element when the button is clicked",
    "dropdown-content"; ul; "Container for dropdown menu items",

    "fab"; div; "FAB (Floating Action Button) stays in the bottom corner of screen",
    "fab-close"; div; "Close button that replaces the original FAB button when open",
    "fab-main-action"; div; "Main action button shown when FAB is open",

    "modal"; dialog; "Modal is used to show a dialog or a box when you click a button",
    "modal-box"; div; "Container for modal content",
    "modal-action"; form; "Container for modal action buttons",
    "modal-backdrop"; form; "Backdrop that closes the modal when clicked",
    "modal-toggle"; input; "Hidden checkbox to control modal state"; typ="checkbox",

    "swap"; label; "Swap allows you to toggle the visibility of two elements",
    "swap-on"; div; "Content shown when swap is active",
    "swap-off"; div; "Content shown when swap is inactive",
    "swap-indeterminate"; div; "Content shown when checkbox is indeterminate",

    "theme-controller"; input; "Controls the page theme based on the input's value"; typ="checkbox",

    // Data display

    "collapse"; div; "Collapse is used for showing and hiding content",
    "collapse-title"; div; "Title section of a collapse component",
    "collapse-content"; div; "Content section of a collapse component",

    "avatar"; div; "Avatars are used to show a thumbnail",
    "avatar-group"; div; "Container for grouping multiple avatars",

    "badge"; span; "Badges are used to inform the user of the status of specific data",

    "pika-single"; input; "Input field that opens Pikaday calendar"; typ="text",

    "card"; div; "Cards are used to group and display content",
    "card-title"; h2; "Title section of a card",
    "card-body"; div; "Body section of a card",
    "card-actions"; div; "Container for card action buttons",

    "carousel"; div; "Carousel shows images or content in a scrollable area",
    "carousel-item"; div; "Individual item within a carousel",

    "chat"; div; "Chat bubbles show one line of conversation and its data",
    "chat-image"; div; "Container for chat avatar image",
    "chat-header"; div; "Header section of a chat bubble",
    "chat-footer"; div; "Footer section of a chat bubble",
    "chat-bubble"; div; "The message bubble content",

    "countdown"; span; "Countdown gives a transition effect for numbers 0-999",

    "diff"; figure; "Diff component shows a side-by-side comparison of two items",
    "diff-item-1"; div; "First item in the diff comparison",
    "diff-item-2"; div; "Second item in the diff comparison",
    "diff-resizer"; div; "Resizer handle for the diff component",

    "hover-3d"; div; "Wrapper that adds a 3D hover effect to its content",
    "hover-gallery"; figure; "Container of images that shows different images on horizontal hover",

    "kbd"; kbd; "Kbd is used to display keyboard shortcuts",

    "list"; ul; "List is a vertical layout to display information in rows",
    "list-row"; li; "Individual row within a list",

    "stats"; div; "Container for stat components",
    "stat"; div; "Stat is used to show numbers and data in a block",
    "stat-title"; div; "Title of a stat",
    "stat-value"; div; "Main value of a stat",
    "stat-desc"; div; "Description of a stat",
    "stat-figure"; div; "Figure/icon area of a stat",
    "stat-actions"; div; "Actions area of a stat",

    "status"; span; "Status is a small icon to show current status like online, offline, error",

    "table"; table; "Table shows a list of data in a table format",

    "text-rotate"; span; "Text Rotate shows up to 6 lines of text with infinite loop animation",

    "timeline"; ul; "Timeline shows a list of events in chronological order",
    "timeline-start"; div; "Start section of a timeline item",
    "timeline-middle"; div; "Middle section of a timeline item (usually icon)",
    "timeline-end"; div; "End section of a timeline item",

    // Navigation

    "breadcrumbs"; div; "Breadcrumbs help users to navigate",

    "dock"; div; "Dock provides navigation options, sticks to the bottom of screen",
    "dock-label"; span; "Label text for a dock item",

    "link"; a; "Link adds the missing underline style to links",

    "menu"; ul; "Menu displays a list of links vertically or horizontally",
    "menu-title"; li; "Title item in a menu",
    "menu-dropdown"; ul; "Dropdown submenu container",
    "menu-dropdown-toggle"; summary; "Toggle element for dropdown menu",

    "navbar"; div; "Navbar shows a navigation bar on the top of the page",
    "navbar-start"; div; "Left section of navbar",
    "navbar-center"; div; "Center section of navbar",
    "navbar-end"; div; "Right section of navbar",

    "join"; div; "Join is a container for grouping multiple items together",
    "join-item"; div; "Individual item within a join group",

    "steps"; ul; "Steps show a list of steps in a process",
    "step"; li; "Individual step in a steps component",
    "step-icon"; span; "Icon for a step",

    "tabs"; div; "Tabs show a list of links in a tabbed format",
    "tab"; button; "Individual tab item",
    "tab-content"; div; "Content associated with a tab",

    // Feedback

    "alert"; div; "Alert informs users about important events",
    "loading"; span; "Loading shows an animation to indicate something is loading",
    "progress"; progress; "Progress bar shows the progress of a task",
    "radial-progress"; div; "Radial progress shows progress in a circular format",
    "skeleton"; div; "Skeleton shows a loading state placeholder",
    "toast"; div; "Toast is a wrapper to stack elements on the corner of page",
    "tooltip"; div; "Tooltip shows extra information on hover",

    // Data input

    "checkbox"; input; "Checkboxes are used to select or deselect a value"; typ="checkbox",
    "fieldset"; fieldset; "Fieldset is a container for grouping related form elements",
    "fieldset-legend"; legend; "Title for a fieldset",
    "file-input"; input; "File Input is an input field for uploading files"; typ="file",
    "filter"; form; "Filter is a group of radio buttons that hides unselected options",
    "label"; label; "Label provides a name or title for an input field",
    "floating-label"; label; "Floating label floats above input when focused",
    "radio"; input; "Radio buttons allow the user to select one option"; typ="radio",
    "range"; input; "Range slider selects a value by sliding a handle"; typ="range",
    "rating"; div; "Rating is a set of radio buttons to rate something",
    "mask"; input; "Mask crops the content of the element to common shapes"; typ="radio",
    "select"; select; "Select is used to pick a value from a list of options",
    "input"; input; "Text Input is a simple input field",
    "textarea"; textarea; "Textarea allows users to enter text in multiple lines",
    "toggle"; input; "Toggle is a checkbox styled to look like a switch button"; typ="checkbox",
    "validator"; input; "Validator changes form element color based on validation rules",
    "validator-hint"; p; "Error message shown when validation fails",

    // Layout

    "divider"; div; "Divider separates content vertically or horizontally",

    "drawer"; div; "Drawer is a grid layout that can show/hide a sidebar",
    "drawer-toggle"; input; "Hidden checkbox to control drawer state"; typ="checkbox",
    "drawer-content"; div; "Main content area of a drawer",
    "drawer-side"; div; "Sidebar content of a drawer",
    "drawer-overlay"; label; "Overlay that closes the drawer when clicked",

    "footer"; footer; "Footer can contain logo, copyright notice, and links",
    "footer-title"; span; "Title for a footer section",

    "hero"; div; "Hero displays a large box or image with title and description",
    "hero-content"; div; "Content area of a hero component",
    "hero-overlay"; div; "Overlay on top of hero background image",

    "indicator"; div; "Indicators place an element on the corner of another element",
    "indicator-item"; span; "The indicator element placed on the corner",

    "stack"; div; "Stack visually puts elements on top of each other",

    // Mockup

    "mockup-browser"; div; "Browser mockup shows a box that looks like a browser window",
    "mockup-browser-toolbar"; div; "Toolbar section of browser mockup",
    "mockup-code"; div; "Code mockup shows a block of code in a code editor style box",
    "mockup-phone"; div; "Phone mockup shows a mockup of an iPhone",
    "mockup-phone-camera"; div; "Camera section of phone mockup",
    "mockup-phone-display"; div; "Display/screen section of phone mockup",
    "mockup-window"; div; "Window mockup shows a box that looks like an OS window",
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Render;

    #[test]
    fn component_works() {
        let res = dc_btn().render();
        println!("{res}");
        let res = dc_modal_toggle().render();
        println!("{res}");
    }
}
