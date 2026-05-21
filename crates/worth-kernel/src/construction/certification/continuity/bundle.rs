use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::continuity::{
    prepare_primitive_construction_continuity_surface_report, PrimitiveConstructionContinuityCase,
    PrimitiveConstructionContinuityHostilitySuiteReport, PrimitiveConstructionContinuityRow,
    PrimitiveConstructionContinuitySurfaceReport,
    PrimitiveConstructionContinuitySurfaceReportError,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::{
    prepare_primitive_construction_continuity_branch_preview_runtime_report,
    prepare_primitive_construction_continuity_replay_parity_report,
    prepare_primitive_construction_query_continuity_inspection_parity_report,
    prepare_primitive_construction_query_continuity_projection_consumption_receipt_report,
    PrimitiveConstructionContinuityBranchPreviewRuntimeError,
    PrimitiveConstructionContinuityBranchPreviewRuntimeReport,
    PrimitiveConstructionContinuityReplayParityError,
    PrimitiveConstructionContinuityReplayParityReport,
    PrimitiveConstructionQueryContinuityParityError,
    PrimitiveConstructionQueryContinuityParityReport,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionContinuityReportBundle {
    case: PrimitiveConstructionContinuityCase,
    continuity_row: PrimitiveConstructionContinuityRow,
    replay_report: PrimitiveConstructionContinuityReplayParityReport,
    inspection_report: PrimitiveConstructionQueryContinuityParityReport,
    projection_report: PrimitiveConstructionQueryContinuityParityReport,
    branch_runtime_report: PrimitiveConstructionContinuityBranchPreviewRuntimeReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionContinuityReportBundle {
    fn new(
        case: PrimitiveConstructionContinuityCase,
        continuity_row: PrimitiveConstructionContinuityRow,
        replay_report: PrimitiveConstructionContinuityReplayParityReport,
        inspection_report: PrimitiveConstructionQueryContinuityParityReport,
        projection_report: PrimitiveConstructionQueryContinuityParityReport,
        branch_runtime_report: PrimitiveConstructionContinuityBranchPreviewRuntimeReport,
    ) -> Self {
        let parity_verified = replay_report.parity_verified()
            && inspection_report.parity_verified()
            && projection_report.parity_verified()
            && inspection_report.continuity_class() == projection_report.continuity_class()
            && inspection_report.source() == projection_report.source()
            && inspection_report.candidate() == projection_report.candidate()
            && inspection_report.blocked_capability() == projection_report.blocked_capability()
            && inspection_report.preserves_subject_identity()
                == projection_report.preserves_subject_identity()
            && inspection_report.preserves_anchor_identity()
                == projection_report.preserves_anchor_identity();
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            continuity_row.row_digest().to_string(),
            replay_report.report_digest().to_string(),
            branch_runtime_report.report_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            case,
            continuity_row,
            replay_report,
            inspection_report,
            projection_report,
            branch_runtime_report,
            parity_verified,
            report_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionContinuityCase {
        self.case
    }

    pub fn continuity_row(&self) -> &PrimitiveConstructionContinuityRow {
        &self.continuity_row
    }

    pub fn replay_report(&self) -> &PrimitiveConstructionContinuityReplayParityReport {
        &self.replay_report
    }

    pub fn inspection_report(&self) -> &PrimitiveConstructionQueryContinuityParityReport {
        &self.inspection_report
    }

    pub fn projection_report(&self) -> &PrimitiveConstructionQueryContinuityParityReport {
        &self.projection_report
    }

    pub fn branch_runtime_report(
        &self,
    ) -> &PrimitiveConstructionContinuityBranchPreviewRuntimeReport {
        &self.branch_runtime_report
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionContinuityReportBundleError {
    Surface(PrimitiveConstructionContinuitySurfaceReportError),
    MissingRow(PrimitiveConstructionContinuityCase),
    Replay(PrimitiveConstructionContinuityReplayParityError),
    Inspection(PrimitiveConstructionQueryContinuityParityError),
    Projection(PrimitiveConstructionQueryContinuityParityError),
    BranchRuntime(PrimitiveConstructionContinuityBranchPreviewRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionContinuityReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Surface(error) => write!(f, "{error}"),
            Self::MissingRow(case) => write!(f, "missing continuity row for {case:?}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::BranchRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionContinuityReportBundleError {}

pub fn prepare_primitive_construction_continuity_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionContinuityCase,
) -> Result<
    PrimitiveConstructionContinuityReportBundle,
    PrimitiveConstructionContinuityReportBundleError,
> {
    let report: PrimitiveConstructionContinuitySurfaceReport =
        prepare_primitive_construction_continuity_surface_report()
            .map_err(PrimitiveConstructionContinuityReportBundleError::Surface)?;
    let continuity_row = report
        .row(case)
        .ok_or(PrimitiveConstructionContinuityReportBundleError::MissingRow(case))?
        .clone();
    let replay_report = prepare_primitive_construction_continuity_replay_parity_report(case)
        .map_err(PrimitiveConstructionContinuityReportBundleError::Replay)?;
    let inspection_report =
        prepare_primitive_construction_query_continuity_inspection_parity_report(
            workspace,
            continuity_row.clone(),
        )
        .map_err(PrimitiveConstructionContinuityReportBundleError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_continuity_projection_consumption_receipt_report(
            workspace,
            continuity_row.clone(),
        )
        .map_err(PrimitiveConstructionContinuityReportBundleError::Projection)?;
    let branch_runtime_report =
        prepare_primitive_construction_continuity_branch_preview_runtime_report(workspace, case)
            .map_err(PrimitiveConstructionContinuityReportBundleError::BranchRuntime)?;
    Ok(PrimitiveConstructionContinuityReportBundle::new(
        case,
        continuity_row,
        replay_report,
        inspection_report,
        projection_report,
        branch_runtime_report,
    ))
}

pub fn prepare_primitive_construction_continuity_bundle_from_hostility_suite(
    suite: &PrimitiveConstructionContinuityHostilitySuiteReport,
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionContinuityCase,
) -> Result<
    PrimitiveConstructionContinuityReportBundle,
    PrimitiveConstructionContinuityReportBundleError,
> {
    let continuity_row = suite
        .row(case)
        .ok_or(PrimitiveConstructionContinuityReportBundleError::MissingRow(case))?
        .clone();
    let replay_report = prepare_primitive_construction_continuity_replay_parity_report(case)
        .map_err(PrimitiveConstructionContinuityReportBundleError::Replay)?;
    let inspection_report =
        prepare_primitive_construction_query_continuity_inspection_parity_report(
            workspace,
            continuity_row.clone(),
        )
        .map_err(PrimitiveConstructionContinuityReportBundleError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_continuity_projection_consumption_receipt_report(
            workspace,
            continuity_row.clone(),
        )
        .map_err(PrimitiveConstructionContinuityReportBundleError::Projection)?;
    let branch_runtime_report =
        prepare_primitive_construction_continuity_branch_preview_runtime_report(workspace, case)
            .map_err(PrimitiveConstructionContinuityReportBundleError::BranchRuntime)?;
    Ok(PrimitiveConstructionContinuityReportBundle::new(
        case,
        continuity_row,
        replay_report,
        inspection_report,
        projection_report,
        branch_runtime_report,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_continuity_bundle_from_hostility_suite,
        prepare_primitive_construction_continuity_report_bundle,
    };
    use crate::construction::{
        prepare_primitive_construction_continuity_hostility_suite_report,
        PrimitiveConstructionContinuityCase,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn continuity_report_bundle_binds_direct_replay_query_and_branch_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.continuity-bundle".to_string(),
        )
        .expect("workspace");

        let report = prepare_primitive_construction_continuity_report_bundle(
            &mut workspace,
            PrimitiveConstructionContinuityCase::HostAttachReinterpreted,
        )
        .expect("report");

        assert!(report.parity_verified());
        assert_eq!(
            report.continuity_row().continuity_class(),
            report.inspection_report().continuity_class()
        );
        assert_eq!(
            report.inspection_report().continuity_class(),
            report.projection_report().continuity_class()
        );
        assert_eq!(
            report.continuity_row().source(),
            report.inspection_report().source()
        );
        assert_eq!(
            report.inspection_report().source(),
            report.projection_report().source()
        );
        assert_eq!(
            report.continuity_row().candidate(),
            report.inspection_report().candidate()
        );
        assert_eq!(
            report.inspection_report().candidate(),
            report.projection_report().candidate()
        );
    }

    #[test]
    fn continuity_report_bundle_reuses_hostility_suite_rows_directly() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.continuity-bundle-suite".to_string(),
        )
        .expect("workspace");
        let suite =
            prepare_primitive_construction_continuity_hostility_suite_report().expect("suite");

        let report = prepare_primitive_construction_continuity_bundle_from_hostility_suite(
            &suite,
            &mut workspace,
            PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
        )
        .expect("report");

        assert!(report.parity_verified());
        assert_eq!(
            report.continuity_row().blocked_capability(),
            report.projection_report().blocked_capability()
        );
        assert_ne!(report.report_digest(), report.continuity_row().row_digest());
    }
}
