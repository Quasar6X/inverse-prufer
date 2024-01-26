use num_traits::int::PrimInt;
use std::io::{Error, Write};

pub struct LinePosition {
    row: usize,
    col: usize,
}

impl LinePosition {
    pub fn new<Num1, Num2>(row: Num1, col: Num2) -> anyhow::Result<Self>
    where
        Num1: PrimInt,
        Num2: PrimInt,
        usize: TryFrom<Num1> + TryFrom<Num2>,
        <usize as TryFrom<Num1>>::Error: std::error::Error + Send + Sync + 'static,
        <usize as TryFrom<Num2>>::Error: std::error::Error + Send + Sync + 'static,
    {
        let row = usize::try_from(row)?;
        let col = usize::try_from(col)?;
        Ok(Self { row, col })
    }

    #[must_use]
    pub const fn from_usize(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

pub struct LineBuffer<'w, W: Write> {
    out: &'w mut W,
    flushed_row_count: usize,
    lines: Vec<String>,
}

impl<'w, W: Write> LineBuffer<'w, W> {
    pub fn new(out: &'w mut W) -> Self {
        LineBuffer {
            out,
            flushed_row_count: 0,
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, LinePosition { row, col }: LinePosition, text: &str) {
        let text_lines = text.lines();
        for (i, line) in text_lines.enumerate() {
            self.write_line(LinePosition { row: row + i, col }, line);
        }
    }

    //TODO docs
    /// # Errors
    pub fn flush_all(&mut self) -> Result<(), Error> {
        self.flush(self.flushed_row_count + self.lines.len())
    }

    //TODO docs
    /// # Errors
    pub fn flush(&mut self, rows: usize) -> Result<(), Error> {
        if rows <= self.flushed_row_count {
            return Ok(());
        }

        let current_line_count = self.lines.len();
        let delete_line_count = rows - self.flushed_row_count;

        if current_line_count <= delete_line_count {
            for line in &self.lines {
                self.out.write_all((line.clone() + "\n").as_bytes())?;
            }
            self.lines.clear();
        } else {
            for i in 0..delete_line_count {
                self.out
                    .write_all((self.lines[i].clone() + "\n").as_bytes())?;
            }
            self.lines = self.lines[delete_line_count..current_line_count].to_vec();
        }

        Ok(())
    }

    fn write_line(&mut self, LinePosition { row, col }: LinePosition, text_line: &str) {
        if row < self.flushed_row_count {
            return;
        }

        let current_line_count = self.lines.len();
        let line_index = row - self.flushed_row_count;

        let original_line = {
            if line_index < current_line_count {
                &self.lines[line_index]
            } else {
                (current_line_count..=line_index).for_each(|_| self.lines.push(String::new()));
                ""
            }
        };

        let new_line = Self::write_into_line(original_line, col, text_line);
        self.lines[line_index] = new_line;
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
                let end = context_line.chars().map(char::len_utf8).take(pos).sum();
                (context_line[..end].to_owned(), String::new())
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
