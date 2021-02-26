use billboard::{Billboard, BorderColor};

fn main() {
    Billboard::builder()
        .border_color(BorderColor::Blue)
        .build()
        .eprint(&format!(
            "Hello, World!\nThis billboard has a {} border now!",
            console::style("blue").blue()
        ));
}
