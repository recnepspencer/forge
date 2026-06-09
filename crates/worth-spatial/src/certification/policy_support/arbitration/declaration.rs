use forge_query::facade::{
    forge_query_intent_admission_support_matrix,
    forge_query_intent_admission_support_traceability_report,
    ForgeQueryAuthoritativeMutationEvidenceSupport, ForgeQueryGraphCompositionCapabilitySupportRow,
    ForgeQueryGraphCompositionInvariantPackViolation, ForgeQueryIntentAdmissionSupportMatrix,
    ForgeQueryIntentAdmissionSupportTraceabilityReport, ForgeQueryRuntimeSupportProfile,
};

use crate::certification::policy_support::SpatialArbitrationPolicyProfile;

use super::candidates::SpatialArbitrationCandidate;
use super::capabilities::{
    SpatialArbitrationCandidateAvailability, SpatialArbitrationCapabilitySummary,
    SpatialBlockedCapability,
};
use super::facts::{SpatialAuthoredActKind, SpatialObservedRelationFact};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationConflictClass {
    SingleClearIntent,
    MultiplePlausibleIntents,
    UnsafeToAssume,
    BlockedCandidateSet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationExplanationClass {
    AuthoredBaseline,
    RelationInferred,
    BlockedFutureCapability,
    UnsafeBoundary,
    PolicyPreferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationEscalation {
    AutoResolve(SpatialArbitrationCandidate),
    PreserveCandidates,
    AskForClarification,
    BlockedByMissingCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationPreviewHint {
    AutoResolve(SpatialArbitrationCandidate),
    PreserveCandidates,
    ClarificationRequired,
    BlockedByCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationContinuityHint {
    IdentityPreserved(SpatialArbitrationCandidate),
    AnchorContinuityPreserved(SpatialArbitrationCandidate),
    IdentityReinterpreted(SpatialArbitrationCandidate),
    IdentitySplit(SpatialArbitrationCandidate),
    IdentityMerged(SpatialArbitrationCandidate),
    PendingChoice,
    BlockedPendingChoice(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialArbitrationCandidateRank {
    candidate: SpatialArbitrationCandidate,
    availability: SpatialArbitrationCandidateAvailability,
    explanation: SpatialArbitrationExplanationClass,
    priority: u8,
    is_baseline: bool,
    is_policy_preferred: bool,
}

impl SpatialArbitrationCandidateRank {
    pub fn new(
        candidate: SpatialArbitrationCandidate,
        availability: SpatialArbitrationCandidateAvailability,
        explanation: SpatialArbitrationExplanationClass,
        is_baseline: bool,
        is_policy_preferred: bool,
    ) -> Self {
        Self {
            candidate,
            availability,
            explanation,
            priority: candidate.default_priority(),
            is_baseline,
            is_policy_preferred,
        }
    }

    pub fn candidate(&self) -> SpatialArbitrationCandidate {
        self.candidate
    }

    pub fn availability(&self) -> SpatialArbitrationCandidateAvailability {
        self.availability
    }

    pub fn explanation(&self) -> SpatialArbitrationExplanationClass {
        self.explanation
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn is_baseline(&self) -> bool {
        self.is_baseline
    }

    pub fn is_policy_preferred(&self) -> bool {
        self.is_policy_preferred
    }

    pub fn blocked_capability(&self) -> Option<SpatialBlockedCapability> {
        match self.availability {
            SpatialArbitrationCandidateAvailability::Available => None,
            SpatialArbitrationCandidateAvailability::Blocked(capability) => Some(capability),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialArbitrationDeclaration {
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: Vec<SpatialObservedRelationFact>,
    candidates: Vec<SpatialArbitrationCandidateRank>,
    conflict_class: SpatialArbitrationConflictClass,
    escalation: SpatialArbitrationEscalation,
    chosen_candidate: Option<SpatialArbitrationCandidate>,
    policy_profile: SpatialArbitrationPolicyProfile,
    capability_summary: SpatialArbitrationCapabilitySummary,
}

impl SpatialArbitrationDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: Vec<SpatialObservedRelationFact>,
        candidates: Vec<SpatialArbitrationCandidateRank>,
        conflict_class: SpatialArbitrationConflictClass,
        escalation: SpatialArbitrationEscalation,
        chosen_candidate: Option<SpatialArbitrationCandidate>,
        policy_profile: SpatialArbitrationPolicyProfile,
        capability_summary: SpatialArbitrationCapabilitySummary,
    ) -> Self {
        Self {
            authored_act,
            observed_relation_facts,
            candidates,
            conflict_class,
            escalation,
            chosen_candidate,
            policy_profile,
            capability_summary,
        }
    }

    pub fn authored_act(&self) -> SpatialAuthoredActKind {
        self.authored_act
    }

    pub fn observed_relation_facts(&self) -> &[SpatialObservedRelationFact] {
        &self.observed_relation_facts
    }

    pub fn candidates(&self) -> &[SpatialArbitrationCandidateRank] {
        &self.candidates
    }

    pub fn conflict_class(&self) -> SpatialArbitrationConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialArbitrationEscalation {
        self.escalation
    }

    pub fn chosen_candidate(&self) -> Option<SpatialArbitrationCandidate> {
        self.chosen_candidate
    }

    pub fn policy_profile(&self) -> SpatialArbitrationPolicyProfile {
        self.policy_profile
    }

    pub fn policy_profile_name(&self) -> &'static str {
        self.policy_profile.name()
    }

    pub fn capability_summary(&self) -> &SpatialArbitrationCapabilitySummary {
        &self.capability_summary
    }

    pub fn query_support_matrix(&self) -> ForgeQueryIntentAdmissionSupportMatrix {
        let _ = self;
        forge_query_intent_admission_support_matrix()
    }

    pub fn query_support_traceability_report(
        &self,
    ) -> ForgeQueryIntentAdmissionSupportTraceabilityReport {
        let _ = self;
        forge_query_intent_admission_support_traceability_report()
    }

    pub fn graph_composition_capability_support_rows(
        &self,
    ) -> Vec<ForgeQueryGraphCompositionCapabilitySupportRow> {
        let support = ForgeQueryAuthoritativeMutationEvidenceSupport::derive(
            &ForgeQueryRuntimeSupportProfile::bridge_backed(
                "worth-spatial arbitration subscription activation",
                "worth-spatial arbitration preview basis",
                "worth-spatial arbitration inspection",
            ),
        );
        let relevant_families = graph_composition_capability_families(self);
        support
            .graph_composition_capability_support_rows()
            .iter()
            .filter(|row| {
                relevant_families
                    .iter()
                    .any(|family| *family == row.capability_family())
            })
            .cloned()
            .collect()
    }

    pub fn graph_composition_invariant_violations(
        &self,
    ) -> Vec<ForgeQueryGraphCompositionInvariantPackViolation> {
        self.candidates()
            .iter()
            .filter_map(|candidate| {
                candidate.blocked_capability().map(|capability| {
                    ForgeQueryGraphCompositionInvariantPackViolation::new(
                        invariant_family(capability),
                        format!(
                            "spatial arbitration candidate `{}` is blocked by missing capability `{}`",
                            candidate.candidate().as_str(),
                            capability.as_str()
                        ),
                    )
                })
            })
            .collect()
    }

    pub fn preview_hint(&self) -> SpatialArbitrationPreviewHint {
        match self.escalation {
            SpatialArbitrationEscalation::AutoResolve(candidate) => {
                SpatialArbitrationPreviewHint::AutoResolve(candidate)
            }
            SpatialArbitrationEscalation::PreserveCandidates => {
                SpatialArbitrationPreviewHint::PreserveCandidates
            }
            SpatialArbitrationEscalation::AskForClarification => {
                SpatialArbitrationPreviewHint::ClarificationRequired
            }
            SpatialArbitrationEscalation::BlockedByMissingCapability(capability) => {
                SpatialArbitrationPreviewHint::BlockedByCapability(capability)
            }
        }
    }

    pub fn continuity_hint(&self) -> SpatialArbitrationContinuityHint {
        match self.escalation {
            SpatialArbitrationEscalation::AutoResolve(
                candidate @ (SpatialArbitrationCandidate::MoveOnly
                | SpatialArbitrationCandidate::AlignFrames),
            ) => SpatialArbitrationContinuityHint::IdentityPreserved(candidate),
            SpatialArbitrationEscalation::AutoResolve(
                candidate @ SpatialArbitrationCandidate::SnapFlush,
            ) => SpatialArbitrationContinuityHint::AnchorContinuityPreserved(candidate),
            SpatialArbitrationEscalation::AutoResolve(
                candidate @ (SpatialArbitrationCandidate::AttachRelationally
                | SpatialArbitrationCandidate::NestInside),
            ) => SpatialArbitrationContinuityHint::IdentityReinterpreted(candidate),
            SpatialArbitrationEscalation::AutoResolve(
                candidate @ (SpatialArbitrationCandidate::SubtractCandidate
                | SpatialArbitrationCandidate::CutOpeningCandidate),
            ) => SpatialArbitrationContinuityHint::IdentitySplit(candidate),
            SpatialArbitrationEscalation::AutoResolve(
                candidate @ (SpatialArbitrationCandidate::MergeCandidate
                | SpatialArbitrationCandidate::JoinCandidate),
            ) => SpatialArbitrationContinuityHint::IdentityMerged(candidate),
            SpatialArbitrationEscalation::PreserveCandidates
            | SpatialArbitrationEscalation::AskForClarification => {
                SpatialArbitrationContinuityHint::PendingChoice
            }
            SpatialArbitrationEscalation::BlockedByMissingCapability(capability) => {
                SpatialArbitrationContinuityHint::BlockedPendingChoice(capability)
            }
        }
    }

    pub fn policy_preferred_candidate(&self) -> Option<SpatialArbitrationCandidate> {
        self.candidates
            .iter()
            .find(|candidate| candidate.is_policy_preferred())
            .map(SpatialArbitrationCandidateRank::candidate)
    }
}

pub type SpatialArbitrationAnalysis = SpatialArbitrationDeclaration;

fn graph_composition_capability_families(
    declaration: &SpatialArbitrationDeclaration,
) -> Vec<&'static str> {
    let mut families = Vec::new();
    if declaration.candidates().iter().any(|candidate| {
        matches!(
            candidate.candidate(),
            SpatialArbitrationCandidate::AttachRelationally
                | SpatialArbitrationCandidate::NestInside
                | SpatialArbitrationCandidate::MergeCandidate
                | SpatialArbitrationCandidate::SubtractCandidate
                | SpatialArbitrationCandidate::CutOpeningCandidate
                | SpatialArbitrationCandidate::JoinCandidate
        )
    }) {
        families.push("same_batch_entity_relation_identity_edges");
    }
    if matches!(
        declaration.escalation(),
        SpatialArbitrationEscalation::AutoResolve(_)
            | SpatialArbitrationEscalation::BlockedByMissingCapability(_)
    ) {
        families.push("mixed_existing_target_followup_mutation");
    }
    families
}

fn invariant_family(capability: SpatialBlockedCapability) -> &'static str {
    match capability {
        SpatialBlockedCapability::MergeBoolean => "worth_spatial.merge_boolean_capability",
        SpatialBlockedCapability::SubtractBoolean => "worth_spatial.subtract_boolean_capability",
        SpatialBlockedCapability::CutOpening => "worth_spatial.cut_opening_capability",
        SpatialBlockedCapability::Join => "worth_spatial.join_capability",
        SpatialBlockedCapability::HostAttach => "worth_spatial.host_attach_capability",
    }
}
