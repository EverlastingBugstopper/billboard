use crate::BorderComponents;

/// Different styles for the border of a `Boxx`
#[derive(Clone, PartialEq, Debug)]
pub enum BorderStyle {
    /// Border with a single line
    Single,
    /// Border with a double line
    Double,
    /// Border with a rounded line
    Round,
    /// Border with a bolded single line
    Bold,
    /// Border with a single line on the top and bottom
    /// and a double line on the left and right
    SingleDouble,
    /// Border with a double line on the top and bottom
    /// and a single line on the left and right
    DoubleSingle,
    /// Border with `+` on the corners, `|` on the left and right
    /// and `-` on the top
    Classic,
    /// Border with custom `BorderComponents`
    Custom(BorderComponents),
}
