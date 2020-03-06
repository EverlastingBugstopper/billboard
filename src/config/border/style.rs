use crate::BorderComponents;

/// Different styles for the border of a `Billboard`.
#[derive(Clone, PartialEq, Debug)]
pub enum BorderStyle {
    /// Border with a single line:
    ///
    /// ```md
    /// ┌───┐
    /// │box│
    /// └───┘
    /// ```
    Single,

    /// Border with a double line:
    ///
    /// ```md
    /// ╔═══╗
    /// ║box║
    /// ╚═══╝
    /// ```
    Double,

    /// Border with a rounded line:
    ///
    /// ```md
    /// ╭───╮
    /// │box│
    /// ╰───╯
    /// ```
    Round,

    /// Border with a bolded single line:
    ///
    /// ```md
    /// ┏━━━┓
    /// ┃box┃
    /// ┗━━━┛
    /// ```
    Bold,

    /// Border with a single line on the top and bottom
    /// and a double line on the left and right:
    ///
    /// ```md
    /// ╓───╖
    /// ║box║
    /// ╙───╜
    /// ```
    SingleDouble,

    /// Border with a double line on the top and bottom
    /// and a single line on the left and right:
    ///
    /// ```md
    /// ╒═══╕
    /// │box│
    /// ╘═══╛
    /// ```
    DoubleSingle,

    /// Border with `+` on the corners, `|` on the left and right
    /// and `-` on the top:
    ///
    /// ```md
    /// +---+
    /// |box|
    /// +---+
    /// ```
    Classic,

    /// Border with custom `BorderComponents`.
    Custom(BorderComponents),
}
