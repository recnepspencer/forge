#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphIndexInventoryCounters {
    inventory_row_count: usize,
    requirement_row_count: usize,
    matched_requirement_count: usize,
    unsupported_requirement_count: usize,
    generic_missing_index_count: usize,
}

impl ForgeQueryGraphIndexInventoryCounters {
    pub fn inventory_row_count(&self) -> usize {
        self.inventory_row_count
    }

    pub fn requirement_row_count(&self) -> usize {
        self.requirement_row_count
    }

    pub fn matched_requirement_count(&self) -> usize {
        self.matched_requirement_count
    }

    pub fn unsupported_requirement_count(&self) -> usize {
        self.unsupported_requirement_count
    }

    pub fn generic_missing_index_count(&self) -> usize {
        self.generic_missing_index_count
    }

    pub(crate) fn new(
        inventory_row_count: usize,
        requirement_row_count: usize,
        matched_requirement_count: usize,
        unsupported_requirement_count: usize,
        generic_missing_index_count: usize,
    ) -> Self {
        Self {
            inventory_row_count,
            requirement_row_count,
            matched_requirement_count,
            unsupported_requirement_count,
            generic_missing_index_count,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "counters:{}:{}:{}:{}:{}",
            self.inventory_row_count,
            self.requirement_row_count,
            self.matched_requirement_count,
            self.unsupported_requirement_count,
            self.generic_missing_index_count
        )
    }
}
