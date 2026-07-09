use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeActiveSubscriptionIdentity, BridgeAdmittedSubscriptionIdentity,
    BridgeSubscriptionActivationReady, BridgeSubscriptionBasisIdentity,
    BridgeSubscriptionConsumerContract, BridgeSubscriptionCounters,
    BridgeSubscriptionDeliveryBufferPlan, BridgeSubscriptionDeliveryCostProfile,
    BridgeSubscriptionDeliveryDiagnosticsReference, BridgeSubscriptionDeliveryFamily,
    BridgeSubscriptionDeliveryFamilyKind, BridgeSubscriptionDeliveryMemberInput,
    BridgeSubscriptionDeliveryMemberRecord, BridgeSubscriptionDeliveryWindowIdentity,
    BridgeSubscriptionDeliveryWindowOpenIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionDeliveryWindowRejectionKind {
    EmptyDeliveryWindow,
    MemberCountExceedsCostProfile,
}

impl BridgeSubscriptionDeliveryWindowRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyDeliveryWindow => "empty_delivery_window",
            Self::MemberCountExceedsCostProfile => "member_count_exceeds_cost_profile",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryWindowRejection {
    rejection_kind: BridgeSubscriptionDeliveryWindowRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryWindowRejection {
    fn new(rejection_kind: BridgeSubscriptionDeliveryWindowRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-window-rejection|kind={}",
            rejection_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_delivery_cost_profile_rejection(matches!(
                rejection_kind,
                BridgeSubscriptionDeliveryWindowRejectionKind::MemberCountExceedsCostProfile
            )),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-window-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionDeliveryWindowRejectionKind {
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
pub struct BridgeActiveSubscription {
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    activation_ready: BridgeSubscriptionActivationReady,
    cost_profile: BridgeSubscriptionDeliveryCostProfile,
    consumer_contract: BridgeSubscriptionConsumerContract,
    buffer_plan: BridgeSubscriptionDeliveryBufferPlan,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeActiveSubscription {
    pub(crate) fn activate(
        activation_ready: BridgeSubscriptionActivationReady,
        cost_profile: BridgeSubscriptionDeliveryCostProfile,
        consumer_contract: BridgeSubscriptionConsumerContract,
    ) -> Self {
        let buffer_plan = BridgeSubscriptionDeliveryBufferPlan::from_cost_profile(&cost_profile);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-active-subscription|ready={}|cost-profile={}|consumer={}|buffer={}",
            activation_ready.digest(),
            cost_profile.cost_profile_identity().as_str(),
            consumer_contract.consumer_contract_identity().as_str(),
            buffer_plan.buffer_lifecycle_identity().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            active_subscription_identity: BridgeActiveSubscriptionIdentity::admit_bridge_owned(
                format!("bridge-active-subscription-id:sha256:{digest:x}"),
            ),
            activation_ready,
            cost_profile,
            consumer_contract,
            buffer_plan,
            counters: BridgeSubscriptionCounters::from_active_subscription(),
            canonical_basis,
            digest: Arc::from(format!("bridge-active-subscription:sha256:{digest:x}")),
        }
    }

    pub fn active_subscription_identity(&self) -> &BridgeActiveSubscriptionIdentity {
        &self.active_subscription_identity
    }

    pub fn activation_ready(&self) -> &BridgeSubscriptionActivationReady {
        &self.activation_ready
    }

    pub fn cost_profile(&self) -> &BridgeSubscriptionDeliveryCostProfile {
        &self.cost_profile
    }

    pub fn consumer_contract(&self) -> &BridgeSubscriptionConsumerContract {
        &self.consumer_contract
    }

    pub fn buffer_plan(&self) -> &BridgeSubscriptionDeliveryBufferPlan {
        &self.buffer_plan
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryWindowOpen {
    delivery_window_open_identity: BridgeSubscriptionDeliveryWindowOpenIdentity,
    active_subscription: BridgeActiveSubscription,
    delivery_family: BridgeSubscriptionDeliveryFamily,
    delivery_window_sequence: u64,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryWindowOpen {
    pub(crate) fn open(
        active_subscription: &BridgeActiveSubscription,
        delivery_family_kind: BridgeSubscriptionDeliveryFamilyKind,
        delivery_window_sequence: u64,
    ) -> Self {
        let delivery_family = BridgeSubscriptionDeliveryFamily::select(delivery_family_kind);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-window-open|active={}|family={}|sequence={}",
            active_subscription.active_subscription_identity().as_str(),
            delivery_family.delivery_family_identity().as_str(),
            delivery_window_sequence,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            delivery_window_open_identity:
                BridgeSubscriptionDeliveryWindowOpenIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-delivery-window-open-id:sha256:{digest:x}"
                )),
            active_subscription: active_subscription.clone(),
            delivery_family,
            delivery_window_sequence,
            counters: BridgeSubscriptionCounters::from_delivery_window(0),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-window-open:sha256:{digest:x}"
            )),
        }
    }

    pub(crate) fn seal(
        self,
        members: Vec<BridgeSubscriptionDeliveryMemberInput>,
    ) -> Result<BridgeSubscriptionDeliveryWindowSealed, BridgeSubscriptionDeliveryWindowRejection>
    {
        BridgeSubscriptionDeliveryWindowSealed::seal(self, members)
    }

    pub fn delivery_window_open_identity(&self) -> &BridgeSubscriptionDeliveryWindowOpenIdentity {
        &self.delivery_window_open_identity
    }

    pub fn delivery_window_sequence(&self) -> u64 {
        self.delivery_window_sequence
    }

    pub fn active_subscription(&self) -> &BridgeActiveSubscription {
        &self.active_subscription
    }

    pub fn delivery_family(&self) -> &BridgeSubscriptionDeliveryFamily {
        &self.delivery_family
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeliveryWindowSealed {
    delivery_window_identity: BridgeSubscriptionDeliveryWindowIdentity,
    active_subscription_identity: BridgeActiveSubscriptionIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    basis_identity: BridgeSubscriptionBasisIdentity,
    delivery_family: BridgeSubscriptionDeliveryFamily,
    delivery_window_sequence: u64,
    members: Arc<[BridgeSubscriptionDeliveryMemberRecord]>,
    diagnostics_reference: BridgeSubscriptionDeliveryDiagnosticsReference,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeliveryWindowSealed {
    fn seal(
        open: BridgeSubscriptionDeliveryWindowOpen,
        members: Vec<BridgeSubscriptionDeliveryMemberInput>,
    ) -> Result<Self, BridgeSubscriptionDeliveryWindowRejection> {
        if members.is_empty() {
            return Err(BridgeSubscriptionDeliveryWindowRejection::new(
                BridgeSubscriptionDeliveryWindowRejectionKind::EmptyDeliveryWindow,
            ));
        }
        if members.len() > open.active_subscription.cost_profile().max_member_count() {
            return Err(BridgeSubscriptionDeliveryWindowRejection::new(
                BridgeSubscriptionDeliveryWindowRejectionKind::MemberCountExceedsCostProfile,
            ));
        }
        let counters = BridgeSubscriptionCounters::from_delivery_window(members.len());
        let counter_digest = Arc::<str>::from(counters.digest().to_owned());
        let admitted = open.active_subscription.activation_ready().admitted();
        let member_input_basis = members
            .iter()
            .enumerate()
            .map(|(index, input)| format!("{index}:{}", input.canonical_input_basis()))
            .collect::<Vec<_>>()
            .join(",");
        let delivery_window_identity_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-window-sealed-identity|open={}|sequence={}|active={}|family={}|member-count={}|member-inputs={}",
            open.delivery_window_open_identity.as_str(),
            open.delivery_window_sequence,
            open.active_subscription.active_subscription_identity().as_str(),
            open.delivery_family.delivery_family_identity().as_str(),
            members.len(),
            member_input_basis,
        ));
        let delivery_window_identity_digest =
            Sha256::digest(delivery_window_identity_basis.as_bytes());
        let delivery_window_identity =
            BridgeSubscriptionDeliveryWindowIdentity::admit_bridge_owned(format!(
                "bridge-subscription-delivery-window-id:sha256:{delivery_window_identity_digest:x}"
            ));
        let records = members
            .into_iter()
            .enumerate()
            .map(|(index, input)| {
                BridgeSubscriptionDeliveryMemberRecord::new(
                    admitted.admitted_subscription_identity().clone(),
                    open.delivery_family.delivery_family_identity().clone(),
                    delivery_window_identity.clone(),
                    admitted.basis_binding().basis_identity().clone(),
                    index,
                    Arc::from("minimal_reference"),
                    counter_digest.clone(),
                    input,
                )
            })
            .collect::<Vec<_>>();
        let diagnostics_reference = BridgeSubscriptionDeliveryDiagnosticsReference::new(
            delivery_window_identity.clone(),
            counter_digest,
        );
        let member_digest_basis = records
            .iter()
            .map(BridgeSubscriptionDeliveryMemberRecord::digest)
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-delivery-window-sealed|window={}|active={}|family={}|member-count={}|members={}|diagnostics={}",
            delivery_window_identity.as_str(),
            open.active_subscription.active_subscription_identity().as_str(),
            open.delivery_family.delivery_family_identity().as_str(),
            records.len(),
            member_digest_basis,
            diagnostics_reference.diagnostics_reference_identity().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            delivery_window_identity,
            active_subscription_identity: open
                .active_subscription
                .active_subscription_identity()
                .clone(),
            admitted_subscription_identity: admitted.admitted_subscription_identity().clone(),
            basis_identity: admitted.basis_binding().basis_identity().clone(),
            delivery_family: open.delivery_family,
            delivery_window_sequence: open.delivery_window_sequence,
            members: records.into(),
            diagnostics_reference,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-delivery-window-sealed:sha256:{digest:x}"
            )),
        })
    }

    pub fn delivery_window_identity(&self) -> &BridgeSubscriptionDeliveryWindowIdentity {
        &self.delivery_window_identity
    }

    pub fn active_subscription_identity(&self) -> &BridgeActiveSubscriptionIdentity {
        &self.active_subscription_identity
    }

    pub fn admitted_subscription_identity(&self) -> &BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }

    pub fn basis_identity(&self) -> &BridgeSubscriptionBasisIdentity {
        &self.basis_identity
    }

    pub fn delivery_window_sequence(&self) -> u64 {
        self.delivery_window_sequence
    }

    pub fn delivery_family(&self) -> &BridgeSubscriptionDeliveryFamily {
        &self.delivery_family
    }

    pub fn members(&self) -> &[BridgeSubscriptionDeliveryMemberRecord] {
        &self.members
    }

    pub fn diagnostics_reference(&self) -> &BridgeSubscriptionDeliveryDiagnosticsReference {
        &self.diagnostics_reference
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
