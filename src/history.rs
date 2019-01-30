pub struct History {
    pub buffer_vec: Vec<String>,
    pub line_idx: Vec<usize>,
    cursor: usize,
    empty: String,
}
impl Default for History {
    fn default() -> Self {
        Self {
            line_idx: vec![0],
            buffer_vec: Vec::new(),
            cursor: 0,
            empty: String::new(),
        }
    }
}
impl History {
    pub fn down(&mut self) -> String {
        self.cursor += 1;
        if self.cursor > self.buffer_vec.len() {
            self.cursor = self.buffer_vec.len();
        }
        self.buffer_vec
            .get(self.cursor)
            .unwrap_or(&self.empty)
            .clone()
    }
    pub fn up(&mut self) -> String {
        let res = self
            .buffer_vec
            .get(self.cursor)
            .unwrap_or(&self.empty)
            .clone();
        if self.cursor >= 1 {
            self.cursor -= 1;
        }
        res
    }
    pub fn go_to_last(&mut self) {
        if !self.buffer_vec.is_empty() {
            self.cursor = self.buffer_vec.len() - 1;
        }
    }
    pub fn push(&mut self, buffer: String) {
        self.buffer_vec.push(buffer);
        self.cursor += 1;
        self.increase_line_idx();
    }
    fn increase_line_idx(&mut self) {
        self.line_idx.push(self.last_idx() + 1);
    }
    pub fn last_idx(&self) -> usize {
        *self.line_idx.last().unwrap()
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
