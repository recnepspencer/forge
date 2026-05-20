use crate::construction::digest::digest_owned_parts;
use crate::spatial_intent::{
    analyze_primitive_intent_conflict, analyze_primitive_intent_conflict_with_capabilities,
    resolve_primitive_intent_conflict_by_choice, resolve_primitive_intent_conflict_by_policy,
};
use worth_spatial::facade::{
    SpatialAuthoredActKind, SpatialChosenIntentAuthority, SpatialChosenIntentResolution,
    SpatialIntentCandidate, SpatialIntentCapabilitySet, SpatialIntentConflictClass,
    SpatialIntentResolutionError, SpatialObservedRelationFact,
};

use super::{
    PrimitiveConstructionIntentArbitrationConflictClass,
    PrimitiveConstructionObservedIntentRelation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionChosenIntentResolutionCase {
    PolicyMoveOnly,
    ExplicitSnapFlush,
    ExplicitMoveOnlyWithBlockedMerge,
    ExplicitNestInsideWithBlockedMerge,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionChosenIntentResolutionAuthority {
    PolicyAutoResolve,
    ExplicitChoice,
}

impl From<SpatialChosenIntentAuthority> for PrimitiveConstructionChosenIntentResolutionAuthority {
    fn from(value: SpatialChosenIntentAuthority) -> Self {
        match value {
            SpatialChosenIntentAuthority::PolicyAutoResolve => Self::PolicyAutoResolve,
            SpatialChosenIntentAuthority::ExplicitChoice => Self::ExplicitChoice,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionChosenIntentResolutionRow {
    case: PrimitiveConstructionChosenIntentResolutionCase,
    authored_act: SpatialAuthoredActKind,
    observed_relations: Vec<PrimitiveConstructionObservedIntentRelation>,
    conflict_class: PrimitiveConstructionIntentArbitrationConflictClass,
    chosen_candidate: SpatialIntentCandidate,
    authority: PrimitiveConstructionChosenIntentResolutionAuthority,
    row_digest: String,
}

impl PrimitiveConstructionChosenIntentResolutionRow {
    fn new(
        case: PrimitiveConstructionChosenIntentResolutionCase,
        resolution: SpatialChosenIntentResolution,
    ) -> Self {
        let analysis = resolution.analysis();
        let observed_relations = analysis
            .observed_relation_facts()
            .iter()
            .copied()
            .map(PrimitiveConstructionObservedIntentRelation::from)
            .collect::<Vec<_>>();
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            analysis.authored_act().as_str().to_string(),
            format!("{observed_relations:?}"),
            format!("{:?}", analysis.conflict_class()),
            resolution.chosen_candidate().as_str().to_string(),
            format!("{:?}", resolution.authority()),
        ]);
        Self {
            case,
            authored_act: analysis.authored_act(),
            observed_relations,
            conflict_class: match analysis.conflict_class() {
                SpatialIntentConflictClass::SingleClearIntent => {
                    PrimitiveConstructionIntentArbitrationConflictClass::SingleClearIntent
                }
                SpatialIntentConflictClass::MultiplePlausibleIntents => {
                    PrimitiveConstructionIntentArbitrationConflictClass::MultiplePlausibleIntents
                }
                SpatialIntentConflictClass::UnsafeToAssume => {
                    PrimitiveConstructionIntentArbitrationConflictClass::UnsafeToAssume
                }
                SpatialIntentConflictClass::BlockedCandidateSet => {
                    PrimitiveConstructionIntentArbitrationConflictClass::BlockedCandidateSet
                }
            },
            chosen_candidate: resolution.chosen_candidate(),
            authority: resolution.authority().into(),
            row_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionChosenIntentResolutionCase {
        self.case
    }

    pub fn authored_act(&self) -> SpatialAuthoredActKind {
        self.authored_act
    }

    pub fn observed_relations(&self) -> &[PrimitiveConstructionObservedIntentRelation] {
        &self.observed_relations
    }

    pub fn conflict_class(&self) -> PrimitiveConstructionIntentArbitrationConflictClass {
        self.conflict_class
    }

    pub fn chosen_candidate(&self) -> SpatialIntentCandidate {
        self.chosen_candidate
    }

    pub fn authority(&self) -> PrimitiveConstructionChosenIntentResolutionAuthority {
        self.authority
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionChosenIntentResolutionReport {
    rows: Vec<PrimitiveConstructionChosenIntentResolutionRow>,
    report_digest: String,
}

impl PrimitiveConstructionChosenIntentResolutionReport {
    fn new(rows: Vec<PrimitiveConstructionChosenIntentResolutionRow>) -> Self {
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

    pub fn rows(&self) -> &[PrimitiveConstructionChosenIntentResolutionRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionChosenIntentResolutionCase,
    ) -> Option<&PrimitiveConstructionChosenIntentResolutionRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionChosenIntentResolutionReportError {
    Resolution(SpatialIntentResolutionError),
}

impl std::fmt::Display for PrimitiveConstructionChosenIntentResolutionReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resolution(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionChosenIntentResolutionReportError {}

pub fn prepare_primitive_chosen_intent_resolution_report() -> Result<
    PrimitiveConstructionChosenIntentResolutionReport,
    PrimitiveConstructionChosenIntentResolutionReportError,
> {
    Ok(PrimitiveConstructionChosenIntentResolutionReport::new(
        vec![
            PrimitiveConstructionChosenIntentResolutionRow::new(
                PrimitiveConstructionChosenIntentResolutionCase::PolicyMoveOnly,
                resolve_primitive_intent_conflict_by_policy(analyze_primitive_intent_conflict(
                    SpatialAuthoredActKind::Move,
                    &[],
                ))
                .map_err(PrimitiveConstructionChosenIntentResolutionReportError::Resolution)?,
            ),
            PrimitiveConstructionChosenIntentResolutionRow::new(
                PrimitiveConstructionChosenIntentResolutionCase::ExplicitSnapFlush,
                resolve_primitive_intent_conflict_by_choice(
                    analyze_primitive_intent_conflict(
                        SpatialAuthoredActKind::Move,
                        &[SpatialObservedRelationFact::GrazingContact],
                    ),
                    SpatialIntentCandidate::SnapFlush,
                )
                .map_err(PrimitiveConstructionChosenIntentResolutionReportError::Resolution)?,
            ),
            PrimitiveConstructionChosenIntentResolutionRow::new(
                PrimitiveConstructionChosenIntentResolutionCase::ExplicitMoveOnlyWithBlockedMerge,
                resolve_primitive_intent_conflict_by_choice(
                    analyze_primitive_intent_conflict(
                        SpatialAuthoredActKind::Move,
                        &[SpatialObservedRelationFact::Overlap],
                    ),
                    SpatialIntentCandidate::MoveOnly,
                )
                .map_err(PrimitiveConstructionChosenIntentResolutionReportError::Resolution)?,
            ),
            PrimitiveConstructionChosenIntentResolutionRow::new(
                PrimitiveConstructionChosenIntentResolutionCase::ExplicitNestInsideWithBlockedMerge,
                resolve_primitive_intent_conflict_by_choice(
                    analyze_primitive_intent_conflict_with_capabilities(
                        SpatialAuthoredActKind::Move,
                        &[
                            SpatialObservedRelationFact::Overlap,
                            SpatialObservedRelationFact::InsideTarget,
                        ],
                        SpatialIntentCapabilitySet::blocked_defaults(),
                    ),
                    SpatialIntentCandidate::NestInside,
                )
                .map_err(PrimitiveConstructionChosenIntentResolutionReportError::Resolution)?,
            ),
        ],
    ))
}
