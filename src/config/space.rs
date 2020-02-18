/// `Space` defines the size of the whitespace in four directions
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Space {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

// TODO: builder api for space?
impl Space {
    /// Creates a uniformly spaced `Space`
    pub fn uniform(size: usize) -> Self {
        Space {
            top: size,
            right: size * 3,
            bottom: size,
            left: size * 3,
        }
    }
}
