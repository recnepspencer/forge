use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::lower_runtime_routing::{
    worth_query_lower_runtime_support_matrix, WorthQueryLowerRuntimeSeamKey,
    WorthQueryLowerRuntimeSupportPosture,
};
use crate::subscription::QuerySubscriptionDeliveryCauseKind;

use super::evidence_identities::{
    lower_runtime_support_row_identity, lower_runtime_support_rows_aggregate_identity,
    runtime_downstream_delivery_contract_identity, runtime_downstream_delivery_identity,
    RuntimeDownstreamDeliveryIdentityParts,
};
use super::{
    aggregate_support_posture, support_gate_resume_kind, WorthQueryRuntimeAsyncResultState,
    WorthQueryRuntimeBackendPosture, WorthQueryRuntimeDownstreamResumePosture,
    WorthQueryRuntimeDownstreamResumePostureKind, WorthQueryRuntimeLiveSubscriptionState,
    WorthQueryRuntimeRemaskDispositionKind, WorthQueryRuntimeRemaskPosture,
    WorthQueryRuntimeSupportProfile,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRuntimeDownstreamDeliveryClass {
    TruthPatch,
    TimeOnly,
    AsyncBacked,
    MixedCause,
}

impl WorthQueryRuntimeDownstreamDeliveryClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TruthPatch => "truth-patch",
            Self::TimeOnly => "time-only",
            Self::AsyncBacked => "async-backed",
            Self::MixedCause => "mixed-cause",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRuntimeDownstreamSupportPosture {
    Supported,
    Remasked,
    Denied,
}

impl WorthQueryRuntimeDownstreamSupportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Remasked => "remasked",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRuntimeDownstreamDeliveryContract {
    backend_posture: WorthQueryRuntimeBackendPosture,
    runtime_resume_support_status: WorthQueryLowerRuntimeSupportPosture,
    runtime_resume_support_identity: WorthQueryEvidenceIdentity,
    durable_resume_support_status: WorthQueryLowerRuntimeSupportPosture,
    durable_resume_support_identity: WorthQueryEvidenceIdentity,
    contract_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryRuntimeDownstreamDeliveryContract {
    pub fn from_support_profile(profile: &WorthQueryRuntimeSupportProfile) -> Self {
        Self::from_backend_posture(profile.posture())
    }

    pub fn from_backend_posture(backend_posture: WorthQueryRuntimeBackendPosture) -> Self {
        let support = worth_query_lower_runtime_support_matrix();
        let runtime_resume = support
            .support_for(WorthQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence)
            .expect("subscription basis readmission support row must exist");
        let durable_resume_rows = [
            WorthQueryLowerRuntimeSeamKey::DurableRouteReplayNeighbor,
            WorthQueryLowerRuntimeSeamKey::PersistedBoundaryExecutionReceiptNeighbor,
            WorthQueryLowerRuntimeSeamKey::RestartStableBoundaryEnvelopeReloadNeighbor,
        ]
        .map(|seam| {
            support
                .support_for(seam)
                .expect("durable resume debt row must exist")
        });
        let durable_resume_support_status =
            aggregate_support_posture(durable_resume_rows.iter().map(|row| row.posture()));
        let runtime_resume_support_identity = lower_runtime_support_row_identity(runtime_resume);
        let durable_resume_support_identity =
            lower_runtime_support_rows_aggregate_identity(durable_resume_rows);
        let runtime_resume_support_status = runtime_resume.posture();
        let contract_identity = runtime_downstream_delivery_contract_identity(
            backend_posture,
            runtime_resume_support_status,
            &runtime_resume_support_identity,
            durable_resume_support_status,
            &durable_resume_support_identity,
        );
        Self {
            backend_posture,
            runtime_resume_support_status,
            runtime_resume_support_identity,
            durable_resume_support_status,
            durable_resume_support_identity,
            contract_identity,
        }
    }

    pub fn backend_posture(&self) -> WorthQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn runtime_backed_resume_supported(&self) -> bool {
        self.runtime_resume_support_status == WorthQueryLowerRuntimeSupportPosture::Admitted
    }

    pub fn runtime_resume_support_posture(&self) -> WorthQueryLowerRuntimeSupportPosture {
        self.runtime_resume_support_status
    }

    pub fn runtime_resume_support_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.runtime_resume_support_identity
    }

    pub fn runtime_resume_support_for_reporting(&self) -> &str {
        self.runtime_resume_support_identity.as_str()
    }

    pub fn durable_resume_deferred(&self) -> bool {
        self.durable_resume_support_status == WorthQueryLowerRuntimeSupportPosture::Deferred
    }

    pub fn durable_resume_support_posture(&self) -> WorthQueryLowerRuntimeSupportPosture {
        self.durable_resume_support_status
    }

    pub fn durable_resume_support_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.durable_resume_support_identity
    }

    pub fn durable_resume_support_for_reporting(&self) -> &str {
        self.durable_resume_support_identity.as_str()
    }

    pub fn contract_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.contract_identity
    }

    pub fn contract_for_reporting(&self) -> &str {
        self.contract_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRuntimeDownstreamDelivery {
    view_name: String,
    delivery_batch_identity: WorthQueryEvidenceIdentity,
    delivery_class: WorthQueryRuntimeDownstreamDeliveryClass,
    delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    delivery_cause_identity: WorthQueryEvidenceIdentity,
    sequence: u64,
    basis_identity: WorthQueryEvidenceIdentity,
    support_posture: WorthQueryRuntimeDownstreamSupportPosture,
    support_identity: WorthQueryEvidenceIdentity,
    mixed_cause_identity: Option<WorthQueryEvidenceIdentity>,
    async_result_state_identity: Option<WorthQueryEvidenceIdentity>,
    remask_identity: Option<WorthQueryEvidenceIdentity>,
    runtime_resume_support_posture: WorthQueryLowerRuntimeSupportPosture,
    runtime_resume_support_identity: WorthQueryEvidenceIdentity,
    durable_resume_support_posture: WorthQueryLowerRuntimeSupportPosture,
    durable_resume_support_identity: WorthQueryEvidenceIdentity,
    delivery_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryRuntimeDownstreamDelivery {
    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn delivery_batch_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_batch_identity
    }

    pub fn delivery_batch_for_reporting(&self) -> &str {
        self.delivery_batch_identity.as_str()
    }

    pub fn delivery_class(&self) -> WorthQueryRuntimeDownstreamDeliveryClass {
        self.delivery_class
    }

    pub fn delivery_cause_kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        self.delivery_cause_kind
    }

    pub fn delivery_cause_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_cause_identity
    }

    pub fn delivery_cause_for_reporting(&self) -> &str {
        self.delivery_cause_identity.as_str()
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn basis_for_reporting(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn support_posture(&self) -> WorthQueryRuntimeDownstreamSupportPosture {
        self.support_posture
    }

    pub fn support_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.support_identity
    }

    pub fn support_for_reporting(&self) -> &str {
        self.support_identity.as_str()
    }

    pub fn mixed_cause_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.mixed_cause_identity.as_ref()
    }

    pub fn mixed_cause_for_reporting(&self) -> Option<&str> {
        self.mixed_cause_identity
            .as_ref()
            .map(WorthQueryEvidenceIdentity::as_str)
    }

    pub fn async_result_state_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.async_result_state_identity.as_ref()
    }

    pub fn async_result_state_for_reporting(&self) -> Option<&str> {
        self.async_result_state_identity
            .as_ref()
            .map(WorthQueryEvidenceIdentity::as_str)
    }

    pub fn remask_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.remask_identity.as_ref()
    }

    pub fn remask_for_reporting(&self) -> Option<&str> {
        self.remask_identity
            .as_ref()
            .map(WorthQueryEvidenceIdentity::as_str)
    }

    pub fn delivery_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.delivery_identity
    }

    pub fn delivery_for_reporting(&self) -> &str {
        self.delivery_identity.as_str()
    }

    pub fn runtime_resume_support_posture(&self) -> WorthQueryLowerRuntimeSupportPosture {
        self.runtime_resume_support_posture
    }

    pub fn durable_resume_support_posture(&self) -> WorthQueryLowerRuntimeSupportPosture {
        self.durable_resume_support_posture
    }

    pub fn negotiate_runtime_resume(
        &self,
        basis_identity: Option<&WorthQueryEvidenceIdentity>,
    ) -> WorthQueryRuntimeDownstreamResumePosture {
        let support_posture = self.runtime_resume_support_posture;
        if support_posture != WorthQueryLowerRuntimeSupportPosture::Admitted {
            return WorthQueryRuntimeDownstreamResumePosture::new(
                support_gate_resume_kind(support_posture),
                Some(self.basis_identity.clone()),
                support_posture,
                self.runtime_resume_support_identity.clone(),
            );
        }
        match basis_identity {
            Some(candidate) if candidate == &self.basis_identity => {
                WorthQueryRuntimeDownstreamResumePosture::new(
                    WorthQueryRuntimeDownstreamResumePostureKind::RuntimeBackedAdmitted,
                    Some(self.basis_identity.clone()),
                    support_posture,
                    self.runtime_resume_support_identity.clone(),
                )
            }
            Some(_) => WorthQueryRuntimeDownstreamResumePosture::new(
                WorthQueryRuntimeDownstreamResumePostureKind::StaleBasisDenied,
                Some(self.basis_identity.clone()),
                support_posture,
                self.runtime_resume_support_identity.clone(),
            ),
            None => WorthQueryRuntimeDownstreamResumePosture::new(
                WorthQueryRuntimeDownstreamResumePostureKind::MissingBasisDenied,
                Some(self.basis_identity.clone()),
                support_posture,
                self.runtime_resume_support_identity.clone(),
            ),
        }
    }

    pub fn durable_resume_posture(&self) -> WorthQueryRuntimeDownstreamResumePosture {
        let support_posture = self.durable_resume_support_posture;
        let kind = match support_posture {
            WorthQueryLowerRuntimeSupportPosture::Deferred => {
                WorthQueryRuntimeDownstreamResumePostureKind::DurableDeferredDebt
            }
            other => support_gate_resume_kind(other),
        };
        WorthQueryRuntimeDownstreamResumePosture::new(
            kind,
            Some(self.basis_identity.clone()),
            support_posture,
            self.durable_resume_support_identity.clone(),
        )
    }
}

pub(crate) fn project_downstream_delivery(
    contract: &WorthQueryRuntimeDownstreamDeliveryContract,
    state: &WorthQueryRuntimeLiveSubscriptionState,
) -> Option<WorthQueryRuntimeDownstreamDelivery> {
    let delivery = state.last_delivery.as_ref()?;
    let delivery_class = classify_delivery(delivery, state.async_result_state.as_ref());
    let support_posture = classify_support_posture(state.remask_posture.as_ref());
    let mixed_cause_identity = matches!(
        delivery_class,
        WorthQueryRuntimeDownstreamDeliveryClass::MixedCause
    )
    .then(|| {
        delivery
            .mixed_cause_delivery()
            .mixed_cause_identity()
            .clone()
    });
    let async_result_state_identity = state
        .async_result_state
        .as_ref()
        .map(WorthQueryRuntimeAsyncResultState::result_state_identity)
        .cloned();
    let remask_identity = state
        .remask_posture
        .as_ref()
        .map(WorthQueryRuntimeRemaskPosture::remask_identity)
        .cloned();
    let basis_identity = state.installation.basis_binding_identity().clone();
    let support_identity = state.installation.support_identity().clone();
    let delivery_batch_identity = delivery.delivery_batch_identity().clone();
    let delivery_cause_identity = delivery.delivery_cause_identity().clone();
    let delivery_identity =
        runtime_downstream_delivery_identity(RuntimeDownstreamDeliveryIdentityParts {
            view_name: state.installation.view_name(),
            delivery_batch_identity: &delivery_batch_identity,
            delivery_class,
            delivery_cause_kind: delivery.delivery_cause_kind(),
            delivery_cause_identity: &delivery_cause_identity,
            sequence: delivery.sequence(),
            basis_identity: &basis_identity,
            support_posture,
            support_identity: &support_identity,
            mixed_cause_identity: mixed_cause_identity.as_ref(),
            async_result_state_identity: async_result_state_identity.as_ref(),
            remask_identity: remask_identity.as_ref(),
            runtime_resume_support_identity: contract.runtime_resume_support_identity(),
            durable_resume_support_identity: contract.durable_resume_support_identity(),
        });
    Some(WorthQueryRuntimeDownstreamDelivery {
        view_name: state.installation.view_name().to_string(),
        delivery_batch_identity,
        delivery_class,
        delivery_cause_kind: delivery.delivery_cause_kind(),
        delivery_cause_identity,
        sequence: delivery.sequence(),
        basis_identity,
        support_posture,
        support_identity,
        mixed_cause_identity,
        async_result_state_identity,
        remask_identity,
        runtime_resume_support_posture: contract.runtime_resume_support_posture(),
        runtime_resume_support_identity: contract.runtime_resume_support_identity().clone(),
        durable_resume_support_posture: contract.durable_resume_support_posture(),
        durable_resume_support_identity: contract.durable_resume_support_identity().clone(),
        delivery_identity,
    })
}

fn classify_delivery(
    delivery: &super::delivery::WorthQueryRuntimeRetainedDelivery,
    async_result_state: Option<&WorthQueryRuntimeAsyncResultState>,
) -> WorthQueryRuntimeDownstreamDeliveryClass {
    if delivery.delivery_cause_kind() == QuerySubscriptionDeliveryCauseKind::MixedCause {
        WorthQueryRuntimeDownstreamDeliveryClass::MixedCause
    } else if async_result_state.is_some() {
        WorthQueryRuntimeDownstreamDeliveryClass::AsyncBacked
    } else if !delivery.has_relational_patch() {
        WorthQueryRuntimeDownstreamDeliveryClass::TimeOnly
    } else {
        WorthQueryRuntimeDownstreamDeliveryClass::TruthPatch
    }
}

fn classify_support_posture(
    remask_posture: Option<&WorthQueryRuntimeRemaskPosture>,
) -> WorthQueryRuntimeDownstreamSupportPosture {
    match remask_posture.map(WorthQueryRuntimeRemaskPosture::disposition_kind) {
        Some(WorthQueryRuntimeRemaskDispositionKind::Remasked) => {
            WorthQueryRuntimeDownstreamSupportPosture::Remasked
        }
        Some(WorthQueryRuntimeRemaskDispositionKind::Denied) => {
            WorthQueryRuntimeDownstreamSupportPosture::Denied
        }
        None => WorthQueryRuntimeDownstreamSupportPosture::Supported,
    }
}
