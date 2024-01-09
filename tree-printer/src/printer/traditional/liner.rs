use std::io::Write;

use crate::text::LineBuffer;

pub trait Liner<W: Write> {
    fn print_connections(
        &self,
        buffer: &mut LineBuffer<W>,
        row: i32,
        top_connection: i32,
        bottom_connections: &[i32],
    ) -> i32;
}

#[derive(Debug)]
pub struct DefaultLiner {
    connections: [char; 13],
    top_height: i32,
    bottom_height: i32,
    display_bracket: bool,
}

impl Default for DefaultLiner {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl DefaultLiner {
    pub fn new_unicode() -> Self {
        Self::builder().unicde().build()
    }

    pub const fn builder() -> DefaultLinerBuilder {
        DefaultLinerBuilder::new()
    }

    fn print_top_connection<W: Write>(
        &self,
        buffer: &mut LineBuffer<W>,
        row: i32,
        start: i32,
        top_connection: i32,
    ) {
        let mut top_connection_line = " ".repeat((top_connection - start) as usize);
        top_connection_line.push(self.connections[0]);

        for i in 0..self.top_height {
            buffer.write((row + i) as usize, start as usize, &top_connection_line);
        }
    }

    fn print_bottom_connections<W: Write>(
        &self,
        buffer: &mut LineBuffer<W>,
        row: i32,
        start: i32,
        top_height_with_bracket: i32,
        full_height: i32,
        bottom_connections: &[i32],
    ) {
        let mut bottom_connection_line = String::new();
        let mut pos = start;

        for &bottom_connection in bottom_connections {
            for _ in pos..bottom_connection {
                bottom_connection_line += " ";
            }
            bottom_connection_line.push(self.connections[12]);
            pos = bottom_connection + 1;
        }

        for i in top_height_with_bracket..full_height {
            buffer.write((row + i) as usize, start as usize, &bottom_connection_line);
        }
    }

    fn print_connection_bracket_line<W: Write>(
        &self,
        buffer: &mut LineBuffer<W>,
        row: i32,
        start: i32,
        end: i32,
        top_connection: i32,
        bottom_connections: &[i32],
    ) {
        let mut bracket_line = String::new();

        for i in start..=end {
            let line_char =
                self.get_nth_bracket_line_char(i, start, end, top_connection, bottom_connections);
            bracket_line.push(line_char);
        }

        buffer.write(
            (row + self.top_height) as usize,
            start as usize,
            &bracket_line,
        );
    }

    fn get_nth_bracket_line_char(
        &self,
        i: i32,
        start: i32,
        end: i32,
        top_connection: i32,
        bottom_connections: &[i32],
    ) -> char {
        if start == end {
            self.connections[11]
        } else if i == top_connection {
            self.get_bracket_line_char_at_top_connection(
                top_connection,
                start,
                end,
                bottom_connections,
            )
        } else if i == start {
            self.connections[1]
        } else if i == end {
            self.connections[10]
        } else if bottom_connections.contains(&i) {
            self.connections[6]
        } else {
            self.connections[2]
        }
    }

    fn get_bracket_line_char_at_top_connection(
        &self,
        i: i32,
        start: i32,
        end: i32,
        bottom_connections: &[i32],
    ) -> char {
        if bottom_connections.contains(&i) {
            if i == start {
                self.connections[8]
            } else if i == end {
                self.connections[9]
            } else {
                self.connections[7]
            }
        } else {
            if i == start {
                self.connections[4]
            } else if i == end {
                self.connections[5]
            } else {
                self.connections[6]
            }
        }
    }
}

impl<W: Write> Liner<W> for DefaultLiner {
    fn print_connections(
        &self,
        buffer: &mut LineBuffer<W>,
        row: i32,
        top_connection: i32,
        bottom_connections: &[i32],
    ) -> i32 {
        use std::cmp::{max, min};

        let start = min(
            top_connection,
            *bottom_connections
                .first()
                .expect("Invalid bottom connections vector"),
        );
        let end = max(
            top_connection,
            *bottom_connections
                .last()
                .expect("Invalid bottom connections vector"),
        );
        let top_height_with_bracket = self.top_height
            + match self.display_bracket {
                true => 1,
                false => 0,
            };
        let full_height = top_height_with_bracket + self.bottom_height;

        self.print_top_connection(buffer, row, start, top_connection);
        self.print_connection_bracket_line(
            buffer,
            row,
            start,
            end,
            top_connection,
            bottom_connections,
        );
        self.print_bottom_connections(
            buffer,
            row,
            start,
            top_height_with_bracket,
            full_height,
            bottom_connections,
        );

        full_height
    }
}

pub struct DefaultLinerBuilder {
    connections: [char; 13],
    top_height: i32,
    bottom_height: i32,
    display_bracket: bool,
}

impl DefaultLinerBuilder {
    const LINE_CHARS_ASCII: [char; 13] = [
        '|', ' ', '_', '|', '|', '|', '_', '|', '|', '|', ' ', '|', '|',
    ];

    const LINE_CHARS_UNICODE: [char; 13] = [
        '│', '┌', '─', '┴', '└', '┘', '┬', '┼', '├', '┤', '┐', '│', '│',
    ];

    const fn new() -> Self {
        Self {
            connections: Self::LINE_CHARS_ASCII,
            top_height: 0,
            bottom_height: 1,
            display_bracket: true,
        }
    }

    pub fn top_height(&mut self, top_height: i32) -> &mut Self {
        self.top_height = top_height;
        self
    }

    pub fn bottom_height(&mut self, bottom_height: i32) -> &mut Self {
        self.bottom_height = bottom_height;
        self
    }

    pub fn display_bracket(&mut self, display_bracket: bool) -> &mut Self {
        self.display_bracket = display_bracket;
        self
    }

    pub fn unicde(&mut self) -> &mut Self {
        self.connections = Self::LINE_CHARS_UNICODE;
        self
    }

    pub fn ascii(&mut self) -> &mut Self {
        self.connections = Self::LINE_CHARS_ASCII;
        self
    }

    pub fn top_connection_char(&mut self, top_connection_char: char) -> &mut Self {
        self.connections[0] = top_connection_char;
        self
    }

    pub fn bracket_left_char(&mut self, bracket_left_char: char) -> &mut Self {
        self.connections[1] = bracket_left_char;
        self
    }

    pub fn bracket_char(&mut self, bracket_char: char) -> &mut Self {
        self.connections[2] = bracket_char;
        self
    }

    pub fn bracket_top_char(&mut self, bracket_top_char: char) -> &mut Self {
        self.connections[3] = bracket_top_char;
        self
    }

    pub fn bracket_top_left_char(&mut self, bracket_top_left_char: char) -> &mut Self {
        self.connections[4] = bracket_top_left_char;
        self
    }

    pub fn bracket_top_right_char(&mut self, bracket_top_right_char: char) -> &mut Self {
        self.connections[5] = bracket_top_right_char;
        self
    }

    pub fn bracket_bottom_char(&mut self, bracket_bottom_char: char) -> &mut Self {
        self.connections[6] = bracket_bottom_char;
        self
    }

    pub fn bracket_top_and_bottom_char(&mut self, bracket_top_and_bottom_char: char) -> &mut Self {
        self.connections[7] = bracket_top_and_bottom_char;
        self
    }

    pub fn bracket_top_and_bottom_left_char(
        &mut self,
        bracket_top_and_bottom_left_char: char,
    ) -> &mut Self {
        self.connections[8] = bracket_top_and_bottom_left_char;
        self
    }

    pub fn bracket_top_and_bottom_right_char(
        &mut self,
        bracket_top_and_bottom_right_char: char,
    ) -> &mut Self {
        self.connections[9] = bracket_top_and_bottom_right_char;
        self
    }

    pub fn bracket_right_char(&mut self, bracket_right_char: char) -> &mut Self {
        self.connections[10] = bracket_right_char;
        self
    }

    pub fn bracket_only_char(&mut self, bracket_only_char: char) -> &mut Self {
        self.connections[11] = bracket_only_char;
        self
    }

    pub fn bottom_connection_char(&mut self, bottom_connection_char: char) -> &mut Self {
        self.connections[12] = bottom_connection_char;
        self
    }

    pub const fn build(&self) -> DefaultLiner {
        let DefaultLinerBuilder {
            connections,
            top_height,
            bottom_height,
            display_bracket,
        } = *self;
        DefaultLiner {
            connections,
            top_height,
            bottom_height,
            display_bracket,
        }
    }
}
