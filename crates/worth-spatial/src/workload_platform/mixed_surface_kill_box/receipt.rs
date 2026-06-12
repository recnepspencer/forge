use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::family_run::{MixedSurfaceFamilyRun, MixedSurfaceFamilyRunStatus};
use crate::workload_platform::surface_support::SurfaceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MixedSurfaceKillBoxCounters {
    family_run_count: usize,
    certified_plane_count: usize,
    unsupported_family_count: usize,
    support_receipt_count: usize,
    user_outcome_count: usize,
    upstream_geometry_carriers: usize,
}

impl MixedSurfaceKillBoxCounters {
    pub(crate) fn from_runs(runs: &[MixedSurfaceFamilyRun]) -> Self {
        let certified_plane_count = runs
            .iter()
            .filter(|run| run.status() == MixedSurfaceFamilyRunStatus::AdmittedPlane)
            .count();
        let unsupported_family_count = runs
            .iter()
            .filter(|run| run.status() == MixedSurfaceFamilyRunStatus::Unsupported)
            .count();
        let upstream_geometry_carriers = runs
            .first()
            .map(|run| run.upstream_geometry_carriers())
            .unwrap_or(0);
        Self {
            family_run_count: runs.len(),
            certified_plane_count,
            unsupported_family_count,
            support_receipt_count: runs.len(),
            user_outcome_count: runs.len(),
            upstream_geometry_carriers,
        }
    }

    pub fn family_run_count(self) -> usize {
        self.family_run_count
    }

    pub fn certified_plane_count(self) -> usize {
        self.certified_plane_count
    }

    pub fn unsupported_family_count(self) -> usize {
        self.unsupported_family_count
    }

    pub fn support_receipt_count(self) -> usize {
        self.support_receipt_count
    }

    pub fn user_outcome_count(self) -> usize {
        self.user_outcome_count
    }

    pub fn upstream_geometry_carriers(self) -> usize {
        self.upstream_geometry_carriers
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MixedSurfaceKillBoxReceipt {
    declaration: String,
    stable_geometry_binding_identity: String,
    kill_box_digest: String,
    runs: Vec<MixedSurfaceFamilyRun>,
    counters: MixedSurfaceKillBoxCounters,
}

impl MixedSurfaceKillBoxReceipt {
    pub(crate) fn new(
        declaration: String,
        stable_geometry_binding_identity: String,
        runs: Vec<MixedSurfaceFamilyRun>,
    ) -> Self {
        let counters = MixedSurfaceKillBoxCounters::from_runs(&runs);
        let mut parts = vec![
            "mixed-surface-kill-box".to_string(),
            declaration.clone(),
            stable_geometry_binding_identity.clone(),
        ];
        parts.extend(runs.iter().map(|run| {
            format!(
                "{:?}:{}:{}",
                run.family(),
                run.support_evidence_digest(),
                run.user_response_digest()
            )
        }));
        let kill_box_digest = truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts);
        Self {
            declaration,
            stable_geometry_binding_identity,
            kill_box_digest,
            runs,
            counters,
        }
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn stable_geometry_binding_identity(&self) -> &str {
        &self.stable_geometry_binding_identity
    }

    pub fn kill_box_digest(&self) -> &str {
        &self.kill_box_digest
    }

    pub fn runs(&self) -> &[MixedSurfaceFamilyRun] {
        &self.runs
    }

    pub fn counters(&self) -> MixedSurfaceKillBoxCounters {
        self.counters
    }

    pub fn run_for_family(&self, family: SurfaceFamily) -> Option<&MixedSurfaceFamilyRun> {
        self.runs.iter().find(|run| run.family() == family)
    }

    pub fn plane_control(&self) -> Option<&MixedSurfaceFamilyRun> {
        self.run_for_family(SurfaceFamily::Plane)
    }

    pub fn unsupported_family_runs(&self) -> impl Iterator<Item = &MixedSurfaceFamilyRun> {
        self.runs
            .iter()
            .filter(|run| run.family() != SurfaceFamily::Plane)
    }

    pub fn attempt_generated_feature_partial_admission(
        &self,
    ) -> Result<(), super::denial::MixedSurfaceKillBoxDenial> {
        let run = self.run_for_family(SurfaceFamily::GeneratedFeature).ok_or(
            super::denial::MixedSurfaceKillBoxDenial::MissingSurfaceSupportEvidence {
                family: SurfaceFamily::GeneratedFeature,
            },
        )?;
        if run.is_acceptable_m7_input() {
            Ok(())
        } else {
            Err(super::denial::MixedSurfaceKillBoxDenial::GeneratedFeatureSmugglingAttempt)
        }
    }
}
