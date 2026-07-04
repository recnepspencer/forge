#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAspectEvidenceNeighborhood {
    declaration_artifact_indexes: Box<[usize]>,
    refs: Box<[crate::evidence::UiEvidenceRef]>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiAspectEvidenceLookupCost {
    aspect_identity_index_lookups: usize,
    aspect_scan_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiAspectEvidenceLookup<'a> {
    neighborhood: &'a UiAspectEvidenceNeighborhood,
    cost: UiAspectEvidenceLookupCost,
}

impl UiAspectEvidenceNeighborhood {
    pub(crate) fn new(
        declaration_artifact_indexes: Box<[usize]>,
        refs: Box<[crate::evidence::UiEvidenceRef]>,
    ) -> Self {
        Self {
            declaration_artifact_indexes,
            refs,
        }
    }

    pub(crate) fn declaration_artifact_indexes(&self) -> &[usize] {
        &self.declaration_artifact_indexes
    }

    pub(crate) fn refs(&self) -> &[crate::evidence::UiEvidenceRef] {
        &self.refs
    }
}

impl UiAspectEvidenceLookupCost {
    pub(crate) const fn index_lookups(self) -> usize {
        self.aspect_identity_index_lookups
    }

    #[cfg(test)]
    pub(crate) const fn aspect_identity_index_lookups(self) -> usize {
        self.aspect_identity_index_lookups
    }

    #[cfg(test)]
    pub(crate) const fn aspect_scan_count(self) -> usize {
        self.aspect_scan_count
    }
}

impl<'a> UiAspectEvidenceLookup<'a> {
    pub(crate) fn indexed_hit(neighborhood: &'a UiAspectEvidenceNeighborhood) -> Self {
        Self {
            neighborhood,
            cost: UiAspectEvidenceLookupCost {
                aspect_identity_index_lookups: 1,
                aspect_scan_count: 0,
            },
        }
    }

    pub(crate) const fn neighborhood(self) -> &'a UiAspectEvidenceNeighborhood {
        self.neighborhood
    }

    pub(crate) const fn cost(self) -> UiAspectEvidenceLookupCost {
        self.cost
    }
}
