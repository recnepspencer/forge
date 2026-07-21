use std::collections::BTreeSet;

type AllocationBasisEntry = (
    crate::evidence::UiMeasurementBasis,
    crate::obligations::selection::UiSelectedObligationSet,
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAllocationCatalogDeltaAdmissionDenial {
    EmptyDelta,
    DuplicateChangedRoot,
    DuplicateRemovedRoot,
    ChangedAndRemovedRoot,
    OverlappingChangedCoverage,
    Neighborhood(super::UiAllocationNeighborhoodDenial),
}

/// Candidate-graph-owned replacement input. Absence is not a caller claim that
/// a row is unchanged: runtime replacement authority derives the required
/// affected closure and rejects a delta that omits any affected row.
#[derive(Clone, Debug)]
pub struct UiAdmittedAllocationCatalogDelta {
    pub(crate) snapshot: crate::graph::UiGraphSnapshot,
    pub(crate) changed: Box<[AllocationBasisEntry]>,
    pub(crate) removed_roots: Box<[crate::graph::UiGraphNodeIdentity]>,
}

impl UiAdmittedAllocationCatalogDelta {
    pub(crate) fn graph_authority_identity(&self) -> crate::graph::UiGraphAuthorityIdentity {
        self.snapshot.authority_identity()
    }

    pub fn changed_row_count(&self) -> usize {
        self.changed.len()
    }

    pub fn removed_row_count(&self) -> usize {
        self.removed_roots.len()
    }

    pub fn changed_roots(&self) -> impl Iterator<Item = crate::graph::UiGraphNodeIdentity> + '_ {
        self.changed
            .iter()
            .map(|(basis, _)| basis.graph_node_identity())
    }
}

impl crate::graph::UiGraphSnapshot {
    pub fn admit_allocation_catalog_delta(
        &self,
        mut changed: Vec<AllocationBasisEntry>,
        mut removed_roots: Vec<crate::graph::UiGraphNodeIdentity>,
    ) -> Result<UiAdmittedAllocationCatalogDelta, UiAllocationCatalogDeltaAdmissionDenial> {
        if changed.is_empty() && removed_roots.is_empty() {
            return Err(UiAllocationCatalogDeltaAdmissionDenial::EmptyDelta);
        }
        removed_roots.sort_unstable();
        if removed_roots.windows(2).any(|roots| roots[0] == roots[1]) {
            return Err(UiAllocationCatalogDeltaAdmissionDenial::DuplicateRemovedRoot);
        }
        let removed = removed_roots.iter().copied().collect::<BTreeSet<_>>();
        let mut roots = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for (basis, selected) in &changed {
            let root = basis.graph_node_identity();
            if !roots.insert(root) {
                return Err(UiAllocationCatalogDeltaAdmissionDenial::DuplicateChangedRoot);
            }
            if removed.contains(&root) {
                return Err(UiAllocationCatalogDeltaAdmissionDenial::ChangedAndRemovedRoot);
            }
            let neighborhood = basis
                .admit_allocation_neighborhood(self, selected)
                .map_err(UiAllocationCatalogDeltaAdmissionDenial::Neighborhood)?;
            for member in neighborhood.members() {
                if !covered.insert(member.graph_node_identity()) {
                    return Err(
                        UiAllocationCatalogDeltaAdmissionDenial::OverlappingChangedCoverage,
                    );
                }
            }
        }
        changed.sort_by_key(|(basis, _)| basis.graph_node_identity());
        Ok(UiAdmittedAllocationCatalogDelta {
            snapshot: self.clone(),
            changed: changed.into_boxed_slice(),
            removed_roots: removed_roots.into_boxed_slice(),
        })
    }
}
