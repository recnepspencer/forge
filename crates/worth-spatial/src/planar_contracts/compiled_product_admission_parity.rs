use crate::spatial_compiled_product_family::{
    current_spatial_compiled_product_family_catalog, SpatialCompiledProductConsumer,
};
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
};
use crate::workload_platform::retained_replay_workload::ReplayParityReport;

#[test]
fn retained_replay_consumer_routes_through_spatial_admission_lane() {
    let (retained, projected) =
        crate::spatial_compiled_product_family::retained_and_projected_receipts(
            "phase-7-planar-admission-parity",
        );
    let historical = retained
        .historical_replay(&retained.replay_subject())
        .expect("historical replay");
    let expected = admit_spatial_compiled_product_input(
        &current_spatial_compiled_product_family_catalog(),
        SpatialCompiledProductAdmissionRequest::for_retained_replay(
            &historical,
            &retained,
            &projected,
        ),
    )
    .expect("retained replay admission");
    let report =
        ReplayParityReport::from_retained_projection_match(&retained, &historical, &projected);

    assert_eq!(
        report.admitted_consumer(),
        SpatialCompiledProductConsumer::RetainedReplayParity
    );
    assert_eq!(
        report.admission_witness().consumer(),
        SpatialCompiledProductConsumer::RetainedReplayParity
    );
    assert_eq!(report.admission_witness(), expected.witness());
    assert_eq!(
        report.admission_witness().family_identity(),
        report.selected_family_identity()
    );
    assert!(
        !report
            .admission_witness()
            .admission_token()
            .trim()
            .is_empty(),
        "retained replay consumer proof must carry a lane-owned admission witness"
    );
    assert_eq!(
        report.admission_provenance().source_authority_digest(),
        historical.historical_digest()
    );
    assert_eq!(
        report.admission_provenance().locality_footprint_digest(),
        projected.projection_consumption_digest()
    );
    assert!(
        !report
            .admission_provenance()
            .family_digest()
            .trim()
            .is_empty(),
        "retained replay consumer proof must carry the lowered family digest"
    );
    assert_ne!(
        report.admission_provenance().authority_truth_identity_digest(),
        historical.historical_digest(),
        "retained replay consumer proof must carry lowered authority truth, not just raw historical digest"
    );
    assert!(
        !report
            .admission_provenance()
            .equivalence_policy_identity_digest()
            .trim()
            .is_empty(),
        "retained replay consumer proof must carry a lowered equivalence policy identity"
    );
    assert_eq!(
        report.admission_provenance().prior_proof_identity_digest(),
        None
    );
    assert_eq!(
        report
            .admission_provenance()
            .compiled_product_identity_digest(),
        report.rows()[0].parity_identity()
    );
    assert!(
        !report
            .admission_provenance()
            .evidence_support_digest()
            .trim()
            .is_empty(),
        "retained replay consumer proof must carry admission-derived support provenance"
    );
}
