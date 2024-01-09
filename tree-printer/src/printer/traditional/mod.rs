pub mod aligner;
pub mod liner;

use self::{
    aligner::{Aligner, Placement},
    liner::Liner,
};
use super::TreePrinter;
use crate::{text::LineBuffer, tree_node::TreeNode, util::content_dimension};
use std::{collections::HashMap, io::Write, iter::zip, marker::PhantomData};

pub struct TraditionalTreePrinter<W: Write, A: Aligner, L: Liner<W>> {
    aligner: A,
    liner: L,
    display_placeholders: bool,
    _phantom: PhantomData<W>,
}

impl<W, A, L> Default for TraditionalTreePrinter<W, A, L>
where
    W: Write,
    A: Aligner + Default,
    L: Liner<W> + Default,
{
    fn default() -> Self {
        Self {
            aligner: A::default(),
            liner: L::default(),
            display_placeholders: false,
            _phantom: PhantomData,
        }
    }
}

impl<W, A, L> TraditionalTreePrinter<W, A, L>
where
    W: Write,
    A: Aligner,
    L: Liner<W>,
{
    pub const fn new(aligner: A, liner: L, display_placeholders: bool) -> Self {
        Self {
            aligner,
            liner,
            display_placeholders,
            _phantom: PhantomData,
        }
    }

    pub const fn from_aligner_and_liner(aligner: A, liner: L) -> Self {
        Self::new(aligner, liner, false)
    }

    fn print_next_generation<'a>(
        &self,
        buffer: &mut LineBuffer<W>,
        position_map: &HashMap<&'a Box<dyn TreeNode>, Position>,
        width_map: &HashMap<&Box<dyn TreeNode>, i32>,
    ) -> HashMap<&'a Box<dyn TreeNode>, Position> {
        let mut new_position_map = HashMap::new();
        let mut child_bottoms = Vec::new();

        for (node, pos) in position_map.iter() {
            self.handle_node_children(
                buffer,
                node,
                pos,
                &mut new_position_map,
                width_map,
                &mut child_bottoms,
            );
        }

        if !new_position_map.is_empty() {
            let min_child_bottom = child_bottoms.iter().min().unwrap(); // TODO: Error handling
            buffer.flush(*min_child_bottom as usize).unwrap(); // TODO: Error handling
        }

        new_position_map
    }

    fn handle_node_children<'a>(
        &self,
        buffer: &mut LineBuffer<W>,
        root: &'a Box<dyn TreeNode>,
        Position {
            row,
            col,
            connection,
            left: _,
            height,
        }: &Position,
        new_position_map: &mut HashMap<&'a Box<dyn TreeNode>, Position>,
        width_map: &HashMap<&Box<dyn TreeNode>, i32>,
        child_bottoms: &mut Vec<i32>,
    ) {
        let mut children_position_map = HashMap::new();

        if (!self.display_placeholders
            && root.children().iter().all(|child| child.is_placeholder()))
            || root.children().is_empty()
        {
            return;
        }

        let child_count = root.children().len();
        let children_align = self
            .aligner
            .align_children(root, &root.children(), *col, width_map);
        let mut child_connections = Vec::with_capacity(child_count);

        for (child, &child_col) in zip(root.children().iter(), children_align.iter()) {
            let child_width = *width_map.get(child).unwrap(); // TODO: Error handling
            let (child_content_w, child_content_h) = content_dimension(&child.content());
            let Placement {
                left: placement_left,
                top_connection,
                bottom_connection,
            } = self
                .aligner
                .align_node(child_col, child_width, child_content_w);
            let child_positioning = Position {
                row: row + height,
                col: child_col,
                connection: bottom_connection,
                left: placement_left,
                height: child_content_h,
            };

            children_position_map.insert(child, child_positioning);
            child_connections.push(top_connection)
        }

        let connection_rows =
            self.liner
                .print_connections(buffer, row + height, *connection, &child_connections);

        for (
            &child,
            Position {
                row,
                col: _,
                connection: _,
                left,
                height,
            },
        ) in children_position_map.iter_mut()
        {
            *row += connection_rows;
            buffer.write(*row as usize, *left as usize, &child.content());
            child_bottoms.push(*row + *height);
        }

        new_position_map.extend(children_position_map)
    }
}

impl<W, A, L> TreePrinter<W> for TraditionalTreePrinter<W, A, L>
where
    W: Write,
    A: Aligner,
    L: Liner<W>,
{
    fn print(&self, root_node: Box<dyn TreeNode>, mut out: W) {
        let (width_map, root_width) = self.aligner.collect_widths(&root_node);
        let mut position_map = HashMap::new();

        let root_content = root_node.content();
        let (root_content_w, root_content_h) = content_dimension(&root_content);
        let Placement {
            left,
            top_connection: _,
            bottom_connection,
        } = self.aligner.align_node(0, root_width, root_content_w);

        position_map.insert(
            &root_node,
            Position {
                row: 0,
                col: 0,
                connection: bottom_connection,
                left,
                height: root_content_h,
            },
        );

        let mut buffer = LineBuffer::new(&mut out);
        buffer.write(0, left as usize, &root_content);
        buffer.flush_all().unwrap(); // TODO: Error handling

        while !position_map.is_empty() {
            position_map = self.print_next_generation(&mut buffer, &position_map, &width_map);
        }

        buffer.flush_all().unwrap(); // TODO: Error handling
    }
}

#[derive(Debug)]
struct Position {
    pub row: i32,
    pub col: i32,
    pub connection: i32,
    pub left: i32,
    pub height: i32,
}
