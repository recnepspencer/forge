use super::*;
use crate::identity::hash_parts;

#[test]
fn runtime_floor_certification_topology_and_representative_family_artifacts_match_phase_six_scope()
{
    let bundle = certify_intent_admission_runtime_floor();
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
            row.lane() == ForgeQueryIntentAdmissionRepresentativeFamilyLane::RoutingFutureNeighbor
        })
        .expect("routing representative lane should exist");

    assert_eq!(bundle.topology_audit().rows().len(), 8);
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
    assert_eq!(rows.len(), 4);
    assert_eq!(
        basis_row.authority_surface(),
        "intent_admission::forge_query_basis_observation_intent"
    );
    assert_eq!(
        projection_row.authority_surface(),
        "intent_admission::forge_query_projection_consumption_intent"
    );
    assert!(!basis_row.evidence_digest().is_empty());
    assert!(!projection_row.evidence_digest().is_empty());
    assert_ne!(
        inspection_row.evidence_digest(),
        routing_row.evidence_digest(),
        "inspection advisory-redaction must not collapse into routing future-neighbor evidence"
    );
    assert!(inspection_row
        .posture_detail()
        .contains("preserving one causal identity"));
    assert!(routing_row.posture_detail().contains("typed deferred lane"));
    assert!(routing_row
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
