use std::io::Write;

use anyhow::Context;

use crate::text::{LineBuffer, LinePosition};

pub trait Liner<W: Write> {
    fn print_connections(
        &self,
        buffer: &mut LineBuffer<W>,
        row: u32,
        top_connection: u32,
        bottom_connections: &[u32],
    ) -> anyhow::Result<u32>;
}

#[derive(Debug)]
pub struct DefaultLiner {
    connections: [char; 13],
    top_height: u32,
    bottom_height: u32,
    display_bracket: bool,
}

impl Default for DefaultLiner {
    fn default() -> Self {
        Self::builder().unicode().build()
    }
}

impl DefaultLiner {
    #[must_use]
    pub fn new_ascii() -> Self {
        Self::builder().ascii().build()
    }

    #[must_use]
    pub const fn builder() -> DefaultLinerBuilder {
        DefaultLinerBuilder::new()
    }

    fn print_top_connection<W: Write>(
        &self,
        buffer: &mut LineBuffer<W>,
        row: u32,
        start: u32,
        top_connection: u32,
    ) -> anyhow::Result<()> {
        let mut top_connection_line = " ".repeat(usize::try_from(top_connection - start)?);
        top_connection_line.push(self.connections[0]);

        for i in 0..self.top_height {
            buffer.write(LinePosition::new(row + i, start)?, &top_connection_line);
        }

        Ok(())
    }

    fn print_bottom_connections<W: Write>(
        &self,
        buffer: &mut LineBuffer<W>,
        row: u32,
        start: u32,
        top_height_with_bracket: u32,
        full_height: u32,
        bottom_connections: &[u32],
    ) -> anyhow::Result<()> {
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
            buffer.write(LinePosition::new(row + i, start)?, &bottom_connection_line);
        }

        Ok(())
    }

    fn print_connection_bracket_line<W: Write>(
        &self,
        buffer: &mut LineBuffer<W>,
        row: u32,
        start: u32,
        end: u32,
        top_connection: u32,
        bottom_connections: &[u32],
    ) -> anyhow::Result<()> {
        let mut bracket_line = String::new();

        for i in start..=end {
            let line_char =
                self.get_nth_bracket_line_char(i, start, end, top_connection, bottom_connections);
            bracket_line.push(line_char);
        }

        buffer.write(
            LinePosition::new(row + self.top_height, start)?,
            &bracket_line,
        );

        Ok(())
    }

    fn get_nth_bracket_line_char(
        &self,
        i: u32,
        start: u32,
        end: u32,
        top_connection: u32,
        bottom_connections: &[u32],
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
        i: u32,
        start: u32,
        end: u32,
        bottom_connections: &[u32],
    ) -> char {
        if bottom_connections.contains(&i) {
            if i == start {
                self.connections[8]
            } else if i == end {
                self.connections[9]
            } else {
                self.connections[7]
            }
        } else if i == start {
            self.connections[4]
        } else if i == end {
            self.connections[5]
        } else {
            self.connections[6]
        }
    }
}

impl<W: Write> Liner<W> for DefaultLiner {
    fn print_connections(
        &self,
        buffer: &mut LineBuffer<W>,
        row: u32,
        top_connection: u32,
        bottom_connections: &[u32],
    ) -> anyhow::Result<u32> {
        use std::cmp::{max, min};

        let start = min(
            top_connection,
            *bottom_connections
                .first()
                .context("Invalid bottom connections vector")?,
        );
        let end = max(
            top_connection,
            *bottom_connections
                .last()
                .context("Invalid bottom connections vector")?,
        );
        let top_height_with_bracket = self.top_height + u32::from(self.display_bracket);
        let full_height = top_height_with_bracket + self.bottom_height;

        self.print_top_connection(buffer, row, start, top_connection)?;
        self.print_connection_bracket_line(
            buffer,
            row,
            start,
            end,
            top_connection,
            bottom_connections,
        )?;
        self.print_bottom_connections(
            buffer,
            row,
            start,
            top_height_with_bracket,
            full_height,
            bottom_connections,
        )?;

        Ok(full_height)
    }
}

pub struct DefaultLinerBuilder {
    connections: [char; 13],
    top_height: u32,
    bottom_height: u32,
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

    pub fn top_height(&mut self, top_height: u32) -> &mut Self {
        self.top_height = top_height;
        self
    }

    pub fn bottom_height(&mut self, bottom_height: u32) -> &mut Self {
        self.bottom_height = bottom_height;
        self
    }

    pub fn display_bracket(&mut self, display_bracket: bool) -> &mut Self {
        self.display_bracket = display_bracket;
        self
    }

    pub fn unicode(&mut self) -> &mut Self {
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

    #[must_use]
    pub const fn build(&self) -> DefaultLiner {
        let Self {
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
