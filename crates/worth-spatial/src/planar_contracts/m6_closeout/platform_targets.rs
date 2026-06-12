use crate::planar_contracts::contract_bundle::planar_contract_bundle_digest;
use crate::workload_platform::{
    boolean_readiness_workload::PlanarBooleanReadinessWorkloadReceipt,
    coplanar_overlap_storm::CoplanarOverlapStormReceipt,
    dirty_planar_clean_fail::DirtyPlanarCleanFailReceipt,
    grazing_basket_stack::GrazingBasketStackReceipt,
    high_valence_singularity::HighValenceSingularityReceipt,
    mixed_surface_kill_box::MixedSurfaceKillBoxReceipt, nmt_radial_fan::NmtRadialFanReceipt,
    open_class_triad_parity::OpenClassTriadParityReceipt,
    open_planar_posture::OpenPlanarPostureReceipt,
    projection_fact_parity::ProjectionFactParityReceipt,
    retained_cancellation_chain::RetainedCancellationChainReceipt,
    thin_feature_scale_separation::ThinFeatureScaleSeparationReceipt,
};

use super::basis::M6PremetabossFamily;
use super::source_rows::{
    boolean_readiness_source_rows, coplanar_overlap_storm_source_rows,
    dirty_clean_fail_source_rows, grazing_basket_stack_source_rows, high_valence_source_rows,
    mixed_surface_kill_box_source_rows, nmt_radial_fan_source_rows, open_class_triad_source_rows,
    open_posture_source_rows, projection_parity_source_rows, retained_cancellation_source_rows,
    thin_feature_source_rows,
};

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

    pub fn from_nmt_open_radial_fan(receipt: &NmtRadialFanReceipt) -> Self {
        receipt_target(
            M6PremetabossFamily::NmtOpenRadialFan,
            "NMT open radial fan workload receipt",
            receipt.fan_digest(),
            [
                receipt.workload_identity(),
                receipt.topology_construction_identity(),
                receipt.projected_workload_identity(),
                receipt.open_boundary_digest(),
                receipt.radial_adjacency_digest(),
                receipt.transform_posture_identity(),
                receipt.retained_replay_identity(),
                receipt.fan_digest(),
            ],
            nmt_radial_fan_source_rows(receipt),
        )
    }

    pub fn from_mixed_surface_kill_box(receipt: &MixedSurfaceKillBoxReceipt) -> Self {
        let mut identities = vec![
            receipt.declaration(),
            receipt.stable_geometry_binding_identity(),
            receipt.kill_box_digest(),
        ];
        identities.extend(
            receipt
                .runs()
                .iter()
                .map(|run| run.support_evidence_digest()),
        );
        receipt_target(
            M6PremetabossFamily::NmtMixedSurfaceKillBox,
            "NMT mixed-surface kill-box workload receipt",
            receipt.kill_box_digest(),
            identities,
            mixed_surface_kill_box_source_rows(receipt),
        )
    }

    pub fn from_open_class_triad_parity(receipt: &OpenClassTriadParityReceipt) -> Self {
        let mut identities = vec![receipt.declaration(), receipt.triad_digest()];
        identities.extend(
            receipt
                .lane_sets()
                .iter()
                .map(|lane_set| lane_set.parity().parity_digest()),
        );
        receipt_target(
            M6PremetabossFamily::NmtOpenClassTriadParity,
            "NMT open-class triad parity workload receipt",
            receipt.triad_digest(),
            identities,
            open_class_triad_source_rows(receipt),
        )
    }

    pub fn from_grazing_basket_stack(receipt: &GrazingBasketStackReceipt) -> Self {
        let mut identities = vec![
            receipt.stack_identity(),
            receipt.topology_construction_identity(),
            receipt.projected_workload_identity(),
            receipt.retained_replay_identity(),
            receipt.transform_posture_identity(),
        ];
        identities.extend(receipt.layers().iter().map(|layer| layer.layer_identity()));
        receipt_target(
            M6PremetabossFamily::NmtGrazingBasketStack,
            "NMT grazing basket stack workload receipt",
            receipt.stack_identity(),
            identities,
            grazing_basket_stack_source_rows(receipt),
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
