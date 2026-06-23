#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiAuthoredDeltaCounters {
    observed_modules: usize,
    parsed_modules: usize,
    authored_declarations_inspected: usize,
    authored_declarations_touched: usize,
    semantic_slices_emitted: usize,
}

impl WorthUiAuthoredDeltaCounters {
    pub(crate) fn new(
        observed_modules: usize,
        parsed_modules: usize,
        authored_declarations_inspected: usize,
        authored_declarations_touched: usize,
        semantic_slices_emitted: usize,
    ) -> Self {
        Self {
            observed_modules,
            parsed_modules,
            authored_declarations_inspected,
            authored_declarations_touched,
            semantic_slices_emitted,
        }
    }

    pub fn observed_modules(&self) -> usize {
        self.observed_modules
    }

    pub fn parsed_modules(&self) -> usize {
        self.parsed_modules
    }

    pub fn authored_declarations_inspected(&self) -> usize {
        self.authored_declarations_inspected
    }

    pub fn authored_declarations_touched(&self) -> usize {
        self.authored_declarations_touched
    }

    pub fn semantic_slices_emitted(&self) -> usize {
        self.semantic_slices_emitted
    }
}
