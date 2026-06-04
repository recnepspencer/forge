use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionAcknowledgementFrontierIdentity, BridgeSubscriptionBasisIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity,
    BridgeSubscriptionDeliveryFamilyIdentity, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryMemberIdentity, BridgeSubscriptionDeliveryMemberRecord,
    BridgeSubscriptionDeliveryWindowIdentity, BridgeSubscriptionDeliveryWindowSealed,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionAcknowledgementFrontierRejectionKind {
    EmptyWindow,
    AcknowledgedSequenceOutOfRange,
    AcknowledgedMemberIdentityMismatch,
    DescriptorOnlyFamilyCannotPublishCanonicalCheckpoint,
}

impl BridgeSubscriptionAcknowledgementFrontierRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyWindow => "empty_window",
            Self::AcknowledgedSequenceOutOfRange => "acknowledged_sequence_out_of_range",
            Self::AcknowledgedMemberIdentityMismatch => "acknowledged_member_identity_mismatch",
            Self::DescriptorOnlyFamilyCannotPublishCanonicalCheckpoint => {
                "descriptor_only_family_cannot_publish_canonical_checkpoint"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionAcknowledgementFrontierRejection {
    rejection_kind: BridgeSubscriptionAcknowledgementFrontierRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionAcknowledgementFrontierRejection {
    fn new(rejection_kind: BridgeSubscriptionAcknowledgementFrontierRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-acknowledgement-frontier-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_acknowledgement_frontier_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-acknowledgement-frontier-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionAcknowledgementFrontierRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionAcknowledgementFrontier {
    acknowledgement_frontier_identity: BridgeSubscriptionAcknowledgementFrontierIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    delivery_family_identity: BridgeSubscriptionDeliveryFamilyIdentity,
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    delivery_window_sequence: u64,
    basis_identity: BridgeSubscriptionBasisIdentity,
    acknowledged_canonical_sequence: usize,
    acknowledged_member_identity: BridgeSubscriptionDeliveryMemberIdentity,
    acknowledged_member_digest: Arc<str>,
    acknowledged_prefix_digest: Arc<str>,
    diagnostics_reference_identity: BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity,
    counter_digest: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionAcknowledgementFrontier {
    pub(crate) fn admit(
        sealed_window: &BridgeSubscriptionDeliveryWindowSealed,
        acknowledged_sequence: usize,
        acknowledged_member: &BridgeSubscriptionDeliveryMemberRecord,
    ) -> Result<Self, BridgeSubscriptionAcknowledgementFrontierRejection> {
        if matches!(
            sealed_window.delivery_family().family_kind(),
            BridgeSubscriptionDeliveryFamilyKind::ReplayAuditDescriptor
                | BridgeSubscriptionDeliveryFamilyKind::RouteFocusedDescriptor
        ) {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::DescriptorOnlyFamilyCannotPublishCanonicalCheckpoint,
            ));
        }
        if sealed_window.members().is_empty() {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::EmptyWindow,
            ));
        }
        let Some(member) = sealed_window.members().get(acknowledged_sequence) else {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::AcknowledgedSequenceOutOfRange,
            ));
        };
        if member.delivery_member_identity() != acknowledged_member.delivery_member_identity() {
            return Err(BridgeSubscriptionAcknowledgementFrontierRejection::new(
                BridgeSubscriptionAcknowledgementFrontierRejectionKind::AcknowledgedMemberIdentityMismatch,
            ));
        }
        let prefix_basis = sealed_window
            .members()
            .iter()
            .take(acknowledged_sequence + 1)
            .map(|member| member.digest())
            .collect::<Vec<_>>()
            .join(",");
        let acknowledged_prefix_digest = Arc::<str>::from(format!(
            "bridge-subscription-acknowledged-prefix:sha256:{:x}",
            Sha256::digest(prefix_basis.as_bytes())
        ));
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-acknowledgement-frontier|active={}|admitted={}|family={}|window={}|sequence={}|basis={}|ack-sequence={}|ack-member={}|ack-digest={}|prefix={}|diagnostics={}|counter={}",
            sealed_window.active_subscription_identity().as_str(),
            sealed_window.admitted_subscription_identity().as_str(),
            sealed_window.delivery_family().delivery_family_identity().as_str(),
            sealed_window.delivery_window_identity().as_str(),
            sealed_window.delivery_window_sequence(),
            sealed_window.basis_identity().as_str(),
            acknowledged_sequence,
            member.delivery_member_identity().as_str(),
            member.digest(),
            acknowledged_prefix_digest.as_ref(),
            sealed_window.diagnostics_reference().diagnostics_reference_identity().as_str(),
            sealed_window.diagnostics_reference().counter_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            acknowledgement_frontier_identity:
                BridgeSubscriptionAcknowledgementFrontierIdentity::new(format!(
                    "bridge-subscription-acknowledgement-frontier-id:sha256:{digest:x}"
                )),
            active_subscription_identity: sealed_window.active_subscription_identity().clone(),
            admitted_subscription_identity: sealed_window.admitted_subscription_identity().clone(),
            delivery_family_identity: sealed_window
                .delivery_family()
                .delivery_family_identity()
                .clone(),
            delivery_window_identity: sealed_window.delivery_window_identity().clone(),
            delivery_window_sequence: sealed_window.delivery_window_sequence(),
            basis_identity: sealed_window.basis_identity().clone(),
            acknowledged_canonical_sequence: acknowledged_sequence,
            acknowledged_member_identity: member.delivery_member_identity().clone(),
            acknowledged_member_digest: Arc::from(member.digest().to_owned()),
            acknowledged_prefix_digest,
            diagnostics_reference_identity: sealed_window
                .diagnostics_reference()
                .diagnostics_reference_identity()
                .clone(),
            counter_digest: Arc::from(
                sealed_window
                    .diagnostics_reference()
                    .counter_digest()
                    .to_owned(),
            ),
            counters: BridgeSubscriptionCounters::from_acknowledgement_frontier_admission(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-acknowledgement-frontier:sha256:{digest:x}"
            )),
        })
    }

    pub fn acknowledgement_frontier_identity(
        &self,
    ) -> &BridgeSubscriptionAcknowledgementFrontierIdentity {
        &self.acknowledgement_frontier_identity
    }

    pub fn active_subscription_identity(&self) -> &BridgeActiveSubscriptionIdentity {
        &self.active_subscription_identity
    }

    pub fn admitted_subscription_identity(&self) -> &BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }

    pub fn delivery_family_identity(&self) -> &BridgeSubscriptionDeliveryFamilyIdentity {
        &self.delivery_family_identity
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn delivery_window_sequence(&self) -> u64 {
        self.delivery_window_sequence
    }

    pub fn basis_identity(&self) -> &BridgeSubscriptionBasisIdentity {
        &self.basis_identity
    }

    pub fn acknowledged_canonical_sequence(&self) -> usize {
        self.acknowledged_canonical_sequence
    }

    pub fn acknowledged_member_identity(&self) -> &BridgeSubscriptionDeliveryMemberIdentity {
        &self.acknowledged_member_identity
    }

    pub fn acknowledged_member_digest(&self) -> &str {
        self.acknowledged_member_digest.as_ref()
    }

    pub fn acknowledged_prefix_digest(&self) -> &str {
        self.acknowledged_prefix_digest.as_ref()
    }

    pub fn diagnostics_reference_identity(
        &self,
    ) -> &BridgeSubscriptionDeliveryDiagnosticsReferenceIdentity {
        &self.diagnostics_reference_identity
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
