use forge_query::facade::consumer_kit::{
    project_support_snapshot, support_pinning_contract, ForgeQueryPinnedSupportStatus,
    ForgeQueryPinnedTeachingPosture, ForgeQueryRuntimeFacadeFamily, ForgeQuerySupportPinContract,
    ForgeQuerySupportPinReport, ForgeQuerySupportPinningError, ForgeQuerySupportSnapshot,
};
use forge_query::facade::runtime::{
    ForgeQueryRuntimePublicApiContract, ForgeQueryRuntimePublicSupportMatrix,
    ForgeQueryRuntimeSupportProfile,
};

const REQUIRED_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 3] = [
    ForgeQueryRuntimeFacadeFamily::Read,
    ForgeQueryRuntimeFacadeFamily::Computed,
    ForgeQueryRuntimeFacadeFamily::Replay,
];

const SPATIAL_QUERY_ADOPTION_CONSUMER: &str = "worth-spatial.query-adoption.phase-six";

const WORKLOAD_SUPPORT_REQUIREMENTS: [(
    WorthSpatialWorkloadSupportFamily,
    ForgeQueryRuntimeFacadeFamily,
); 7] = [
    (
        WorthSpatialWorkloadSupportFamily::RetainedReplay,
        ForgeQueryRuntimeFacadeFamily::Replay,
    ),
    (
        WorthSpatialWorkloadSupportFamily::ProjectionFactParity,
        ForgeQueryRuntimeFacadeFamily::Read,
    ),
    (
        WorthSpatialWorkloadSupportFamily::ProjectionFactParity,
        ForgeQueryRuntimeFacadeFamily::Computed,
    ),
    (
        WorthSpatialWorkloadSupportFamily::ProjectionFactParity,
        ForgeQueryRuntimeFacadeFamily::Replay,
    ),
    (
        WorthSpatialWorkloadSupportFamily::BooleanReadiness,
        ForgeQueryRuntimeFacadeFamily::Read,
    ),
    (
        WorthSpatialWorkloadSupportFamily::BooleanReadiness,
        ForgeQueryRuntimeFacadeFamily::Computed,
    ),
    (
        WorthSpatialWorkloadSupportFamily::BooleanReadiness,
        ForgeQueryRuntimeFacadeFamily::Replay,
    ),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthSpatialWorkloadSupportFamily {
    RetainedReplay,
    ProjectionFactParity,
    BooleanReadiness,
}

impl WorthSpatialWorkloadSupportFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetainedReplay => "retained-replay",
            Self::ProjectionFactParity => "projection-fact-parity",
            Self::BooleanReadiness => "boolean-readiness",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthSpatialWorkloadSupportPinRow {
    workload_family: WorthSpatialWorkloadSupportFamily,
    query_runtime_family: ForgeQueryRuntimeFacadeFamily,
    query_support_surface: String,
    query_snapshot_row_digest: String,
    support_pin_report_digest: String,
}

impl WorthSpatialWorkloadSupportPinRow {
    fn new(
        workload_family: WorthSpatialWorkloadSupportFamily,
        query_runtime_family: ForgeQueryRuntimeFacadeFamily,
        query_support_surface: impl Into<String>,
        query_snapshot_row_digest: impl Into<String>,
        support_pin_report_digest: impl Into<String>,
    ) -> Self {
        Self {
            workload_family,
            query_runtime_family,
            query_support_surface: query_support_surface.into(),
            query_snapshot_row_digest: query_snapshot_row_digest.into(),
            support_pin_report_digest: support_pin_report_digest.into(),
        }
    }

    pub const fn workload_family(&self) -> WorthSpatialWorkloadSupportFamily {
        self.workload_family
    }

    pub const fn query_runtime_family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.query_runtime_family
    }

    pub fn query_support_surface(&self) -> &str {
        &self.query_support_surface
    }

    pub fn query_snapshot_row_digest(&self) -> &str {
        &self.query_snapshot_row_digest
    }

    pub fn support_pin_report_digest(&self) -> &str {
        &self.support_pin_report_digest
    }
}

pub(super) fn current_spatial_support_snapshot() -> ForgeQuerySupportSnapshot {
    snapshot_from_profile(ForgeQueryRuntimeSupportProfile::scaffold_backend_profile())
}

pub(super) fn current_spatial_support_pin_contract(
    snapshot: &ForgeQuerySupportSnapshot,
) -> Result<ForgeQuerySupportPinContract, ForgeQuerySupportPinningError> {
    let mut builder =
        support_pinning_contract(SPATIAL_QUERY_ADOPTION_CONSUMER).against_snapshot(snapshot)?;

    for family in REQUIRED_FAMILIES {
        builder = builder.require_family(family, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })?;
    }

    builder
        .observe_family(ForgeQueryRuntimeFacadeFamily::SharedRead)?
        .seal()
}

pub(super) fn evaluate_current_spatial_support_pins(
) -> Result<ForgeQuerySupportPinReport, ForgeQuerySupportPinningError> {
    let snapshot = current_spatial_support_snapshot();
    current_spatial_support_pin_contract(&snapshot)?.evaluate_snapshot(&snapshot)
}

pub fn current_spatial_workload_support_pin_rows(
) -> Result<Vec<WorthSpatialWorkloadSupportPinRow>, ForgeQuerySupportPinningError> {
    let snapshot = current_spatial_support_snapshot();
    let report = current_spatial_support_pin_contract(&snapshot)?.evaluate_snapshot(&snapshot)?;
    report.assert_satisfied()?;
    Ok(spatial_workload_support_pin_rows(&snapshot, &report))
}

pub(super) fn spatial_workload_support_pin_rows(
    snapshot: &ForgeQuerySupportSnapshot,
    report: &ForgeQuerySupportPinReport,
) -> Vec<WorthSpatialWorkloadSupportPinRow> {
    WORKLOAD_SUPPORT_REQUIREMENTS
        .iter()
        .map(|(workload_family, query_runtime_family)| {
            let family_label = query_runtime_family.as_str();
            let row = snapshot
                .rows()
                .iter()
                .find(|row| row.facade_family() == Some(family_label))
                .expect("required Query support row must exist after pin evaluation");
            WorthSpatialWorkloadSupportPinRow::new(
                *workload_family,
                *query_runtime_family,
                row.surface(),
                row.snapshot_row_digest(),
                report.report_digest(),
            )
        })
        .collect()
}

fn snapshot_from_profile(profile: ForgeQueryRuntimeSupportProfile) -> ForgeQuerySupportSnapshot {
    let contract = ForgeQueryRuntimePublicApiContract::from_support_profile(&profile);
    let matrix = ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    project_support_snapshot(&matrix)
}

#[cfg(test)]
mod tests {
    use forge_query::facade::consumer_kit::ForgeQuerySupportPinFindingKind;
    use forge_query::facade::runtime::ForgeQueryRuntimeFamilySupport;

    use super::*;

    #[test]
    fn spatial_support_pins_evaluate_against_real_query_snapshot() {
        let report = evaluate_current_spatial_support_pins()
            .expect("spatial support pins should evaluate through Query");

        assert!(report.satisfied());
        assert_eq!(report.consumer_name(), SPATIAL_QUERY_ADOPTION_CONSUMER);
        assert_eq!(report.requirement_count(), REQUIRED_FAMILIES.len());
        assert_eq!(report.matched_required_count(), REQUIRED_FAMILIES.len());
        assert_eq!(report.observed_count(), 1);
        assert_eq!(report.blocking_finding_count(), 0);
    }

    #[test]
    fn spatial_workload_families_are_pinned_to_real_query_support_rows() {
        let rows = current_spatial_workload_support_pin_rows()
            .expect("spatial workload support rows should evaluate through Query");
        let report = evaluate_current_spatial_support_pins().expect("support pin report");

        assert_eq!(rows.len(), WORKLOAD_SUPPORT_REQUIREMENTS.len());
        for (workload_family, query_runtime_family) in WORKLOAD_SUPPORT_REQUIREMENTS {
            let row = rows
                .iter()
                .find(|row| {
                    row.workload_family() == workload_family
                        && row.query_runtime_family() == query_runtime_family
                })
                .expect("workload family must declare Query support family");
            assert_eq!(row.support_pin_report_digest(), report.report_digest());
            assert!(!row.query_support_surface().is_empty());
            assert!(!row.query_snapshot_row_digest().is_empty());
        }
    }

    #[test]
    fn spatial_support_pin_drift_is_localized_to_replay_requirement() {
        let basis = current_spatial_support_snapshot();
        let drifted = snapshot_from_profile(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
                ForgeQueryRuntimeFamilySupport::deferred(
                    ForgeQueryRuntimeFacadeFamily::Replay,
                    "worth-spatial phase six drift fixture",
                ),
            ),
        );
        let contract = current_spatial_support_pin_contract(&basis)
            .expect("spatial support pin contract should seal");

        let report = contract
            .evaluate_snapshot(&drifted)
            .expect("Query support pin evaluation should report drift");

        assert!(!report.satisfied());
        assert!(report.findings().iter().any(|finding| {
            finding.family() == Some(ForgeQueryRuntimeFacadeFamily::Replay)
                && finding.kind() == ForgeQuerySupportPinFindingKind::StatusMismatch
                && finding.blocking()
                && finding.expected() == Some("supported")
                && finding.found() == Some("deferred-debt")
        }));
    }
}
