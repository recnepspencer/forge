use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::PlanarBooleanLoopRole;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanAdmittedReconstructedLoopSet,
    PlanarBooleanBornLoop, PlanarBooleanBornLoopSet, PlanarBooleanFragmentMembershipMap,
    PlanarBooleanLoopClassifiedProductKind, PlanarBooleanLoopContainmentEvidencePostureKind,
    PlanarBooleanLoopIslandKind, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopIslandPartitionCounters, PlanarBooleanLoopIslandPartitionRow,
    PlanarBooleanLoopOverlapChainLineageMap, PlanarBooleanLoopRoleOutcomeBoundary,
    PlanarBooleanLoopRoleOutcomeBoundaryInput, PlanarBooleanLoopRoleOutcomeKind,
    PlanarBooleanLoopSourceCarrierRow, PlanarBooleanLoopSourceCarrierSet,
    PlanarBooleanLoopSourceProvenanceBundle, PlanarBooleanLoopSourceProvenanceCounters,
    PlanarBooleanReconstructedLoopBoundary, PlanarBooleanReconstructedLoopBoundaryCounters,
    PlanarBooleanSourceLoopSplitAttribution, PlanarBooleanSourceLoopSplitAttributionCounters,
    PlanarBooleanSourceLoopSplitAttributionKind, PlanarBooleanSourceLoopSplitAttributionRow,
};

#[test]
fn single_source_born_loop_does_not_coerce_into_preserved_source_role() {
    let request_identity = "request:single-source-born".to_string();
    let reconstructed_boundary = PlanarBooleanReconstructedLoopBoundary::new(
        PlanarBooleanAdmittedReconstructedLoopSet::new(
            "reconstructed-set".to_string(),
            request_identity.clone(),
            Vec::new(),
        ),
        PlanarBooleanBornLoopSet::new(
            "born-set".to_string(),
            request_identity.clone(),
            vec![PlanarBooleanBornLoop::new(
                "born-loop:solo".to_string(),
                "loop-candidate:solo".to_string(),
                vec!["source-loop:solo".to_string()],
                vec!["chain:solo".to_string()],
                "local-frame:solo".to_string(),
                "precision-basis:solo".to_string(),
                vec!["fragment:solo".to_string()],
                vec!["split-vertex:solo".to_string()],
            )],
        ),
        PlanarBooleanReconstructedLoopBoundaryCounters::default(),
    );
    let partition = PlanarBooleanLoopIslandPartition::new(
        "partition:solo".to_string(),
        request_identity.clone(),
        vec![PlanarBooleanLoopIslandPartitionRow::new(
            "island:solo".to_string(),
            "source-loop:solo".to_string(),
            vec!["born-loop:solo".to_string()],
            PlanarBooleanLoopIslandKind::BornFromOverlapNeighborhood,
        )],
        PlanarBooleanLoopIslandPartitionCounters::default(),
    );
    let split_attribution = PlanarBooleanSourceLoopSplitAttribution::new(
        "split-attribution:solo".to_string(),
        request_identity.clone(),
        vec![PlanarBooleanSourceLoopSplitAttributionRow::new(
            "attribution:solo".to_string(),
            "source-loop:solo".to_string(),
            vec!["island:solo".to_string()],
            PlanarBooleanSourceLoopSplitAttributionKind::ContributedToBornLoop,
        )],
        PlanarBooleanSourceLoopSplitAttributionCounters::default(),
    );
    let source_provenance = PlanarBooleanLoopSourceProvenanceBundle::new(
        "provenance:solo".to_string(),
        request_identity.clone(),
        "split-ledger:solo".to_string(),
        PlanarBooleanLoopSourceCarrierSet::new(
            "carrier-set:solo".to_string(),
            request_identity.clone(),
            "split-ledger:solo".to_string(),
            vec![PlanarBooleanLoopSourceCarrierRow::new(
                "source-carrier:solo".to_string(),
                "recovered-carrier:solo".to_string(),
                "carrier:solo".to_string(),
                PlanarBooleanCommonPlaneOperandSide::Left,
                "source-face:solo".to_string(),
                "source-loop:solo".to_string(),
                "source-edge:solo".to_string(),
                "source-endpoint:start".to_string(),
                [0.0f64.to_bits(), 0.0f64.to_bits()],
                "source-endpoint:end".to_string(),
                [1.0f64.to_bits(), 0.0f64.to_bits()],
                PlanarBooleanLoopRole::OuterBoundary,
            )],
        ),
        PlanarBooleanFragmentMembershipMap::new(
            "fragment-membership:solo".to_string(),
            request_identity.clone(),
            "fragment-set:solo".to_string(),
            Vec::new(),
        ),
        PlanarBooleanLoopOverlapChainLineageMap::new(
            "overlap-lineage:solo".to_string(),
            request_identity.clone(),
            "overlap-set:solo".to_string(),
            Vec::new(),
        ),
        PlanarBooleanLoopSourceProvenanceCounters::default(),
    );

    let role_boundary = PlanarBooleanLoopRoleOutcomeBoundary::classify(
        PlanarBooleanLoopRoleOutcomeBoundaryInput::from_reconstructed_loop_products_and_provenance(
            &reconstructed_boundary,
            &partition,
            &split_attribution,
            &source_provenance,
        ),
    );

    let born_role_outcome = role_boundary
        .role_outcomes()
        .rows()
        .first()
        .expect("single-source born loop should emit role outcome");
    assert_eq!(
        born_role_outcome.loop_kind(),
        PlanarBooleanLoopClassifiedProductKind::BornLoop
    );
    assert_eq!(
        born_role_outcome.kind(),
        PlanarBooleanLoopRoleOutcomeKind::SingleSourceBornLoopRoleDerivedFromEvidence
    );
    assert_eq!(
        born_role_outcome.preserved_source_role(),
        Some(PlanarBooleanLoopRole::OuterBoundary)
    );

    let containment_posture = role_boundary
        .containment_evidence_postures()
        .rows()
        .first()
        .expect("single-source born loop should emit containment posture");
    assert_eq!(
        containment_posture.kind(),
        PlanarBooleanLoopContainmentEvidencePostureKind::SingleSourceBornLoopContainmentEvidence
    );
}

#[test]
fn reconstructed_loop_records_split_source_containment_evidence() {
    let request_identity = "request:split-source-containment".to_string();
    let reconstructed_boundary = PlanarBooleanReconstructedLoopBoundary::new(
        PlanarBooleanAdmittedReconstructedLoopSet::new(
            "reconstructed-set:split-source-containment".to_string(),
            request_identity.clone(),
            vec![PlanarBooleanAdmittedReconstructedLoop::new(
                "reconstructed-loop:split-source-containment".to_string(),
                "loop-candidate:split-source-containment".to_string(),
                "source-loop:split-source-containment".to_string(),
                "source-face:split-source-containment".to_string(),
                "local-frame:split-source-containment".to_string(),
                "precision-basis:split-source-containment".to_string(),
                vec!["fragment:a".to_string(), "fragment:b".to_string()],
                vec!["split-vertex:a".to_string(), "split-vertex:b".to_string()],
            )],
        ),
        PlanarBooleanBornLoopSet::new(
            "born-set:split-source-containment".to_string(),
            request_identity.clone(),
            Vec::new(),
        ),
        PlanarBooleanReconstructedLoopBoundaryCounters::default(),
    );
    let partition = PlanarBooleanLoopIslandPartition::new(
        "partition:split-source-containment".to_string(),
        request_identity.clone(),
        vec![
            PlanarBooleanLoopIslandPartitionRow::new(
                "island:split-source-containment:1".to_string(),
                "source-loop:split-source-containment".to_string(),
                vec!["reconstructed-loop:split-source-containment".to_string()],
                PlanarBooleanLoopIslandKind::PreservedSourceLoop,
            ),
            PlanarBooleanLoopIslandPartitionRow::new(
                "island:split-source-containment:2".to_string(),
                "source-loop:split-source-containment".to_string(),
                vec!["reconstructed-loop:split-source-containment".to_string()],
                PlanarBooleanLoopIslandKind::PreservedSourceLoop,
            ),
        ],
        PlanarBooleanLoopIslandPartitionCounters::default(),
    );
    let split_attribution = PlanarBooleanSourceLoopSplitAttribution::new(
        "split-attribution:split-source-containment".to_string(),
        request_identity.clone(),
        vec![PlanarBooleanSourceLoopSplitAttributionRow::new(
            "attribution:split-source-containment".to_string(),
            "source-loop:split-source-containment".to_string(),
            vec![
                "island:split-source-containment:1".to_string(),
                "island:split-source-containment:2".to_string(),
            ],
            PlanarBooleanSourceLoopSplitAttributionKind::SplitIntoMultipleIslands,
        )],
        PlanarBooleanSourceLoopSplitAttributionCounters::default(),
    );
    let source_provenance = PlanarBooleanLoopSourceProvenanceBundle::new(
        "provenance:split-source-containment".to_string(),
        request_identity.clone(),
        "split-ledger:split-source-containment".to_string(),
        PlanarBooleanLoopSourceCarrierSet::new(
            "carrier-set:split-source-containment".to_string(),
            request_identity.clone(),
            "split-ledger:split-source-containment".to_string(),
            vec![PlanarBooleanLoopSourceCarrierRow::new(
                "source-carrier:split-source-containment".to_string(),
                "recovered-carrier:split-source-containment".to_string(),
                "carrier:split-source-containment".to_string(),
                PlanarBooleanCommonPlaneOperandSide::Left,
                "source-face:split-source-containment".to_string(),
                "source-loop:split-source-containment".to_string(),
                "source-edge:split-source-containment".to_string(),
                "source-endpoint:start".to_string(),
                [0.0f64.to_bits(), 0.0f64.to_bits()],
                "source-endpoint:end".to_string(),
                [1.0f64.to_bits(), 0.0f64.to_bits()],
                PlanarBooleanLoopRole::OuterBoundary,
            )],
        ),
        PlanarBooleanFragmentMembershipMap::new(
            "fragment-membership:split-source-containment".to_string(),
            request_identity.clone(),
            "fragment-set:split-source-containment".to_string(),
            Vec::new(),
        ),
        PlanarBooleanLoopOverlapChainLineageMap::new(
            "overlap-lineage:split-source-containment".to_string(),
            request_identity.clone(),
            "overlap-set:split-source-containment".to_string(),
            Vec::new(),
        ),
        PlanarBooleanLoopSourceProvenanceCounters::default(),
    );

    let role_boundary = PlanarBooleanLoopRoleOutcomeBoundary::classify(
        PlanarBooleanLoopRoleOutcomeBoundaryInput::from_reconstructed_loop_products_and_provenance(
            &reconstructed_boundary,
            &partition,
            &split_attribution,
            &source_provenance,
        ),
    );

    assert_eq!(
        role_boundary.role_outcomes().rows()[0].kind(),
        PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole
    );
    assert_eq!(
        role_boundary.containment_evidence_postures().rows()[0].kind(),
        PlanarBooleanLoopContainmentEvidencePostureKind::SplitSourceContainmentEvidence
    );
}
