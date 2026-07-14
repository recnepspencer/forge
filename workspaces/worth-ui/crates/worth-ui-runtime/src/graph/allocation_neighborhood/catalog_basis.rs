use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAllocationCatalogBasisAdmissionDenial {
    EmptyCatalog,
    DuplicateMeasurementRoot,
    OverlappingGraphCoverage,
    IncompleteGraphCoverage,
    Neighborhood(super::UiAllocationNeighborhoodDenial),
}

/// Graph-owned complete partition of admitted allocation planning inputs.
#[derive(Clone, Debug)]
pub struct UiAdmittedAllocationCatalogBasisSet {
    pub(crate) snapshot: crate::graph::UiGraphSnapshot,
    pub(crate) entries: Box<
        [(
            crate::evidence::UiMeasurementBasis,
            crate::obligations::selection::UiSelectedObligationSet,
        )],
    >,
}

impl crate::graph::UiGraphSnapshot {
    pub fn admit_allocation_catalog_basis_set(
        &self,
        entries: Vec<(
            crate::evidence::UiMeasurementBasis,
            crate::obligations::selection::UiSelectedObligationSet,
        )>,
    ) -> Result<UiAdmittedAllocationCatalogBasisSet, UiAllocationCatalogBasisAdmissionDenial> {
        if entries.is_empty() {
            return Err(UiAllocationCatalogBasisAdmissionDenial::EmptyCatalog);
        }
        let mut roots = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for (basis, selected) in &entries {
            if !roots.insert(basis.graph_node_identity()) {
                return Err(UiAllocationCatalogBasisAdmissionDenial::DuplicateMeasurementRoot);
            }
            let neighborhood = basis
                .admit_allocation_neighborhood(self, selected)
                .map_err(UiAllocationCatalogBasisAdmissionDenial::Neighborhood)?;
            for member in neighborhood.members() {
                if !covered.insert(member.graph_node_identity()) {
                    return Err(UiAllocationCatalogBasisAdmissionDenial::OverlappingGraphCoverage);
                }
            }
        }
        let expected = self
            .allocation_planning_node_identities()
            .collect::<BTreeSet<_>>();
        if covered != expected {
            return Err(UiAllocationCatalogBasisAdmissionDenial::IncompleteGraphCoverage);
        }
        Ok(UiAdmittedAllocationCatalogBasisSet {
            snapshot: self.clone(),
            entries: entries.into_boxed_slice(),
        })
    }
}
