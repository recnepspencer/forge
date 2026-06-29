use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphLocalityScope;
use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};

use super::topology::{TopologyConflictPlanDenial, TopologyConflictPlanDenialKind};
use crate::workload_composition::conflict_input::{
    AdmittedTopologyConflictInput, AdmittedTopologyConflictRoute,
};
use crate::workload_composition::conflict_plan::ConflictPlanDownstreamProofCategory;
use topology::touched_graph_conflict::{
    TopologyConflictDiagnosticWitness, TopologyConflictFamilyCatalogCloseout,
    TopologyConflictFamilyDeclaration, TopologyConflictLocalityAuthorityRequirement,
    TopologyConflictPriorProofPosture, TopologyConflictSelectionProductPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TopologySelectionMiss {
    NoMatchingFamily,
    MissingRequiredPriorProof,
}

pub(super) struct TopologySelectionContext<'a> {
    pub matching_families: Vec<&'a TopologyConflictFamilyDeclaration>,
    pub admitted_prior_proof: TopologyConflictPriorProofPosture,
    pub downstream_proof_category: ConflictPlanDownstreamProofCategory,
    pub miss: TopologySelectionMiss,
}

pub(super) fn route_selection_context<'a>(
    catalog_closeout: &'a TopologyConflictFamilyCatalogCloseout,
    admitted_input: &AdmittedTopologyConflictInput<'a>,
) -> TopologySelectionContext<'a> {
    let (admitted_prior_proof, downstream_proof_category) = match admitted_input.route() {
        AdmittedTopologyConflictRoute::AspectLocality(_) => (
            TopologyConflictPriorProofPosture::NoPriorProofRequired,
            ConflictPlanDownstreamProofCategory::ProjectionConsumption,
        ),
        AdmittedTopologyConflictRoute::ReplayBoundary(_) => (
            TopologyConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
            ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope,
        ),
    };
    let mut matching_families = Vec::new();
    let mut prior_proof_miss = false;
    for declaration in catalog_closeout.catalog().declarations() {
        if !matches_topology_route_shape(declaration, &admitted_input.route()) {
            continue;
        }
        if !matches_topology_applicability(declaration, admitted_input) {
            continue;
        }
        if declaration.prior_proof_posture() == admitted_prior_proof {
            matching_families.push(declaration);
        } else {
            prior_proof_miss = true;
        }
    }
    TopologySelectionContext {
        matching_families,
        admitted_prior_proof,
        downstream_proof_category,
        miss: if prior_proof_miss {
            TopologySelectionMiss::MissingRequiredPriorProof
        } else {
            TopologySelectionMiss::NoMatchingFamily
        },
    }
}

pub(super) fn selected_plan_denial(
    miss: TopologySelectionMiss,
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
    selected_none: bool,
) -> Option<TopologyConflictPlanDenial> {
    if !selected_none {
        return None;
    }
    let (kind, detail) = match miss {
        TopologySelectionMiss::MissingRequiredPriorProof => (
            TopologyConflictPlanDenialKind::MissingRequiredPriorProof,
            "selected topology conflict plan requires prior-proof posture already declared by the admitted route before execution",
        ),
        TopologySelectionMiss::NoMatchingFamily => (
            TopologyConflictPlanDenialKind::NoMatchingFamily,
            "selected topology conflict plan found no conflict family declaration for the admitted route and overlap class",
        ),
    };
    Some(TopologyConflictPlanDenial {
        kind,
        downstream_proof_category,
        detail: detail.to_string(),
    })
}

fn matches_topology_route_shape(
    declaration: &TopologyConflictFamilyDeclaration,
    route: &AdmittedTopologyConflictRoute,
) -> bool {
    match route {
        AdmittedTopologyConflictRoute::AspectLocality(_) => matches!(
            (
                declaration.locality_authority_requirement(),
                declaration.primary_overlap_category(),
                declaration.secondary_overlap_category(),
                declaration.diagnostic_witness(),
                declaration.selection_product_posture(),
            ),
            (
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
                ConflictOverlapCategory::Aspect,
                Some(ConflictOverlapCategory::Locality),
                TopologyConflictDiagnosticWitness::TouchedClosureDigest,
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
            )
        ),
        AdmittedTopologyConflictRoute::ReplayBoundary(_) => matches!(
            (
                declaration.locality_authority_requirement(),
                declaration.primary_overlap_category(),
                declaration.secondary_overlap_category(),
                declaration.diagnostic_witness(),
                declaration.selection_product_posture(),
            ),
            (
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
                ConflictOverlapCategory::ReplayUndo,
                Some(ConflictOverlapCategory::Transaction),
                TopologyConflictDiagnosticWitness::ReplayBoundaryScope,
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
            )
        ),
    }
}

fn matches_topology_applicability(
    declaration: &TopologyConflictFamilyDeclaration,
    admitted_input: &AdmittedTopologyConflictInput<'_>,
) -> bool {
    let contract = admitted_input.routing_contract();
    let overlap = contract.overlap_identity();
    let Some(locality) = overlap.locality_identity() else {
        return false;
    };
    locality.scope() == ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure
        && locality.authority_digest() == admitted_input.touched_closure().closure_digest()
        && contract.posture() == ConflictRoutingPosture::RequiresFamilySelection
        && (overlap.category() == declaration.primary_overlap_category()
            || declaration.secondary_overlap_category() == Some(overlap.category()))
        && if declaration.primary_overlap_category() == ConflictOverlapCategory::Validator {
            !overlap.participants().is_empty()
        } else {
            true
        }
}
