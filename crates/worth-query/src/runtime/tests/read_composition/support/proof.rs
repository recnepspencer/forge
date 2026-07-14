use crate::relationship_proof::{RelationshipProofSupportStatus, RelationshipProofSurface};
use crate::runtime::{
    WorthQueryReadBuiltInOperator, WorthQueryReadBuiltInOperatorDenialReason, WorthQueryReadDenial,
    WorthQueryReadDenialKind, WorthQueryReadGraphFamily, WorthQueryReadRelationshipProofPosture,
    WorthQueryReadResult, WorthQueryReadScopeClass,
};

pub(crate) fn assert_scope_shape_denial(
    denial: &WorthQueryReadDenial,
    expected: WorthQueryReadScopeClass,
    actual: WorthQueryReadScopeClass,
) {
    assert_eq!(denial.kind(), &WorthQueryReadDenialKind::ScopeShapeDenied);
    let mismatch = denial
        .scope_shape_mismatch()
        .expect("scope-shape denials should expose structured mismatch evidence");
    assert_eq!(mismatch.expected(), &expected);
    assert_eq!(mismatch.actual(), &actual);
}

pub(crate) fn assert_built_in_operator_denial(
    denial: &WorthQueryReadDenial,
    expected_operator: WorthQueryReadBuiltInOperator,
    expected_reason: WorthQueryReadBuiltInOperatorDenialReason,
) {
    assert_eq!(
        denial.kind(),
        &WorthQueryReadDenialKind::BuiltInOperatorDenied
    );
    let operator_denial = denial
        .built_in_operator_denial()
        .expect("built-in operator denials should expose structured evidence");
    assert_eq!(operator_denial.operator(), &expected_operator);
    assert_eq!(operator_denial.reason(), &expected_reason);
}

pub(crate) fn assert_collection_receipt(
    result: &WorthQueryReadResult,
    expected_scope: WorthQueryReadScopeClass,
) {
    let receipt = result.receipt();
    assert_eq!(
        receipt.graph_family(),
        &WorthQueryReadGraphFamily::Collection
    );
    assert_eq!(receipt.scope_class(), &expected_scope);
    assert!(receipt
        .operator_families()
        .contains(&crate::runtime::WorthQueryReadOperatorFamily::Ordering));
    assert_eq!(receipt.breadth().execution_cursor_advance_count(), 1);
    assert!(receipt.breadth().execution_page_width() > 0);
    assert_eq!(receipt.breadth().execution_page_truncation_count(), 0);
}

pub(crate) fn assert_relationship_proof_not_required(result: &WorthQueryReadResult) {
    let receipt = result.receipt();
    assert_eq!(
        receipt.relationship_proof_posture(),
        &WorthQueryReadRelationshipProofPosture::NotRequired
    );
    assert_eq!(receipt.relationship_proof_support_profile_digest(), None);
    assert_eq!(receipt.relationship_proof_verified_surface_count(), 0);
    assert_eq!(receipt.relationship_proof_deferred_surface_count(), 0);
    assert_eq!(receipt.relationship_proof_forbidden_surface_count(), 0);
}

pub(crate) fn assert_descriptor_admitted_synthetic_runtime_relationship_proof(
    result: &WorthQueryReadResult,
    expected_descriptor_count: usize,
    expected_surfaces: &[(RelationshipProofSurface, RelationshipProofSupportStatus)],
) {
    let receipt = result.receipt();
    let profile = receipt
        .relationship_proof_support_profile()
        .expect("descriptor-admitted traversal reads should expose a support profile");
    assert_eq!(
        receipt.relationship_proof_posture(),
        &WorthQueryReadRelationshipProofPosture::DescriptorAdmittedSyntheticRuntime
    );
    assert_eq!(
        receipt.relationship_proof_descriptor_count(),
        expected_descriptor_count
    );
    assert!(receipt
        .relationship_proof_admission_identity()
        .is_some_and(|identity| !identity.is_empty()));
    assert_eq!(profile.surfaces(), expected_surfaces);
    assert_eq!(
        receipt.relationship_proof_support_profile_digest(),
        Some(profile.profile_digest())
    );
    assert_eq!(
        receipt.relationship_proof_verified_surface_count(),
        expected_surfaces
            .iter()
            .filter(|(_, status)| *status == RelationshipProofSupportStatus::Verified)
            .count()
    );
    assert_eq!(
        receipt.relationship_proof_deferred_surface_count(),
        expected_surfaces
            .iter()
            .filter(|(_, status)| *status == RelationshipProofSupportStatus::Deferred)
            .count()
    );
    assert_eq!(
        receipt.relationship_proof_forbidden_surface_count(),
        expected_surfaces
            .iter()
            .filter(|(_, status)| *status == RelationshipProofSupportStatus::Forbidden)
            .count()
    );
}

pub(crate) fn direct_edge_synthetic_runtime_surfaces(
) -> [(RelationshipProofSurface, RelationshipProofSupportStatus); 5] {
    [
        (
            RelationshipProofSurface::DescriptorAdmission,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::DirectEdgeTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::TenantMembershipTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::RuntimeProofEvaluation,
            RelationshipProofSupportStatus::Deferred,
        ),
        (
            RelationshipProofSurface::HostCallbackProofs,
            RelationshipProofSupportStatus::Forbidden,
        ),
    ]
}

pub(crate) fn bounded_ancestor_synthetic_runtime_surfaces(
) -> [(RelationshipProofSurface, RelationshipProofSupportStatus); 5] {
    [
        (
            RelationshipProofSurface::DescriptorAdmission,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::BoundedAncestorTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::TenantMembershipTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::RuntimeProofEvaluation,
            RelationshipProofSupportStatus::Deferred,
        ),
        (
            RelationshipProofSurface::HostCallbackProofs,
            RelationshipProofSupportStatus::Forbidden,
        ),
    ]
}

pub(crate) fn bounded_descendant_synthetic_runtime_surfaces(
) -> [(RelationshipProofSurface, RelationshipProofSupportStatus); 5] {
    [
        (
            RelationshipProofSurface::DescriptorAdmission,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::BoundedDescendantTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::TenantMembershipTopology,
            RelationshipProofSupportStatus::Verified,
        ),
        (
            RelationshipProofSurface::RuntimeProofEvaluation,
            RelationshipProofSupportStatus::Deferred,
        ),
        (
            RelationshipProofSurface::HostCallbackProofs,
            RelationshipProofSupportStatus::Forbidden,
        ),
    ]
}
