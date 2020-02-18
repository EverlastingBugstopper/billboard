use crate::BorderComponents;

#[derive(Clone, PartialEq, Debug)]
pub enum BorderStyle {
    Single,
    Double,
    Custom(BorderComponents),
}
