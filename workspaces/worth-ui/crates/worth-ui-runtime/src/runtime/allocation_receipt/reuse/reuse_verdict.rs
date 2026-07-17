/// Closed reuse outcomes. Only leaf remeasurement is partial reuse in 3.8.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAllocationReuseVerdict {
    NewCommit,
    FullReuse,
    StructureReuseLeafRemeasure(UiAllocationLeafRemeasureWitness),
    Denied(UiAllocationReuseDenial),
}

/// Commit-owned proof that 3.8's sole partial-reuse posture preserved structure
/// while remeasuring admitted changed leaves. Raw identifiers cannot enter the verdict.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationLeafRemeasureWitness {
    preserved_structure_scope: crate::evidence::UiAllocationNeighborhoodScope,
    leaf_measurement_generation: crate::evidence::UiMeasurementBasisGeneration,
    preserved_structure_graph_node_identities: Box<[crate::graph::UiGraphNodeIdentity]>,
    leaf_graph_node_identities: Box<[crate::graph::UiGraphNodeIdentity]>,
}

impl UiAllocationLeafRemeasureWitness {
    pub(crate) fn from_admitted_leaf_difference(
        candidate: &super::UiAllocationCandidate,
        previous: &super::UiAllocationReceipt,
    ) -> Option<Self> {
        if !matches!(
            candidate.allocation_neighborhood().neighborhood_class(),
            crate::evidence::UiAllocationNeighborhoodClass::LocalIntrinsicContent
        ) {
            return None;
        }
        let current_basis = candidate.measurement_basis();
        let previous_basis = previous.committed_allocation().measurement_basis();
        if non_child_inputs(current_basis) != non_child_inputs(previous_basis) {
            return None;
        }
        let changed_leaf_graph_node_identities =
            changed_child_intrinsic_nodes(current_basis, previous_basis)?;
        if changed_leaf_graph_node_identities.is_empty() {
            return None;
        }
        let mut neighborhood_nodes = candidate
            .allocation_neighborhood()
            .members()
            .iter()
            .map(crate::evidence::UiAllocationNeighborhoodMember::graph_node_identity)
            .collect::<Vec<_>>();
        neighborhood_nodes.sort_unstable();
        if changed_leaf_graph_node_identities
            .iter()
            .any(|identity| neighborhood_nodes.binary_search(identity).is_err())
        {
            return None;
        }
        let preserved_structure_graph_node_identities: Vec<_> = neighborhood_nodes
            .iter()
            .copied()
            .filter(|identity| {
                changed_leaf_graph_node_identities
                    .binary_search(identity)
                    .is_err()
            })
            .collect();
        Some(Self {
            preserved_structure_scope: previous.identity().neighborhood_scope().clone(),
            leaf_measurement_generation: candidate.measurement_basis().generation(),
            preserved_structure_graph_node_identities: preserved_structure_graph_node_identities
                .into_boxed_slice(),
            leaf_graph_node_identities: changed_leaf_graph_node_identities.into_boxed_slice(),
        })
    }
    pub fn preserved_structure_scope(&self) -> &crate::evidence::UiAllocationNeighborhoodScope {
        &self.preserved_structure_scope
    }

    pub fn leaf_measurement_generation(&self) -> crate::evidence::UiMeasurementBasisGeneration {
        self.leaf_measurement_generation
    }

    pub fn preserved_structure_graph_node_identities(
        &self,
    ) -> &[crate::graph::UiGraphNodeIdentity] {
        &self.preserved_structure_graph_node_identities
    }

    pub fn leaf_graph_node_identities(&self) -> &[crate::graph::UiGraphNodeIdentity] {
        &self.leaf_graph_node_identities
    }
}

fn non_child_inputs(
    basis: &crate::evidence::UiMeasurementBasis,
) -> Vec<crate::evidence::MeasurementEvidenceInput> {
    basis
        .evidence_inputs()
        .iter()
        .filter(|input| input.as_child_intrinsic_measurement().is_none())
        .cloned()
        .collect()
}

fn changed_child_intrinsic_nodes(
    current: &crate::evidence::UiMeasurementBasis,
    previous: &crate::evidence::UiMeasurementBasis,
) -> Option<Vec<crate::graph::UiGraphNodeIdentity>> {
    let mut current_children = child_intrinsic_inputs(current);
    let mut previous_children = child_intrinsic_inputs(previous);
    if current_children.is_empty() || current_children.len() != previous_children.len() {
        return None;
    }
    current_children.sort_unstable_by_key(|(identity, _)| *identity);
    previous_children.sort_unstable_by_key(|(identity, _)| *identity);
    let mut changed = Vec::new();
    for ((current_identity, current_evidence), (previous_identity, previous_evidence)) in
        current_children.into_iter().zip(previous_children)
    {
        if current_identity != previous_identity {
            return None;
        }
        if current_evidence != previous_evidence {
            changed.push(current_identity);
        }
    }
    Some(changed)
}

fn child_intrinsic_inputs(
    basis: &crate::evidence::UiMeasurementBasis,
) -> Vec<(
    crate::graph::UiGraphNodeIdentity,
    crate::evidence::measurement::UiChildIntrinsicMeasurementEvidence,
)> {
    basis
        .evidence_inputs()
        .iter()
        .filter_map(|input| {
            input
                .as_child_intrinsic_measurement()
                .map(|evidence| (evidence.contributor_graph_node_identity(), evidence.clone()))
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationReuseDenial {
    ReceiptIdentityMismatch,
    GenerationMismatch,
    EquivalenceBasisMismatch,
    UnsupportedPartialReuse,
}
