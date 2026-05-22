use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::preview::{
    prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewHostilitySuiteReport, PrimitiveConstructionPreviewRow,
    PrimitiveConstructionPreviewSurfaceReport, PrimitiveConstructionPreviewSurfaceReportError,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::{
    prepare_primitive_construction_preview_branch_preview_runtime_report,
    prepare_primitive_construction_preview_replay_parity_report,
    prepare_primitive_construction_query_preview_inspection_parity_report,
    prepare_primitive_construction_query_preview_projection_consumption_receipt_report,
    PrimitiveConstructionPreviewBranchPreviewRuntimeError,
    PrimitiveConstructionPreviewBranchPreviewRuntimeReport,
    PrimitiveConstructionPreviewReplayParityError, PrimitiveConstructionPreviewReplayParityReport,
    PrimitiveConstructionQueryPreviewParityError, PrimitiveConstructionQueryPreviewParityReport,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreviewReportBundle {
    case: PrimitiveConstructionPreviewCase,
    preview_row: PrimitiveConstructionPreviewRow,
    replay_report: PrimitiveConstructionPreviewReplayParityReport,
    inspection_report: PrimitiveConstructionQueryPreviewParityReport,
    projection_report: PrimitiveConstructionQueryPreviewParityReport,
    branch_runtime_report: PrimitiveConstructionPreviewBranchPreviewRuntimeReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPreviewReportBundle {
    fn new(
        case: PrimitiveConstructionPreviewCase,
        preview_row: PrimitiveConstructionPreviewRow,
        replay_report: PrimitiveConstructionPreviewReplayParityReport,
        inspection_report: PrimitiveConstructionQueryPreviewParityReport,
        projection_report: PrimitiveConstructionQueryPreviewParityReport,
        branch_runtime_report: PrimitiveConstructionPreviewBranchPreviewRuntimeReport,
    ) -> Self {
        let parity_verified = replay_report.parity_verified()
            && inspection_report.parity_verified()
            && projection_report.parity_verified()
            && inspection_report.candidates() == projection_report.candidates()
            && inspection_report.blocked_candidates() == projection_report.blocked_candidates()
            && inspection_report.warnings() == projection_report.warnings();
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            preview_row.row_digest().to_string(),
            replay_report.report_digest().to_string(),
            branch_runtime_report.report_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            case,
            preview_row,
            replay_report,
            inspection_report,
            projection_report,
            branch_runtime_report,
            parity_verified,
            report_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionPreviewCase {
        self.case
    }

    pub fn preview_row(&self) -> &PrimitiveConstructionPreviewRow {
        &self.preview_row
    }

    pub fn replay_report(&self) -> &PrimitiveConstructionPreviewReplayParityReport {
        &self.replay_report
    }

    pub fn inspection_report(&self) -> &PrimitiveConstructionQueryPreviewParityReport {
        &self.inspection_report
    }

    pub fn projection_report(&self) -> &PrimitiveConstructionQueryPreviewParityReport {
        &self.projection_report
    }

    pub fn branch_runtime_report(&self) -> &PrimitiveConstructionPreviewBranchPreviewRuntimeReport {
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
pub enum PrimitiveConstructionPreviewReportBundleError {
    Surface(PrimitiveConstructionPreviewSurfaceReportError),
    MissingRow(PrimitiveConstructionPreviewCase),
    Replay(PrimitiveConstructionPreviewReplayParityError),
    Inspection(PrimitiveConstructionQueryPreviewParityError),
    Projection(PrimitiveConstructionQueryPreviewParityError),
    BranchRuntime(PrimitiveConstructionPreviewBranchPreviewRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionPreviewReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Surface(error) => write!(f, "{error}"),
            Self::MissingRow(case) => write!(f, "missing preview row for {case:?}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::BranchRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPreviewReportBundleError {}

pub fn prepare_primitive_construction_preview_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPreviewCase,
) -> Result<PrimitiveConstructionPreviewReportBundle, PrimitiveConstructionPreviewReportBundleError>
{
    let report: PrimitiveConstructionPreviewSurfaceReport =
        prepare_primitive_construction_preview_surface_report()
            .map_err(PrimitiveConstructionPreviewReportBundleError::Surface)?;
    let preview_row = report
        .row(case)
        .ok_or(PrimitiveConstructionPreviewReportBundleError::MissingRow(
            case,
        ))?
        .clone();
    let replay_report = prepare_primitive_construction_preview_replay_parity_report(case)
        .map_err(PrimitiveConstructionPreviewReportBundleError::Replay)?;
    let inspection_report = prepare_primitive_construction_query_preview_inspection_parity_report(
        workspace,
        preview_row.clone(),
    )
    .map_err(PrimitiveConstructionPreviewReportBundleError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_preview_projection_consumption_receipt_report(
            workspace,
            preview_row.clone(),
        )
        .map_err(PrimitiveConstructionPreviewReportBundleError::Projection)?;
    let branch_runtime_report =
        prepare_primitive_construction_preview_branch_preview_runtime_report(workspace, case)
            .map_err(PrimitiveConstructionPreviewReportBundleError::BranchRuntime)?;
    Ok(PrimitiveConstructionPreviewReportBundle::new(
        case,
        preview_row,
        replay_report,
        inspection_report,
        projection_report,
        branch_runtime_report,
    ))
}

pub fn prepare_primitive_construction_preview_bundle_from_hostility_suite(
    suite: &PrimitiveConstructionPreviewHostilitySuiteReport,
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPreviewCase,
) -> Result<PrimitiveConstructionPreviewReportBundle, PrimitiveConstructionPreviewReportBundleError>
{
    let preview_row = suite
        .row(case)
        .ok_or(PrimitiveConstructionPreviewReportBundleError::MissingRow(
            case,
        ))?
        .clone();
    let replay_report = prepare_primitive_construction_preview_replay_parity_report(case)
        .map_err(PrimitiveConstructionPreviewReportBundleError::Replay)?;
    let inspection_report = prepare_primitive_construction_query_preview_inspection_parity_report(
        workspace,
        preview_row.clone(),
    )
    .map_err(PrimitiveConstructionPreviewReportBundleError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_preview_projection_consumption_receipt_report(
            workspace,
            preview_row.clone(),
        )
        .map_err(PrimitiveConstructionPreviewReportBundleError::Projection)?;
    let branch_runtime_report =
        prepare_primitive_construction_preview_branch_preview_runtime_report(workspace, case)
            .map_err(PrimitiveConstructionPreviewReportBundleError::BranchRuntime)?;
    Ok(PrimitiveConstructionPreviewReportBundle::new(
        case,
        preview_row,
        replay_report,
        inspection_report,
        projection_report,
        branch_runtime_report,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_preview_bundle_from_hostility_suite,
        prepare_primitive_construction_preview_report_bundle,
    };
    use crate::construction::{
        prepare_primitive_construction_preview_hostility_suite_report,
        PrimitiveConstructionPreviewCase,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn preview_report_bundle_binds_direct_replay_query_and_branch_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.preview-bundle".to_string(),
        )
        .expect("workspace");

        let report = prepare_primitive_construction_preview_report_bundle(
            &mut workspace,
            PrimitiveConstructionPreviewCase::GrazingAggressiveSnap,
        )
        .expect("report");

        assert!(report.parity_verified());
        assert_eq!(
            report.preview_row().candidates(),
            report.inspection_report().candidates()
        );
        assert_eq!(
            report.inspection_report().candidates(),
            report.projection_report().candidates()
        );
        assert_eq!(
            report.preview_row().blocked_candidates(),
            report.inspection_report().blocked_candidates()
        );
        assert_eq!(
            report.inspection_report().blocked_candidates(),
            report.projection_report().blocked_candidates()
        );
        assert_eq!(
            report.preview_row().warnings(),
            report.inspection_report().warnings()
        );
        assert_eq!(
            report.inspection_report().warnings(),
            report.projection_report().warnings()
        );
    }

    #[test]
    fn preview_report_bundle_reuses_hostility_suite_rows_directly() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.preview-bundle-suite".to_string(),
        )
        .expect("workspace");
        let suite = prepare_primitive_construction_preview_hostility_suite_report().expect("suite");

        let report = prepare_primitive_construction_preview_bundle_from_hostility_suite(
            &suite,
            &mut workspace,
            PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
        )
        .expect("report");

        assert!(report.parity_verified());
        assert_eq!(
            report.preview_row().candidates(),
            report.projection_report().candidates()
        );
        assert_ne!(report.report_digest(), report.preview_row().row_digest());
    }
}
