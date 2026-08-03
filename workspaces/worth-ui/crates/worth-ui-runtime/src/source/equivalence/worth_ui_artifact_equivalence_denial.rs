#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactEquivalenceDenial {
    structural_entry_limit: usize,
    observed_structural_entries: usize,
}

impl WorthUiArtifactEquivalenceDenial {
    pub(crate) const fn structural_capacity_exceeded(
        structural_entry_limit: usize,
        observed_structural_entries: usize,
    ) -> Self {
        Self {
            structural_entry_limit,
            observed_structural_entries,
        }
    }

    pub(crate) const fn structural_entry_limit(self) -> usize {
        self.structural_entry_limit
    }

    pub(crate) const fn observed_structural_entries(self) -> usize {
        self.observed_structural_entries
    }
}
