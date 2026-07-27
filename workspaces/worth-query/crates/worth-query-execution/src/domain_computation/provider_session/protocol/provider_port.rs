use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::execution_digest::hash_parts;

use super::{
    WorthQueryProviderExecutionPlanContract, WorthQueryProviderSessionDenialKind,
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolCounters,
    WorthQueryProviderSessionProtocolStage,
};

static NEXT_PROTOCOL_SESSION: AtomicU64 = AtomicU64::new(1);

pub struct WorthQueryProviderSessionTokenAdmission {
    plan_identity: Arc<str>,
    provider_identity: Arc<str>,
    provider_generation: u64,
}

impl WorthQueryProviderSessionTokenAdmission {
    pub(super) fn new(plan: &WorthQueryProviderExecutionPlanContract) -> Self {
        Self {
            plan_identity: plan.identity().into(),
            provider_identity: plan.provider_identity().into(),
            provider_generation: plan.provider_generation(),
        }
    }

    pub fn admit(
        self,
        physical_session_identity: impl Into<String>,
    ) -> Result<WorthQueryProviderSessionToken, WorthQueryProviderSessionFailure> {
        let physical_session_identity = physical_session_identity.into();
        if physical_session_identity.trim().is_empty()
            || physical_session_identity.trim() != physical_session_identity
        {
            return Err(WorthQueryProviderSessionFailure::new(
                WorthQueryProviderSessionDenialKind::EmptyPhysicalSessionIdentity,
                WorthQueryProviderSessionProtocolStage::PlanReadmission,
                "provider physical session identity must be non-empty and canonical",
                WorthQueryProviderSessionProtocolCounters::default(),
            ));
        }
        let generation = NEXT_PROTOCOL_SESSION.fetch_add(1, Ordering::Relaxed);
        let identity = hash_parts(&[
            "worth_query_provider_session_token_v1".into(),
            format!("plan:{}", self.plan_identity),
            format!("provider:{}", self.provider_identity),
            format!("provider-generation:{}", self.provider_generation),
            format!("session-generation:{generation}"),
            format!("physical:{physical_session_identity}"),
        ]);
        Ok(WorthQueryProviderSessionToken {
            identity: identity.into(),
            plan_identity: self.plan_identity,
            provider_identity: self.provider_identity,
            provider_generation: self.provider_generation,
            generation,
            physical_session_identity: physical_session_identity.into(),
        })
    }
}

pub struct WorthQueryProviderSessionToken {
    identity: Arc<str>,
    plan_identity: Arc<str>,
    provider_identity: Arc<str>,
    provider_generation: u64,
    generation: u64,
    physical_session_identity: Arc<str>,
}

impl std::fmt::Debug for WorthQueryProviderSessionToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryProviderSessionToken")
            .field("identity", &self.identity)
            .field("generation", &self.generation)
            .finish_non_exhaustive()
    }
}

impl WorthQueryProviderSessionToken {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub(super) fn belongs_to(&self, plan: &WorthQueryProviderExecutionPlanContract) -> bool {
        self.plan_identity.as_ref() == plan.identity()
            && self.provider_identity.as_ref() == plan.provider_identity()
            && self.provider_generation == plan.provider_generation()
    }

    pub(super) fn view(&self) -> WorthQueryProviderSessionView<'_> {
        WorthQueryProviderSessionView { token: self }
    }
}

#[derive(Clone, Copy)]
pub struct WorthQueryProviderSessionView<'session> {
    token: &'session WorthQueryProviderSessionToken,
}

impl WorthQueryProviderSessionView<'_> {
    pub fn identity(&self) -> &str {
        self.token.identity()
    }

    pub fn generation(&self) -> u64 {
        self.token.generation()
    }

    pub fn provider_identity(&self) -> &str {
        &self.token.provider_identity
    }

    pub fn provider_generation(&self) -> u64 {
        self.token.provider_generation
    }

    pub fn physical_session_identity(&self) -> &str {
        &self.token.physical_session_identity
    }
}

#[derive(Clone, Copy)]
pub struct WorthQueryProviderExecutionPlanView<'plan> {
    contract: &'plan WorthQueryProviderExecutionPlanContract,
}

impl<'plan> WorthQueryProviderExecutionPlanView<'plan> {
    pub(super) fn new(contract: &'plan WorthQueryProviderExecutionPlanContract) -> Self {
        Self { contract }
    }

    pub fn contract(self) -> &'plan WorthQueryProviderExecutionPlanContract {
        self.contract
    }
}
