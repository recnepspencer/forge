use crate::construction::digest::digest_owned_parts;
use worth_spatial::facade::{
    SpatialAuthoredActKind, SpatialBlockedCapability, SpatialIntentCandidate,
    SpatialIntentEscalation,
};

use super::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_intent_arbitration_policy_report,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyRow, PrimitiveConstructionObservedIntentRelation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreservedIntentResolutionCase {
    PolicyMoveOnly,
    FrameAlignedPolicy,
    GrazingClarificationRequired,
    ExplicitSnapFlush,
    ExplicitMoveOnlyWithBlockedMerge,
    HostPenetrationBlockedCut,
    OverlapAdvancedClarification,
}

impl PrimitiveConstructionPreservedIntentResolutionCase {
    fn policy_case(&self) -> PrimitiveConstructionIntentArbitrationPolicyCase {
        match self {
            Self::PolicyMoveOnly => {
                PrimitiveConstructionIntentArbitrationPolicyCase::DirectMoveOnly
            }
            Self::FrameAlignedPolicy => {
                PrimitiveConstructionIntentArbitrationPolicyCase::FrameAlignedIntent
            }
            Self::GrazingClarificationRequired | Self::ExplicitSnapFlush => {
                PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict
            }
            Self::ExplicitMoveOnlyWithBlockedMerge => {
                PrimitiveConstructionIntentArbitrationPolicyCase::OverlapBlockedCandidates
            }
            Self::HostPenetrationBlockedCut => {
                PrimitiveConstructionIntentArbitrationPolicyCase::HostPenetrationBlockedCut
            }
            Self::OverlapAdvancedClarification => {
                PrimitiveConstructionIntentArbitrationPolicyCase::OverlapAdvancedCapabilities
            }
        }
    }

    fn chosen_case(&self) -> Option<PrimitiveConstructionChosenIntentResolutionCase> {
        match self {
            Self::PolicyMoveOnly => {
                Some(PrimitiveConstructionChosenIntentResolutionCase::PolicyMoveOnly)
            }
            Self::FrameAlignedPolicy => None,
            Self::ExplicitSnapFlush => {
                Some(PrimitiveConstructionChosenIntentResolutionCase::ExplicitSnapFlush)
            }
            Self::ExplicitMoveOnlyWithBlockedMerge => Some(
                PrimitiveConstructionChosenIntentResolutionCase::ExplicitMoveOnlyWithBlockedMerge,
            ),
            Self::GrazingClarificationRequired
            | Self::HostPenetrationBlockedCut
            | Self::OverlapAdvancedClarification => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreservedIntentTruth {
    Unresolved {
        escalation: SpatialIntentEscalation,
        blocked_capability: Option<SpatialBlockedCapability>,
    },
    Resolved {
        candidate: SpatialIntentCandidate,
        authority: PrimitiveConstructionChosenIntentResolutionAuthority,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreservedIntentResolutionRow {
    case: PrimitiveConstructionPreservedIntentResolutionCase,
    authored_act: SpatialAuthoredActKind,
    observed_relations: Vec<PrimitiveConstructionObservedIntentRelation>,
    conflict_class: super::PrimitiveConstructionIntentArbitrationConflictClass,
    escalation: SpatialIntentEscalation,
    candidates: Vec<SpatialIntentCandidate>,
    blocked_candidates: Vec<(SpatialIntentCandidate, SpatialBlockedCapability)>,
    preserved_truth: PrimitiveConstructionPreservedIntentTruth,
    row_digest: String,
}

impl PrimitiveConstructionPreservedIntentResolutionRow {
    fn new(
        case: PrimitiveConstructionPreservedIntentResolutionCase,
        policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
        preserved_truth: PrimitiveConstructionPreservedIntentTruth,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            policy_row.authored_act().as_str().to_string(),
            format!("{:?}", policy_row.observed_relations()),
            format!("{:?}", policy_row.conflict_class()),
            format!("{:?}", policy_row.escalation()),
            format!("{:?}", policy_row.candidates()),
            format!("{:?}", policy_row.blocked_candidates()),
            format!("{preserved_truth:?}"),
        ]);
        Self {
            case,
            authored_act: policy_row.authored_act(),
            observed_relations: policy_row.observed_relations().to_vec(),
            conflict_class: policy_row.conflict_class(),
            escalation: policy_row.escalation(),
            candidates: policy_row.candidates().to_vec(),
            blocked_candidates: policy_row.blocked_candidates().to_vec(),
            preserved_truth,
            row_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionPreservedIntentResolutionCase {
        self.case
    }

    pub fn authored_act(&self) -> SpatialAuthoredActKind {
        self.authored_act
    }

    pub fn observed_relations(&self) -> &[PrimitiveConstructionObservedIntentRelation] {
        &self.observed_relations
    }

    pub fn conflict_class(&self) -> super::PrimitiveConstructionIntentArbitrationConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }

    pub fn candidates(&self) -> &[SpatialIntentCandidate] {
        &self.candidates
    }

    pub fn blocked_candidates(&self) -> &[(SpatialIntentCandidate, SpatialBlockedCapability)] {
        &self.blocked_candidates
    }

    pub fn preserved_truth(&self) -> PrimitiveConstructionPreservedIntentTruth {
        self.preserved_truth
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreservedIntentResolutionReport {
    rows: Vec<PrimitiveConstructionPreservedIntentResolutionRow>,
    report_digest: String,
}

impl PrimitiveConstructionPreservedIntentResolutionReport {
    fn new(rows: Vec<PrimitiveConstructionPreservedIntentResolutionRow>) -> Self {
        let report_digest = digest_owned_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionPreservedIntentResolutionRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionPreservedIntentResolutionCase,
    ) -> Option<&PrimitiveConstructionPreservedIntentResolutionRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPreservedIntentResolutionReportError {
    PolicyReport(super::PrimitiveConstructionIntentArbitrationPolicyReportError),
    ChosenReport(super::PrimitiveConstructionChosenIntentResolutionReportError),
    MissingPolicyRow(PrimitiveConstructionPreservedIntentResolutionCase),
    MissingChosenRow(PrimitiveConstructionPreservedIntentResolutionCase),
    ChosenRowMismatch(PrimitiveConstructionPreservedIntentResolutionCase),
}

impl std::fmt::Display for PrimitiveConstructionPreservedIntentResolutionReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PolicyReport(error) => write!(f, "{error}"),
            Self::ChosenReport(error) => write!(f, "{error}"),
            Self::MissingPolicyRow(case) => {
                write!(f, "missing preserved intent policy row for {case:?}")
            }
            Self::MissingChosenRow(case) => {
                write!(f, "missing preserved intent chosen row for {case:?}")
            }
            Self::ChosenRowMismatch(case) => {
                write!(
                    f,
                    "chosen row does not match preserved intent policy row for {case:?}"
                )
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionPreservedIntentResolutionReportError {}

pub fn prepare_primitive_construction_preserved_intent_resolution_report() -> Result<
    PrimitiveConstructionPreservedIntentResolutionReport,
    PrimitiveConstructionPreservedIntentResolutionReportError,
> {
    let policy_report = prepare_primitive_intent_arbitration_policy_report()
        .map_err(PrimitiveConstructionPreservedIntentResolutionReportError::PolicyReport)?;
    let chosen_report = prepare_primitive_chosen_intent_resolution_report()
        .map_err(PrimitiveConstructionPreservedIntentResolutionReportError::ChosenReport)?;

    let rows = [
        PrimitiveConstructionPreservedIntentResolutionCase::PolicyMoveOnly,
        PrimitiveConstructionPreservedIntentResolutionCase::FrameAlignedPolicy,
        PrimitiveConstructionPreservedIntentResolutionCase::GrazingClarificationRequired,
        PrimitiveConstructionPreservedIntentResolutionCase::ExplicitSnapFlush,
        PrimitiveConstructionPreservedIntentResolutionCase::ExplicitMoveOnlyWithBlockedMerge,
        PrimitiveConstructionPreservedIntentResolutionCase::HostPenetrationBlockedCut,
        PrimitiveConstructionPreservedIntentResolutionCase::OverlapAdvancedClarification,
    ]
    .into_iter()
    .map(|case| build_row(case, &policy_report, &chosen_report))
    .collect::<Result<Vec<_>, _>>()?;

    Ok(PrimitiveConstructionPreservedIntentResolutionReport::new(
        rows,
    ))
}

fn build_row(
    case: PrimitiveConstructionPreservedIntentResolutionCase,
    policy_report: &super::PrimitiveConstructionIntentArbitrationPolicyReport,
    chosen_report: &super::PrimitiveConstructionChosenIntentResolutionReport,
) -> Result<
    PrimitiveConstructionPreservedIntentResolutionRow,
    PrimitiveConstructionPreservedIntentResolutionReportError,
> {
    let policy_row = policy_report
        .row(case.policy_case())
        .ok_or(PrimitiveConstructionPreservedIntentResolutionReportError::MissingPolicyRow(case))?
        .clone();
    let preserved_truth = match case.chosen_case() {
        Some(chosen_case) => {
            let chosen_row = chosen_report.row(chosen_case).ok_or(
                PrimitiveConstructionPreservedIntentResolutionReportError::MissingChosenRow(case),
            )?;
            if chosen_row.authored_act() != policy_row.authored_act()
                || chosen_row.observed_relations() != policy_row.observed_relations()
                || chosen_row.conflict_class() != policy_row.conflict_class()
                || !policy_row
                    .candidates()
                    .contains(&chosen_row.chosen_candidate())
            {
                return Err(
                    PrimitiveConstructionPreservedIntentResolutionReportError::ChosenRowMismatch(
                        case,
                    ),
                );
            }
            PrimitiveConstructionPreservedIntentTruth::Resolved {
                candidate: chosen_row.chosen_candidate(),
                authority: chosen_row.authority(),
            }
        }
        None => match policy_row.chosen_candidate() {
            Some(candidate) => PrimitiveConstructionPreservedIntentTruth::Resolved {
                candidate,
                authority: PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve,
            },
            None => PrimitiveConstructionPreservedIntentTruth::Unresolved {
                escalation: policy_row.escalation(),
                blocked_capability: match policy_row.escalation() {
                    SpatialIntentEscalation::BlockedByMissingCapability(blocked) => Some(blocked),
                    SpatialIntentEscalation::AutoResolve(_)
                    | SpatialIntentEscalation::PreserveCandidates
                    | SpatialIntentEscalation::AskForClarification => None,
                },
            },
        },
    };
    Ok(PrimitiveConstructionPreservedIntentResolutionRow::new(
        case,
        policy_row,
        preserved_truth,
    ))
}
