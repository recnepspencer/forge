#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyReplayScopeProductCounters {
    touched_subject_count: usize,
}

impl TopologyReplayScopeProductCounters {
    pub(crate) const fn new(touched_subject_count: usize) -> Self {
        Self {
            touched_subject_count,
        }
    }

    pub const fn touched_subject_count(&self) -> usize {
        self.touched_subject_count
    }
}
