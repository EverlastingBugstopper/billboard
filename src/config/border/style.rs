use crate::BorderComponents;

/// Different styles for the border of a `Boxx`
#[derive(Clone, PartialEq, Debug)]
pub enum BorderStyle {
    /// Border with a single line:
    ///
    /// ┌───┐
    /// │box│
    /// └───┘
    Single,

    /// Border with a double line:
    ///
    /// ╔═══╗
    /// ║box║
    /// ╚═══╝
    Double,

    /// Border with a rounded line:
    ///
    /// ╭───╮
    /// │box│
    /// ╰───╯
    Round,

    /// Border with a bolded single line:
    ///
    /// ┏━━━┓
    /// ┃box┃
    /// ┗━━━┛
    Bold,

    /// Border with a single line on the top and bottom
    /// and a double line on the left and right:
    ///
    /// ╓───╖
    /// ║box║
    /// ╙───╜
    SingleDouble,

    /// Border with a double line on the top and bottom
    /// and a single line on the left and right:
    ///
    /// ╒═══╕
    /// │box│
    /// ╘═══╛
    DoubleSingle,

    /// Border with `+` on the corners, `|` on the left and right
    /// and `-` on the top:
    /// +---+
    /// |box|
    /// +---+
    Classic,

    /// Border with custom `BorderComponents`
    Custom(BorderComponents),
}
