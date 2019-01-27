use std::iter::once;

pub struct History {
    pub buffer_vec: Vec<String>,
    cursor: usize,
    pub line_idx: Vec<usize>,
    internal_line_counter: usize,
}
impl Default for History {
    fn default() -> Self {
        Self {
            line_idx: (0..)
                .zip(0..)
                .take(5)
                .collect::<Vec<(usize, usize)>>()
                .iter()
                .flat_map(|(a, b)| once(*a).chain(once(*b)))
                .collect::<Vec<usize>>(),
            buffer_vec: Vec::new(),
            cursor: 0,
            internal_line_counter: 0,
        }
    }
}
impl History {
    pub fn down(&mut self) -> String {
        if self.cursor != self.buffer_vec.len() - 1 {
            self.cursor += 1;
        }
        self.buffer_vec[self.cursor].clone()
    }
    pub fn up(&mut self) -> String {
        if self.cursor != 0 {
            self.cursor -= 1;
        }
        self.buffer_vec[self.cursor].clone()
    }
    pub fn push(&mut self, buffer: String) {
        self.buffer_vec.push(buffer);
        self.cursor += 1;
        self.internal_line_counter += 1;
    }
    pub fn last_idx(&self) -> usize {
        self.line_idx[self.internal_line_counter]
    }
}
