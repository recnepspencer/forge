use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::arbitration::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_construction_preserved_intent_resolution_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionChosenIntentResolutionReportError,
    PrimitiveConstructionChosenIntentResolutionRow,
    PrimitiveConstructionIntentArbitrationDxSurfaceRow,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
    PrimitiveConstructionIntentArbitrationPolicyRow,
    PrimitiveConstructionPreservedIntentResolutionCase,
    PrimitiveConstructionPreservedIntentResolutionReportError,
    PrimitiveConstructionPreservedIntentResolutionRow,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::intent_arbitration_replay::{
    prepare_primitive_construction_intent_arbitration_replay_parity_report,
    PrimitiveConstructionIntentArbitrationReplayParityError,
    PrimitiveConstructionIntentArbitrationReplayParityReport,
};
use crate::construction::query::intent_arbitration::{
    prepare_primitive_construction_query_intent_arbitration_inspection_parity_report,
    prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report,
    PrimitiveConstructionQueryIntentArbitrationParityError,
    PrimitiveConstructionQueryIntentArbitrationParityReport,
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
    pub(crate) fn policy_case(self) -> PrimitiveConstructionIntentArbitrationPolicyCase {
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

    fn chosen_case(self) -> Option<PrimitiveConstructionChosenIntentResolutionCase> {
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

    fn preserved_case(self) -> PrimitiveConstructionPreservedIntentResolutionCase {
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

pub(crate) fn required_arbitration_representative_cases(
) -> &'static [PrimitiveConstructionIntentArbitrationBundleCase] {
    &[
        PrimitiveConstructionIntentArbitrationBundleCase::DirectMoveOnlyPolicy,
        PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice,
        PrimitiveConstructionIntentArbitrationBundleCase::OverlapMoveOnlyWithBlockedMerge,
        PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut,
        PrimitiveConstructionIntentArbitrationBundleCase::FrameAlignedIntent,
        PrimitiveConstructionIntentArbitrationBundleCase::OverlapAdvancedCapabilities,
    ]
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationRepresentativeEvidence {
    case: PrimitiveConstructionIntentArbitrationBundleCase,
    policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
    chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
    preserved_row: PrimitiveConstructionPreservedIntentResolutionRow,
    dx_row: PrimitiveConstructionIntentArbitrationDxSurfaceRow,
    replay_report: PrimitiveConstructionIntentArbitrationReplayParityReport,
    inspection_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    projection_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionIntentArbitrationRepresentativeEvidence {
    fn new(
        case: PrimitiveConstructionIntentArbitrationBundleCase,
        policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
        chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
        preserved_row: PrimitiveConstructionPreservedIntentResolutionRow,
        dx_row: PrimitiveConstructionIntentArbitrationDxSurfaceRow,
        replay_report: PrimitiveConstructionIntentArbitrationReplayParityReport,
        inspection_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
        projection_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    ) -> Self {
        let parity_verified = replay_report.parity_verified()
            && inspection_report.parity_verified()
            && projection_report.parity_verified()
            && inspection_report.authored_act() == projection_report.authored_act()
            && inspection_report.observed_relations() == projection_report.observed_relations()
            && inspection_report.conflict_class() == projection_report.conflict_class()
            && inspection_report.escalation() == projection_report.escalation()
            && inspection_report.candidates() == projection_report.candidates()
            && inspection_report.blocked_candidates() == projection_report.blocked_candidates()
            && inspection_report.chosen_truth() == projection_report.chosen_truth()
            && preserved_row.authored_act() == policy_row.authored_act()
            && preserved_row.observed_relations() == policy_row.observed_relations()
            && preserved_row.conflict_class() == policy_row.conflict_class()
            && preserved_row.escalation() == policy_row.escalation()
            && preserved_row.candidates() == policy_row.candidates()
            && preserved_row.blocked_candidates() == policy_row.blocked_candidates()
            && preserved_row.conflict_class() == dx_row.conflict_class()
            && preserved_row.escalation() == dx_row.escalation()
            && preserved_row.candidates().len() == dx_row.candidate_count()
            && preserved_row.blocked_candidates().len() == dx_row.blocked_candidate_count();
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ArtifactIdentity,
            &[
                format!("{case:?}"),
                policy_row.row_digest().to_string(),
                chosen_row
                    .as_ref()
                    .map(|row| row.row_digest().to_string())
                    .unwrap_or_else(|| "unresolved".to_string()),
                preserved_row.row_digest().to_string(),
                format!("{:?}", dx_row.case()),
                format!("{:?}", dx_row.dx_surface()),
                format!("{:?}", dx_row.conflict_class()),
                format!("{:?}", dx_row.escalation()),
                dx_row.candidate_count().to_string(),
                dx_row.blocked_candidate_count().to_string(),
                format!("{:?}", dx_row.chosen_candidate()),
                replay_report.report_digest().to_string(),
                inspection_report.report_digest().to_string(),
                projection_report.report_digest().to_string(),
                parity_verified.to_string(),
            ],
        );
        Self {
            case,
            policy_row,
            chosen_row,
            preserved_row,
            dx_row,
            replay_report,
            inspection_report,
            projection_report,
            parity_verified,
            report_digest,
        }
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

    pub fn preserved_row(&self) -> &PrimitiveConstructionPreservedIntentResolutionRow {
        &self.preserved_row
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_intent_arbitration_representative_evidence(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionIntentArbitrationBundleCase,
) -> Result<
    PrimitiveConstructionIntentArbitrationRepresentativeEvidence,
    PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError,
> {
    let policy_report = prepare_primitive_intent_arbitration_policy_report()
        .map_err(PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::Policy)?;
    let dx_report = prepare_primitive_intent_conflict_dx_surface_report()
        .map_err(PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::Policy)?;
    let chosen_report = prepare_primitive_chosen_intent_resolution_report()
        .map_err(PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::Chosen)?;
    let preserved_report = prepare_primitive_construction_preserved_intent_resolution_report()
        .map_err(PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::Preserved)?;

    let policy_row = policy_report.row(case.policy_case()).cloned().ok_or(
        PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::MissingPolicy(case),
    )?;
    let chosen_row = match case.chosen_case() {
        Some(chosen_case) => Some(chosen_report.row(chosen_case).cloned().ok_or(
            PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::MissingChosen(case),
        )?),
        None => None,
    };
    let preserved_row = preserved_report.row(case.preserved_case()).cloned().ok_or(
        PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::MissingPreserved(case),
    )?;
    let dx_row = dx_report.row(case.policy_case()).cloned().ok_or(
        PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::MissingDx(case),
    )?;
    let replay_report = prepare_primitive_construction_intent_arbitration_replay_parity_report(
        case.preserved_case(),
    )
    .map_err(PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::Replay)?;
    let inspection_report =
        prepare_primitive_construction_query_intent_arbitration_inspection_parity_report(
            workspace,
            policy_row.clone(),
            chosen_row.clone(),
        )
        .map_err(PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_intent_arbitration_projection_consumption_receipt_report(
            workspace,
            policy_row.clone(),
            chosen_row.clone(),
        )
        .map_err(PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError::Projection)?;

    Ok(
        PrimitiveConstructionIntentArbitrationRepresentativeEvidence::new(
            case,
            policy_row,
            chosen_row,
            preserved_row,
            dx_row,
            replay_report,
            inspection_report,
            projection_report,
        ),
    )
}

#[derive(Debug)]
pub enum PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError {
    Policy(PrimitiveConstructionIntentArbitrationPolicyReportError),
    Chosen(PrimitiveConstructionChosenIntentResolutionReportError),
    Preserved(PrimitiveConstructionPreservedIntentResolutionReportError),
    Replay(PrimitiveConstructionIntentArbitrationReplayParityError),
    Inspection(PrimitiveConstructionQueryIntentArbitrationParityError),
    Projection(PrimitiveConstructionQueryIntentArbitrationParityError),
    MissingPolicy(PrimitiveConstructionIntentArbitrationBundleCase),
    MissingChosen(PrimitiveConstructionIntentArbitrationBundleCase),
    MissingPreserved(PrimitiveConstructionIntentArbitrationBundleCase),
    MissingDx(PrimitiveConstructionIntentArbitrationBundleCase),
}

impl std::fmt::Display for PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Policy(error) => write!(f, "{error}"),
            Self::Chosen(error) => write!(f, "{error}"),
            Self::Preserved(error) => write!(f, "{error}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::MissingPolicy(case) => write!(f, "missing arbitration policy row for {case:?}"),
            Self::MissingChosen(case) => write!(f, "missing arbitration chosen row for {case:?}"),
            Self::MissingPreserved(case) => {
                write!(f, "missing arbitration preserved row for {case:?}")
            }
            Self::MissingDx(case) => write!(f, "missing arbitration dx row for {case:?}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError {}
