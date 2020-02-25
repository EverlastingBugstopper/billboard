use boxx::{BorderComponents, BorderStyle, Boxx};
use console::style;

pub fn main() {
    Boxx::builder()
        .border_style(BorderStyle::Custom(BorderComponents {
            horizontal: "10".to_string(),
            vertical: "10".to_string(),
            bottom_left: "0".to_string(),
            bottom_right: "0".to_string(),
            top_left: "0".to_string(),
            top_right: "0".to_string(),
        }))
        .build()
        .display(&format!(
            "This is a box\nwith a custom border\nwhich i think is {} {} cool",
            style("pretty").red(),
            style("pretty").green()
        ));
}
