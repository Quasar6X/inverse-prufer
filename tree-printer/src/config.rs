#[derive(Debug)]
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
    pub const EMPTY: Inset = Self {
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
    };

    pub const fn builder() -> InsetBuilder {
        InsetBuilder::new()
    }
}

impl InsetBuilder {
    const fn new() -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }

    pub fn top(&mut self, top: i32) -> &mut Self {
        self.top = top;
        self
    }

    pub fn right(&mut self, right: i32) -> &mut Self {
        self.right = right;
        self
    }

    pub fn bottom(&mut self, bottom: i32) -> &mut Self {
        self.bottom = bottom;
        self
    }

    pub fn left(&mut self, left: i32) -> &mut Self {
        self.left = left;
        self
    }

    pub const fn build(&self) -> Inset {
        let Self {
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

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlign {
    LEFT,
    CENTER,
    RIGHT,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlign {
    TOP,
    MIDDLE,
    BOTTOM,
}
