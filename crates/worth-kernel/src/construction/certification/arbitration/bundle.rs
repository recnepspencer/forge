use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::digest::digest_owned_parts;
use crate::construction::query::{
    prepare_primitive_construction_query_intent_arbitration_inspection_parity_report,
    prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report,
    PrimitiveConstructionQueryIntentArbitrationParityError,
    PrimitiveConstructionQueryIntentArbitrationParityReport,
};
use crate::construction::{
    prepare_primitive_construction_intent_arbitration_replay_parity_report,
    PrimitiveConstructionIntentArbitrationReplayParityError,
    PrimitiveConstructionIntentArbitrationReplayParityReport,
};

use super::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionChosenIntentResolutionReport,
    PrimitiveConstructionChosenIntentResolutionRow,
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionIntentArbitrationPolicyRow,
    PrimitiveConstructionPreservedIntentResolutionCase,
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
    fn policy_case(&self) -> PrimitiveConstructionIntentArbitrationPolicyCase {
        match self {
            Self::DirectMoveOnlyPolicy => {
                PrimitiveConstructionIntentArbitrationPolicyCase::DirectMoveOnly
            }
            Self::GrazingSnapExplicitChoice => {
                PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict
            }
            Self::OverlapMoveOnlyWithBlockedMerge => {
                PrimitiveConstructionIntentArbitrationPolicyCase::OverlapBlockedCandidates
            }
            Self::HostPenetrationBlockedCut => {
                PrimitiveConstructionIntentArbitrationPolicyCase::HostPenetrationBlockedCut
            }
            Self::FrameAlignedIntent => {
                PrimitiveConstructionIntentArbitrationPolicyCase::FrameAlignedIntent
            }
            Self::OverlapAdvancedCapabilities => {
                PrimitiveConstructionIntentArbitrationPolicyCase::OverlapAdvancedCapabilities
            }
        }
    }

    fn chosen_case(&self) -> Option<PrimitiveConstructionChosenIntentResolutionCase> {
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

    fn preserved_case(&self) -> PrimitiveConstructionPreservedIntentResolutionCase {
        match self {
            Self::DirectMoveOnlyPolicy => {
                PrimitiveConstructionPreservedIntentResolutionCase::PolicyMoveOnly
            }
            Self::FrameAlignedIntent => {
                PrimitiveConstructionPreservedIntentResolutionCase::FrameAlignedPolicy
            }
            Self::GrazingSnapExplicitChoice => {
                PrimitiveConstructionPreservedIntentResolutionCase::ExplicitSnapFlush
            }
            Self::OverlapMoveOnlyWithBlockedMerge => {
                PrimitiveConstructionPreservedIntentResolutionCase::ExplicitMoveOnlyWithBlockedMerge
            }
            Self::HostPenetrationBlockedCut => {
                PrimitiveConstructionPreservedIntentResolutionCase::HostPenetrationBlockedCut
            }
            Self::OverlapAdvancedCapabilities => {
                PrimitiveConstructionPreservedIntentResolutionCase::OverlapAdvancedClarification
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationReportBundle {
    case: PrimitiveConstructionIntentArbitrationBundleCase,
    policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
    chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
    dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    replay_parity_report: PrimitiveConstructionIntentArbitrationReplayParityReport,
    query_inspection_parity_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    query_projection_receipt_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    bundle_verified: bool,
    bundle_digest: String,
}

impl PrimitiveConstructionIntentArbitrationReportBundle {
    fn new(
        case: PrimitiveConstructionIntentArbitrationBundleCase,
        policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
        chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
        dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
        replay_parity_report: PrimitiveConstructionIntentArbitrationReplayParityReport,
        query_inspection_parity_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
        query_projection_receipt_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    ) -> Result<Self, PrimitiveConstructionIntentArbitrationReportBundleError> {
        let dx_row = dx_surface_report
            .row(case.policy_case())
            .ok_or(PrimitiveConstructionIntentArbitrationReportBundleError::MissingDxRow(case))?;
        let chosen_matches = match chosen_row.as_ref() {
            Some(row) => {
                row.authored_act() == policy_row.authored_act()
                    && row.observed_relations() == policy_row.observed_relations()
                    && row.conflict_class() == policy_row.conflict_class()
                    && policy_row.candidates().contains(&row.chosen_candidate())
            }
            None => true,
        };
        let dx_matches = dx_row.conflict_class() == policy_row.conflict_class()
            && dx_row.escalation() == policy_row.escalation()
            && dx_row.candidate_count() == policy_row.candidates().len()
            && dx_row.blocked_candidate_count() == policy_row.blocked_candidates().len()
            && dx_row.chosen_candidate() == policy_row.chosen_candidate();
        let query_matches = query_inspection_parity_report.parity_verified()
            && query_projection_receipt_report.parity_verified()
            && query_inspection_parity_report.authored_act() == policy_row.authored_act()
            && query_projection_receipt_report.authored_act() == policy_row.authored_act()
            && query_inspection_parity_report.observed_relations()
                == policy_row.observed_relations()
            && query_projection_receipt_report.observed_relations()
                == policy_row.observed_relations()
            && query_inspection_parity_report.conflict_class() == policy_row.conflict_class()
            && query_projection_receipt_report.conflict_class() == policy_row.conflict_class()
            && query_inspection_parity_report.escalation() == policy_row.escalation()
            && query_projection_receipt_report.escalation() == policy_row.escalation();
        let replay_matches = replay_parity_report.parity_verified()
            && replay_parity_report.direct_row().authored_act() == policy_row.authored_act()
            && replay_parity_report.direct_row().observed_relations()
                == policy_row.observed_relations()
            && replay_parity_report.direct_row().conflict_class() == policy_row.conflict_class()
            && replay_parity_report.direct_row().escalation() == policy_row.escalation()
            && replay_parity_report.direct_row().candidates() == policy_row.candidates()
            && replay_parity_report.direct_row().blocked_candidates()
                == policy_row.blocked_candidates();
        let bundle_verified = chosen_matches && dx_matches && replay_matches && query_matches;
        let bundle_digest = digest_owned_parts(&[
            format!("{case:?}"),
            policy_row.row_digest().to_string(),
            dx_surface_report.report_digest().to_string(),
            replay_parity_report.report_digest().to_string(),
            query_inspection_parity_report.report_digest().to_string(),
            query_projection_receipt_report.report_digest().to_string(),
            chosen_row
                .as_ref()
                .map(|row| row.row_digest().to_string())
                .unwrap_or_else(|| "unresolved".to_string()),
            bundle_verified.to_string(),
        ]);
        Ok(Self {
            case,
            policy_row,
            chosen_row,
            dx_surface_report,
            replay_parity_report,
            query_inspection_parity_report,
            query_projection_receipt_report,
            bundle_verified,
            bundle_digest,
        })
    }

    pub fn case(&self) -> PrimitiveConstructionIntentArbitrationBundleCase {
        self.case
    }

    pub fn policy_row(&self) -> &PrimitiveConstructionIntentArbitrationPolicyRow {
        &self.policy_row
    }

    pub fn chosen_row(&self) -> Option<&PrimitiveConstructionChosenIntentResolutionRow> {
        self.chosen_row.as_ref()
    }

    pub fn dx_surface_report(&self) -> &PrimitiveConstructionIntentArbitrationDxSurfaceReport {
        &self.dx_surface_report
    }

    pub fn replay_parity_report(
        &self,
    ) -> &PrimitiveConstructionIntentArbitrationReplayParityReport {
        &self.replay_parity_report
    }

    pub fn query_inspection_parity_report(
        &self,
    ) -> &PrimitiveConstructionQueryIntentArbitrationParityReport {
        &self.query_inspection_parity_report
    }

    pub fn query_projection_receipt_report(
        &self,
    ) -> &PrimitiveConstructionQueryIntentArbitrationParityReport {
        &self.query_projection_receipt_report
    }

    pub fn bundle_verified(&self) -> bool {
        self.bundle_verified
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionIntentArbitrationReportBundleError {
    PolicyReport(super::PrimitiveConstructionIntentArbitrationPolicyReportError),
    ChosenReport(super::PrimitiveConstructionChosenIntentResolutionReportError),
    Replay(PrimitiveConstructionIntentArbitrationReplayParityError),
    Query(PrimitiveConstructionQueryIntentArbitrationParityError),
    MissingPolicyRow(PrimitiveConstructionIntentArbitrationBundleCase),
    MissingDxRow(PrimitiveConstructionIntentArbitrationBundleCase),
    MissingChosenRow(PrimitiveConstructionIntentArbitrationBundleCase),
}

impl std::fmt::Display for PrimitiveConstructionIntentArbitrationReportBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PolicyReport(error) => write!(f, "{error}"),
            Self::ChosenReport(error) => write!(f, "{error}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::Query(error) => write!(f, "{error}"),
            Self::MissingPolicyRow(case) => {
                write!(f, "missing policy arbitration row for {case:?}")
            }
            Self::MissingDxRow(case) => write!(f, "missing dx arbitration row for {case:?}"),
            Self::MissingChosenRow(case) => {
                write!(f, "missing chosen arbitration row for {case:?}")
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionIntentArbitrationReportBundleError {}

pub fn prepare_primitive_construction_intent_arbitration_report_bundle(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionIntentArbitrationBundleCase,
) -> Result<
    PrimitiveConstructionIntentArbitrationReportBundle,
    PrimitiveConstructionIntentArbitrationReportBundleError,
> {
    let policy_report = prepare_primitive_intent_arbitration_policy_report()
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::PolicyReport)?;
    let dx_surface_report = prepare_primitive_intent_conflict_dx_surface_report()
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::PolicyReport)?;
    let chosen_report = prepare_primitive_chosen_intent_resolution_report()
        .map_err(PrimitiveConstructionIntentArbitrationReportBundleError::ChosenReport)?;
    prepare_bundle_from_reports(
        workspace,
        case,
        &policy_report,
        &dx_surface_report,
        &chosen_report,
    )
}

fn prepare_bundle_from_reports(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionIntentArbitrationBundleCase,
    policy_report: &PrimitiveConstructionIntentArbitrationPolicyReport,
    dx_surface_report: &PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    chosen_report: &PrimitiveConstructionChosenIntentResolutionReport,
) -> Result<
    PrimitiveConstructionIntentArbitrationReportBundle,
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
    PrimitiveConstructionIntentArbitrationReportBundle::new(
        case,
        policy_row,
        chosen_row,
        dx_surface_report.clone(),
        replay_parity_report,
        query_inspection_parity_report,
        query_projection_receipt_report,
    )
}
