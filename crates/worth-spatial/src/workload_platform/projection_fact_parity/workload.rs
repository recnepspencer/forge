use std::collections::{BTreeMap, BTreeSet};

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    case::ProjectionFactParityCase,
    counters::ProjectionFactParityCounters,
    denial::{ProjectionFactParityDenial, ProjectionFactParityDenialKind},
    evidence_basis::{ProjectionFactParityEvidenceBasis, ProjectionFactParityLaneEvidence},
    lane::{ProjectionFactParityLane, ProjectionFactParityLaneStatus},
    receipt::ProjectionFactParityReceipt,
};

pub struct ProjectionFactParityWorkload {
    evidence_basis: ProjectionFactParityEvidenceBasis,
    declaration: String,
}

pub struct ProjectionFactParityComparison {
    evidence_basis: ProjectionFactParityEvidenceBasis,
    declaration: String,
}

impl ProjectionFactParityWorkload {
    pub fn from_evidence_basis(evidence_basis: ProjectionFactParityEvidenceBasis) -> Self {
        Self {
            evidence_basis,
            declaration: String::new(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn compare_lanes(self) -> ProjectionFactParityComparison {
        ProjectionFactParityComparison {
            evidence_basis: self.evidence_basis,
            declaration: self.declaration,
        }
    }
}

impl ProjectionFactParityComparison {
    pub fn certify(self) -> Result<ProjectionFactParityReceipt, ProjectionFactParityDenial> {
        if self.declaration.trim().is_empty() {
            return Err(ProjectionFactParityDenial::new(
                ProjectionFactParityDenialKind::MissingDeclaration,
                None,
                "Projection fact parity requires a human-readable declaration.",
            ));
        }
        self.assert_ledger_is_real()?;
        self.assert_required_lanes_once()?;
        self.assert_no_mismatch()?;
        let case = self.certified_case()?;
        let lanes = self.evidence_basis.lanes().to_vec();
        let counters = counters_for(&lanes);
        let parity_digest = parity_digest(
            case,
            &self.declaration,
            self.evidence_basis.workload_basis_identity(),
            &lanes,
        );
        Ok(ProjectionFactParityReceipt::new(
            case,
            parity_digest,
            self.evidence_basis.workload_basis_identity().to_string(),
            self.evidence_basis.topology_evidence_identity()?,
            self.declaration,
            lanes,
            counters,
        ))
    }

    fn assert_ledger_is_real(&self) -> Result<(), ProjectionFactParityDenial> {
        self.evidence_basis
            .evidence_ledger()
            .guards()
            .assert_uses_real_topology()
            .and_then(|guard| guard.assert_binding_is_receipt_backed())
            .and_then(|guard| guard.assert_projection_is_receipt_backed())
            .and_then(|guard| guard.assert_transform_changed_geometry())
            .and_then(|guard| guard.assert_replay_consumed_retained_artifact())
            .and_then(|guard| guard.assert_counters_are_receipt_backed())
            .and_then(|guard| guard.assert_no_fixture_arithmetic_as_truth())
            .and_then(|guard| guard.assert_no_synthetic_end_to_end_claim())
            .map(|_| ())
            .map_err(|error| {
                ProjectionFactParityDenial::from_ledger_error(
                    crate::workload_platform::evidence_ledger::WorkloadEvidenceLedgerError::from(
                        error,
                    ),
                )
            })
    }

    fn assert_required_lanes_once(&self) -> Result<(), ProjectionFactParityDenial> {
        let lanes = self.evidence_basis.lanes();
        for required in ProjectionFactParityLane::REQUIRED {
            let count = lanes
                .iter()
                .filter(|evidence| evidence.lane() == required)
                .count();
            if count == 0 {
                return Err(ProjectionFactParityDenial::new_with_workload_basis(
                    ProjectionFactParityDenialKind::MissingLane,
                    Some(required),
                    self.evidence_basis.workload_basis_identity(),
                    format!(
                        "Projection fact parity is missing the {}.",
                        required.human_name()
                    ),
                ));
            }
            if count > 1 {
                return Err(ProjectionFactParityDenial::new_with_workload_basis(
                    ProjectionFactParityDenialKind::DuplicateLane,
                    Some(required),
                    self.evidence_basis.workload_basis_identity(),
                    format!(
                        "Projection fact parity has duplicate evidence for the {}.",
                        required.human_name()
                    ),
                ));
            }
        }
        Ok(())
    }

    fn assert_no_mismatch(&self) -> Result<(), ProjectionFactParityDenial> {
        if let Some(policy) = self.lane_with_status(ProjectionFactParityLaneStatus::PolicyRequired)
        {
            return Err(ProjectionFactParityDenial::new_with_workload_basis(
                ProjectionFactParityDenialKind::PolicyRequired,
                Some(policy.lane()),
                self.evidence_basis.workload_basis_identity(),
                format!(
                    "Projection fact parity needs a user policy before comparing the {}.",
                    policy.lane().human_name()
                ),
            ));
        }
        self.assert_denied_paths_do_not_upgrade()?;
        let expected = self.evidence_basis.workload_basis_identity();
        if let Some(mismatch) = self
            .evidence_basis
            .lanes()
            .iter()
            .find(|lane| lane.parity_basis_identity() != expected)
        {
            return Err(self.mismatch_denial(mismatch));
        }
        if let Some(mismatch) = first_cross_lane_link_mismatch(self.evidence_basis.lanes()) {
            return Err(self.mismatch_denial(mismatch));
        }
        Ok(())
    }

    fn assert_denied_paths_do_not_upgrade(&self) -> Result<(), ProjectionFactParityDenial> {
        let lanes = self.evidence_basis.lanes();
        let has_denied = lanes
            .iter()
            .any(|lane| lane.status() == ProjectionFactParityLaneStatus::Denied);
        let has_admitted = lanes
            .iter()
            .any(|lane| lane.status() == ProjectionFactParityLaneStatus::Admitted);
        if has_denied && has_admitted {
            let upgraded = lanes
                .iter()
                .find(|lane| lane.status() == ProjectionFactParityLaneStatus::Admitted)
                .expect("admitted lane exists");
            return Err(ProjectionFactParityDenial::new_with_workload_basis(
                ProjectionFactParityDenialKind::DeniedLaneUpgraded,
                Some(upgraded.lane()),
                self.evidence_basis.workload_basis_identity(),
                format!(
                    "Denied projection parity cannot be upgraded through the {}.",
                    upgraded.lane().human_name()
                ),
            ));
        }
        Ok(())
    }

    fn mismatch_denial(
        &self,
        mismatch: &ProjectionFactParityLaneEvidence,
    ) -> ProjectionFactParityDenial {
        let kind = match mismatch.lane() {
            ProjectionFactParityLane::Live
            | ProjectionFactParityLane::Projected
            | ProjectionFactParityLane::ProjectionConsumed => {
                ProjectionFactParityDenialKind::LiveProjectionMismatch
            }
            ProjectionFactParityLane::Retained | ProjectionFactParityLane::Replayed => {
                ProjectionFactParityDenialKind::RetainedReplayMismatch
            }
            ProjectionFactParityLane::Recovered => ProjectionFactParityDenialKind::RecoveryMismatch,
            ProjectionFactParityLane::Transformed => {
                ProjectionFactParityDenialKind::TransformParityMismatch
            }
            ProjectionFactParityLane::LocalRebuild => {
                ProjectionFactParityDenialKind::LocalRebuildMismatch
            }
            ProjectionFactParityLane::Diagnostics => {
                ProjectionFactParityDenialKind::DiagnosticsMismatch
            }
        };
        ProjectionFactParityDenial::new_with_workload_basis(
            kind,
            Some(mismatch.lane()),
            self.evidence_basis.workload_basis_identity(),
            format!(
                "Projection fact parity found that the {} came from a different workload basis.",
                mismatch.lane().human_name()
            ),
        )
    }

    fn certified_case(&self) -> Result<ProjectionFactParityCase, ProjectionFactParityDenial> {
        let lanes = self.evidence_basis.lanes();
        if lanes
            .iter()
            .all(|lane| lane.status() == ProjectionFactParityLaneStatus::Denied)
        {
            return Ok(ProjectionFactParityCase::DeniedPreservedAcrossAllLanes);
        }
        Ok(ProjectionFactParityCase::AdmittedAcrossAllLanes)
    }

    fn lane_with_status(
        &self,
        status: ProjectionFactParityLaneStatus,
    ) -> Option<&ProjectionFactParityLaneEvidence> {
        self.evidence_basis
            .lanes()
            .iter()
            .find(|lane| lane.status() == status)
    }
}

fn first_cross_lane_link_mismatch(
    lanes: &[ProjectionFactParityLaneEvidence],
) -> Option<&ProjectionFactParityLaneEvidence> {
    let mut lanes_by_identity_by_kind: BTreeMap<
        _,
        BTreeMap<&str, Vec<&ProjectionFactParityLaneEvidence>>,
    > = BTreeMap::new();
    for lane in lanes {
        for link in lane.basis_links() {
            lanes_by_identity_by_kind
                .entry(link.kind())
                .or_default()
                .entry(link.identity())
                .or_default()
                .push(lane);
        }
    }
    for lanes_by_identity in lanes_by_identity_by_kind.values() {
        if lanes_by_identity.len() <= 1 {
            continue;
        }
        if let Some(outlier) = single_lane_outlier(lanes_by_identity) {
            return Some(outlier);
        }
        if let Some(producing_lane) = source_receipt_identity_outlier(lanes_by_identity) {
            return Some(producing_lane);
        }
    }
    None
}

fn single_lane_outlier<'a>(
    lanes_by_identity: &BTreeMap<&str, Vec<&'a ProjectionFactParityLaneEvidence>>,
) -> Option<&'a ProjectionFactParityLaneEvidence> {
    let majority_width = lanes_by_identity
        .values()
        .map(Vec::len)
        .max()
        .expect("mismatched link map is non-empty");
    if majority_width <= 1 {
        return None;
    }
    lanes_by_identity
        .values()
        .find(|lanes| lanes.len() == 1)
        .map(|lanes| lanes[0])
}

fn source_receipt_identity_outlier<'a>(
    lanes_by_identity: &BTreeMap<&str, Vec<&'a ProjectionFactParityLaneEvidence>>,
) -> Option<&'a ProjectionFactParityLaneEvidence> {
    lanes_by_identity
        .iter()
        .flat_map(|(identity, lanes)| {
            lanes.iter().copied().filter(move |lane| {
                lane.source_receipt_identity() == *identity && lanes_by_identity.len() > 1
            })
        })
        .next()
}

fn counters_for(lanes: &[ProjectionFactParityLaneEvidence]) -> ProjectionFactParityCounters {
    ProjectionFactParityCounters::new(
        lanes.len(),
        lanes
            .iter()
            .map(|lane| lane.source_receipt_identity())
            .collect::<BTreeSet<_>>()
            .len(),
        lanes
            .iter()
            .filter(|lane| lane.status() == ProjectionFactParityLaneStatus::Denied)
            .count(),
        lanes
            .iter()
            .filter(|lane| lane.status() == ProjectionFactParityLaneStatus::PolicyRequired)
            .count(),
    )
}

fn parity_digest(
    case: ProjectionFactParityCase,
    declaration: &str,
    workload_basis_identity: &str,
    lanes: &[ProjectionFactParityLaneEvidence],
) -> String {
    let mut parts = vec![
        format!("case:{case:?}"),
        format!("declaration:{declaration}"),
        format!("basis:{workload_basis_identity}"),
    ];
    parts.extend(lanes.iter().map(|lane| {
        format!(
            "{:?}:{}:{}:{:?}",
            lane.lane(),
            lane.source_receipt_identity(),
            lane.parity_basis_identity(),
            lane.status()
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
