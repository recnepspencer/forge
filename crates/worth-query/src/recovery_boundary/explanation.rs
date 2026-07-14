use crate::application::{
    WorthQueryDeclarationEntryOrchestrationRefusalClass,
    WorthQueryDeclarationEntryOrchestrationStage, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDeclarationRoutePlanDenialCause,
};
use crate::contribution_composed_orchestration::WorthQueryContributionComposedIntentRequestDescriptor;
use crate::grouped_authoring::{
    WorthQueryGroupedDeclarationAspectRecord, WorthQueryGroupedMemberRole,
};
use crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology;
use crate::recovery_boundary::family::{
    WorthQueryRecoveryAspectPosture, WorthQueryRecoveryBasisPosture,
    WorthQueryRecoveryConflictPosture, WorthQueryRecoveryEvidenceStrength,
    WorthQueryRecoveryFoundationalDiagnosticContext, WorthQueryRecoveryFoundationalSupportContext,
    WorthQueryRecoverySourceFamily,
};
use crate::recovery_boundary::materialization::WorthQueryRecoveryMaterialization;
use worth_foundational::facade::{
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportTruthKind, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticOutcomeKind, MaterializedFoundationalProfileSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRecoveryGroupedMemberContext {
    member_index: usize,
    member_role: WorthQueryGroupedMemberRole,
    aspect_record: WorthQueryGroupedDeclarationAspectRecord,
}

impl WorthQueryRecoveryGroupedMemberContext {
    pub(crate) fn new(
        member_index: usize,
        member_role: WorthQueryGroupedMemberRole,
        aspect_record: WorthQueryGroupedDeclarationAspectRecord,
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

    pub fn member_role(&self) -> WorthQueryGroupedMemberRole {
        self.member_role
    }

    pub fn aspect_record(&self) -> &WorthQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRecoveryExplanation {
    checked_topology: WorthQueryOrdinaryCheckedTopology,
    source_family: WorthQueryRecoverySourceFamily,
    evidence_strength: WorthQueryRecoveryEvidenceStrength,
    basis_posture: WorthQueryRecoveryBasisPosture,
    aspect_posture: WorthQueryRecoveryAspectPosture,
    conflict_posture: WorthQueryRecoveryConflictPosture,
    support_context: Option<WorthQueryRecoveryFoundationalSupportContext>,
    diagnostic_context: Option<WorthQueryRecoveryFoundationalDiagnosticContext>,
    profile: Option<MaterializedFoundationalProfileSet>,
    contribution_intent_descriptor: Option<WorthQueryContributionComposedIntentRequestDescriptor>,
    grouped_member_context: Option<WorthQueryRecoveryGroupedMemberContext>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: Option<String>,
    receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
}

impl WorthQueryRecoveryExplanation {
    pub(crate) fn new_with_source_family(
        checked_topology: WorthQueryOrdinaryCheckedTopology,
        source_family: WorthQueryRecoverySourceFamily,
    ) -> Self {
        Self {
            checked_topology,
            source_family,
            evidence_strength: WorthQueryRecoveryEvidenceStrength::OrdinaryProjection,
            basis_posture: WorthQueryRecoveryBasisPosture::Unknown,
            aspect_posture: WorthQueryRecoveryAspectPosture::None,
            conflict_posture: WorthQueryRecoveryConflictPosture::None,
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
        route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    ) -> Self {
        self.route_governing_reason = Some(route_governing_reason.into());
        self.route_denial_cause = route_denial_cause;
        self
    }

    pub(crate) fn with_source_family(
        mut self,
        source_family: WorthQueryRecoverySourceFamily,
    ) -> Self {
        self.source_family = source_family;
        self
    }

    pub(crate) fn with_receipt_context(
        mut self,
        receipt_governing_reason: impl Into<String>,
        receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
    ) -> Self {
        self.receipt_governing_reason = Some(receipt_governing_reason.into());
        self.receipt_denial_cause = receipt_denial_cause;
        self
    }

    pub(crate) fn with_evidence_strength(
        mut self,
        evidence_strength: WorthQueryRecoveryEvidenceStrength,
    ) -> Self {
        self.evidence_strength = evidence_strength;
        self
    }

    pub(crate) fn with_basis_posture(
        mut self,
        basis_posture: WorthQueryRecoveryBasisPosture,
    ) -> Self {
        self.basis_posture = basis_posture;
        self
    }

    pub(crate) fn with_aspect_posture(
        mut self,
        aspect_posture: WorthQueryRecoveryAspectPosture,
    ) -> Self {
        self.aspect_posture = aspect_posture;
        self
    }

    pub(crate) fn with_conflict_posture(
        mut self,
        conflict_posture: WorthQueryRecoveryConflictPosture,
    ) -> Self {
        self.conflict_posture = conflict_posture;
        self
    }

    pub(crate) fn with_support_context(
        mut self,
        support_context: WorthQueryRecoveryFoundationalSupportContext,
    ) -> Self {
        self.support_context = Some(support_context);
        self
    }

    pub(crate) fn with_diagnostic_context(
        mut self,
        diagnostic_context: WorthQueryRecoveryFoundationalDiagnosticContext,
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
        contribution_intent_descriptor: WorthQueryContributionComposedIntentRequestDescriptor,
    ) -> Self {
        self.contribution_intent_descriptor = Some(contribution_intent_descriptor);
        self
    }

    pub(crate) fn with_grouped_member_context(
        mut self,
        grouped_member_context: WorthQueryRecoveryGroupedMemberContext,
    ) -> Self {
        self.grouped_member_context = Some(grouped_member_context);
        self
    }

    pub fn checked_topology(&self) -> &WorthQueryOrdinaryCheckedTopology {
        &self.checked_topology
    }

    pub fn source_family(&self) -> WorthQueryRecoverySourceFamily {
        self.source_family
    }

    pub fn evidence_strength(&self) -> WorthQueryRecoveryEvidenceStrength {
        self.evidence_strength
    }

    pub fn basis_posture(&self) -> WorthQueryRecoveryBasisPosture {
        self.basis_posture
    }

    pub fn aspect_posture(&self) -> WorthQueryRecoveryAspectPosture {
        self.aspect_posture
    }

    pub fn conflict_posture(&self) -> WorthQueryRecoveryConflictPosture {
        self.conflict_posture
    }

    pub fn stop_stage(&self) -> Option<WorthQueryDeclarationEntryOrchestrationStage> {
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

    pub fn refusal_class(&self) -> Option<WorthQueryDeclarationEntryOrchestrationRefusalClass> {
        self.checked_topology.orchestration_refusal_class()
    }

    pub fn route_governing_reason(&self) -> Option<&str> {
        self.route_governing_reason.as_deref()
    }

    pub fn route_denial_cause(&self) -> Option<WorthQueryDeclarationRoutePlanDenialCause> {
        self.route_denial_cause
    }

    pub fn receipt_governing_reason(&self) -> Option<&str> {
        self.receipt_governing_reason.as_deref()
    }

    pub fn receipt_denial_cause(&self) -> Option<WorthQueryDeclarationReceiptDenialCause> {
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

    pub fn materialization(&self) -> WorthQueryRecoveryMaterialization {
        WorthQueryRecoveryMaterialization::from_profile(self.profile.clone())
    }

    pub fn contribution_intent_descriptor(
        &self,
    ) -> Option<&WorthQueryContributionComposedIntentRequestDescriptor> {
        self.contribution_intent_descriptor.as_ref()
    }

    pub fn grouped_member_context(&self) -> Option<&WorthQueryRecoveryGroupedMemberContext> {
        self.grouped_member_context.as_ref()
    }

    pub fn has_retained_intent_level_aspect_context(&self) -> bool {
        self.contribution_intent_descriptor.is_some()
            && self.aspect_posture == WorthQueryRecoveryAspectPosture::RetainedContractAndCoverage
    }

    pub fn has_retained_grouped_member_aspect_context(&self) -> bool {
        self.grouped_member_context.is_some()
            && self.aspect_posture == WorthQueryRecoveryAspectPosture::RetainedContractAndCoverage
    }
}
