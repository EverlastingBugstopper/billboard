use crate::BorderStyle;

/// `BorderComponents` defines the `String`s to use when dislaying
/// the corners and sides of a `Boxx`.
#[derive(Clone, PartialEq, Debug)]
pub struct BorderComponents {
    /// `String` to display as the top left corner of a `Boxx` border.
    ///
    /// This should be one visual character wide.
    pub top_left: String,

    /// `String` to repeat as the top and bottom of a `Boxx` border.
    pub horizontal: String,

    /// `String` to display as the top right corner of a `Boxx` border.
    ///
    /// This should be one visual character wide.
    pub top_right: String,

    /// `String` to repeat as the left and right of a `Boxx` border.
    pub vertical: String,

    /// `String` to display as the bottom right corner of a `Boxx` border.
    ///
    /// This should be one visual character wide.
    pub bottom_right: String,

    /// `String` to display as the bottom left corner of a `Boxx` border.
    ///
    /// This should be one visual character wide.
    pub bottom_left: String,
}

impl BorderComponents {
    /// Creates new `BorderComponents` from a `BorderStyle`.
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
