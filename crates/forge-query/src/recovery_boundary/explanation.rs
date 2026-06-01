use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause,
};
use crate::contribution_composed_orchestration::ForgeQueryContributionComposedIntentRequestDescriptor;
use crate::grouped_authoring::{
    ForgeQueryGroupedDeclarationAspectRecord, ForgeQueryGroupedMemberRole,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology;
use crate::recovery_boundary::family::{
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryBasisPosture,
    ForgeQueryRecoveryConflictPosture, ForgeQueryRecoveryEvidenceStrength,
    ForgeQueryRecoveryFoundationalDiagnosticContext, ForgeQueryRecoveryFoundationalSupportContext,
    ForgeQueryRecoverySourceFamily,
};
use crate::recovery_boundary::materialization::ForgeQueryRecoveryMaterialization;
use forge_foundational::facade::{
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportTruthKind, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticOutcomeKind, MaterializedFoundationalProfileSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRecoveryGroupedMemberContext {
    member_index: usize,
    member_role: ForgeQueryGroupedMemberRole,
    aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
}

impl ForgeQueryRecoveryGroupedMemberContext {
    pub(crate) fn new(
        member_index: usize,
        member_role: ForgeQueryGroupedMemberRole,
        aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
    ) -> Self {
        Self {
            member_index,
            member_role,
            aspect_record,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn member_role(&self) -> ForgeQueryGroupedMemberRole {
        self.member_role
    }

    pub fn aspect_record(&self) -> &ForgeQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRecoveryExplanation {
    checked_topology: ForgeQueryOrdinaryCheckedTopology,
    source_family: ForgeQueryRecoverySourceFamily,
    evidence_strength: ForgeQueryRecoveryEvidenceStrength,
    basis_posture: ForgeQueryRecoveryBasisPosture,
    aspect_posture: ForgeQueryRecoveryAspectPosture,
    conflict_posture: ForgeQueryRecoveryConflictPosture,
    support_context: Option<ForgeQueryRecoveryFoundationalSupportContext>,
    diagnostic_context: Option<ForgeQueryRecoveryFoundationalDiagnosticContext>,
    profile: Option<MaterializedFoundationalProfileSet>,
    contribution_intent_descriptor: Option<ForgeQueryContributionComposedIntentRequestDescriptor>,
    grouped_member_context: Option<ForgeQueryRecoveryGroupedMemberContext>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: Option<String>,
    receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
}

impl ForgeQueryRecoveryExplanation {
    pub(crate) fn new_with_source_family(
        checked_topology: ForgeQueryOrdinaryCheckedTopology,
        source_family: ForgeQueryRecoverySourceFamily,
    ) -> Self {
        Self {
            checked_topology,
            source_family,
            evidence_strength: ForgeQueryRecoveryEvidenceStrength::OrdinaryProjection,
            basis_posture: ForgeQueryRecoveryBasisPosture::Unknown,
            aspect_posture: ForgeQueryRecoveryAspectPosture::None,
            conflict_posture: ForgeQueryRecoveryConflictPosture::None,
            support_context: None,
            diagnostic_context: None,
            profile: None,
            contribution_intent_descriptor: None,
            grouped_member_context: None,
            route_governing_reason: None,
            route_denial_cause: None,
            receipt_governing_reason: None,
            receipt_denial_cause: None,
        }
    }

    pub(crate) fn with_route_context(
        mut self,
        route_governing_reason: impl Into<String>,
        route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    ) -> Self {
        self.route_governing_reason = Some(route_governing_reason.into());
        self.route_denial_cause = route_denial_cause;
        self
    }

    pub(crate) fn with_source_family(
        mut self,
        source_family: ForgeQueryRecoverySourceFamily,
    ) -> Self {
        self.source_family = source_family;
        self
    }

    pub(crate) fn with_receipt_context(
        mut self,
        receipt_governing_reason: impl Into<String>,
        receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    ) -> Self {
        self.receipt_governing_reason = Some(receipt_governing_reason.into());
        self.receipt_denial_cause = receipt_denial_cause;
        self
    }

    pub(crate) fn with_evidence_strength(
        mut self,
        evidence_strength: ForgeQueryRecoveryEvidenceStrength,
    ) -> Self {
        self.evidence_strength = evidence_strength;
        self
    }

    pub(crate) fn with_basis_posture(
        mut self,
        basis_posture: ForgeQueryRecoveryBasisPosture,
    ) -> Self {
        self.basis_posture = basis_posture;
        self
    }

    pub(crate) fn with_aspect_posture(
        mut self,
        aspect_posture: ForgeQueryRecoveryAspectPosture,
    ) -> Self {
        self.aspect_posture = aspect_posture;
        self
    }

    pub(crate) fn with_conflict_posture(
        mut self,
        conflict_posture: ForgeQueryRecoveryConflictPosture,
    ) -> Self {
        self.conflict_posture = conflict_posture;
        self
    }

    pub(crate) fn with_support_context(
        mut self,
        support_context: ForgeQueryRecoveryFoundationalSupportContext,
    ) -> Self {
        self.support_context = Some(support_context);
        self
    }

    pub(crate) fn with_diagnostic_context(
        mut self,
        diagnostic_context: ForgeQueryRecoveryFoundationalDiagnosticContext,
    ) -> Self {
        self.diagnostic_context = Some(diagnostic_context);
        self
    }

    pub(crate) fn with_profile(mut self, profile: MaterializedFoundationalProfileSet) -> Self {
        self.profile = Some(profile);
        self
    }

    pub(crate) fn with_contribution_intent_descriptor(
        mut self,
        contribution_intent_descriptor: ForgeQueryContributionComposedIntentRequestDescriptor,
    ) -> Self {
        self.contribution_intent_descriptor = Some(contribution_intent_descriptor);
        self
    }

    pub(crate) fn with_grouped_member_context(
        mut self,
        grouped_member_context: ForgeQueryRecoveryGroupedMemberContext,
    ) -> Self {
        self.grouped_member_context = Some(grouped_member_context);
        self
    }

    pub fn checked_topology(&self) -> &ForgeQueryOrdinaryCheckedTopology {
        &self.checked_topology
    }

    pub fn source_family(&self) -> ForgeQueryRecoverySourceFamily {
        self.source_family
    }

    pub fn evidence_strength(&self) -> ForgeQueryRecoveryEvidenceStrength {
        self.evidence_strength
    }

    pub fn basis_posture(&self) -> ForgeQueryRecoveryBasisPosture {
        self.basis_posture
    }

    pub fn aspect_posture(&self) -> ForgeQueryRecoveryAspectPosture {
        self.aspect_posture
    }

    pub fn conflict_posture(&self) -> ForgeQueryRecoveryConflictPosture {
        self.conflict_posture
    }

    pub fn stop_stage(&self) -> Option<ForgeQueryDeclarationEntryOrchestrationStage> {
        self.checked_topology.orchestration_stop_stage()
    }

    pub fn retained_digest(&self) -> Option<&str> {
        self.checked_topology
            .orchestration_retained_digest()
            .or_else(|| {
                self.checked_topology
                    .contribution_composed_digest()
                    .or_else(|| {
                        self.checked_topology
                            .binding_linked_artifacts()?
                            .envelope_digest()
                    })
                    .or_else(|| {
                        self.checked_topology
                            .continuation_linked_artifacts()?
                            .envelope_digest()
                    })
                    .or_else(|| {
                        self.checked_topology
                            .signal_compatibility_orchestration_linked_artifacts()?
                            .envelope_digest()
                    })
            })
    }

    pub fn refusal_class(&self) -> Option<ForgeQueryDeclarationEntryOrchestrationRefusalClass> {
        self.checked_topology.orchestration_refusal_class()
    }

    pub fn route_governing_reason(&self) -> Option<&str> {
        self.route_governing_reason.as_deref()
    }

    pub fn route_denial_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.route_denial_cause
    }

    pub fn receipt_governing_reason(&self) -> Option<&str> {
        self.receipt_governing_reason.as_deref()
    }

    pub fn receipt_denial_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        self.receipt_denial_cause
    }

    pub fn contribution_digest(&self) -> Option<&str> {
        self.checked_topology.contribution_composed_digest()
    }

    pub fn support_truth_kind(&self) -> Option<FoundationalBoundaryEvidenceSupportTruthKind> {
        self.support_context.map(|value| value.truth_kind())
    }

    pub fn basis_disclosure(&self) -> Option<FoundationalBoundaryEvidenceSupportBasisDisclosure> {
        self.support_context.map(|value| value.basis_disclosure())
    }

    pub fn degraded_recovery_posture(
        &self,
    ) -> Option<FoundationalBoundaryEvidenceSupportRecoveryPosture> {
        self.support_context
            .and_then(|value| value.recovery_posture())
    }

    pub fn diagnostic_outcome_kind(&self) -> Option<FoundationalDiagnosticOutcomeKind> {
        self.diagnostic_context.map(|value| value.outcome_kind())
    }

    pub fn diagnostic_denial_class(&self) -> Option<FoundationalDiagnosticDenialClass> {
        self.diagnostic_context
            .and_then(|value| value.denial_class())
    }

    pub fn profile(&self) -> Option<&MaterializedFoundationalProfileSet> {
        self.profile.as_ref()
    }

    pub fn materialization(&self) -> ForgeQueryRecoveryMaterialization {
        ForgeQueryRecoveryMaterialization::from_profile(self.profile.clone())
    }

    pub fn contribution_intent_descriptor(
        &self,
    ) -> Option<&ForgeQueryContributionComposedIntentRequestDescriptor> {
        self.contribution_intent_descriptor.as_ref()
    }

    pub fn grouped_member_context(&self) -> Option<&ForgeQueryRecoveryGroupedMemberContext> {
        self.grouped_member_context.as_ref()
    }

    pub fn has_retained_intent_level_aspect_context(&self) -> bool {
        self.contribution_intent_descriptor.is_some()
            && self.aspect_posture == ForgeQueryRecoveryAspectPosture::RetainedContractAndCoverage
    }

    pub fn has_retained_grouped_member_aspect_context(&self) -> bool {
        self.grouped_member_context.is_some()
            && self.aspect_posture == ForgeQueryRecoveryAspectPosture::RetainedContractAndCoverage
    }
}
