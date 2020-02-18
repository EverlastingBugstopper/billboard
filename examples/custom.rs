use boxx::{BorderComponents, BorderStyle, Boxx};

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
        .display("This is a box\nwith a custom border\nwhich i think is pretty pretty cool");
}
