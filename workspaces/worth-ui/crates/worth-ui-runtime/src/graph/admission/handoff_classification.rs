use crate::declaration::UiDeclarationGraphHandoff;
use crate::graph::{
    UiGraphInstantiationLocalDenial, UiGraphNodeInstantiationEntry, UiGraphTopologyLocalDenial,
    UiRepeatedInstanceBasis, UiRepeatedInstanceBasisDenial,
};

use super::runtime_basis_assignment::UiRuntimeBasisAssignments;

pub(super) enum HandoffBasisDecision {
    Admitted(UiRepeatedInstanceBasis),
    Denied {
        declaration_identity: crate::declaration::UiDeclarationIdentity,
        denial: UiRepeatedInstanceBasisDenial,
    },
}

pub(super) enum RootTopologyDecision {
    Valid,
    Invalid { denial: UiGraphTopologyLocalDenial },
}

pub(super) fn classify_handoff_basis(
    handoff: &UiDeclarationGraphHandoff,
    occurrence_index: usize,
    runtime_basis_assignments: &UiRuntimeBasisAssignments,
) -> HandoffBasisDecision {
    let declaration_identity = handoff.identity().clone();
    let declaration_digest = declaration_identity.digest().raw();
    let repeated_instance_basis = runtime_basis_assignments
        .basis_for(declaration_digest, occurrence_index)
        .cloned()
        .unwrap_or_else(|| UiRepeatedInstanceBasis::declaration_keyed(declaration_identity.digest()));

    if repeated_instance_basis.denial()
        == Some(&UiRepeatedInstanceBasisDenial::BasisFreeRuntimeIdentityDenied)
    {
        HandoffBasisDecision::Denied {
            declaration_identity,
            denial: UiRepeatedInstanceBasisDenial::BasisFreeRuntimeIdentityDenied,
        }
    } else {
        HandoffBasisDecision::Admitted(repeated_instance_basis)
    }
}

pub(super) fn classify_root_topology_cardinality(
    observed_root_pages: usize,
) -> RootTopologyDecision {
    if observed_root_pages == 1 {
        RootTopologyDecision::Valid
    } else {
        RootTopologyDecision::Invalid {
            denial: UiGraphTopologyLocalDenial::RootPageCardinality { observed_root_pages },
        }
    }
}

pub(super) fn assemble_topology_denials(
    node_entries: &mut Vec<UiGraphNodeInstantiationEntry>,
    denial: UiGraphTopologyLocalDenial,
) -> Vec<UiGraphInstantiationLocalDenial> {
    node_entries
        .drain(..)
        .map(|entry| {
            UiGraphInstantiationLocalDenial::topology(
                entry.declaration_identity().clone(),
                denial.clone(),
            )
        })
        .collect()
}