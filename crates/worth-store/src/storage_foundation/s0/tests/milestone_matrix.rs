use super::support::*;
use crate::storage_foundation::s0::*;

#[test]
fn phase1_milestone_physical_status_matrix_json_round_trips_through_schema_gate() {
    let matrix = MilestonePhysicalStatusMatrix::new(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("milestone-matrix"),
        milestone_sequence_for_13_3(),
        vec!["13.3".to_string()],
        vec![semantic_cleanup_row()],
    )
    .unwrap();

    let bytes = matrix.to_canonical_json_bytes().unwrap();
    let parsed = MilestonePhysicalStatusMatrix::validate_canonical_json_bytes(&bytes).unwrap();

    assert_eq!(
        parsed.matrix().envelope().deterministic_digest(),
        matrix.envelope().deterministic_digest()
    );
    assert_eq!(parsed.matrix().rows().len(), 1);
    assert_eq!(
        parsed
            .matrix()
            .roadmap_sequence_status()
            .declarations()
            .len(),
        1
    );
}

#[test]
fn phase1_milestone_physical_status_matrix_rejects_missing_required_row() {
    let error = MilestonePhysicalStatusMatrix::new(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("milestone-matrix-missing"),
        milestone_sequence_for_13_3(),
        vec!["13.3".to_string(), "13.2".to_string()],
        vec![semantic_cleanup_row()],
    )
    .expect_err("missing declared milestone row must reject");

    assert_eq!(
        error,
        S0MilestoneMatrixBuildRejection::MissingRequiredMilestoneRow
    );
}

#[test]
fn phase1_milestone_physical_status_matrix_round_trips_platform_grade_row() {
    let sequence = milestone_sequence_for_13_3();
    let gate = sequence.gate_readiness_witness("13.3").unwrap();
    let row = MilestonePhysicalStatusRow::new(
        "13.3",
        "foundation-backed physical lane",
        "_docs/worth-store/milestone-13.3-closeout.md",
        "Shipped store capability reclassification test",
        vec!["physical-foundation".to_string()],
        vec![SemanticPhysicalClaimFamily::PhysicalSubstrate],
        S0PhysicalStatus::PlatformGrade,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        None,
        None,
        vec![],
        vec![Roadmap2SequenceId::new("S12").unwrap()],
        vec![],
        Some(&gate),
    )
    .unwrap();
    let matrix = MilestonePhysicalStatusMatrix::new(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("milestone-matrix-platform"),
        sequence,
        vec!["13.3".to_string()],
        vec![row],
    )
    .unwrap();

    let parsed = MilestonePhysicalStatusMatrix::validate_canonical_json_bytes(
        &matrix.to_canonical_json_bytes().unwrap(),
    )
    .unwrap();

    assert_eq!(
        parsed.matrix().rows()[0]
            .physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalSubstrate),
        S0PhysicalStatus::PlatformGrade
    );
}
