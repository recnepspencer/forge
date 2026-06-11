use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::lane::{ProjectionFactParityLane, ProjectionFactParityLaneStatus};
use crate::planar_contracts::local_rebuild_parity::PlanarLocalRebuildParityReceipt;
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::planar_contracts::planar_recovery::PlanarRecoveryPostureReceipt;
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::{
    RetainedPlanarFactsReceipt, RetainedPlanarHistoricalInspection,
};
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionFactParityLaneEvidence {
    lane: ProjectionFactParityLane,
    source_receipt_identity: String,
    parity_basis_identity: String,
    basis_links: Vec<ProjectionFactParityBasisLink>,
    status: ProjectionFactParityLaneStatus,
}

impl ProjectionFactParityLaneEvidence {
    fn new(
        lane: ProjectionFactParityLane,
        source_receipt_identity: impl Into<String>,
        parity_basis_identity: impl Into<String>,
        status: ProjectionFactParityLaneStatus,
    ) -> Self {
        Self::new_with_links(
            lane,
            source_receipt_identity,
            parity_basis_identity,
            Vec::new(),
            status,
        )
    }

    fn new_with_links(
        lane: ProjectionFactParityLane,
        source_receipt_identity: impl Into<String>,
        parity_basis_identity: impl Into<String>,
        basis_links: Vec<ProjectionFactParityBasisLink>,
        status: ProjectionFactParityLaneStatus,
    ) -> Self {
        Self {
            lane,
            source_receipt_identity: source_receipt_identity.into(),
            parity_basis_identity: parity_basis_identity.into(),
            basis_links,
            status,
        }
    }

    pub fn lane(&self) -> ProjectionFactParityLane {
        self.lane
    }

    pub fn source_receipt_identity(&self) -> &str {
        &self.source_receipt_identity
    }

    pub fn parity_basis_identity(&self) -> &str {
        &self.parity_basis_identity
    }

    pub fn status(&self) -> ProjectionFactParityLaneStatus {
        self.status
    }

    pub(crate) fn basis_links(&self) -> &[ProjectionFactParityBasisLink] {
        &self.basis_links
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectionFactParityBasisLink {
    kind: ProjectionFactParityBasisLinkKind,
    identity: String,
}

impl ProjectionFactParityBasisLink {
    fn new(kind: ProjectionFactParityBasisLinkKind, identity: impl Into<String>) -> Self {
        Self {
            kind,
            identity: identity.into(),
        }
    }

    pub(crate) fn kind(&self) -> ProjectionFactParityBasisLinkKind {
        self.kind
    }

    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ProjectionFactParityBasisLinkKind {
    RetainedFact,
    ProjectionConsumedFact,
    RecoveryPosture,
    DiagnosticBundle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionFactParityEvidenceBasis {
    evidence_ledger: CompleteWorkloadEvidenceLedger,
    workload_basis_identity: String,
    lanes: Vec<ProjectionFactParityLaneEvidence>,
}

impl ProjectionFactParityEvidenceBasis {
    pub fn from_evidence_ledger(evidence_ledger: CompleteWorkloadEvidenceLedger) -> Self {
        let workload_basis_identity = workload_basis_identity(&evidence_ledger);
        Self {
            evidence_ledger,
            workload_basis_identity,
            lanes: Vec::new(),
        }
    }

    pub fn with_live_lane_from_ledger(mut self) -> Self {
        let identity = self.ledger_stage_set_identity(&[
            WorkloadEvidenceStage::Topology,
            WorkloadEvidenceStage::GeometryBinding,
            WorkloadEvidenceStage::SurfaceSupport,
        ]);
        self.push_lane(ProjectionFactParityLane::Live, identity);
        self
    }

    pub fn with_projected_lane_from_ledger(mut self) -> Self {
        let identity = self.ledger_stage_identity(WorkloadEvidenceStage::Projection);
        self.push_lane(ProjectionFactParityLane::Projected, identity);
        self
    }

    pub fn with_transformed_lane_from_ledger(mut self) -> Self {
        let identity = self.ledger_stage_identity(WorkloadEvidenceStage::Transform);
        self.push_lane(ProjectionFactParityLane::Transformed, identity);
        self
    }

    pub fn with_projection_consumed_facts(
        self,
        receipt: &ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.with_projection_consumed_facts_status(
            receipt,
            ProjectionFactParityLaneStatus::Admitted,
        )
    }

    pub fn with_projection_consumed_facts_status(
        mut self,
        receipt: &ProjectionConsumedPlanarFactsReceipt,
        status: ProjectionFactParityLaneStatus,
    ) -> Self {
        self.push_receipt_lane_with_links(
            ProjectionFactParityLane::ProjectionConsumed,
            receipt.projection_consumption_digest(),
            vec![
                ProjectionFactParityBasisLink::new(
                    ProjectionFactParityBasisLinkKind::RetainedFact,
                    receipt.retained_planar_fact_digest(),
                ),
                ProjectionFactParityBasisLink::new(
                    ProjectionFactParityBasisLinkKind::ProjectionConsumedFact,
                    receipt.projection_consumption_digest(),
                ),
            ],
            status,
        );
        self
    }

    pub fn with_retained_workload(self, receipt: &RetainedPlanarFactsReceipt) -> Self {
        self.with_retained_workload_status(receipt, ProjectionFactParityLaneStatus::Admitted)
    }

    pub fn with_retained_workload_status(
        mut self,
        receipt: &RetainedPlanarFactsReceipt,
        status: ProjectionFactParityLaneStatus,
    ) -> Self {
        self.push_receipt_lane_with_links(
            ProjectionFactParityLane::Retained,
            receipt.retained_fact_digest(),
            vec![ProjectionFactParityBasisLink::new(
                ProjectionFactParityBasisLinkKind::RetainedFact,
                receipt.retained_fact_digest(),
            )],
            status,
        );
        self
    }

    pub fn with_replay(mut self, replay: &RetainedPlanarHistoricalInspection) -> Self {
        self.push_receipt_lane_with_links(
            ProjectionFactParityLane::Replayed,
            replay.historical_digest(),
            vec![ProjectionFactParityBasisLink::new(
                ProjectionFactParityBasisLinkKind::RetainedFact,
                replay.retained_fact_digest(),
            )],
            ProjectionFactParityLaneStatus::Admitted,
        );
        self
    }

    pub fn with_recovery(self, receipt: &PlanarRecoveryPostureReceipt) -> Self {
        self.with_recovery_status(receipt, ProjectionFactParityLaneStatus::Admitted)
    }

    pub fn with_recovery_status(
        mut self,
        receipt: &PlanarRecoveryPostureReceipt,
        status: ProjectionFactParityLaneStatus,
    ) -> Self {
        let retained_fact = receipt
            .basis()
            .retained_planar_facts()
            .map(|retained| retained.retained_fact_digest().to_string());
        let projection_consumed = receipt
            .basis()
            .projection_consumed_facts()
            .map(|projected| projected.projection_consumption_digest().to_string());
        self.push_receipt_lane_with_links(
            ProjectionFactParityLane::Recovered,
            receipt.recovery_posture_digest(),
            optional_links([
                retained_fact.map(|identity| {
                    ProjectionFactParityBasisLink::new(
                        ProjectionFactParityBasisLinkKind::RetainedFact,
                        identity,
                    )
                }),
                projection_consumed.map(|identity| {
                    ProjectionFactParityBasisLink::new(
                        ProjectionFactParityBasisLinkKind::ProjectionConsumedFact,
                        identity,
                    )
                }),
                Some(ProjectionFactParityBasisLink::new(
                    ProjectionFactParityBasisLinkKind::RecoveryPosture,
                    receipt.recovery_posture_digest(),
                )),
            ]),
            status,
        );
        self
    }

    pub fn with_local_rebuild(mut self, receipt: &PlanarLocalRebuildParityReceipt) -> Self {
        self.push_receipt_lane_with_links(
            ProjectionFactParityLane::LocalRebuild,
            receipt.parity_digest(),
            vec![
                ProjectionFactParityBasisLink::new(
                    ProjectionFactParityBasisLinkKind::RetainedFact,
                    receipt.basis().retained().retained_fact_digest(),
                ),
                ProjectionFactParityBasisLink::new(
                    ProjectionFactParityBasisLinkKind::ProjectionConsumedFact,
                    receipt
                        .basis()
                        .projection_consumed()
                        .projection_consumption_digest(),
                ),
                ProjectionFactParityBasisLink::new(
                    ProjectionFactParityBasisLinkKind::RecoveryPosture,
                    receipt.basis().recovery().recovery_posture_digest(),
                ),
                ProjectionFactParityBasisLink::new(
                    ProjectionFactParityBasisLinkKind::DiagnosticBundle,
                    receipt.basis().diagnostics().diagnostic_bundle_digest(),
                ),
            ],
            ProjectionFactParityLaneStatus::Admitted,
        );
        self
    }

    pub fn with_diagnostics(mut self, receipt: &PlanarDiagnosticBundleReceipt) -> Self {
        self.push_receipt_lane_with_links(
            ProjectionFactParityLane::Diagnostics,
            receipt.diagnostic_bundle_digest(),
            vec![ProjectionFactParityBasisLink::new(
                ProjectionFactParityBasisLinkKind::DiagnosticBundle,
                receipt.diagnostic_bundle_digest(),
            )],
            ProjectionFactParityLaneStatus::Admitted,
        );
        self
    }

    #[doc(hidden)]
    pub fn with_adversarial_foreign_ledger_basis_for_lane(
        mut self,
        lane: ProjectionFactParityLane,
        foreign_ledger: &CompleteWorkloadEvidenceLedger,
    ) -> Self {
        let foreign_basis = workload_basis_identity(foreign_ledger);
        if let Some(existing) = self
            .lanes
            .iter_mut()
            .find(|candidate| candidate.lane == lane)
        {
            existing.parity_basis_identity = foreign_basis;
        }
        self
    }

    pub fn with_lane_status(
        mut self,
        lane: ProjectionFactParityLane,
        status: ProjectionFactParityLaneStatus,
    ) -> Self {
        if let Some(existing) = self
            .lanes
            .iter_mut()
            .find(|candidate| candidate.lane == lane)
        {
            existing.status = status;
        }
        self
    }

    pub fn with_all_lane_statuses(mut self, status: ProjectionFactParityLaneStatus) -> Self {
        for lane in &mut self.lanes {
            lane.status = status;
        }
        self
    }

    pub(crate) fn evidence_ledger(&self) -> &CompleteWorkloadEvidenceLedger {
        &self.evidence_ledger
    }

    pub(crate) fn lanes(&self) -> &[ProjectionFactParityLaneEvidence] {
        &self.lanes
    }

    pub(crate) fn workload_basis_identity(&self) -> &str {
        &self.workload_basis_identity
    }

    fn push_lane(&mut self, lane: ProjectionFactParityLane, source_receipt_identity: String) {
        self.push_receipt_lane(
            lane,
            source_receipt_identity,
            ProjectionFactParityLaneStatus::Admitted,
        );
    }

    fn push_receipt_lane(
        &mut self,
        lane: ProjectionFactParityLane,
        source_receipt_identity: impl Into<String>,
        status: ProjectionFactParityLaneStatus,
    ) {
        self.lanes.push(ProjectionFactParityLaneEvidence::new(
            lane,
            source_receipt_identity,
            self.workload_basis_identity.clone(),
            status,
        ));
    }

    fn push_receipt_lane_with_links(
        &mut self,
        lane: ProjectionFactParityLane,
        source_receipt_identity: impl Into<String>,
        basis_links: Vec<ProjectionFactParityBasisLink>,
        status: ProjectionFactParityLaneStatus,
    ) {
        self.lanes
            .push(ProjectionFactParityLaneEvidence::new_with_links(
                lane,
                source_receipt_identity,
                self.workload_basis_identity.clone(),
                basis_links,
                status,
            ));
    }

    fn ledger_stage_identity(&self, stage: WorkloadEvidenceStage) -> String {
        self.evidence_ledger
            .evidence_for_stage(stage)
            .unwrap_or_else(|| stage.human_name())
            .to_string()
    }

    fn ledger_stage_set_identity(&self, stages: &[WorkloadEvidenceStage]) -> String {
        let parts = stages
            .iter()
            .map(|stage| format!("{stage:?}:{}", self.ledger_stage_identity(*stage)))
            .collect::<Vec<_>>();
        truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
    }
}

fn optional_links<const N: usize>(
    links: [Option<ProjectionFactParityBasisLink>; N],
) -> Vec<ProjectionFactParityBasisLink> {
    links.into_iter().flatten().collect()
}

fn workload_basis_identity(ledger: &CompleteWorkloadEvidenceLedger) -> String {
    let parts = ProjectionFactParityLane::REQUIRED
        .iter()
        .filter_map(|lane| stage_for_lane(*lane))
        .filter_map(|stage| {
            ledger
                .evidence_for_stage(stage)
                .map(|identity| format!("{stage:?}:{identity}"))
        })
        .collect::<Vec<_>>();
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn stage_for_lane(lane: ProjectionFactParityLane) -> Option<WorkloadEvidenceStage> {
    match lane {
        ProjectionFactParityLane::Live => Some(WorkloadEvidenceStage::Topology),
        ProjectionFactParityLane::Projected | ProjectionFactParityLane::ProjectionConsumed => {
            Some(WorkloadEvidenceStage::Projection)
        }
        ProjectionFactParityLane::Retained | ProjectionFactParityLane::Replayed => {
            Some(WorkloadEvidenceStage::RetainedReplay)
        }
        ProjectionFactParityLane::Transformed => Some(WorkloadEvidenceStage::Transform),
        ProjectionFactParityLane::Recovered | ProjectionFactParityLane::Diagnostics => {
            Some(WorkloadEvidenceStage::Diagnostics)
        }
        ProjectionFactParityLane::LocalRebuild => Some(WorkloadEvidenceStage::Response),
    }
}
