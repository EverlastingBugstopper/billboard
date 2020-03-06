use billboard::{Alignment, Billboard};

pub fn main() {
    Billboard::builder()
        .text_alignment(Alignment::Right)
        .box_alignment(Alignment::Right)
        .build()
        .display("This billboard has been\nright aligned while we weren't looking!\n...why!??! ☹️");
}
