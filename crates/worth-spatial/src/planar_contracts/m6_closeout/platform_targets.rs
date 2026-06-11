use crate::planar_contracts::contract_bundle::planar_contract_bundle_digest;
use crate::workload_platform::{
    boolean_readiness_workload::PlanarBooleanReadinessWorkloadReceipt,
    coplanar_overlap_storm::CoplanarOverlapStormReceipt,
    dirty_planar_clean_fail::DirtyPlanarCleanFailReceipt,
    high_valence_singularity::HighValenceSingularityReceipt,
    open_planar_posture::OpenPlanarPostureReceipt,
    projection_fact_parity::ProjectionFactParityReceipt,
    retained_cancellation_chain::RetainedCancellationChainReceipt,
    thin_feature_scale_separation::ThinFeatureScaleSeparationReceipt,
};

use super::basis::M6PremetabossFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum M6PremetabossEvidencePosture {
    WorkloadPlatform,
    SyntheticEndToEndClaim,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M6PremetabossEvidenceRow {
    family: M6PremetabossFamily,
    evidence_digest: String,
    source_rows: usize,
    posture: M6PremetabossEvidencePosture,
    human_reason: String,
}

impl M6PremetabossEvidenceRow {
    pub fn from_workload_platform_target(
        target: M6PremetabossPlatformTarget,
    ) -> M6PremetabossEvidenceRow {
        Self {
            family: target.family,
            evidence_digest: target.target_digest,
            source_rows: target.source_rows,
            posture: M6PremetabossEvidencePosture::WorkloadPlatform,
            human_reason: target.human_reason,
        }
    }

    pub fn synthetic_end_to_end_claim(
        family: M6PremetabossFamily,
        evidence_digest: impl Into<String>,
    ) -> Self {
        Self {
            family,
            evidence_digest: evidence_digest.into(),
            source_rows: 0,
            posture: M6PremetabossEvidencePosture::SyntheticEndToEndClaim,
            human_reason: "synthetic end-to-end claims cannot register as MB closeout evidence"
                .to_string(),
        }
    }

    pub fn family(&self) -> M6PremetabossFamily {
        self.family
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn source_rows(&self) -> usize {
        self.source_rows
    }

    pub fn posture(&self) -> M6PremetabossEvidencePosture {
        self.posture
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M6PremetabossPlatformTarget {
    family: M6PremetabossFamily,
    target_digest: String,
    source_rows: usize,
    human_reason: String,
}

impl M6PremetabossPlatformTarget {
    pub fn from_coplanar_overlap_storm(receipt: &CoplanarOverlapStormReceipt) -> Self {
        receipt_target(
            M6PremetabossFamily::CoplanarOverlapStorm,
            "coplanar overlap storm workload receipt",
            receipt.storm_digest(),
            [
                receipt.workload_identity(),
                receipt.operator_identity(),
                receipt.storm_digest(),
            ],
            coplanar_overlap_storm_source_rows(receipt),
        )
    }

    pub fn from_high_valence_singularity(receipt: &HighValenceSingularityReceipt) -> Self {
        receipt_target(
            M6PremetabossFamily::HighValencePlanarSingularity,
            "high-valence singularity workload receipt",
            receipt.singularity_digest(),
            [
                receipt.workload_identity(),
                receipt.center_vertex_identity(),
                receipt.local_rebuild_evidence_digest(),
                receipt.singularity_digest(),
            ],
            high_valence_source_rows(receipt),
        )
    }

    pub fn from_thin_feature_scale_separation(receipt: &ThinFeatureScaleSeparationReceipt) -> Self {
        receipt_target(
            M6PremetabossFamily::ThinFeatureScaleSeparation,
            "thin-feature scale-separation workload receipt",
            receipt.thin_feature_digest(),
            [
                receipt.workload_identity(),
                receipt.precision_identity(),
                receipt.local_frame_identity(),
                receipt.projection_consumption_identity(),
                receipt.projection_consumed_local_frame_identity(),
                receipt.thin_feature_digest(),
            ],
            thin_feature_source_rows(receipt),
        )
    }

    pub fn from_retained_cancellation_chain(receipt: &RetainedCancellationChainReceipt) -> Self {
        let mut identities = vec![
            receipt.workload_identity(),
            receipt.retained_basis_identity(),
            receipt.projection_consumed_identity(),
            receipt.chain_digest(),
        ];
        identities.extend(
            receipt
                .checkpoints()
                .iter()
                .map(|checkpoint| checkpoint.checkpoint_identity()),
        );
        receipt_target(
            M6PremetabossFamily::RetainedHistoryCancellationChain,
            "retained cancellation-chain workload receipt",
            receipt.chain_digest(),
            identities,
            retained_cancellation_source_rows(receipt),
        )
    }

    pub fn from_dirty_planar_clean_fail(receipt: &DirtyPlanarCleanFailReceipt) -> Self {
        receipt_target(
            M6PremetabossFamily::DirtyPlanarInputCleanFail,
            "dirty planar clean-fail workload receipt",
            receipt.clean_fail_digest(),
            [
                receipt.workload_identity(),
                receipt.topology_clean_fail_identity(),
                receipt.clean_fail_boundary_identity(),
                receipt.clean_fail_digest(),
            ],
            dirty_clean_fail_source_rows(receipt),
        )
    }

    pub fn from_open_planar_posture(receipt: &OpenPlanarPostureReceipt) -> Self {
        receipt_target(
            M6PremetabossFamily::UnboundedHalfSpacePosture,
            "open planar posture workload receipt",
            receipt.posture_digest(),
            [
                receipt.workload_identity(),
                receipt.topology_receipt_identity(),
                receipt.unsupported_surface_identity(),
                receipt.clean_fail_boundary_identity(),
                receipt.diagnostic_receipt_identity(),
                receipt.posture_digest(),
            ],
            open_posture_source_rows(receipt),
        )
    }

    pub fn from_projection_fact_parity(receipt: &ProjectionFactParityReceipt) -> Self {
        let mut identities = vec![
            receipt.workload_basis_identity(),
            receipt.declaration(),
            receipt.parity_digest(),
        ];
        identities.extend(
            receipt
                .lane_evidence()
                .iter()
                .map(|lane| lane.source_receipt_identity()),
        );
        receipt_target(
            M6PremetabossFamily::ProjectionConsumedPlanarFactParity,
            "projection fact-parity workload receipt",
            receipt.parity_digest(),
            identities,
            projection_parity_source_rows(receipt),
        )
    }

    pub fn from_boolean_readiness_final_boss(
        receipt: &PlanarBooleanReadinessWorkloadReceipt,
    ) -> Self {
        receipt_target(
            M6PremetabossFamily::BooleanReadinessFinalBoss,
            "boolean-readiness final-boss workload receipt",
            receipt.workload_digest(),
            [
                receipt.declaration(),
                receipt.workload_digest(),
                receipt.m7_readiness_receipt().readiness_digest(),
                receipt.m7_readiness_receipt().declaration_digest(),
                receipt.m7_readiness_receipt().envelope_digest(),
            ],
            boolean_readiness_source_rows(receipt),
        )
    }

    pub fn family(&self) -> M6PremetabossFamily {
        self.family
    }

    pub fn target_digest(&self) -> &str {
        &self.target_digest
    }

    pub fn source_rows(&self) -> usize {
        self.source_rows
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

fn receipt_target<'a>(
    family: M6PremetabossFamily,
    receipt_name: &'static str,
    receipt_digest: &str,
    identities: impl IntoIterator<Item = &'a str>,
    source_rows: usize,
) -> M6PremetabossPlatformTarget {
    let mut parts = vec![
        format!("mb-family:{}", family.as_str()),
        format!("receipt:{receipt_digest}"),
    ];
    parts.extend(
        identities
            .into_iter()
            .map(|identity| format!("receipt-link:{identity}")),
    );
    M6PremetabossPlatformTarget {
        family,
        target_digest: planar_contract_bundle_digest(&parts),
        source_rows,
        human_reason: format!(
            "{} has workload-platform evidence backed by the real {receipt_name}",
            family.as_str()
        ),
    }
}

fn coplanar_overlap_storm_source_rows(receipt: &CoplanarOverlapStormReceipt) -> usize {
    let counters = receipt.counters();
    counters.topology_entity_count()
        + counters.topology_face_count()
        + counters.topology_relation_count()
        + counters.projected_entity_count()
        + counters.transform_step_count()
        + counters.transform_cancellation_step_count()
        + counters.retained_artifact_count()
        + counters.replay_checkpoint_count()
        + counters.operator_input_count()
        + counters.operator_receipt_count()
        + counters.overlap_extraction_receipt_count()
        + counters.overlap_candidate_pair_breadth()
        + counters.overlap_segment_contacts_certified()
        + counters.overlap_shared_intervals()
        + counters.overlap_islands()
        + counters.overlap_ambiguous_contacts()
}

fn high_valence_source_rows(receipt: &HighValenceSingularityReceipt) -> usize {
    let counters = receipt.counters();
    counters.topology_entity_count()
        + counters.topology_face_count()
        + counters.topology_relation_count()
        + counters.binding_target_count()
        + counters.surface_support_count()
        + counters.neighborhood_valence()
        + counters.projected_entity_count()
        + counters.local_basis_part_count()
        + counters.transform_step_count()
        + counters.local_rebuild_evidence_row_count()
        + counters.retained_artifact_count()
        + counters.replay_checkpoint_count()
        + counters.diagnostic_count()
        + counters.user_outcome_count()
}

fn thin_feature_source_rows(receipt: &ThinFeatureScaleSeparationReceipt) -> usize {
    let counters = receipt.counters();
    counters.thin_feature_count()
        + counters.local_scale_order_count()
        + counters.world_magnitude_order_count()
        + counters.precision_escalation_count()
        + counters.local_basis_part_count()
        + counters.projected_entity_count()
        + counters.transform_step_count()
        + counters.tiny_rotation_pressure_count()
        + counters.projection_consumed_basis_count()
        + counters.diagnostic_count()
        + counters.user_outcome_count()
}

fn retained_cancellation_source_rows(receipt: &RetainedCancellationChainReceipt) -> usize {
    let counters = receipt.counters();
    counters.checkpoint_count()
        + counters.transform_step_count()
        + counters.replayed_checkpoint_count()
        + counters.trigger_local_replay_count()
        + counters.retained_artifact_count()
        + counters.projection_consumed_fact_count()
        + counters.diagnostic_trigger_count()
        + counters.user_outcome_count()
}

fn dirty_clean_fail_source_rows(receipt: &DirtyPlanarCleanFailReceipt) -> usize {
    let counters = receipt.counters();
    counters.topology_clean_fail_receipts()
        + counters.clean_fail_boundary_receipts()
        + counters.recovery_receipts()
        + counters.transform_posture_receipts()
        + counters.diagnostic_receipts()
        + counters.user_outcome_receipts()
}

fn open_posture_source_rows(receipt: &OpenPlanarPostureReceipt) -> usize {
    let counters = receipt.counters();
    counters.topology_receipts()
        + counters.unsupported_surface_receipts()
        + counters.clean_fail_boundary_receipts()
        + counters.transform_posture_receipts()
        + counters.diagnostic_receipts()
        + counters.user_outcome_receipts()
        + counters.bounded_surrogate_rejections()
}

fn projection_parity_source_rows(receipt: &ProjectionFactParityReceipt) -> usize {
    let counters = receipt.counters();
    counters.lanes_compared()
        + counters.receipt_backed_lanes()
        + counters.denied_lanes()
        + counters.policy_required_lanes()
}

fn boolean_readiness_source_rows(receipt: &PlanarBooleanReadinessWorkloadReceipt) -> usize {
    let counters = receipt.counters();
    counters.required_evidence_stages_consumed()
        + counters.ledger_rows_consumed()
        + counters.parity_lanes_consumed()
        + counters.closeout_rows_consumed()
        + counters.query_boundary_rows()
        + counters.blocked_branch_count()
}
