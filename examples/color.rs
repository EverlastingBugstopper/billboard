use boxx::{BorderColor, Boxx};

fn main() {
    Boxx::builder()
        .border_color(BorderColor::Blue)
        .build()
        .display(&format!(
            "Hello, World!\nThis box has a {} border now!",
            console::style("blue").blue()
        ));
}
