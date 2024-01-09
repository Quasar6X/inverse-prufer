use std::io::{Error, Write};

pub struct LineBuffer<'a, W: Write> {
    out: &'a mut W,
    flushed_row_count: usize,
    lines: Vec<String>,
}

impl<'a, W: Write> LineBuffer<'a, W> {
    pub fn new(out: &'a mut W) -> Self {
        LineBuffer {
            out,
            flushed_row_count: 0,
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, row: usize, col: usize, text: &str) {
        let text_lines = text.lines();
        for (i, line) in text_lines.enumerate() {
            self.write_line(row + i, col, line);
        }
    }

    pub fn flush_all(&mut self) -> Result<(), Error> {
        self.flush(self.flushed_row_count + self.lines.len())
    }

    pub fn flush(&mut self, rows: usize) -> Result<(), Error> {
        if rows <= self.flushed_row_count {
            return Ok(());
        }

        let current_line_count = self.lines.len();
        let delete_line_count = rows - self.flushed_row_count;

        if current_line_count <= delete_line_count {
            for line in self.lines.iter() {
                self.out.write((line.clone() + "\n").as_bytes())?;
            }
            self.lines.clear();
        } else {
            for i in 0..delete_line_count {
                self.out.write((self.lines[i].clone() + "\n").as_bytes())?;
            }
            self.lines = Vec::from_iter(
                self.lines[delete_line_count..current_line_count]
                    .iter()
                    .cloned(),
            );
        }

        Ok(())
    }

    fn write_line(&mut self, row: usize, col: usize, text_line: &str) {
        if row < self.flushed_row_count {
            return;
        }

        let current_line_count = self.lines.len();
        let line_index = row - self.flushed_row_count;

        let original_line = {
            if line_index < current_line_count {
                &self.lines[line_index as usize]
            } else {
                (current_line_count..=line_index).for_each(|_| self.lines.push("".to_owned()));
                ""
            }
        };

        let new_line = Self::write_into_line(original_line, col, text_line);
        self.lines[line_index] = new_line
    }

    fn write_into_line(context_line: &str, pos: usize, text_line: &str) -> String {
        let context_line_length = context_line.len();
        let (before_content, before_pad) = {
            if context_line_length <= pos {
                (
                    context_line.to_owned(),
                    [' ']
                        .repeat(pos - context_line_length)
                        .iter()
                        .collect::<String>(),
                )
            } else {
                let end = context_line.chars().map(|c| c.len_utf8()).take(pos).sum();
                (context_line[..end].to_owned(), "".to_owned())
            }
        };

        let text_line_length = text_line.len();
        let after_content = {
            if pos + text_line_length < context_line_length {
                &context_line[pos + text_line_length..]
            } else {
                ""
            }
        };

        before_content + &before_pad + text_line + after_content
    }
}
