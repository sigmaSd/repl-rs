pub struct History {
    pub buffer_vec: Vec<String>,
    cursor: usize,
    pub line_idx: Vec<usize>,
    internal_line_counter: usize,
    empty: String,
}
impl Default for History {
    fn default() -> Self {
        Self {
            line_idx: vec![0],
            buffer_vec: Vec::new(),
            cursor: 0,
            internal_line_counter: 0,
            empty: String::new(),
        }
    }
}
impl History {
    pub fn down(&mut self) -> String {
        self.cursor += 2;
        if self.cursor > self.buffer_vec.len() {
            self.cursor = self.buffer_vec.len();
        }
        self.buffer_vec
            .get(self.cursor)
            .unwrap_or(&self.empty)
            .clone()
    }
    pub fn up(&mut self) -> String {
        if self.cursor >= 2 {
            self.cursor -= 2;
        }
        self.buffer_vec
            .get(self.cursor)
            .unwrap_or(&self.empty)
            .clone()
    }
    pub fn push(&mut self, buffer: String) {
        self.buffer_vec.push(buffer);
        self.cursor += 1;

        let last = *self.line_idx.last().unwrap();
        let before_last = before_last(&self.line_idx);

        if self.internal_line_counter != 0 && last == before_last {
            self.line_idx.push(last + 1);
        } else {
            self.line_idx.push(last);
        }
        self.internal_line_counter += 1;
    }
    pub fn last_idx(&self) -> usize {
        *self.line_idx.last().unwrap()
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// helper fns
fn before_last(vec: &[usize]) -> usize {
    let mut before_last_n = 0;
    if vec.len() > 1 {
        for (idx, x) in vec.iter().enumerate() {
            if idx == vec.len() - 2 {
                before_last_n = *x;
            }
        }
    }
    before_last_n
}
