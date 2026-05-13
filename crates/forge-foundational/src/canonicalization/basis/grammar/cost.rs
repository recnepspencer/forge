#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalizationCost {
    entry_count: u32,
    ordering_comparisons: u32,
    nested_sequence_count: u32,
    compatibility_lowering_count: u32,
}

impl CanonicalizationCost {
    pub const fn new(
        entry_count: u32,
        ordering_comparisons: u32,
        nested_sequence_count: u32,
        compatibility_lowering_count: u32,
    ) -> Self {
        Self {
            entry_count,
            ordering_comparisons,
            nested_sequence_count,
            compatibility_lowering_count,
        }
    }

    pub const fn entry_count(&self) -> u32 {
        self.entry_count
    }

    pub const fn ordering_comparisons(&self) -> u32 {
        self.ordering_comparisons
    }

    pub const fn nested_sequence_count(&self) -> u32 {
        self.nested_sequence_count
    }

    pub const fn compatibility_lowering_count(&self) -> u32 {
        self.compatibility_lowering_count
    }
}
