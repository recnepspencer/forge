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

use super::capabilities::SpatialBlockedCapability;
use super::declared_analysis::{SpatialIntentArbitrationDeclaration, SpatialIntentEscalation};

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialArbitrationRuntimeDeclaration {
    declaration: SpatialIntentArbitrationDeclaration,
    invariant_violations: Vec<ForgeQueryGraphCompositionInvariantPackViolation>,
}

impl SpatialArbitrationRuntimeDeclaration {
    pub fn from_declaration(declaration: SpatialIntentArbitrationDeclaration) -> Self {
        let invariant_violations = declaration
            .candidates()
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
            .collect();
        Self {
            declaration,
            invariant_violations,
        }
    }

    pub fn declaration(&self) -> &SpatialIntentArbitrationDeclaration {
        &self.declaration
    }

    pub fn to_query_intent_declaration(&self) -> ForgeQueryIntentDeclaration {
        ForgeQueryIntentDeclaration::strategy_commit(
            "worth.spatial.arbitration".to_string(),
            "worth.spatial.arbitration.runtime_handoff",
            "1.0",
            "worth.spatial.arbitration.runtime_declaration.v1",
            json!({
                "authored_act": self.declaration.authored_act().as_str(),
                "conflict_class": format!("{:?}", self.declaration.conflict_class()),
                "escalation": format!("{:?}", self.declaration.escalation()),
                "chosen_candidate": self.declaration.chosen_candidate().map(|candidate| candidate.as_str()),
                "policy_profile_name": self.declaration.policy_profile_name(),
                "candidates": self.declaration.candidates().iter().map(|candidate| json!({
                    "candidate": candidate.candidate().as_str(),
                    "availability": match candidate.availability() {
                        super::capabilities::SpatialIntentCandidateAvailability::Available => "available",
                        super::capabilities::SpatialIntentCandidateAvailability::Blocked(capability) => capability.as_str(),
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
        let relevant_families = graph_composition_capability_families(&self.declaration);
        support
            .graph_composition_capability_support_rows()
            .into_iter()
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
    ) -> &[ForgeQueryGraphCompositionInvariantPackViolation] {
        &self.invariant_violations
    }
}

pub fn declare_spatial_arbitration_runtime(
    declaration: SpatialIntentArbitrationDeclaration,
) -> SpatialArbitrationRuntimeDeclaration {
    SpatialArbitrationRuntimeDeclaration::from_declaration(declaration)
}

fn graph_composition_capability_families(
    declaration: &SpatialIntentArbitrationDeclaration,
) -> Vec<&'static str> {
    let mut families = Vec::new();
    if declaration.candidates().iter().any(|candidate| {
        matches!(
            candidate.candidate(),
            super::candidates::SpatialIntentCandidate::AttachRelationally
                | super::candidates::SpatialIntentCandidate::NestInside
                | super::candidates::SpatialIntentCandidate::MergeCandidate
                | super::candidates::SpatialIntentCandidate::SubtractCandidate
                | super::candidates::SpatialIntentCandidate::CutOpeningCandidate
                | super::candidates::SpatialIntentCandidate::JoinCandidate
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
