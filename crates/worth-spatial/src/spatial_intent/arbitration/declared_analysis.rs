use serde_json::json;

use forge_query::facade::{
    admit_runtime_intent_request, forge_query_intent_admission_support_matrix,
    forge_query_intent_admission_support_traceability_report,
    ForgeQueryAuthoritativeMutationEvidenceSupport, ForgeQueryGraphCompositionCapabilitySupportRow,
    ForgeQueryGraphCompositionInvariantPackViolation, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentAdmissionSupportMatrix,
    ForgeQueryIntentAdmissionSupportTraceabilityReport, ForgeQueryIntentDeclaration,
    ForgeQueryIntentViolationDecision, ForgeQueryRawIntentAdmissionRequest,
    ForgeQueryRuntimeSupportProfile,
};

use crate::spatial_intent::policy::SpatialIntentPolicyProfile;

use super::candidates::SpatialIntentCandidate;
use super::capabilities::{
    SpatialBlockedCapability, SpatialIntentCandidateAvailability, SpatialIntentCapabilitySummary,
};
use super::facts::{SpatialAuthoredActKind, SpatialObservedRelationFact};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentConflictClass {
    SingleClearIntent,
    MultiplePlausibleIntents,
    UnsafeToAssume,
    BlockedCandidateSet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentExplanationClass {
    AuthoredBaseline,
    RelationInferred,
    BlockedFutureCapability,
    UnsafeBoundary,
    PolicyPreferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentEscalation {
    AutoResolve(SpatialIntentCandidate),
    PreserveCandidates,
    AskForClarification,
    BlockedByMissingCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationPreviewHint {
    AutoResolve(SpatialIntentCandidate),
    PreserveCandidates,
    ClarificationRequired,
    BlockedByCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialArbitrationContinuityHint {
    IdentityPreserved(SpatialIntentCandidate),
    AnchorContinuityPreserved(SpatialIntentCandidate),
    IdentityReinterpreted(SpatialIntentCandidate),
    IdentitySplit(SpatialIntentCandidate),
    IdentityMerged(SpatialIntentCandidate),
    PendingChoice,
    BlockedPendingChoice(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialIntentCandidateRank {
    candidate: SpatialIntentCandidate,
    availability: SpatialIntentCandidateAvailability,
    explanation: SpatialIntentExplanationClass,
    priority: u8,
    is_baseline: bool,
    is_policy_preferred: bool,
}

impl SpatialIntentCandidateRank {
    pub fn new(
        candidate: SpatialIntentCandidate,
        availability: SpatialIntentCandidateAvailability,
        explanation: SpatialIntentExplanationClass,
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

    pub fn candidate(&self) -> SpatialIntentCandidate {
        self.candidate
    }

    pub fn availability(&self) -> SpatialIntentCandidateAvailability {
        self.availability
    }

    pub fn explanation(&self) -> SpatialIntentExplanationClass {
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
            SpatialIntentCandidateAvailability::Available => None,
            SpatialIntentCandidateAvailability::Blocked(capability) => Some(capability),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialIntentArbitrationDeclaration {
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: Vec<SpatialObservedRelationFact>,
    candidates: Vec<SpatialIntentCandidateRank>,
    conflict_class: SpatialIntentConflictClass,
    escalation: SpatialIntentEscalation,
    chosen_candidate: Option<SpatialIntentCandidate>,
    policy_profile: SpatialIntentPolicyProfile,
    capability_summary: SpatialIntentCapabilitySummary,
}

impl SpatialIntentArbitrationDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        authored_act: SpatialAuthoredActKind,
        observed_relation_facts: Vec<SpatialObservedRelationFact>,
        candidates: Vec<SpatialIntentCandidateRank>,
        conflict_class: SpatialIntentConflictClass,
        escalation: SpatialIntentEscalation,
        chosen_candidate: Option<SpatialIntentCandidate>,
        policy_profile: SpatialIntentPolicyProfile,
        capability_summary: SpatialIntentCapabilitySummary,
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

    pub fn candidates(&self) -> &[SpatialIntentCandidateRank] {
        &self.candidates
    }

    pub fn conflict_class(&self) -> SpatialIntentConflictClass {
        self.conflict_class
    }

    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }

    pub fn chosen_candidate(&self) -> Option<SpatialIntentCandidate> {
        self.chosen_candidate
    }

    pub fn policy_profile(&self) -> SpatialIntentPolicyProfile {
        self.policy_profile
    }

    pub fn policy_profile_name(&self) -> &'static str {
        self.policy_profile.name()
    }

    pub fn capability_summary(&self) -> &SpatialIntentCapabilitySummary {
        &self.capability_summary
    }

    pub fn to_query_intent_declaration(&self) -> ForgeQueryIntentDeclaration {
        ForgeQueryIntentDeclaration::strategy_commit(
            "worth.spatial.arbitration".to_string(),
            "worth.spatial.arbitration.query_handoff",
            "1.0",
            "worth.spatial.arbitration.declaration.v1",
            json!({
                "authored_act": self.authored_act().as_str(),
                "conflict_class": format!("{:?}", self.conflict_class()),
                "escalation": format!("{:?}", self.escalation()),
                "chosen_candidate": self.chosen_candidate().map(|candidate| candidate.as_str()),
                "policy_profile_name": self.policy_profile_name(),
                "capability_summary": {
                    "supported": self.capability_summary().supported().iter().map(|capability| capability.as_str()).collect::<Vec<_>>(),
                    "blocked": self.capability_summary().blocked().iter().map(|capability| capability.as_str()).collect::<Vec<_>>(),
                },
                "candidates": self.candidates().iter().map(|candidate| json!({
                    "candidate": candidate.candidate().as_str(),
                    "availability": match candidate.availability() {
                        SpatialIntentCandidateAvailability::Available => "available",
                        SpatialIntentCandidateAvailability::Blocked(capability) => capability.as_str(),
                    },
                    "explanation": format!("{:?}", candidate.explanation()),
                    "priority": candidate.priority(),
                    "baseline": candidate.is_baseline(),
                    "policy_preferred": candidate.is_policy_preferred(),
                })).collect::<Vec<_>>(),
            }),
        )
    }

    pub fn to_query_runtime_request(
        &self,
    ) -> Result<ForgeQueryRawIntentAdmissionRequest, ForgeQueryIntentViolationDecision> {
        ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            self.to_query_intent_declaration(),
        )
    }

    pub fn to_query_eligibility(
        &self,
    ) -> Result<ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentViolationDecision> {
        self.to_query_runtime_request()
            .map(ForgeQueryIntentAdmissionEligibility::from_request)
    }

    pub fn admit_query_intent(
        &self,
    ) -> Result<ForgeQueryIntentAdmissionDecision, ForgeQueryIntentViolationDecision> {
        self.to_query_runtime_request()
            .map(admit_runtime_intent_request)
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
            SpatialIntentEscalation::AutoResolve(candidate) => {
                SpatialArbitrationPreviewHint::AutoResolve(candidate)
            }
            SpatialIntentEscalation::PreserveCandidates => {
                SpatialArbitrationPreviewHint::PreserveCandidates
            }
            SpatialIntentEscalation::AskForClarification => {
                SpatialArbitrationPreviewHint::ClarificationRequired
            }
            SpatialIntentEscalation::BlockedByMissingCapability(capability) => {
                SpatialArbitrationPreviewHint::BlockedByCapability(capability)
            }
        }
    }

    pub fn continuity_hint(&self) -> SpatialArbitrationContinuityHint {
        match self.escalation {
            SpatialIntentEscalation::AutoResolve(
                candidate
                @ (SpatialIntentCandidate::MoveOnly | SpatialIntentCandidate::AlignFrames),
            ) => SpatialArbitrationContinuityHint::IdentityPreserved(candidate),
            SpatialIntentEscalation::AutoResolve(candidate @ SpatialIntentCandidate::SnapFlush) => {
                SpatialArbitrationContinuityHint::AnchorContinuityPreserved(candidate)
            }
            SpatialIntentEscalation::AutoResolve(
                candidate @ (SpatialIntentCandidate::AttachRelationally
                | SpatialIntentCandidate::NestInside),
            ) => SpatialArbitrationContinuityHint::IdentityReinterpreted(candidate),
            SpatialIntentEscalation::AutoResolve(
                candidate @ (SpatialIntentCandidate::SubtractCandidate
                | SpatialIntentCandidate::CutOpeningCandidate),
            ) => SpatialArbitrationContinuityHint::IdentitySplit(candidate),
            SpatialIntentEscalation::AutoResolve(
                candidate @ (SpatialIntentCandidate::MergeCandidate
                | SpatialIntentCandidate::JoinCandidate),
            ) => SpatialArbitrationContinuityHint::IdentityMerged(candidate),
            SpatialIntentEscalation::PreserveCandidates
            | SpatialIntentEscalation::AskForClarification => {
                SpatialArbitrationContinuityHint::PendingChoice
            }
            SpatialIntentEscalation::BlockedByMissingCapability(capability) => {
                SpatialArbitrationContinuityHint::BlockedPendingChoice(capability)
            }
        }
    }

    pub fn policy_preferred_candidate(&self) -> Option<SpatialIntentCandidate> {
        self.candidates
            .iter()
            .find(|candidate| candidate.is_policy_preferred())
            .map(SpatialIntentCandidateRank::candidate)
    }
}

pub type SpatialIntentArbitrationAnalysis = SpatialIntentArbitrationDeclaration;

fn graph_composition_capability_families(
    declaration: &SpatialIntentArbitrationDeclaration,
) -> Vec<&'static str> {
    let mut families = Vec::new();
    if declaration.candidates().iter().any(|candidate| {
        matches!(
            candidate.candidate(),
            SpatialIntentCandidate::AttachRelationally
                | SpatialIntentCandidate::NestInside
                | SpatialIntentCandidate::MergeCandidate
                | SpatialIntentCandidate::SubtractCandidate
                | SpatialIntentCandidate::CutOpeningCandidate
                | SpatialIntentCandidate::JoinCandidate
        )
    }) {
        families.push("same_batch_entity_relation_identity_edges");
    }
    if matches!(
        declaration.escalation(),
        SpatialIntentEscalation::AutoResolve(_)
            | SpatialIntentEscalation::BlockedByMissingCapability(_)
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
