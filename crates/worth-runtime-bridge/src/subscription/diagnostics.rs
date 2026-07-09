use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionActivationReady, BridgeSubscriptionAdmissionRejection,
    BridgeSubscriptionCounters, BridgeSubscriptionDeactivated,
    BridgeSubscriptionPreviewPromotionRecord,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionExplanation {
    declaration_identity: super::BridgeSubscriptionDeclarationIdentity,
    requested_family_kind: super::BridgeSubscriptionDeclarationFamilyKind,
    registry_identity: Option<super::BridgeSubscriptionFamilyRegistryIdentity>,
    admitted_subscription_identity: Option<super::BridgeAdmittedSubscriptionIdentity>,
    lifecycle_identity: Option<super::BridgeSubscriptionLifecycleIdentity>,
    basis_kind: Option<super::BridgeSubscriptionBasisKind>,
    admission_rejection_kind: Option<super::BridgeSubscriptionAdmissionRejectionKind>,
    basis_resolution_failure_kind: Option<super::BridgeSubscriptionBasisResolutionFailureKind>,
    signal_strategy_kind: Option<super::BridgeSignalStrategyKind>,
    lifecycle_state_kind: Option<super::BridgeSubscriptionLifecycleStateKind>,
    normalized_slice_intent_count: usize,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewPromotionExplanation {
    promotion_record_identity: super::BridgeSubscriptionPreviewPromotionRecordIdentity,
    outcome_class: super::BridgeSubscriptionPreviewPromotionOutcomeClass,
    preview_active_subscription_identity: super::BridgePreviewActiveSubscriptionIdentity,
    preview_basis_identity: super::BridgeSubscriptionPreviewBasisIdentity,
    preview_scope_identity: super::BridgeSubscriptionPreviewScopeIdentity,
    preview_work_trace_identity: super::BridgeSubscriptionPreviewWorkTraceIdentity,
    preview_work_trace_digest: Arc<str>,
    promoted_admitted_subscription_identity: super::BridgeAdmittedSubscriptionIdentity,
    speculation_promotion_record_digest: Arc<str>,
    authoritative_commit_boundary_digest: Arc<str>,
    authoritative_artifact_digest: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewPromotionExplanation {
    pub(crate) fn from_promotion_record(record: &BridgeSubscriptionPreviewPromotionRecord) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-promotion-explanation|record={}|outcome={}|preview-active={}|preview-basis={}|preview-scope={}|preview-work-trace={}|preview-work-digest={}|promoted-admitted={}|speculation-promotion={}|commit-boundary={}|authoritative-artifact={}",
            record.promotion_record_identity().as_str(),
            record.outcome_class().as_str(),
            record.preview_active_subscription_identity().as_str(),
            record.preview_basis_identity().as_str(),
            record.preview_scope_identity().as_str(),
            record.preview_work_trace_identity().as_str(),
            record.preview_work_trace_digest(),
            record.promoted_admitted_subscription_identity().as_str(),
            record.speculation_promotion_record_digest(),
            record.authoritative_commit_boundary_digest(),
            record.authoritative_artifact_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            promotion_record_identity: record.promotion_record_identity().clone(),
            outcome_class: record.outcome_class(),
            preview_active_subscription_identity: record
                .preview_active_subscription_identity()
                .clone(),
            preview_basis_identity: record.preview_basis_identity().clone(),
            preview_scope_identity: record.preview_scope_identity().clone(),
            preview_work_trace_identity: record.preview_work_trace_identity().clone(),
            preview_work_trace_digest: Arc::from(record.preview_work_trace_digest()),
            promoted_admitted_subscription_identity: record
                .promoted_admitted_subscription_identity()
                .clone(),
            speculation_promotion_record_digest: Arc::from(
                record.speculation_promotion_record_digest(),
            ),
            authoritative_commit_boundary_digest: Arc::from(
                record.authoritative_commit_boundary_digest(),
            ),
            authoritative_artifact_digest: Arc::from(record.authoritative_artifact_digest()),
            counters: BridgeSubscriptionCounters::from_diagnostics_bundle(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-promotion-explanation:sha256:{digest:x}"
            )),
        }
    }

    pub fn promotion_record_identity(
        &self,
    ) -> &super::BridgeSubscriptionPreviewPromotionRecordIdentity {
        &self.promotion_record_identity
    }

    pub fn outcome_class(&self) -> super::BridgeSubscriptionPreviewPromotionOutcomeClass {
        self.outcome_class
    }

    pub fn preview_active_subscription_identity(
        &self,
    ) -> &super::BridgePreviewActiveSubscriptionIdentity {
        &self.preview_active_subscription_identity
    }

    pub fn preview_basis_identity(&self) -> &super::BridgeSubscriptionPreviewBasisIdentity {
        &self.preview_basis_identity
    }

    pub fn preview_scope_identity(&self) -> &super::BridgeSubscriptionPreviewScopeIdentity {
        &self.preview_scope_identity
    }

    pub fn preview_work_trace_identity(
        &self,
    ) -> &super::BridgeSubscriptionPreviewWorkTraceIdentity {
        &self.preview_work_trace_identity
    }

    pub fn preview_work_trace_digest(&self) -> &str {
        self.preview_work_trace_digest.as_ref()
    }

    pub fn promoted_admitted_subscription_identity(
        &self,
    ) -> &super::BridgeAdmittedSubscriptionIdentity {
        &self.promoted_admitted_subscription_identity
    }

    pub fn speculation_promotion_record_digest(&self) -> &str {
        self.speculation_promotion_record_digest.as_ref()
    }

    pub fn authoritative_commit_boundary_digest(&self) -> &str {
        self.authoritative_commit_boundary_digest.as_ref()
    }

    pub fn authoritative_artifact_digest(&self) -> &str {
        self.authoritative_artifact_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

impl BridgeSubscriptionExplanation {
    pub(crate) fn from_activation_ready(ready: &BridgeSubscriptionActivationReady) -> Self {
        let admitted = ready.admitted();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-explanation|declaration={}|family={}|registry={}|admitted={}|lifecycle={}|basis={}|strategy={}|state={}|slice-count={}",
            admitted.declaration().declaration_identity().as_str(),
            admitted.declaration().requested_family_kind().as_str(),
            ready.retained_bundle().registry_identity().as_str(),
            admitted.admitted_subscription_identity().as_str(),
            ready.lifecycle_record().lifecycle_identity().as_str(),
            admitted.basis_binding().basis_kind().as_str(),
            admitted.signal_strategy().strategy_kind().as_str(),
            ready.lifecycle_record().state_kind().as_str(),
            admitted.declaration().normalized_slice_intent_count(),
        ));
        Self::from_success(
            admitted.declaration().declaration_identity().clone(),
            admitted.declaration().requested_family_kind(),
            Some(ready.retained_bundle().registry_identity().clone()),
            Some(admitted.admitted_subscription_identity().clone()),
            Some(ready.lifecycle_record().lifecycle_identity().clone()),
            Some(admitted.basis_binding().basis_kind()),
            None,
            None,
            Some(admitted.signal_strategy().strategy_kind()),
            Some(ready.lifecycle_record().state_kind()),
            admitted.declaration().normalized_slice_intent_count(),
            canonical_basis,
        )
    }

    pub(crate) fn from_deactivated(deactivated: &BridgeSubscriptionDeactivated) -> Self {
        let admitted = deactivated.admitted();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-explanation|declaration={}|family={}|registry={}|admitted={}|lifecycle={}|basis={}|strategy={}|state={}|slice-count={}",
            admitted.declaration().declaration_identity().as_str(),
            admitted.declaration().requested_family_kind().as_str(),
            deactivated.retained_bundle().registry_identity().as_str(),
            admitted.admitted_subscription_identity().as_str(),
            deactivated.lifecycle_record().lifecycle_identity().as_str(),
            admitted.basis_binding().basis_kind().as_str(),
            admitted.signal_strategy().strategy_kind().as_str(),
            deactivated.lifecycle_record().state_kind().as_str(),
            admitted.declaration().normalized_slice_intent_count(),
        ));
        Self::from_success(
            admitted.declaration().declaration_identity().clone(),
            admitted.declaration().requested_family_kind(),
            Some(deactivated.retained_bundle().registry_identity().clone()),
            Some(admitted.admitted_subscription_identity().clone()),
            Some(deactivated.lifecycle_record().lifecycle_identity().clone()),
            Some(admitted.basis_binding().basis_kind()),
            None,
            None,
            Some(admitted.signal_strategy().strategy_kind()),
            Some(deactivated.lifecycle_record().state_kind()),
            admitted.declaration().normalized_slice_intent_count(),
            canonical_basis,
        )
    }

    pub(crate) fn from_rejection(rejection: &BridgeSubscriptionAdmissionRejection) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-explanation|declaration={}|family={}|basis={}|rejection={}|basis-resolution-kind={}",
            rejection.declaration_identity().as_str(),
            rejection.requested_family_kind().as_str(),
            rejection.requested_basis_kind().as_str(),
            rejection.rejection_kind().as_str(),
            rejection
                .basis_resolution_failure_kind()
                .map(super::BridgeSubscriptionBasisResolutionFailureKind::as_str)
                .unwrap_or("-"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration_identity: rejection.declaration_identity().clone(),
            requested_family_kind: rejection.requested_family_kind(),
            registry_identity: None,
            admitted_subscription_identity: None,
            lifecycle_identity: None,
            basis_kind: Some(rejection.requested_basis_kind()),
            admission_rejection_kind: Some(rejection.rejection_kind()),
            basis_resolution_failure_kind: rejection.basis_resolution_failure_kind(),
            signal_strategy_kind: None,
            lifecycle_state_kind: None,
            normalized_slice_intent_count: 0,
            counters: BridgeSubscriptionCounters::from_diagnostics_bundle(),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-explanation:sha256:{digest:x}")),
        }
    }

    fn from_success(
        declaration_identity: super::BridgeSubscriptionDeclarationIdentity,
        requested_family_kind: super::BridgeSubscriptionDeclarationFamilyKind,
        registry_identity: Option<super::BridgeSubscriptionFamilyRegistryIdentity>,
        admitted_subscription_identity: Option<super::BridgeAdmittedSubscriptionIdentity>,
        lifecycle_identity: Option<super::BridgeSubscriptionLifecycleIdentity>,
        basis_kind: Option<super::BridgeSubscriptionBasisKind>,
        admission_rejection_kind: Option<super::BridgeSubscriptionAdmissionRejectionKind>,
        basis_resolution_failure_kind: Option<super::BridgeSubscriptionBasisResolutionFailureKind>,
        signal_strategy_kind: Option<super::BridgeSignalStrategyKind>,
        lifecycle_state_kind: Option<super::BridgeSubscriptionLifecycleStateKind>,
        normalized_slice_intent_count: usize,
        canonical_basis: Arc<str>,
    ) -> Self {
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration_identity,
            requested_family_kind,
            registry_identity,
            admitted_subscription_identity,
            lifecycle_identity,
            basis_kind,
            admission_rejection_kind,
            basis_resolution_failure_kind,
            signal_strategy_kind,
            lifecycle_state_kind,
            normalized_slice_intent_count,
            counters: BridgeSubscriptionCounters::from_diagnostics_bundle(),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-explanation:sha256:{digest:x}")),
        }
    }

    pub fn declaration_identity(&self) -> &super::BridgeSubscriptionDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn requested_family_kind(&self) -> super::BridgeSubscriptionDeclarationFamilyKind {
        self.requested_family_kind
    }

    pub fn registry_identity(&self) -> Option<&super::BridgeSubscriptionFamilyRegistryIdentity> {
        self.registry_identity.as_ref()
    }

    pub fn admitted_subscription_identity(
        &self,
    ) -> Option<&super::BridgeAdmittedSubscriptionIdentity> {
        self.admitted_subscription_identity.as_ref()
    }

    pub fn lifecycle_identity(&self) -> Option<&super::BridgeSubscriptionLifecycleIdentity> {
        self.lifecycle_identity.as_ref()
    }

    pub fn basis_kind(&self) -> Option<super::BridgeSubscriptionBasisKind> {
        self.basis_kind
    }

    pub fn admission_rejection_kind(
        &self,
    ) -> Option<super::BridgeSubscriptionAdmissionRejectionKind> {
        self.admission_rejection_kind
    }

    pub fn basis_resolution_failure_kind(
        &self,
    ) -> Option<super::BridgeSubscriptionBasisResolutionFailureKind> {
        self.basis_resolution_failure_kind
    }

    pub fn signal_strategy_kind(&self) -> Option<super::BridgeSignalStrategyKind> {
        self.signal_strategy_kind
    }

    pub fn lifecycle_state_kind(&self) -> Option<super::BridgeSubscriptionLifecycleStateKind> {
        self.lifecycle_state_kind
    }

    pub fn normalized_slice_intent_count(&self) -> usize {
        self.normalized_slice_intent_count
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
