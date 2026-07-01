use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphLocalityScope;
use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};

use super::spatial::{SpatialConflictPlanDenial, SpatialConflictPlanDenialKind};
use crate::workload_composition::conflict_input::{
    AdmittedSpatialConflictInput, AdmittedSpatialConflictRoute,
};
use crate::workload_composition::conflict_plan::ConflictPlanDownstreamProofCategory;
use worth_spatial::touched_graph_conflict::{
    SpatialConflictDiagnosticWitness, SpatialConflictFamilyCatalogCloseout,
    SpatialConflictFamilyDeclaration, SpatialConflictLocalityAuthorityRequirement,
    SpatialConflictPriorProofPosture, SpatialConflictSelectionProductPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SpatialSelectionMiss {
    NoMatchingFamily,
    MissingRequiredPriorProof,
}

pub(super) struct SpatialSelectionContext<'a> {
    pub matching_families: Vec<&'a SpatialConflictFamilyDeclaration>,
    pub admitted_prior_proof: SpatialConflictPriorProofPosture,
    pub downstream_proof_category: ConflictPlanDownstreamProofCategory,
    pub miss: SpatialSelectionMiss,
}

pub(super) fn route_selection_context<'a>(
    catalog_closeout: &'a SpatialConflictFamilyCatalogCloseout,
    admitted_input: &AdmittedSpatialConflictInput<'a>,
) -> SpatialSelectionContext<'a> {
    let (admitted_prior_proof, downstream_proof_category) = match admitted_input.route() {
        AdmittedSpatialConflictRoute::EvidenceLookup { .. }
        | AdmittedSpatialConflictRoute::LookupCompiledProduct { .. } => (
            SpatialConflictPriorProofPosture::NoPriorProofRequired,
            ConflictPlanDownstreamProofCategory::ProjectionConsumption,
        ),
        AdmittedSpatialConflictRoute::ReplayBoundary(_) => (
            SpatialConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
            ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope,
        ),
    };
    let mut matching_families = Vec::new();
    let mut prior_proof_miss = false;
    for declaration in catalog_closeout.catalog().declarations() {
        if !matches_spatial_route_shape(declaration, &admitted_input.route()) {
            continue;
        }
        if !matches_spatial_applicability(declaration, admitted_input) {
            continue;
        }
        if declaration.prior_proof_posture() == admitted_prior_proof {
            matching_families.push(declaration);
        } else {
            prior_proof_miss = true;
        }
    }
    SpatialSelectionContext {
        matching_families,
        admitted_prior_proof,
        downstream_proof_category,
        miss: if prior_proof_miss {
            SpatialSelectionMiss::MissingRequiredPriorProof
        } else {
            SpatialSelectionMiss::NoMatchingFamily
        },
    }
}

pub(super) fn selected_plan_denial(
    miss: SpatialSelectionMiss,
    downstream_proof_category: ConflictPlanDownstreamProofCategory,
    selected_none: bool,
) -> Option<SpatialConflictPlanDenial> {
    if !selected_none {
        return None;
    }
    let (kind, detail) = match miss {
        SpatialSelectionMiss::MissingRequiredPriorProof => (
            SpatialConflictPlanDenialKind::MissingRequiredPriorProof,
            "selected spatial conflict plan requires prior-proof posture already declared by the admitted route before execution",
        ),
        SpatialSelectionMiss::NoMatchingFamily => (
            SpatialConflictPlanDenialKind::NoMatchingFamily,
            "selected spatial conflict plan found no conflict family declaration for the admitted route and overlap class",
        ),
    };
    Some(SpatialConflictPlanDenial {
        kind,
        downstream_proof_category,
        detail: detail.to_string(),
    })
}

fn matches_spatial_route_shape(
    declaration: &SpatialConflictFamilyDeclaration,
    route: &AdmittedSpatialConflictRoute,
) -> bool {
    match route {
        AdmittedSpatialConflictRoute::EvidenceLookup { .. }
        | AdmittedSpatialConflictRoute::LookupCompiledProduct { .. } => matches!(
            (
                declaration.locality_authority_requirement(),
                declaration.primary_overlap_category(),
                declaration.secondary_overlap_category(),
                declaration.diagnostic_witness(),
                declaration.selection_product_posture(),
            ),
            (
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
                ConflictOverlapCategory::Evidence,
                None,
                SpatialConflictDiagnosticWitness::EvidenceFamilyDigest,
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
            )
        ),
        AdmittedSpatialConflictRoute::ReplayBoundary(_) => matches!(
            (
                declaration.locality_authority_requirement(),
                declaration.primary_overlap_category(),
                declaration.secondary_overlap_category(),
                declaration.diagnostic_witness(),
                declaration.selection_product_posture(),
            ),
            (
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
                ConflictOverlapCategory::ReplayUndo,
                Some(ConflictOverlapCategory::Transaction),
                SpatialConflictDiagnosticWitness::ReplayBoundaryScope,
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
            )
        ),
    }
}

fn matches_spatial_applicability(
    declaration: &SpatialConflictFamilyDeclaration,
    admitted_input: &AdmittedSpatialConflictInput<'_>,
) -> bool {
    let contract = admitted_input.routing_contract();
    let overlap = contract.overlap_identity();
    let Some(locality) = overlap.locality_identity() else {
        return false;
    };
    locality.scope() == ReplayUndoSemanticGraphLocalityScope::SpatialTouchAuthority
        && locality.authority_digest() == admitted_input.authority().digest().as_str()
        && contract.posture() == ConflictRoutingPosture::RequiresFamilySelection
        && (overlap.category() == declaration.primary_overlap_category()
            || declaration.secondary_overlap_category() == Some(overlap.category()))
        && if declaration.primary_overlap_category() == ConflictOverlapCategory::Evidence {
            admitted_input
                .authority()
                .conflict_participant_identity()
                .ok()
                .is_some_and(|participant| {
                    overlap
                        .participants()
                        .iter()
                        .any(|candidate| candidate.digest() == participant.digest())
                })
        } else {
            true
        }
}
