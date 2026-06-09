use worth_spatial::facade::bindings::{
    primitive_rebinding_projection_facts, primitive_rebinding_retained_fact_source,
    PrimitiveRebindingFactProvenance, PrimitiveRebindingFactReadSurface,
    PrimitiveRebindingProjectionFactReceipt,
};

use super::super::support::{admitted_rebinding_handle, face_surface_rebinding_fixture};

#[test]
fn rebinding_projection_facts_preserve_family_owned_receipt_truth() {
    let fixture = face_surface_rebinding_fixture();
    let handle = admitted_rebinding_handle("rebinding-projection-facts");
    let facts: PrimitiveRebindingProjectionFactReceipt =
        primitive_rebinding_projection_facts(&fixture.declaration, &handle).expect("facts");
    let receipt = primitive_rebinding_retained_fact_source(&fixture.declaration, &handle)
        .expect("retained fact source")
        .receipt()
        .clone();

    assert_eq!(
        facts.prior_binding_identity(),
        receipt.prior_binding_identity()
    );
    assert_eq!(facts.prior_site_identity(), receipt.prior_site_identity());
    assert_eq!(facts.neighborhood_family(), receipt.neighborhood_family());
    assert_eq!(facts.outcome_class(), receipt.outcome_class());
    assert_eq!(facts.continuity_class(), receipt.continuity_class());
    assert_eq!(facts.motion_posture(), receipt.motion_posture());
    assert_eq!(
        facts.selected_candidate_label(),
        receipt.selected_candidate_label()
    );
    assert_eq!(
        facts.selected_candidate_identity(),
        Some(fixture.successor_identity.as_str())
    );
    assert_eq!(facts.candidate_identities(), receipt.candidate_identities());
    assert_eq!(facts.candidate_labels(), receipt.candidate_labels());
    assert_eq!(
        facts.candidate_site_identities(),
        receipt.candidate_site_identities()
    );
    assert_eq!(
        facts.read_surface(),
        PrimitiveRebindingFactReadSurface::ProjectionConsumptionFromDeclarationEnvelope
    );
    assert_eq!(
        facts.fact_provenance(),
        PrimitiveRebindingFactProvenance::DeclarationEnvelopeBackedProjectionConsumption
    );
    assert!(facts.progression_digest().is_some());
    assert!(facts.route_plan_digest().is_some());
    assert!(!facts.receipt_digest().is_empty());
    assert!(!facts.envelope_digest().is_empty());
    assert!(!facts.fact_digest().is_empty());
}
