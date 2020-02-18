use crate::BorderStyle;

/// `BorderComponents` defines the `String` to use when dislaying
/// the border of a `Boxx`
#[derive(Clone, PartialEq, Debug)]
pub struct BorderComponents {
    pub(crate) top_left: String,
    pub(crate) horizontal: String,
    pub(crate) top_right: String,
    pub(crate) vertical: String,
    pub(crate) bottom_right: String,
    pub(crate) bottom_left: String,
}

impl BorderComponents {
    /// Creates new `BorderComponents` from a `BorderStyle`
    pub fn new(style: BorderStyle) -> BorderComponents {
        match style {
            BorderStyle::Single => BorderComponents {
                top_left: "┌".to_string(),
                top_right: "┐".to_string(),
                bottom_right: "┘".to_string(),
                bottom_left: "└".to_string(),
                vertical: "│".to_string(),
                horizontal: "─".to_string(),
            },
            BorderStyle::Double => BorderComponents {
                top_left: "╔".to_string(),
                top_right: "╗".to_string(),
                bottom_right: "╝".to_string(),
                bottom_left: "╚".to_string(),
                vertical: "║".to_string(),
                horizontal: "═".to_string(),
            },
            BorderStyle::Round => BorderComponents {
                top_left: "╭".to_string(),
                top_right: "╮".to_string(),
                bottom_right: "╯".to_string(),
                bottom_left: "╰".to_string(),
                vertical: "│".to_string(),
                horizontal: "─".to_string(),
            },
            BorderStyle::Bold => BorderComponents {
                top_left: "┏".to_string(),
                top_right: "┓".to_string(),
                bottom_right: "┛".to_string(),
                bottom_left: "┗".to_string(),
                vertical: "┃".to_string(),
                horizontal: "━".to_string(),
            },
            BorderStyle::SingleDouble => BorderComponents {
                top_left: "╓".to_string(),
                top_right: "╖".to_string(),
                bottom_right: "╜".to_string(),
                bottom_left: "╙".to_string(),
                vertical: "║".to_string(),
                horizontal: "─".to_string(),
            },
            BorderStyle::DoubleSingle => BorderComponents {
                top_left: "╒".to_string(),
                top_right: "╕".to_string(),
                bottom_right: "╛".to_string(),
                bottom_left: "╘".to_string(),
                vertical: "│".to_string(),
                horizontal: "═".to_string(),
            },
            BorderStyle::Classic => BorderComponents {
                top_left: "+".to_string(),
                top_right: "+".to_string(),
                bottom_right: "+".to_string(),
                bottom_left: "+".to_string(),
                vertical: "|".to_string(),
                horizontal: "-".to_string(),
            },
            BorderStyle::Custom(components) => components,
        }
    }
}
