use boxx::{Alignment, Boxx};

pub fn main() {
    Boxx::builder()
        .text_alignment(Alignment::Right)
        .box_alignment(Alignment::Right)
        .build()
        .display("This box has been\nright aligned while we weren't looking!\n...why!??! ☹️");
}
