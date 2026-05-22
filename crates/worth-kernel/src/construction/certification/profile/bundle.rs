use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::profile::{
    prepare_primitive_construction_policy_profile_report, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPolicyProfileRow, PrimitiveConstructionPolicyProfileSurfaceReport,
    PrimitiveConstructionPreviewContinuityHostilitySuiteReport,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::{
    prepare_primitive_construction_continuity_bundle_from_hostility_suite,
    prepare_primitive_construction_continuity_report_bundle,
    prepare_primitive_construction_policy_profile_branch_preview_runtime_report,
    prepare_primitive_construction_policy_profile_replay_parity_report,
    prepare_primitive_construction_preview_bundle_from_hostility_suite,
    prepare_primitive_construction_preview_report_bundle,
    prepare_primitive_construction_query_policy_profile_inspection_parity_report,
    prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report,
    PrimitiveConstructionContinuityHostilitySuiteReport,
    PrimitiveConstructionContinuityReportBundle, PrimitiveConstructionContinuityReportBundleError,
    PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError,
    PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport,
    PrimitiveConstructionPolicyProfileReplayParityError,
    PrimitiveConstructionPolicyProfileReplayParityReport,
    PrimitiveConstructionPreviewHostilitySuiteReport, PrimitiveConstructionPreviewReportBundle,
    PrimitiveConstructionPreviewReportBundleError,
    PrimitiveConstructionQueryPolicyProfileParityError,
    PrimitiveConstructionQueryPolicyProfileParityReport,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyProfileReportBundle {
    case: PrimitiveConstructionPolicyProfileCase,
    profile_row: PrimitiveConstructionPolicyProfileRow,
    replay_report: PrimitiveConstructionPolicyProfileReplayParityReport,
    inspection_report: PrimitiveConstructionQueryPolicyProfileParityReport,
    projection_report: PrimitiveConstructionQueryPolicyProfileParityReport,
    preview_bundle: PrimitiveConstructionPreviewReportBundle,
    continuity_bundle: Option<PrimitiveConstructionContinuityReportBundle>,
    branch_runtime_report: PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPolicyProfileReportBundle {
    fn new(
        case: PrimitiveConstructionPolicyProfileCase,
        profile_row: PrimitiveConstructionPolicyProfileRow,
        replay_report: PrimitiveConstructionPolicyProfileReplayParityReport,
        inspection_report: PrimitiveConstructionQueryPolicyProfileParityReport,
        projection_report: PrimitiveConstructionQueryPolicyProfileParityReport,
        preview_bundle: PrimitiveConstructionPreviewReportBundle,
        continuity_bundle: Option<PrimitiveConstructionContinuityReportBundle>,
        branch_runtime_report: PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport,
    ) -> Self {
        let continuity_parity = continuity_bundle
            .as_ref()
            .is_none_or(|bundle| bundle.parity_verified());
        let continuity_profile_match = continuity_bundle.as_ref().is_none_or(|bundle| {
            bundle.continuity_row().profile_name() == profile_row.profile_name()
        });
        let parity_verified = replay_report.parity_verified()
            && inspection_report.parity_verified()
            && projection_report.parity_verified()
            && preview_bundle.parity_verified()
            && continuity_parity
            && continuity_profile_match
            && inspection_report.profile_name() == projection_report.profile_name()
            && inspection_report.proximity_posture() == projection_report.proximity_posture()
            && inspection_report.alignment_posture() == projection_report.alignment_posture()
            && inspection_report.arbitration_posture() == projection_report.arbitration_posture()
            && inspection_report.preview_richness() == projection_report.preview_richness()
            && preview_bundle.preview_row().profile_name() == profile_row.profile_name()
            && branch_runtime_report.profile_row().profile_name() == profile_row.profile_name();
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            profile_row.row_digest().to_string(),
            replay_report.report_digest().to_string(),
            inspection_report.report_digest().to_string(),
            preview_bundle.report_digest().to_string(),
            continuity_bundle
                .as_ref()
                .map(|bundle| bundle.report_digest().to_string())
                .unwrap_or_else(|| "no-continuity-bundle".to_string()),
            branch_runtime_report.report_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            case,
            profile_row,
            replay_report,
            inspection_report,
            projection_report,
            preview_bundle,
            continuity_bundle,
            branch_runtime_report,
            parity_verified,
            report_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionPolicyProfileCase {
        self.case
    }

    pub fn profile_row(&self) -> &PrimitiveConstructionPolicyProfileRow {
        &self.profile_row
    }

    pub fn replay_report(&self) -> &PrimitiveConstructionPolicyProfileReplayParityReport {
        &self.replay_report
    }

    pub fn inspection_report(&self) -> &PrimitiveConstructionQueryPolicyProfileParityReport {
        &self.inspection_report
    }

    pub fn projection_report(&self) -> &PrimitiveConstructionQueryPolicyProfileParityReport {
        &self.projection_report
    }

    pub fn preview_bundle(&self) -> &PrimitiveConstructionPreviewReportBundle {
        &self.preview_bundle
    }

    pub fn continuity_bundle(&self) -> Option<&PrimitiveConstructionContinuityReportBundle> {
        self.continuity_bundle.as_ref()
    }

    pub fn branch_runtime_report(
        &self,
    ) -> &PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport {
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
pub enum PrimitiveConstructionPolicyProfileReportBundleError {
    MissingRow(PrimitiveConstructionPolicyProfileCase),
    Replay(PrimitiveConstructionPolicyProfileReplayParityError),
    Inspection(PrimitiveConstructionQueryPolicyProfileParityError),
    Projection(PrimitiveConstructionQueryPolicyProfileParityError),
    Preview(PrimitiveConstructionPreviewReportBundleError),
    Continuity(PrimitiveConstructionContinuityReportBundleError),
    BranchRuntime(PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionPolicyProfileReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRow(case) => write!(f, "missing policy profile row for {case:?}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::Preview(error) => write!(f, "{error}"),
            Self::Continuity(error) => write!(f, "{error}"),
            Self::BranchRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPolicyProfileReportBundleError {}

pub fn prepare_primitive_construction_policy_profile_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPolicyProfileCase,
) -> Result<
    PrimitiveConstructionPolicyProfileReportBundle,
    PrimitiveConstructionPolicyProfileReportBundleError,
> {
    let report: PrimitiveConstructionPolicyProfileSurfaceReport =
        prepare_primitive_construction_policy_profile_report();
    let profile_row = report
        .row(case)
        .ok_or(PrimitiveConstructionPolicyProfileReportBundleError::MissingRow(case))?
        .clone();
    let replay_report = prepare_primitive_construction_policy_profile_replay_parity_report(case)
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Replay)?;
    let inspection_report =
        prepare_primitive_construction_query_policy_profile_inspection_parity_report(
            workspace,
            profile_row.clone(),
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report(
            workspace,
            profile_row.clone(),
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Projection)?;
    let preview_bundle = prepare_primitive_construction_preview_report_bundle(
        workspace,
        profile_row.representative_preview_case(),
    )
    .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Preview)?;
    let continuity_bundle = profile_row
        .representative_continuity_case()
        .map(|continuity_case| {
            prepare_primitive_construction_continuity_report_bundle(workspace, continuity_case)
        })
        .transpose()
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Continuity)?;
    let branch_runtime_report =
        prepare_primitive_construction_policy_profile_branch_preview_runtime_report(
            workspace, case,
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::BranchRuntime)?;
    Ok(PrimitiveConstructionPolicyProfileReportBundle::new(
        case,
        profile_row,
        replay_report,
        inspection_report,
        projection_report,
        preview_bundle,
        continuity_bundle,
        branch_runtime_report,
    ))
}

pub fn prepare_primitive_construction_policy_profile_bundle_from_hostility_suites(
    preview_suite: &PrimitiveConstructionPreviewHostilitySuiteReport,
    continuity_suite: &PrimitiveConstructionContinuityHostilitySuiteReport,
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPolicyProfileCase,
) -> Result<
    PrimitiveConstructionPolicyProfileReportBundle,
    PrimitiveConstructionPolicyProfileReportBundleError,
> {
    let report = prepare_primitive_construction_policy_profile_report();
    let profile_row = report
        .row(case)
        .ok_or(PrimitiveConstructionPolicyProfileReportBundleError::MissingRow(case))?
        .clone();
    let replay_report = prepare_primitive_construction_policy_profile_replay_parity_report(case)
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Replay)?;
    let inspection_report =
        prepare_primitive_construction_query_policy_profile_inspection_parity_report(
            workspace,
            profile_row.clone(),
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report(
            workspace,
            profile_row.clone(),
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Projection)?;
    let preview_bundle = prepare_primitive_construction_preview_bundle_from_hostility_suite(
        preview_suite,
        workspace,
        profile_row.representative_preview_case(),
    )
    .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Preview)?;
    let continuity_bundle = profile_row
        .representative_continuity_case()
        .map(|continuity_case| {
            prepare_primitive_construction_continuity_bundle_from_hostility_suite(
                continuity_suite,
                workspace,
                continuity_case,
            )
        })
        .transpose()
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Continuity)?;
    let branch_runtime_report =
        prepare_primitive_construction_policy_profile_branch_preview_runtime_report(
            workspace, case,
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::BranchRuntime)?;
    Ok(PrimitiveConstructionPolicyProfileReportBundle::new(
        case,
        profile_row,
        replay_report,
        inspection_report,
        projection_report,
        preview_bundle,
        continuity_bundle,
        branch_runtime_report,
    ))
}

pub fn prepare_primitive_construction_policy_profile_bundle_from_combined_hostility_suite(
    suite: &PrimitiveConstructionPreviewContinuityHostilitySuiteReport,
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPolicyProfileCase,
) -> Result<
    PrimitiveConstructionPolicyProfileReportBundle,
    PrimitiveConstructionPolicyProfileReportBundleError,
> {
    let report = prepare_primitive_construction_policy_profile_report();
    let profile_row = report
        .row(case)
        .ok_or(PrimitiveConstructionPolicyProfileReportBundleError::MissingRow(case))?
        .clone();
    let combined_row = suite
        .rows()
        .iter()
        .find(|row| row.profile_case() == case)
        .ok_or(PrimitiveConstructionPolicyProfileReportBundleError::MissingRow(case))?;
    let replay_report = prepare_primitive_construction_policy_profile_replay_parity_report(case)
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Replay)?;
    let inspection_report =
        prepare_primitive_construction_query_policy_profile_inspection_parity_report(
            workspace,
            profile_row.clone(),
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report(
            workspace,
            profile_row.clone(),
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Projection)?;
    let preview_bundle = prepare_primitive_construction_preview_report_bundle(
        workspace,
        combined_row.preview_case(),
    )
    .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Preview)?;
    let continuity_bundle = combined_row
        .continuity_case()
        .map(|continuity_case| {
            prepare_primitive_construction_continuity_report_bundle(workspace, continuity_case)
        })
        .transpose()
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::Continuity)?;
    let branch_runtime_report =
        prepare_primitive_construction_policy_profile_branch_preview_runtime_report(
            workspace, case,
        )
        .map_err(PrimitiveConstructionPolicyProfileReportBundleError::BranchRuntime)?;
    Ok(PrimitiveConstructionPolicyProfileReportBundle::new(
        case,
        profile_row,
        replay_report,
        inspection_report,
        projection_report,
        preview_bundle,
        continuity_bundle,
        branch_runtime_report,
    ))
}
