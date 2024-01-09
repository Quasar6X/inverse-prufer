use std::collections::HashMap;

use crate::{config::HorizontalAlign, tree_node::TreeNode};

#[derive(Debug)]
pub struct Placement {
    pub left: i32,
    pub top_connection: i32,
    pub bottom_connection: i32,
}

pub trait Aligner {
    fn align_node(&self, position: i32, width: i32, content_width: i32) -> Placement;

    fn align_children(
        &self,
        parent_node: &Box<dyn TreeNode>,
        children: &[Box<dyn TreeNode>],
        position: i32,
        width_map: &HashMap<&Box<dyn TreeNode>, i32>,
    ) -> Vec<i32>;

    fn collect_widths<'a>(
        &self,
        node: &'a Box<dyn TreeNode>,
    ) -> (HashMap<&'a Box<dyn TreeNode>, i32>, i32);
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
        pub offset: i32,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum VerticalDirection {
        TOP,
        BOTTOM,
    }

    #[derive(Debug, Clone)]
    pub struct CalculationPrimitives {
        pub position: i32,
        pub width: i32,
        pub content_width: i32,
        pub max_left: i32,
    }
}

pub struct DefaultAligner {
    content_align: HorizontalAlign,
    content_offset: i32,
    top_connection: helper::ConnectionDescriptor,
    bottom_connection: helper::ConnectionDescriptor,
    children_align: HorizontalAlign,
    gap: i32,
}

impl Default for DefaultAligner {
    fn default() -> Self {
        Self::from_align(HorizontalAlign::CENTER)
    }
}

impl DefaultAligner {
    pub fn from_align(align: HorizontalAlign) -> Self {
        Self::new(align, 1)
    }

    pub fn new(align: HorizontalAlign, gap: i32) -> Self {
        Self::builder().align(align).gap(gap).build()
    }

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
    ) -> i32 {
        let relative_left = {
            match self.content_align {
                HorizontalAlign::LEFT => position,
                HorizontalAlign::RIGHT => content_max_left,
                HorizontalAlign::CENTER => position + (width - content_width) / 2,
            }
        };

        Self::restrict_i32(relative_left + self.content_offset, 0, content_max_left)
    }

    fn calculate_vertical_connection(
        &self,
        left: i32,
        helper::CalculationPrimitives {
            position,
            width,
            content_width,
            max_left: connection_max_left,
        }: helper::CalculationPrimitives,
        vertical_direction: helper::VerticalDirection,
    ) -> i32 {
        let (align, connect, offset) = match vertical_direction {
            helper::VerticalDirection::TOP => (
                self.top_connection.align,
                self.top_connection.connect,
                self.top_connection.offset,
            ),
            helper::VerticalDirection::BOTTOM => (
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

        Self::restrict_i32(relative_top_connection + offset, 0, connection_max_left)
    }

    fn restrict_i32(value: i32, min: i32, max: i32) -> i32 {
        use std::cmp;
        cmp::max(min, cmp::min(max, value))
    }
}

impl Aligner for DefaultAligner {
    fn align_node(&self, position: i32, width: i32, content_width: i32) -> Placement {
        let content_max_left = position + width + content_width;
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
                helper::VerticalDirection::TOP,
            ),
            bottom_connection: self.calculate_vertical_connection(
                left,
                helper::CalculationPrimitives {
                    position,
                    width,
                    content_width,
                    max_left: connection_max_left,
                },
                helper::VerticalDirection::BOTTOM,
            ),
        }
    }

    fn align_children(
        &self,
        parent_node: &Box<dyn TreeNode>,
        children: &[Box<dyn TreeNode>],
        position: i32,
        width_map: &HashMap<&Box<dyn TreeNode>, i32>,
    ) -> Vec<i32> {
        let mut res = Vec::with_capacity(children.len());
        let mut children_width = -self.gap;

        for child in children.iter() {
            children_width += self.gap;
            let child_width = *width_map
                .get(child)
                .expect(&format!("Width map has no width for child: {:?}", child));
            res.push(position + child_width);
            children_width += child_width;
        }

        let parent_width = *width_map.get(&parent_node).expect(&format!(
            "Width map has no width for parent: {:?}",
            parent_node
        ));

        let offset = match self.children_align {
            HorizontalAlign::LEFT => 0,
            HorizontalAlign::RIGHT => parent_width - children_width,
            HorizontalAlign::CENTER => (parent_width - children_width) / 2,
        };

        if offset > 0 {
            for child_align in res.iter_mut() {
                *child_align += offset;
            }
        }

        res
    }

    fn collect_widths<'a>(
        &self,
        root: &'a Box<dyn TreeNode>,
    ) -> (HashMap<&'a Box<dyn TreeNode>, i32>, i32) {
        let mut map = HashMap::new();

        fn collect_recursive<'a>(
            gap: i32,
            root: &'a Box<dyn TreeNode>,
            map: &mut HashMap<&'a Box<dyn TreeNode>, i32>,
        ) -> i32 {
            let mut children_width = -gap;
            for child in root.children().iter() {
                children_width += gap;
                children_width += collect_recursive(gap, child, map);
            }

            let (content_width, _) = crate::util::content_dimension(&root.content());
            let node_width = std::cmp::max(content_width, children_width);
            map.insert(root, node_width);
            children_width
        }

        let children_width = collect_recursive(self.gap, root, &mut map);
        (map, children_width)
    }
}

// BUILDER

pub struct DefaultAlignerBuilder {
    content_align: HorizontalAlign,
    content_offset: i32,
    top_connection: helper::ConnectionDescriptor,
    bottom_connection: helper::ConnectionDescriptor,
    children_align: HorizontalAlign,
    gap: i32,
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

    pub fn content_offset(&mut self, offset: i32) -> &mut Self {
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

    pub fn top_connection_offset(&mut self, offset: i32) -> &mut Self {
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

    pub fn bottom_connection_offset(&mut self, offset: i32) -> &mut Self {
        self.bottom_connection.offset = offset;
        self
    }

    pub fn children_align(&mut self, align: HorizontalAlign) -> &mut Self {
        self.children_align = align;
        self
    }

    pub fn gap(&mut self, gap: i32) -> &mut Self {
        self.gap = gap;
        self
    }

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
