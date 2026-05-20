use super::*;
use crate::identity::hash_parts;

#[test]
fn intent_admission_certification_topology_and_representative_family_artifacts_match_phase_six_scope(
) {
    let bundle = certify_intent_admission();
    let basis_bundle = crate::basis_lifecycle::certify_basis_lifecycle();
    let projection_bundle =
        crate::projection_consumption::certify_projection_consumption_closeout_core();
    let rows = bundle.representative_family_report().rows();
    let basis_row = rows
        .iter()
        .find(|row| row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::BasisParity)
        .expect("basis representative lane should exist");
    let projection_row = rows
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::ProjectionAdvisory
        })
        .expect("projection representative lane should exist");
    let inspection_row = rows
        .iter()
        .find(|row| {
            row.lane()
                == ForgeQueryIntentAdmissionRepresentativeFamilyLane::InspectionAdvisoryRedaction
        })
        .expect("inspection representative lane should exist");
    let routing_row = rows
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::RoutingAdmitted
        })
        .expect("routing admitted representative lane should exist");
    let routing_future_neighbor_row = rows
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::RoutingFutureNeighbor
        })
        .expect("routing future-neighbor representative lane should exist");

    assert_eq!(bundle.topology_audit().rows().len(), 8);
    assert_eq!(
        rows.iter().map(|row| row.lane()).collect::<Vec<_>>(),
        vec![
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::BasisParity,
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::EffectAdmitted,
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::ProjectionAdvisory,
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::InspectionAdvisoryRedaction,
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::RoutingAdmitted,
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::RoutingFutureNeighbor,
        ]
    );
    assert_eq!(
        bundle
            .topology_audit()
            .rows()
            .iter()
            .map(|row| row.domain().as_str())
            .collect::<Vec<_>>(),
        vec![
            "intent_admission/families",
            "intent_admission/eligibility",
            "intent_admission/decisions",
            "intent_admission/handoffs",
            "intent_admission/trace",
            "intent_admission/dx",
            "intent_admission/support",
            "intent_admission/certification",
        ]
    );
    assert_eq!(rows.len(), 6);
    let effect_row = rows
        .iter()
        .find(|row| row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::EffectAdmitted)
        .expect("effect representative lane should exist");
    assert_eq!(
        basis_row.authority_surface(),
        "intent_admission::forge_query_basis_observation_intent"
    );
    assert_eq!(
        basis_row.neighbor_certification_surface(),
        "basis_lifecycle::certify_basis_lifecycle"
    );
    assert_eq!(
        effect_row.authority_surface(),
        "runtime.next_effect_write_intent(&effect, version, contract).review()?.admit()?.execute()"
    );
    assert_eq!(
        effect_row.neighbor_certification_surface(),
        "effect-execution-covered-surface"
    );
    assert_eq!(
        projection_row.authority_surface(),
        "intent_admission::forge_query_projection_consumption_intent"
    );
    assert!(!effect_row.neighbor_bundle_digest().is_empty());
    assert!(!effect_row.evidence_digest().is_empty());
    assert!(effect_row
        .posture_detail()
        .contains("lowers into execution without rediscovering raw intent"));
    assert_eq!(
        projection_row.neighbor_certification_surface(),
        "projection_consumption::certify_projection_consumption_closeout_core"
    );
    assert_eq!(
        inspection_row.authority_surface(),
        "runtime.inspect_intent(target).review()?.admit()?.execute()"
    );
    assert_eq!(
        inspection_row.neighbor_certification_surface(),
        "inspection-materialization-covered-surface"
    );
    assert_eq!(
        routing_row.authority_surface(),
        "runtime.probe_existing_intent(request).review()?.admit()?.execute()"
    );
    assert_eq!(
        routing_row.neighbor_certification_surface(),
        "routing-covered-surface"
    );
    assert_eq!(
        routing_future_neighbor_row.authority_surface(),
        "intent_admission::{admit_runtime_intent_request, ForgeQueryRawIntentAdmissionRequest::deferred_neighbor}"
    );
    assert_eq!(
        routing_future_neighbor_row.neighbor_certification_surface(),
        "routing-future-neighbor-deferred-owner"
    );
    assert_eq!(
        basis_row.neighbor_bundle_digest(),
        hash_parts(&[
            basis_bundle
                .output_digest("basis_eligibility_digest")
                .expect("basis certification bundle should expose basis_eligibility_digest")
                .to_string(),
            basis_bundle
                .output_digest("scoped_basis_digest")
                .expect("basis certification bundle should expose scoped_basis_digest")
                .to_string(),
            basis_bundle
                .output_digest("basis_support_matrix_digest")
                .expect("basis certification bundle should expose basis_support_matrix_digest")
                .to_string(),
            basis_bundle
                .output_digest("basis_target_dx_digest")
                .expect("basis certification bundle should expose basis_target_dx_digest")
                .to_string(),
        ])
    );
    assert_eq!(
        projection_row.neighbor_bundle_digest(),
        hash_parts(&[
            projection_bundle
                .output_digest("projection_consumption_eligibility_digest")
                .expect(
                    "projection certification bundle should expose projection_consumption_eligibility_digest"
                )
                .to_string(),
            projection_bundle
                .output_digest("materialized_projection_contract_digest")
                .expect(
                    "projection certification bundle should expose materialized_projection_contract_digest"
                )
                .to_string(),
            projection_bundle
                .output_digest("projection_support_matrix_digest")
                .expect("projection certification bundle should expose projection_support_matrix_digest")
                .to_string(),
            projection_bundle
                .output_digest("projection_target_dx_digest")
                .expect("projection certification bundle should expose projection_target_dx_digest")
                .to_string(),
        ])
    );
    assert!(!basis_row.evidence_digest().is_empty());
    assert!(!projection_row.evidence_digest().is_empty());
    assert!(!inspection_row.evidence_digest().is_empty());
    assert!(!routing_row.evidence_digest().is_empty());
    assert!(!routing_future_neighbor_row.evidence_digest().is_empty());
    assert_ne!(
        inspection_row.evidence_digest(),
        routing_future_neighbor_row.evidence_digest(),
        "inspection advisory-redaction must not collapse into routing future-neighbor evidence"
    );
    assert_ne!(
        routing_row.evidence_digest(),
        routing_future_neighbor_row.evidence_digest(),
        "admitted routing and future-neighbor routing must stay mechanically distinct"
    );
    assert!(inspection_row
        .posture_detail()
        .contains("preserving one causal identity"));
    assert!(routing_row
        .posture_detail()
        .contains("retained routing provenance"));
    assert!(routing_future_neighbor_row
        .posture_detail()
        .contains("typed deferred lane"));
    assert!(routing_future_neighbor_row
        .posture_detail()
        .contains("typed unsupported lane"));
    assert_eq!(
        bundle.output_digest("intent_family_digest"),
        Some(
            hash_parts(
                &forge_query_intent_admission_family_inventory()
                    .rows()
                    .iter()
                    .map(|row| row.family().as_str().to_string())
                    .collect::<Vec<_>>()
            )
            .as_str()
        )
    );
}
