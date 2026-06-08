use crate::lower_runtime_routing::{
    forge_query_lower_runtime_support_matrix, ForgeQueryLowerRuntimeSeamKey,
    ForgeQueryLowerRuntimeSupportPosture,
};
use crate::subscription::QuerySubscriptionDeliveryCauseKind;

use super::{
    aggregate_support_posture, support_gate_resume_kind, ForgeQueryRuntimeAsyncResultState,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeDownstreamResumePosture,
    ForgeQueryRuntimeDownstreamResumePostureKind, ForgeQueryRuntimeLiveSubscriptionState,
    ForgeQueryRuntimeRemaskDispositionKind, ForgeQueryRuntimeRemaskPosture,
    ForgeQueryRuntimeSupportProfile,
};
use crate::identity::hash_parts;

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
    runtime_resume_support_digest: String,
    durable_resume_support_status: ForgeQueryLowerRuntimeSupportPosture,
    durable_resume_support_digest: String,
    contract_digest: String,
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
        let durable_resume_support_digest = hash_parts(
            &durable_resume_rows
                .iter()
                .map(|row| row.row_digest())
                .collect::<Vec<_>>(),
        );
        let runtime_resume_support_status = runtime_resume.posture();
        let runtime_resume_support_digest = runtime_resume.row_digest();
        let contract_digest = hash_parts(&[
            "forge_query_runtime_downstream_delivery_contract_v1".to_string(),
            format!("posture:{}", backend_posture.as_str()),
            format!("runtime_resume:{}", runtime_resume_support_status.as_str()),
            format!("runtime_resume_digest:{runtime_resume_support_digest}"),
            format!("durable_resume:{}", durable_resume_support_status.as_str()),
            format!("durable_resume_digest:{durable_resume_support_digest}"),
        ]);
        Self {
            backend_posture,
            runtime_resume_support_status,
            runtime_resume_support_digest,
            durable_resume_support_status,
            durable_resume_support_digest,
            contract_digest,
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

    pub fn runtime_resume_support_digest(&self) -> &str {
        &self.runtime_resume_support_digest
    }

    pub fn durable_resume_deferred(&self) -> bool {
        self.durable_resume_support_status == ForgeQueryLowerRuntimeSupportPosture::Deferred
    }

    pub fn durable_resume_support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.durable_resume_support_status
    }

    pub fn durable_resume_support_digest(&self) -> &str {
        &self.durable_resume_support_digest
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRuntimeDownstreamDelivery {
    view_name: String,
    delivery_batch_digest: String,
    delivery_class: ForgeQueryRuntimeDownstreamDeliveryClass,
    delivery_cause_kind: QuerySubscriptionDeliveryCauseKind,
    delivery_cause_digest: String,
    sequence: u64,
    basis_digest: String,
    support_posture: ForgeQueryRuntimeDownstreamSupportPosture,
    support_evidence_digest: String,
    mixed_cause_digest: Option<String>,
    async_result_state_digest: Option<String>,
    remask_digest: Option<String>,
    runtime_resume_support_posture: ForgeQueryLowerRuntimeSupportPosture,
    runtime_resume_support_digest: String,
    durable_resume_support_posture: ForgeQueryLowerRuntimeSupportPosture,
    durable_resume_support_digest: String,
    delivery_digest: String,
}

impl ForgeQueryRuntimeDownstreamDelivery {
    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn delivery_batch_digest(&self) -> &str {
        &self.delivery_batch_digest
    }

    pub fn delivery_class(&self) -> ForgeQueryRuntimeDownstreamDeliveryClass {
        self.delivery_class
    }

    pub fn delivery_cause_kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        self.delivery_cause_kind
    }

    pub fn delivery_cause_digest(&self) -> &str {
        &self.delivery_cause_digest
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn support_posture(&self) -> ForgeQueryRuntimeDownstreamSupportPosture {
        self.support_posture
    }

    pub fn support_evidence_digest(&self) -> &str {
        &self.support_evidence_digest
    }

    pub fn mixed_cause_digest(&self) -> Option<&str> {
        self.mixed_cause_digest.as_deref()
    }

    pub fn async_result_state_digest(&self) -> Option<&str> {
        self.async_result_state_digest.as_deref()
    }

    pub fn remask_digest(&self) -> Option<&str> {
        self.remask_digest.as_deref()
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn runtime_resume_support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.runtime_resume_support_posture
    }

    pub fn durable_resume_support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.durable_resume_support_posture
    }

    pub fn negotiate_runtime_resume(
        &self,
        basis_digest: Option<&str>,
    ) -> ForgeQueryRuntimeDownstreamResumePosture {
        let support_posture = self.runtime_resume_support_posture;
        if support_posture != ForgeQueryLowerRuntimeSupportPosture::Admitted {
            return ForgeQueryRuntimeDownstreamResumePosture::new(
                support_gate_resume_kind(support_posture),
                Some(self.basis_digest.clone()),
                support_posture,
                self.runtime_resume_support_digest.clone(),
            );
        }
        match basis_digest {
            Some(candidate) if candidate == self.basis_digest => {
                ForgeQueryRuntimeDownstreamResumePosture::new(
                    ForgeQueryRuntimeDownstreamResumePostureKind::RuntimeBackedAdmitted,
                    Some(self.basis_digest.clone()),
                    support_posture,
                    self.runtime_resume_support_digest.clone(),
                )
            }
            Some(_) => ForgeQueryRuntimeDownstreamResumePosture::new(
                ForgeQueryRuntimeDownstreamResumePostureKind::StaleBasisDenied,
                Some(self.basis_digest.clone()),
                support_posture,
                self.runtime_resume_support_digest.clone(),
            ),
            None => ForgeQueryRuntimeDownstreamResumePosture::new(
                ForgeQueryRuntimeDownstreamResumePostureKind::MissingBasisDenied,
                Some(self.basis_digest.clone()),
                support_posture,
                self.runtime_resume_support_digest.clone(),
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
            Some(self.basis_digest.clone()),
            support_posture,
            self.durable_resume_support_digest.clone(),
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
    let mixed_cause_digest = matches!(
        delivery_class,
        ForgeQueryRuntimeDownstreamDeliveryClass::MixedCause
    )
    .then(|| {
        delivery
            .mixed_cause_delivery()
            .mixed_cause_digest()
            .to_string()
    });
    let async_result_state_digest = state
        .async_result_state
        .as_ref()
        .map(ForgeQueryRuntimeAsyncResultState::result_state_digest)
        .map(str::to_string);
    let remask_digest = state
        .remask_posture
        .as_ref()
        .map(ForgeQueryRuntimeRemaskPosture::remask_digest)
        .map(str::to_string);
    let basis_digest = state.installation.basis_binding_digest().to_string();
    let support_evidence_digest = state.installation.support_evidence().to_string();
    let delivery_batch_digest = delivery.delivery_batch_digest().to_string();
    let delivery_digest = hash_parts(&[
        "forge_query_runtime_downstream_delivery_v1".to_string(),
        format!("view:{}", state.installation.view_name()),
        format!("batch:{delivery_batch_digest}"),
        format!("class:{}", delivery_class.as_str()),
        format!("cause:{}", delivery.delivery_cause_kind().as_str()),
        format!("cause_digest:{}", delivery.delivery_cause_digest()),
        format!("sequence:{}", delivery.sequence()),
        format!("basis:{basis_digest}"),
        format!("support_posture:{}", support_posture.as_str()),
        format!("support_evidence:{support_evidence_digest}"),
        format!("mixed:{}", mixed_cause_digest.as_deref().unwrap_or("none")),
        format!(
            "async:{}",
            async_result_state_digest.as_deref().unwrap_or("none")
        ),
        format!("remask:{}", remask_digest.as_deref().unwrap_or("none")),
        format!(
            "runtime_resume:{}",
            contract.runtime_resume_support_digest()
        ),
        format!(
            "durable_resume:{}",
            contract.durable_resume_support_digest()
        ),
    ]);
    Some(ForgeQueryRuntimeDownstreamDelivery {
        view_name: state.installation.view_name().to_string(),
        delivery_batch_digest,
        delivery_class,
        delivery_cause_kind: delivery.delivery_cause_kind(),
        delivery_cause_digest: delivery.delivery_cause_digest().to_string(),
        sequence: delivery.sequence(),
        basis_digest,
        support_posture,
        support_evidence_digest,
        mixed_cause_digest,
        async_result_state_digest,
        remask_digest,
        runtime_resume_support_posture: contract.runtime_resume_support_posture(),
        runtime_resume_support_digest: contract.runtime_resume_support_digest().to_string(),
        durable_resume_support_posture: contract.durable_resume_support_posture(),
        durable_resume_support_digest: contract.durable_resume_support_digest().to_string(),
        delivery_digest,
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
