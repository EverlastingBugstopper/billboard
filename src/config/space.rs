#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Space {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

// TODO: builder api for space?
impl Space {
    pub fn uniform(size: usize) -> Self {
        Space {
            top: size,
            right: size * 3,
            bottom: size,
            left: size * 3,
        }
    }
}
