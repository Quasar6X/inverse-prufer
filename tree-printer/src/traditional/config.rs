use std::io::Write;

use crate::line_buffer::LineBuffer;

pub struct Placement {
    left: i32,
    top_connection: i32,
    bottom_connection: i32,
}

pub trait Liner<W: Write> {
    fn print_connections(
        buffer: &LineBuffer<W>,
        row: i32,
        top_connection: i32,
        bottom_connections: &Vec<i32>,
    ) -> i32;
}
