use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::lower_runtime_routing::{
    forge_query_lower_runtime_support_matrix, ForgeQueryLowerRuntimeSeamKey,
    ForgeQueryLowerRuntimeSupportPosture,
};
use crate::subscription::QuerySubscriptionDeliveryCauseKind;

use super::evidence_identities::{
    lower_runtime_support_row_identity, lower_runtime_support_rows_aggregate_identity,
    runtime_downstream_delivery_contract_identity, runtime_downstream_delivery_identity,
    RuntimeDownstreamDeliveryIdentityParts,
};
use super::{
    aggregate_support_posture, support_gate_resume_kind, ForgeQueryRuntimeAsyncResultState,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeDownstreamResumePosture,
    ForgeQueryRuntimeDownstreamResumePostureKind, ForgeQueryRuntimeLiveSubscriptionState,
    ForgeQueryRuntimeRemaskDispositionKind, ForgeQueryRuntimeRemaskPosture,
    ForgeQueryRuntimeSupportProfile,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRuntimeDownstreamDeliveryClass {
    TruthPatch,
    TimeOnly,
    AsyncBacked,
    MixedCause,
}

impl ForgeQueryRuntimeDownstreamDeliveryClass {
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
pub enum ForgeQueryRuntimeDownstreamSupportPosture {
    Supported,
    Remasked,
    Denied,
}

impl ForgeQueryRuntimeDownstreamSupportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Remasked => "remasked",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeDownstreamDeliveryContract {
    backend_posture: ForgeQueryRuntimeBackendPosture,
    runtime_resume_support_status: ForgeQueryLowerRuntimeSupportPosture,
    runtime_resume_support_identity: ForgeQueryEvidenceIdentity,
    durable_resume_support_status: ForgeQueryLowerRuntimeSupportPosture,
    durable_resume_support_identity: ForgeQueryEvidenceIdentity,
    contract_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeDownstreamDeliveryContract {
    pub fn from_support_profile(profile: &ForgeQueryRuntimeSupportProfile) -> Self {
        Self::from_backend_posture(profile.posture())
    }

    pub fn from_backend_posture(backend_posture: ForgeQueryRuntimeBackendPosture) -> Self {
        let support = forge_query_lower_runtime_support_matrix();
        let runtime_resume = support
            .support_for(ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence)
            .expect("subscription basis readmission support row must exist");
        let durable_resume_rows = [
            ForgeQueryLowerRuntimeSeamKey::DurableRouteReplayNeighbor,
            ForgeQueryLowerRuntimeSeamKey::PersistedBoundaryExecutionReceiptNeighbor,
            ForgeQueryLowerRuntimeSeamKey::RestartStableBoundaryEnvelopeReloadNeighbor,
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
            lower_runtime_support_rows_aggregate_identity(durable_resume_rows.into_iter());
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

    pub fn backend_posture(&self) -> ForgeQueryRuntimeBackendPosture {
        self.backend_posture
    }

    pub fn runtime_backed_resume_supported(&self) -> bool {
        self.runtime_resume_support_status == ForgeQueryLowerRuntimeSupportPosture::Admitted
    }

    pub fn runtime_resume_support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.runtime_resume_support_status
    }

    pub fn runtime_resume_support_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.runtime_resume_support_identity
    }

    pub fn runtime_resume_support_for_reporting(&self) -> &str {
        self.runtime_resume_support_identity.as_str()
    }

    pub fn durable_resume_deferred(&self) -> bool {
        self.durable_resume_support_status == ForgeQueryLowerRuntimeSupportPosture::Deferred
    }

    pub fn durable_resume_support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.durable_resume_support_status
    }

    pub fn durable_resume_support_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.durable_resume_support_identity
    }

    pub fn durable_resume_support_for_reporting(&self) -> &str {
        self.durable_resume_support_identity.as_str()
    }

    pub fn contract_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.contract_identity
    }

    pub fn contract_for_reporting(&self) -> &str {
        self.contract_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeDownstreamDelivery {
    view_name: String,
    delivery_batch_identity: ForgeQueryEvidenceIdentity,
    delivery_class: ForgeQueryRuntimeDownstreamDeliveryClass,
    delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    delivery_cause_identity: ForgeQueryEvidenceIdentity,
    sequence: u64,
    basis_identity: ForgeQueryEvidenceIdentity,
    support_posture: ForgeQueryRuntimeDownstreamSupportPosture,
    support_identity: ForgeQueryEvidenceIdentity,
    mixed_cause_identity: Option<ForgeQueryEvidenceIdentity>,
    async_result_state_identity: Option<ForgeQueryEvidenceIdentity>,
    remask_identity: Option<ForgeQueryEvidenceIdentity>,
    runtime_resume_support_posture: ForgeQueryLowerRuntimeSupportPosture,
    runtime_resume_support_identity: ForgeQueryEvidenceIdentity,
    durable_resume_support_posture: ForgeQueryLowerRuntimeSupportPosture,
    durable_resume_support_identity: ForgeQueryEvidenceIdentity,
    delivery_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryRuntimeDownstreamDelivery {
    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn delivery_batch_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_batch_identity
    }

    pub fn delivery_batch_for_reporting(&self) -> &str {
        self.delivery_batch_identity.as_str()
    }

    pub fn delivery_class(&self) -> ForgeQueryRuntimeDownstreamDeliveryClass {
        self.delivery_class
    }

    pub fn delivery_cause_kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        self.delivery_cause_kind
    }

    pub fn delivery_cause_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_cause_identity
    }

    pub fn delivery_cause_for_reporting(&self) -> &str {
        self.delivery_cause_identity.as_str()
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn basis_for_reporting(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn support_posture(&self) -> ForgeQueryRuntimeDownstreamSupportPosture {
        self.support_posture
    }

    pub fn support_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.support_identity
    }

    pub fn support_for_reporting(&self) -> &str {
        self.support_identity.as_str()
    }

    pub fn mixed_cause_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.mixed_cause_identity.as_ref()
    }

    pub fn mixed_cause_for_reporting(&self) -> Option<&str> {
        self.mixed_cause_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn async_result_state_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.async_result_state_identity.as_ref()
    }

    pub fn async_result_state_for_reporting(&self) -> Option<&str> {
        self.async_result_state_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn remask_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.remask_identity.as_ref()
    }

    pub fn remask_for_reporting(&self) -> Option<&str> {
        self.remask_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn delivery_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_identity
    }

    pub fn delivery_for_reporting(&self) -> &str {
        self.delivery_identity.as_str()
    }

    pub fn runtime_resume_support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.runtime_resume_support_posture
    }

    pub fn durable_resume_support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.durable_resume_support_posture
    }

    pub fn negotiate_runtime_resume(
        &self,
        basis_identity: Option<&ForgeQueryEvidenceIdentity>,
    ) -> ForgeQueryRuntimeDownstreamResumePosture {
        let support_posture = self.runtime_resume_support_posture;
        if support_posture != ForgeQueryLowerRuntimeSupportPosture::Admitted {
            return ForgeQueryRuntimeDownstreamResumePosture::new(
                support_gate_resume_kind(support_posture),
                Some(self.basis_identity.clone()),
                support_posture,
                self.runtime_resume_support_identity.clone(),
            );
        }
        match basis_identity {
            Some(candidate) if candidate == &self.basis_identity => {
                ForgeQueryRuntimeDownstreamResumePosture::new(
                    ForgeQueryRuntimeDownstreamResumePostureKind::RuntimeBackedAdmitted,
                    Some(self.basis_identity.clone()),
                    support_posture,
                    self.runtime_resume_support_identity.clone(),
                )
            }
            Some(_) => ForgeQueryRuntimeDownstreamResumePosture::new(
                ForgeQueryRuntimeDownstreamResumePostureKind::StaleBasisDenied,
                Some(self.basis_identity.clone()),
                support_posture,
                self.runtime_resume_support_identity.clone(),
            ),
            None => ForgeQueryRuntimeDownstreamResumePosture::new(
                ForgeQueryRuntimeDownstreamResumePostureKind::MissingBasisDenied,
                Some(self.basis_identity.clone()),
                support_posture,
                self.runtime_resume_support_identity.clone(),
            ),
        }
    }

    pub fn durable_resume_posture(&self) -> ForgeQueryRuntimeDownstreamResumePosture {
        let support_posture = self.durable_resume_support_posture;
        let kind = match support_posture {
            ForgeQueryLowerRuntimeSupportPosture::Deferred => {
                ForgeQueryRuntimeDownstreamResumePostureKind::DurableDeferredDebt
            }
            other => support_gate_resume_kind(other),
        };
        ForgeQueryRuntimeDownstreamResumePosture::new(
            kind,
            Some(self.basis_identity.clone()),
            support_posture,
            self.durable_resume_support_identity.clone(),
        )
    }
}

pub(crate) fn project_downstream_delivery(
    contract: &ForgeQueryRuntimeDownstreamDeliveryContract,
    state: &ForgeQueryRuntimeLiveSubscriptionState,
) -> Option<ForgeQueryRuntimeDownstreamDelivery> {
    let delivery = state.last_delivery.as_ref()?;
    let delivery_class = classify_delivery(delivery, state.async_result_state.as_ref());
    let support_posture = classify_support_posture(state.remask_posture.as_ref());
    let mixed_cause_identity = matches!(
        delivery_class,
        ForgeQueryRuntimeDownstreamDeliveryClass::MixedCause
    )
    .then(|| delivery.mixed_cause_delivery().mixed_cause_identity().clone());
    let async_result_state_identity = state
        .async_result_state
        .as_ref()
        .map(ForgeQueryRuntimeAsyncResultState::result_state_identity)
        .cloned();
    let remask_identity = state
        .remask_posture
        .as_ref()
        .map(ForgeQueryRuntimeRemaskPosture::remask_identity)
        .cloned();
    let basis_identity = state.installation.basis_binding_identity().clone();
    let support_identity = state.installation.support_identity().clone();
    let delivery_batch_identity = delivery.delivery_batch_identity().clone();
    let delivery_cause_identity = delivery.delivery_cause_identity().clone();
    let delivery_identity = runtime_downstream_delivery_identity(RuntimeDownstreamDeliveryIdentityParts {
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
    Some(ForgeQueryRuntimeDownstreamDelivery {
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
    delivery: &super::delivery::ForgeQueryRuntimeRetainedDelivery,
    async_result_state: Option<&ForgeQueryRuntimeAsyncResultState>,
) -> ForgeQueryRuntimeDownstreamDeliveryClass {
    if delivery.delivery_cause_kind() == QuerySubscriptionDeliveryCauseKind::MixedCause {
        ForgeQueryRuntimeDownstreamDeliveryClass::MixedCause
    } else if async_result_state.is_some() {
        ForgeQueryRuntimeDownstreamDeliveryClass::AsyncBacked
    } else if !delivery.has_relational_patch() {
        ForgeQueryRuntimeDownstreamDeliveryClass::TimeOnly
    } else {
        ForgeQueryRuntimeDownstreamDeliveryClass::TruthPatch
    }
}

fn classify_support_posture(
    remask_posture: Option<&ForgeQueryRuntimeRemaskPosture>,
) -> ForgeQueryRuntimeDownstreamSupportPosture {
    match remask_posture.map(ForgeQueryRuntimeRemaskPosture::disposition_kind) {
        Some(ForgeQueryRuntimeRemaskDispositionKind::Remasked) => {
            ForgeQueryRuntimeDownstreamSupportPosture::Remasked
        }
        Some(ForgeQueryRuntimeRemaskDispositionKind::Denied) => {
            ForgeQueryRuntimeDownstreamSupportPosture::Denied
        }
        None => ForgeQueryRuntimeDownstreamSupportPosture::Supported,
    }
}
