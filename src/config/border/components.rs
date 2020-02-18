use crate::BorderStyle;

#[derive(Clone, PartialEq, Debug)]
pub struct BorderComponents {
    pub top_left: String,
    pub horizontal: String,
    pub top_right: String,
    pub vertical: String,
    pub bottom_right: String,
    pub bottom_left: String,
}

impl BorderComponents {
    pub fn new(style: BorderStyle) -> BorderComponents {
        match style {
            BorderStyle::Single => BorderComponents {
                top_left: "┌".to_string(),
                horizontal: "─".to_string(),
                top_right: "┐".to_string(),
                vertical: "│".to_string(),
                bottom_right: "┘".to_string(),
                bottom_left: "└".to_string(),
            },
            BorderStyle::Double => BorderComponents {
                top_left: "╔".to_string(),
                horizontal: "═".to_string(),
                top_right: "╗".to_string(),
                vertical: "║".to_string(),
                bottom_right: "╝".to_string(),
                bottom_left: "╚".to_string(),
            },
            BorderStyle::Custom(components) => components,
        }
    }
}
