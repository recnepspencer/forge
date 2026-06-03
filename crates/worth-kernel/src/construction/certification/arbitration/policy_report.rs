use crate::construction::digest::digest_owned_parts;
use crate::spatial_intent::PrimitiveIntentConflict;
use worth_spatial::facade::arbitration::{
    SpatialAuthoredActKind, SpatialBlockedCapability, SpatialIntentArbitrationAnalysis,
    SpatialIntentCandidate, SpatialIntentCandidateAvailability, SpatialIntentCapabilitySet,
    SpatialIntentConflictClass, SpatialIntentEscalation, SpatialObservedRelationFact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentArbitrationPolicyCase {
    DirectMoveOnly,
    GrazingSnapConflict,
    OverlapBlockedCandidates,
    HostPenetrationBlockedCut,
    FrameAlignedIntent,
    OverlapAdvancedCapabilities,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionObservedIntentRelation {
    Overlap,
    GrazingContact,
    FrameAligned,
    InsideTarget,
    HostFaceContact,
    HostPenetration,
}

impl From<SpatialObservedRelationFact> for PrimitiveConstructionObservedIntentRelation {
    fn from(value: SpatialObservedRelationFact) -> Self {
        match value {
            SpatialObservedRelationFact::Overlap => Self::Overlap,
            SpatialObservedRelationFact::GrazingContact => Self::GrazingContact,
            SpatialObservedRelationFact::FrameAligned => Self::FrameAligned,
            SpatialObservedRelationFact::InsideTarget => Self::InsideTarget,
            SpatialObservedRelationFact::HostFaceContact => Self::HostFaceContact,
            SpatialObservedRelationFact::HostPenetration => Self::HostPenetration,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentArbitrationConflictClass {
    SingleClearIntent,
    MultiplePlausibleIntents,
    UnsafeToAssume,
    BlockedCandidateSet,
}

impl From<SpatialIntentConflictClass> for PrimitiveConstructionIntentArbitrationConflictClass {
    fn from(value: SpatialIntentConflictClass) -> Self {
        match value {
            SpatialIntentConflictClass::SingleClearIntent => Self::SingleClearIntent,
            SpatialIntentConflictClass::MultiplePlausibleIntents => Self::MultiplePlausibleIntents,
            SpatialIntentConflictClass::UnsafeToAssume => Self::UnsafeToAssume,
            SpatialIntentConflictClass::BlockedCandidateSet => Self::BlockedCandidateSet,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationPolicyRow {
    case: PrimitiveConstructionIntentArbitrationPolicyCase,
    authored_act: SpatialAuthoredActKind,
    observed_relations: Vec<PrimitiveConstructionObservedIntentRelation>,
    conflict_class: PrimitiveConstructionIntentArbitrationConflictClass,
    escalation: SpatialIntentEscalation,
    chosen_candidate: Option<SpatialIntentCandidate>,
    candidates: Vec<SpatialIntentCandidate>,
    blocked_candidates: Vec<(SpatialIntentCandidate, SpatialBlockedCapability)>,
    row_digest: String,
}

impl PrimitiveConstructionIntentArbitrationPolicyRow {
    fn new(
        case: PrimitiveConstructionIntentArbitrationPolicyCase,
        analysis: SpatialIntentArbitrationAnalysis,
    ) -> Self {
        let observed_relations = analysis
            .observed_relation_facts()
            .iter()
            .copied()
            .map(PrimitiveConstructionObservedIntentRelation::from)
            .collect::<Vec<_>>();
        let candidates = analysis
            .candidates()
            .iter()
            .map(|candidate| candidate.candidate())
            .collect::<Vec<_>>();
        let blocked_candidates = analysis
            .candidates()
            .iter()
            .filter_map(|candidate| match candidate.availability() {
                SpatialIntentCandidateAvailability::Blocked(blocked) => {
                    Some((candidate.candidate(), blocked))
                }
                SpatialIntentCandidateAvailability::Available => None,
            })
            .collect::<Vec<_>>();
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            format!("{:?}", analysis.authored_act()),
            format!("{observed_relations:?}"),
            format!("{:?}", analysis.conflict_class()),
            format!("{:?}", analysis.escalation()),
            format!("{:?}", analysis.chosen_candidate()),
            format!("{candidates:?}"),
            format!("{blocked_candidates:?}"),
        ]);
        Self {
            case,
            authored_act: analysis.authored_act(),
            observed_relations,
            conflict_class: analysis.conflict_class().into(),
            escalation: analysis.escalation(),
            chosen_candidate: analysis.chosen_candidate(),
            candidates,
            blocked_candidates,
            row_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionIntentArbitrationPolicyCase {
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

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }

    pub fn chosen_candidate(&self) -> Option<SpatialIntentCandidate> {
        self.chosen_candidate
    }

    pub fn candidates(&self) -> &[SpatialIntentCandidate] {
        &self.candidates
    }

    pub fn blocked_candidates(&self) -> &[(SpatialIntentCandidate, SpatialBlockedCapability)] {
        &self.blocked_candidates
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationPolicyReport {
    rows: Vec<PrimitiveConstructionIntentArbitrationPolicyRow>,
    report_digest: String,
}

impl PrimitiveConstructionIntentArbitrationPolicyReport {
    fn new(rows: Vec<PrimitiveConstructionIntentArbitrationPolicyRow>) -> Self {
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

    pub fn rows(&self) -> &[PrimitiveConstructionIntentArbitrationPolicyRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionIntentArbitrationPolicyCase,
    ) -> Option<&PrimitiveConstructionIntentArbitrationPolicyRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionIntentArbitrationPolicyReportError {}

impl std::fmt::Display for PrimitiveConstructionIntentArbitrationPolicyReportError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

impl std::error::Error for PrimitiveConstructionIntentArbitrationPolicyReportError {}

pub fn prepare_primitive_intent_arbitration_policy_report() -> Result<
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
> {
    Ok(PrimitiveConstructionIntentArbitrationPolicyReport::new(
        vec![
            PrimitiveConstructionIntentArbitrationPolicyRow::new(
                PrimitiveConstructionIntentArbitrationPolicyCase::DirectMoveOnly,
                PrimitiveIntentConflict::analyze(SpatialAuthoredActKind::Move, &[])
                    .analysis()
                    .clone(),
            ),
            PrimitiveConstructionIntentArbitrationPolicyRow::new(
                PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict,
                PrimitiveIntentConflict::analyze(
                    SpatialAuthoredActKind::Move,
                    &[SpatialObservedRelationFact::GrazingContact],
                )
                .analysis()
                .clone(),
            ),
            PrimitiveConstructionIntentArbitrationPolicyRow::new(
                PrimitiveConstructionIntentArbitrationPolicyCase::OverlapBlockedCandidates,
                PrimitiveIntentConflict::analyze(
                    SpatialAuthoredActKind::Move,
                    &[SpatialObservedRelationFact::Overlap],
                )
                .analysis()
                .clone(),
            ),
            PrimitiveConstructionIntentArbitrationPolicyRow::new(
                PrimitiveConstructionIntentArbitrationPolicyCase::HostPenetrationBlockedCut,
                PrimitiveIntentConflict::analyze(
                    SpatialAuthoredActKind::Move,
                    &[SpatialObservedRelationFact::HostPenetration],
                )
                .analysis()
                .clone(),
            ),
            PrimitiveConstructionIntentArbitrationPolicyRow::new(
                PrimitiveConstructionIntentArbitrationPolicyCase::FrameAlignedIntent,
                PrimitiveIntentConflict::analyze(
                    SpatialAuthoredActKind::Align,
                    &[SpatialObservedRelationFact::FrameAligned],
                )
                .analysis()
                .clone(),
            ),
            PrimitiveConstructionIntentArbitrationPolicyRow::new(
                PrimitiveConstructionIntentArbitrationPolicyCase::OverlapAdvancedCapabilities,
                PrimitiveIntentConflict::analyze_with_capabilities(
                    SpatialAuthoredActKind::Move,
                    &[SpatialObservedRelationFact::Overlap],
                    SpatialIntentCapabilitySet::blocked_defaults()
                        .with_merge_boolean()
                        .with_subtract_boolean(),
                )
                .analysis()
                .clone(),
            ),
        ],
    ))
}
