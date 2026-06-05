use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::diagnostics::BridgeTemporalAsyncOfflineDiagnosisBundleSealed;
use crate::subscription::{
    AdmittedBridgeSubscriptionResumeBasis, BridgeActiveSubscription,
    BridgeMixedCauseDeliveryWindowPlan, BridgeSharedConsumerDeliveryBundleSealed,
};
use crate::temporal::AdmittedBridgeTemporalBasis;

use super::async_lifecycle::{
    BridgeTemporalAsyncCertificationAsyncLifecycleSection,
    BridgeTemporalAsyncCertificationAsyncSectionInput,
};
use super::basis::BridgeTemporalAsyncCertificationBasisSection;
use super::counters::BridgeTemporalAsyncCertificationCounters;
use super::failure::BridgeTemporalAsyncCertificationFailureSection;
use super::mixed_cause::BridgeTemporalAsyncCertificationMixedCauseSection;
use super::resume::BridgeTemporalAsyncCertificationResumeSection;

pub const BRIDGE_TEMPORAL_ASYNC_CERTIFICATION_BUNDLE_SCHEMA_V1: &str =
    "bridge-temporal-async-certification-bundle-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeTemporalAsyncCertificationDiagnosticsRichness {
    Minimal,
    Rich,
}

impl BridgeTemporalAsyncCertificationDiagnosticsRichness {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Rich => "rich",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeTemporalAsyncCertificationBundleRejectionKind {
    SharedDeliverySubscriptionMismatch,
    SharedDeliveryWindowMismatch,
    ResumeSubscriptionMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationBundleRejection {
    kind: BridgeTemporalAsyncCertificationBundleRejectionKind,
    detail: Arc<str>,
}

impl BridgeTemporalAsyncCertificationBundleRejection {
    pub(crate) fn new(
        kind: BridgeTemporalAsyncCertificationBundleRejectionKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> BridgeTemporalAsyncCertificationBundleRejectionKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationBundleRequest {
    active_subscription: BridgeActiveSubscription,
    temporal_basis: AdmittedBridgeTemporalBasis,
    async_section_input: BridgeTemporalAsyncCertificationAsyncSectionInput,
    mixed_cause_window: BridgeMixedCauseDeliveryWindowPlan,
    shared_delivery_bundle: BridgeSharedConsumerDeliveryBundleSealed,
    resume_basis: AdmittedBridgeSubscriptionResumeBasis,
    failure_bundle: BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
    diagnostics_richness: BridgeTemporalAsyncCertificationDiagnosticsRichness,
}

impl BridgeTemporalAsyncCertificationBundleRequest {
    pub fn new(
        active_subscription: BridgeActiveSubscription,
        temporal_basis: AdmittedBridgeTemporalBasis,
        async_section_input: BridgeTemporalAsyncCertificationAsyncSectionInput,
        mixed_cause_window: BridgeMixedCauseDeliveryWindowPlan,
        shared_delivery_bundle: BridgeSharedConsumerDeliveryBundleSealed,
        resume_basis: AdmittedBridgeSubscriptionResumeBasis,
        failure_bundle: BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
        diagnostics_richness: BridgeTemporalAsyncCertificationDiagnosticsRichness,
    ) -> Self {
        Self {
            active_subscription,
            temporal_basis,
            async_section_input,
            mixed_cause_window,
            shared_delivery_bundle,
            resume_basis,
            failure_bundle,
            diagnostics_richness,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationBundleDraft {
    schema_version: Arc<str>,
    active_subscription_identity: Arc<str>,
    admitted_subscription_identity: Arc<str>,
    diagnostics_richness: BridgeTemporalAsyncCertificationDiagnosticsRichness,
    basis_section: BridgeTemporalAsyncCertificationBasisSection,
    async_section: BridgeTemporalAsyncCertificationAsyncLifecycleSection,
    mixed_cause_section: BridgeTemporalAsyncCertificationMixedCauseSection,
    resume_section: BridgeTemporalAsyncCertificationResumeSection,
    failure_section: BridgeTemporalAsyncCertificationFailureSection,
    counters: BridgeTemporalAsyncCertificationCounters,
    semantic_digest: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationBundleDraft {
    pub(crate) fn build(
        request: BridgeTemporalAsyncCertificationBundleRequest,
    ) -> Result<Self, BridgeTemporalAsyncCertificationBundleRejection> {
        let basis_section =
            BridgeTemporalAsyncCertificationBasisSection::collect(&request.temporal_basis);
        let async_section = BridgeTemporalAsyncCertificationAsyncLifecycleSection::collect(
            &request.async_section_input,
        );
        let mixed_cause_section = BridgeTemporalAsyncCertificationMixedCauseSection::collect(
            &request.active_subscription,
            &request.mixed_cause_window,
            &request.shared_delivery_bundle,
        )?;
        let resume_section = BridgeTemporalAsyncCertificationResumeSection::collect(
            &request.active_subscription,
            &request.resume_basis,
        )?;
        let failure_section =
            BridgeTemporalAsyncCertificationFailureSection::collect(&request.failure_bundle);
        let counters = BridgeTemporalAsyncCertificationCounters::new(
            request.async_section_input.request_identities().len(),
            request.async_section_input.completion_receipts().len(),
            request
                .async_section_input
                .denied_completion_receipts()
                .len(),
            request.async_section_input.supersession_receipts().len(),
            request
                .async_section_input
                .forward_causality_receipts()
                .len(),
            request.async_section_input.writeback_receipts().len(),
            request.failure_bundle.localized_failures().len(),
            request
                .shared_delivery_bundle
                .consumer_contract_identities()
                .len(),
        );
        let semantic_basis = format!(
            "bridge-temporal-async-certification-bundle-draft|active={}|admitted={}|basis={}|async={}|mixed={}|resume={}|failure={}|counters={}",
            request.active_subscription.active_subscription_identity().as_str(),
            request
                .active_subscription
                .activation_ready()
                .admitted()
                .admitted_subscription_identity()
                .as_str(),
            basis_section.semantic_digest(),
            async_section.semantic_digest(),
            mixed_cause_section.semantic_digest(),
            resume_section.semantic_digest(),
            failure_section.semantic_digest(),
            counters.digest(),
        );
        let semantic_digest = Sha256::digest(semantic_basis.as_bytes());
        let digest = Sha256::digest(
            format!(
                "{semantic_basis}|diagnostics-richness={}",
                request.diagnostics_richness.as_str()
            )
            .as_bytes(),
        );
        Ok(Self {
            schema_version: Arc::from(BRIDGE_TEMPORAL_ASYNC_CERTIFICATION_BUNDLE_SCHEMA_V1),
            active_subscription_identity: Arc::from(
                request.active_subscription.active_subscription_identity().as_str().to_owned(),
            ),
            admitted_subscription_identity: Arc::from(
                request
                    .active_subscription
                    .activation_ready()
                    .admitted()
                    .admitted_subscription_identity()
                    .as_str()
                    .to_owned(),
            ),
            diagnostics_richness: request.diagnostics_richness,
            basis_section,
            async_section,
            mixed_cause_section,
            resume_section,
            failure_section,
            counters,
            semantic_digest: Arc::from(format!(
                "bridge-temporal-async-certification-bundle-draft-semantic:sha256:{semantic_digest:x}"
            )),
            digest: Arc::from(format!(
                "bridge-temporal-async-certification-bundle-draft:sha256:{digest:x}"
            )),
        })
    }

    pub(crate) fn seal(self) -> BridgeTemporalAsyncCertificationBundleSealed {
        let canonical_basis = format!(
            "bridge-temporal-async-certification-bundle-sealed|draft={}|semantic={}",
            self.digest, self.semantic_digest
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        BridgeTemporalAsyncCertificationBundleSealed {
            schema_version: self.schema_version,
            active_subscription_identity: self.active_subscription_identity,
            admitted_subscription_identity: self.admitted_subscription_identity,
            diagnostics_richness: self.diagnostics_richness,
            basis_section: self.basis_section,
            async_section: self.async_section,
            mixed_cause_section: self.mixed_cause_section,
            resume_section: self.resume_section,
            failure_section: self.failure_section,
            counters: self.counters,
            semantic_digest: self.semantic_digest,
            digest: Arc::from(format!(
                "bridge-temporal-async-certification-bundle-sealed:sha256:{digest:x}"
            )),
        }
    }

    pub fn diagnostics_richness(&self) -> BridgeTemporalAsyncCertificationDiagnosticsRichness {
        self.diagnostics_richness
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationBundleSealed {
    schema_version: Arc<str>,
    active_subscription_identity: Arc<str>,
    admitted_subscription_identity: Arc<str>,
    diagnostics_richness: BridgeTemporalAsyncCertificationDiagnosticsRichness,
    basis_section: BridgeTemporalAsyncCertificationBasisSection,
    async_section: BridgeTemporalAsyncCertificationAsyncLifecycleSection,
    mixed_cause_section: BridgeTemporalAsyncCertificationMixedCauseSection,
    resume_section: BridgeTemporalAsyncCertificationResumeSection,
    failure_section: BridgeTemporalAsyncCertificationFailureSection,
    counters: BridgeTemporalAsyncCertificationCounters,
    semantic_digest: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationBundleSealed {
    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    pub fn active_subscription_identity(&self) -> &str {
        self.active_subscription_identity.as_ref()
    }

    pub fn admitted_subscription_identity(&self) -> &str {
        self.admitted_subscription_identity.as_ref()
    }

    pub fn diagnostics_richness(&self) -> BridgeTemporalAsyncCertificationDiagnosticsRichness {
        self.diagnostics_richness
    }

    pub fn basis_section(&self) -> &BridgeTemporalAsyncCertificationBasisSection {
        &self.basis_section
    }

    pub fn async_section(&self) -> &BridgeTemporalAsyncCertificationAsyncLifecycleSection {
        &self.async_section
    }

    pub fn mixed_cause_section(&self) -> &BridgeTemporalAsyncCertificationMixedCauseSection {
        &self.mixed_cause_section
    }

    pub fn resume_section(&self) -> &BridgeTemporalAsyncCertificationResumeSection {
        &self.resume_section
    }

    pub fn failure_section(&self) -> &BridgeTemporalAsyncCertificationFailureSection {
        &self.failure_section
    }

    pub fn counters(&self) -> &BridgeTemporalAsyncCertificationCounters {
        &self.counters
    }

    pub fn semantic_digest(&self) -> &str {
        self.semantic_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationBundleInspection {
    active_subscription_identity: Arc<str>,
    admitted_subscription_identity: Arc<str>,
    diagnostics_richness: BridgeTemporalAsyncCertificationDiagnosticsRichness,
    temporal_basis_digest: Arc<str>,
    async_section_digest: Arc<str>,
    mixed_cause_section_digest: Arc<str>,
    resume_section_digest: Arc<str>,
    failure_section_digest: Arc<str>,
    semantic_digest: Arc<str>,
    bundle_digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationBundleInspection {
    pub(crate) fn inspect(bundle: &BridgeTemporalAsyncCertificationBundleSealed) -> Self {
        Self {
            active_subscription_identity: Arc::from(
                bundle.active_subscription_identity().to_owned(),
            ),
            admitted_subscription_identity: Arc::from(
                bundle.admitted_subscription_identity().to_owned(),
            ),
            diagnostics_richness: bundle.diagnostics_richness(),
            temporal_basis_digest: Arc::from(bundle.basis_section().digest().to_owned()),
            async_section_digest: Arc::from(bundle.async_section().digest().to_owned()),
            mixed_cause_section_digest: Arc::from(bundle.mixed_cause_section().digest().to_owned()),
            resume_section_digest: Arc::from(bundle.resume_section().digest().to_owned()),
            failure_section_digest: Arc::from(bundle.failure_section().digest().to_owned()),
            semantic_digest: Arc::from(bundle.semantic_digest().to_owned()),
            bundle_digest: Arc::from(bundle.digest().to_owned()),
        }
    }

    pub fn bundle_digest(&self) -> &str {
        self.bundle_digest.as_ref()
    }
}
