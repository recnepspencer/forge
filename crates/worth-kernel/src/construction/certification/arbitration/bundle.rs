use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::arbitration::bundle_verified::{
    verify_bundle, PrimitiveConstructionIntentArbitrationBundleVerificationFailure,
    PrimitiveConstructionIntentArbitrationReportBundle,
    PrimitiveConstructionVerifiedIntentArbitrationReportBundle,
};
use crate::construction::query::{
    prepare_primitive_construction_query_intent_arbitration_inspection_parity_report,
    prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report,
    PrimitiveConstructionQueryIntentArbitrationParityError,
};
use crate::construction::{
    prepare_primitive_construction_intent_arbitration_replay_parity_report,
    PrimitiveConstructionIntentArbitrationReplayParityError,
};

use super::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_construction_preserved_intent_resolution_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionChosenIntentResolutionReport,
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionPreservedIntentResolutionReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentArbitrationBundleCase {
    DirectMoveOnlyPolicy,
    GrazingSnapExplicitChoice,
    OverlapMoveOnlyWithBlockedMerge,
    HostPenetrationBlockedCut,
    FrameAlignedIntent,
    OverlapAdvancedCapabilities,
}

impl PrimitiveConstructionIntentArbitrationBundleCase {
    pub(crate) fn policy_case(&self) -> super::PrimitiveConstructionIntentArbitrationPolicyCase {
        match self {
            Self::DirectMoveOnlyPolicy => {
                super::PrimitiveConstructionIntentArbitrationPolicyCase::DirectMoveOnly
            }
            Self::GrazingSnapExplicitChoice => {
                super::PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict
            }
            Self::OverlapMoveOnlyWithBlockedMerge => {
                super::PrimitiveConstructionIntentArbitrationPolicyCase::OverlapBlockedCandidates
            }
            Self::HostPenetrationBlockedCut => {
                super::PrimitiveConstructionIntentArbitrationPolicyCase::HostPenetrationBlockedCut
            }
            Self::FrameAlignedIntent => {
                super::PrimitiveConstructionIntentArbitrationPolicyCase::FrameAlignedIntent
            }
            Self::OverlapAdvancedCapabilities => {
                super::PrimitiveConstructionIntentArbitrationPolicyCase::OverlapAdvancedCapabilities
            }
        }
    }

    pub(crate) fn chosen_case(&self) -> Option<PrimitiveConstructionChosenIntentResolutionCase> {
        match self {
            Self::DirectMoveOnlyPolicy => {
                Some(PrimitiveConstructionChosenIntentResolutionCase::PolicyMoveOnly)
            }
            Self::GrazingSnapExplicitChoice => {
                Some(PrimitiveConstructionChosenIntentResolutionCase::ExplicitSnapFlush)
            }
            Self::OverlapMoveOnlyWithBlockedMerge => Some(
                PrimitiveConstructionChosenIntentResolutionCase::ExplicitMoveOnlyWithBlockedMerge,
            ),
            Self::HostPenetrationBlockedCut
            | Self::FrameAlignedIntent
            | Self::OverlapAdvancedCapabilities => None,
        }
    }

    pub(crate) fn preserved_case(
        &self,
    ) -> super::PrimitiveConstructionPreservedIntentResolutionCase {
        match self {
            Self::DirectMoveOnlyPolicy => {
                super::PrimitiveConstructionPreservedIntentResolutionCase::PolicyMoveOnly
            }
            Self::FrameAlignedIntent => {
                super::PrimitiveConstructionPreservedIntentResolutionCase::FrameAlignedPolicy
            }
            Self::GrazingSnapExplicitChoice => {
                super::PrimitiveConstructionPreservedIntentResolutionCase::ExplicitSnapFlush
            }
            Self::OverlapMoveOnlyWithBlockedMerge => {
                super::PrimitiveConstructionPreservedIntentResolutionCase::ExplicitMoveOnlyWithBlockedMerge
            }
            Self::HostPenetrationBlockedCut => {
                super::PrimitiveConstructionPreservedIntentResolutionCase::HostPenetrationBlockedCut
            }
            Self::OverlapAdvancedCapabilities => {
                super::PrimitiveConstructionPreservedIntentResolutionCase::OverlapAdvancedClarification
            }
        }
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionIntentArbitrationReportBundleError {
    PolicyReport(super::PrimitiveConstructionIntentArbitrationPolicyReportError),
    ChosenReport(super::PrimitiveConstructionChosenIntentResolutionReportError),
    PreservedReport(super::PrimitiveConstructionPreservedIntentResolutionReportError),
    Replay(PrimitiveConstructionIntentArbitrationReplayParityError),
    Query(PrimitiveConstructionQueryIntentArbitrationParityError),
    Verification(PrimitiveConstructionIntentArbitrationBundleVerificationFailure),
    MissingPolicyRow(PrimitiveConstructionIntentArbitrationBundleCase),
    MissingChosenRow(PrimitiveConstructionIntentArbitrationBundleCase),
    MissingPreservedRow(PrimitiveConstructionIntentArbitrationBundleCase),
}

impl std::fmt::Display for PrimitiveConstructionIntentArbitrationReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PolicyReport(error) => write!(f, "{error}"),
            Self::ChosenReport(error) => write!(f, "{error}"),
            Self::PreservedReport(error) => write!(f, "{error}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::Query(error) => write!(f, "{error}"),
            Self::Verification(failure) => write!(
                f,
                "intent arbitration bundle failed coherence verification: {:?}",
                failure.mismatches()
            ),
            Self::MissingPolicyRow(case) => {
                write!(f, "missing policy arbitration row for {case:?}")
            }
            Self::MissingChosenRow(case) => {
                write!(f, "missing chosen arbitration row for {case:?}")
            }
            Self::MissingPreservedRow(case) => {
                write!(f, "missing preserved arbitration row for {case:?}")
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionIntentArbitrationReportBundleError {}

pub fn prepare_primitive_construction_intent_arbitration_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionIntentArbitrationBundleCase,
) -> Result<
    PrimitiveConstructionVerifiedIntentArbitrationReportBundle,
    PrimitiveConstructionIntentArbitrationReportBundleError,
> {
    let policy_report = prepare_primitive_intent_arbitration_policy_report()
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::PolicyReport)?;
    let dx_surface_report = prepare_primitive_intent_conflict_dx_surface_report()
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::PolicyReport)?;
    let chosen_report = prepare_primitive_chosen_intent_resolution_report()
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::ChosenReport)?;
    let preserved_report = prepare_primitive_construction_preserved_intent_resolution_report()
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::PreservedReport)?;
    prepare_bundle_from_reports(
        workspace,
        case,
        &policy_report,
        &dx_surface_report,
        &chosen_report,
        &preserved_report,
    )
}

fn prepare_bundle_from_reports(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionIntentArbitrationBundleCase,
    policy_report: &PrimitiveConstructionIntentArbitrationPolicyReport,
    dx_surface_report: &PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    chosen_report: &PrimitiveConstructionChosenIntentResolutionReport,
    preserved_report: &PrimitiveConstructionPreservedIntentResolutionReport,
) -> Result<
    PrimitiveConstructionVerifiedIntentArbitrationReportBundle,
    PrimitiveConstructionIntentArbitrationReportBundleError,
> {
    let policy_row = policy_report
        .row(case.policy_case())
        .ok_or(PrimitiveConstructionIntentArbitrationReportBundleError::MissingPolicyRow(case))?
        .clone();
    let chosen_row = match case.chosen_case() {
        Some(chosen_case) => Some(
            chosen_report
                .row(chosen_case)
                .ok_or(
                    PrimitiveConstructionIntentArbitrationReportBundleError::MissingChosenRow(case),
                )?
                .clone(),
        ),
        None => None,
    };
    let preserved_row = preserved_report
        .row(case.preserved_case())
        .ok_or(PrimitiveConstructionIntentArbitrationReportBundleError::MissingPreservedRow(case))?
        .clone();
    let replay_parity_report =
        prepare_primitive_construction_intent_arbitration_replay_parity_report(
            case.preserved_case(),
        )
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::Replay)?;
    let query_inspection_parity_report =
        prepare_primitive_construction_query_intent_arbitration_inspection_parity_report(
            workspace,
            policy_row.clone(),
            chosen_row.clone(),
        )
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::Query)?;
    let query_projection_receipt_report =
        prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report(
            workspace,
            policy_row.clone(),
            chosen_row.clone(),
        )
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::Query)?;
    verify_bundle(PrimitiveConstructionIntentArbitrationReportBundle::new(
        case,
        policy_row,
        chosen_row,
        preserved_row,
        dx_surface_report.clone(),
        replay_parity_report,
        query_inspection_parity_report,
        query_projection_receipt_report,
    ))
    .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::Verification)
}
