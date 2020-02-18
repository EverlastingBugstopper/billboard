use boxx::{BorderColor, Boxx};

fn main() {
    Boxx::builder()
        .border_color(BorderColor::Blue)
        .build()
        .display("Hello, World!\nThis box has a blue border now!");
}
