#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Inset {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

pub struct InsetBuilder {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

impl Inset {
    pub const fn empty_inset() -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }

    pub const fn builder() -> InsetBuilder {
        InsetBuilder {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl InsetBuilder {
    pub const fn top(mut self, top: i32) -> InsetBuilder {
        self.top = top;
        self
    }

    pub const fn right(mut self, right: i32) -> InsetBuilder {
        self.right = right;
        self
    }

    pub const fn bottom(mut self, bottom: i32) -> InsetBuilder {
        self.bottom = bottom;
        self
    }

    pub const fn left(mut self, left: i32) -> InsetBuilder {
        self.left = left;
        self
    }

    pub const fn build(&self) -> Inset {
        let InsetBuilder {
            top,
            right,
            bottom,
            left,
        } = *self;
        Inset {
            top,
            right,
            bottom,
            left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HorizontalAlign {
    LEFT,
    CENTER,
    RIGHT,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VerticalAlign {
    TOP,
    MIDDLE,
    BOTTOM,
}
