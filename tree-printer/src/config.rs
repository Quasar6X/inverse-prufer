#[derive(Debug)]
pub struct Inset {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
}

pub struct InsetBuilder {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
}

impl Inset {
    pub const EMPTY: Self = Self {
        top: 0,
        right: 0,
        bottom: 0,
        left: 0,
    };

    #[must_use]
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

    pub fn top(&mut self, top: u32) -> &mut Self {
        self.top = top;
        self
    }

    pub fn right(&mut self, right: u32) -> &mut Self {
        self.right = right;
        self
    }

    pub fn bottom(&mut self, bottom: u32) -> &mut Self {
        self.bottom = bottom;
        self
    }

    pub fn left(&mut self, left: u32) -> &mut Self {
        self.left = left;
        self
    }

    #[must_use]
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
