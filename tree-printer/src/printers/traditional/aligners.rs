use super::WidthMapFromBox;
use crate::{config::HorizontalAlign, tree_node::TreeNode};

#[derive(Debug)]
pub struct Placement {
    pub left: u32,
    pub top_connection: u32,
    pub bottom_connection: u32,
}

pub trait Aligner {
    fn align_node(&self, position: u32, width: u32, content_width: u32) -> Placement;

    fn align_children(
        &self,
        parent_node: &Box<dyn TreeNode>,
        children: &[Box<dyn TreeNode>],
        position: u32,
        width_map: &WidthMapFromBox<'_>,
    ) -> Vec<u32>;

    fn collect_widths<'a>(
        &self,
        node: &'a Box<dyn TreeNode>,
    ) -> anyhow::Result<(WidthMapFromBox<'a>, u32)>;
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectMode {
    CONTENT,
    CONTEXT,
}

mod helper {
    #[derive(Debug, Clone, Copy)]
    pub struct ConnectionDescriptor {
        pub align: super::HorizontalAlign,
        pub connect: super::ConnectMode,
        pub offset: u32,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum VerticalDirection {
        Top,
        Bottom,
    }

    #[derive(Debug, Clone)]
    pub struct CalculationPrimitives {
        pub position: u32,
        pub width: u32,
        pub content_width: u32,
        pub max_left: u32,
    }
}

pub struct DefaultAligner {
    content_align: HorizontalAlign,
    content_offset: u32,
    top_connection: helper::ConnectionDescriptor,
    bottom_connection: helper::ConnectionDescriptor,
    children_align: HorizontalAlign,
    gap: u32,
}

impl Default for DefaultAligner {
    fn default() -> Self {
        Self::from_align(HorizontalAlign::CENTER)
    }
}

impl DefaultAligner {
    #[must_use]
    pub fn from_align(align: HorizontalAlign) -> Self {
        Self::new(align, 1)
    }

    #[must_use]
    pub fn new(align: HorizontalAlign, gap: u32) -> Self {
        Self::builder().align(align).gap(gap).build()
    }

    #[must_use]
    pub const fn builder() -> DefaultAlignerBuilder {
        DefaultAlignerBuilder::new()
    }

    fn calculate_left(
        &self,
        helper::CalculationPrimitives {
            position,
            width,
            content_width,
            max_left: content_max_left,
        }: helper::CalculationPrimitives,
    ) -> u32 {
        let relative_left = {
            match self.content_align {
                HorizontalAlign::LEFT => position,
                HorizontalAlign::RIGHT => content_max_left,
                HorizontalAlign::CENTER => position + (width - content_width) / 2,
            }
        };

        Self::restrict_u32(relative_left + self.content_offset, 0, content_max_left)
    }

    fn calculate_vertical_connection(
        &self,
        left: u32,
        helper::CalculationPrimitives {
            position,
            width,
            content_width,
            max_left: connection_max_left,
        }: helper::CalculationPrimitives,
        vertical_direction: helper::VerticalDirection,
    ) -> u32 {
        let (align, connect, offset) = match vertical_direction {
            helper::VerticalDirection::Top => (
                self.top_connection.align,
                self.top_connection.connect,
                self.top_connection.offset,
            ),
            helper::VerticalDirection::Bottom => (
                self.bottom_connection.align,
                self.bottom_connection.connect,
                self.bottom_connection.offset,
            ),
        };

        let relative_top_connection = match connect {
            ConnectMode::CONTENT => match align {
                HorizontalAlign::LEFT => left,
                HorizontalAlign::RIGHT => left + content_width - 1,
                HorizontalAlign::CENTER => left + content_width / 2,
            },
            ConnectMode::CONTEXT => match align {
                HorizontalAlign::LEFT => position,
                HorizontalAlign::RIGHT => connection_max_left,
                HorizontalAlign::CENTER => position + (width - content_width) / 2,
            },
        };

        Self::restrict_u32(relative_top_connection + offset, 0, connection_max_left)
    }

    fn restrict_u32(value: u32, min: u32, max: u32) -> u32 {
        use std::cmp;
        cmp::max(min, cmp::min(max, value))
    }
}

impl Aligner for DefaultAligner {
    fn align_node(&self, position: u32, width: u32, content_width: u32) -> Placement {
        let content_max_left = position + width - content_width;
        let connection_max_left = position + width - 1;
        let left = self.calculate_left(helper::CalculationPrimitives {
            position,
            width,
            content_width,
            max_left: content_max_left,
        });

        Placement {
            left,
            top_connection: self.calculate_vertical_connection(
                left,
                helper::CalculationPrimitives {
                    position,
                    width,
                    content_width,
                    max_left: connection_max_left,
                },
                helper::VerticalDirection::Top,
            ),
            bottom_connection: self.calculate_vertical_connection(
                left,
                helper::CalculationPrimitives {
                    position,
                    width,
                    content_width,
                    max_left: connection_max_left,
                },
                helper::VerticalDirection::Bottom,
            ),
        }
    }

    fn align_children(
        &self,
        parent_node: &Box<dyn TreeNode>,
        children: &[Box<dyn TreeNode>],
        position: u32,
        width_map: &WidthMapFromBox<'_>,
    ) -> Vec<u32> {
        let mut res = Vec::with_capacity(children.len());
        let mut children_width = 0;
        let mut first = true;

        for child in children {
            if first {
                first = false;
            } else {
                children_width += self.gap;
            }

            let child_width = *width_map
                .get(child)
                .unwrap_or_else(|| panic!("Width map has no width for child: {child:?}"));
            res.push(position + children_width);
            children_width += child_width;
        }

        let parent_width = *width_map
            .get(&parent_node)
            .unwrap_or_else(|| panic!("Width map has no width for parent: {parent_node:?}"));

        let offset = match self.children_align {
            HorizontalAlign::LEFT => 0,
            HorizontalAlign::RIGHT => parent_width - children_width,
            HorizontalAlign::CENTER => (parent_width - children_width) / 2,
        };

        if offset > 0 {
            for child_align in &mut res {
                *child_align += offset;
            }
        }

        res
    }

    fn collect_widths<'a>(
        &self,
        root: &'a Box<dyn TreeNode>,
    ) -> anyhow::Result<(WidthMapFromBox<'a>, u32)> {
        fn collect_recursive<'a>(
            gap: u32,
            root: &'a Box<dyn TreeNode>,
            width_map: &mut WidthMapFromBox<'a>,
        ) -> anyhow::Result<u32> {
            let mut children_width = 0;
            let mut first = true;
            for child in root.children() {
                if first {
                    first = false;
                } else {
                    children_width += gap;
                }
                children_width += collect_recursive(gap, child, width_map)?;
            }

            let (content_width, _) = crate::util::content_dimension(&root.content())?;
            let node_width = std::cmp::max(content_width, children_width);
            width_map.insert(root, node_width);
            Ok(node_width)
        }

        let mut width_map = WidthMapFromBox::new();
        let children_width = collect_recursive(self.gap, root, &mut width_map)?;
        Ok((width_map, children_width))
    }
}

// BUILDER

pub struct DefaultAlignerBuilder {
    content_align: HorizontalAlign,
    content_offset: u32,
    top_connection: helper::ConnectionDescriptor,
    bottom_connection: helper::ConnectionDescriptor,
    children_align: HorizontalAlign,
    gap: u32,
}

impl DefaultAlignerBuilder {
    const fn new() -> Self {
        Self {
            content_align: HorizontalAlign::CENTER,
            content_offset: 0,
            top_connection: helper::ConnectionDescriptor {
                align: HorizontalAlign::CENTER,
                connect: ConnectMode::CONTENT,
                offset: 0,
            },
            bottom_connection: helper::ConnectionDescriptor {
                align: HorizontalAlign::CENTER,
                connect: ConnectMode::CONTENT,
                offset: 0,
            },
            children_align: HorizontalAlign::CENTER,
            gap: 1,
        }
    }

    pub fn align(&mut self, align: HorizontalAlign) -> &mut Self {
        self.content_align = align;
        self.top_connection.align = align;
        self.bottom_connection.align = align;
        self.children_align = align;
        self
    }

    pub fn content_align(&mut self, align: HorizontalAlign) -> &mut Self {
        self.content_align = align;
        self
    }

    pub fn content_offset(&mut self, offset: u32) -> &mut Self {
        self.content_offset = offset;
        self
    }

    pub fn top_connection_align(&mut self, align: HorizontalAlign) -> &mut Self {
        self.top_connection.align = align;
        self
    }

    pub fn top_connection_connect(&mut self, connect: ConnectMode) -> &mut Self {
        self.top_connection.connect = connect;
        self
    }

    pub fn top_connection_offset(&mut self, offset: u32) -> &mut Self {
        self.top_connection.offset = offset;
        self
    }

    pub fn bottom_connection_align(&mut self, align: HorizontalAlign) -> &mut Self {
        self.bottom_connection.align = align;
        self
    }

    pub fn bottom_connection_connect(&mut self, connect: ConnectMode) -> &mut Self {
        self.bottom_connection.connect = connect;
        self
    }

    pub fn bottom_connection_offset(&mut self, offset: u32) -> &mut Self {
        self.bottom_connection.offset = offset;
        self
    }

    pub fn children_align(&mut self, align: HorizontalAlign) -> &mut Self {
        self.children_align = align;
        self
    }

    pub fn gap(&mut self, gap: u32) -> &mut Self {
        self.gap = gap;
        self
    }

    #[must_use]
    pub const fn build(&self) -> DefaultAligner {
        let Self {
            content_align,
            content_offset,
            top_connection,
            bottom_connection,
            children_align,
            gap,
        } = *self;
        DefaultAligner {
            content_align,
            content_offset,
            top_connection,
            bottom_connection,
            children_align,
            gap,
        }
    }
}
