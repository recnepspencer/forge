#[derive(Debug, Clone, Default)]
pub(crate) struct TraversalScratch {
    pub(crate) visited: VisitMarks,
    pub(crate) active: VisitMarks,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct VisitMarks {
    marks: Vec<u32>,
    epoch: u32,
}

impl VisitMarks {
    pub(crate) fn next_pass(&mut self, len: usize) {
        if self.marks.len() < len {
            self.marks.resize(len, 0);
        }
        if self.epoch == u32::MAX {
            self.marks.fill(0);
            self.epoch = 1;
        } else {
            self.epoch += 1;
        }
    }

    pub(crate) fn is_marked(&self, idx: usize) -> bool {
        idx < self.marks.len() && self.marks[idx] == self.epoch
    }

    pub(crate) fn mark(&mut self, idx: usize) -> bool {
        if idx >= self.marks.len() {
            self.marks.resize(idx + 1, 0);
        }
        if self.marks[idx] == self.epoch {
            false
        } else {
            self.marks[idx] = self.epoch;
            true
        }
    }

    pub(crate) fn clear(&mut self, idx: usize) {
        if idx < self.marks.len() {
            self.marks[idx] = 0;
        }
    }
}
