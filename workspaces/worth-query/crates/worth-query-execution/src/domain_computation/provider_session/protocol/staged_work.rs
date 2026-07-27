use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    WorthQueryGraphProviderAnchor, WorthQueryPreparedProviderSession,
    WorthQueryProviderExecutionPlanContract, WorthQueryProviderRunBorrow,
    WorthQueryProviderSessionLease, WorthQueryProviderSessionProtocolCounters,
    WorthQueryProviderSessionView, WorthQuerySessionBinding,
};

pub struct WorthQuerySessionBoundReadsAndEffects<'run> {
    pub(super) _run: WorthQueryProviderRunBorrow<'run>,
    pub(super) contract: WorthQueryProviderExecutionPlanContract,
    pub(super) session: WorthQueryProviderSessionLease,
    pub(super) binding: WorthQuerySessionBinding,
    pub(super) counters: WorthQueryProviderSessionProtocolCounters,
}

impl std::fmt::Debug for WorthQuerySessionBoundReadsAndEffects<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQuerySessionBoundReadsAndEffects")
            .field("plan_identity", &self.contract.identity())
            .field("token_identity", &self.session.token().identity())
            .finish_non_exhaustive()
    }
}

impl<'run> WorthQueryPreparedProviderSession<'run> {
    pub fn bind_reads_and_effects(self) -> WorthQuerySessionBoundReadsAndEffects<'run> {
        let binding = WorthQuerySessionBinding::seal(self.session.token(), &self.contract);
        WorthQuerySessionBoundReadsAndEffects {
            _run: self.run,
            contract: self.contract,
            session: self.session,
            binding,
            counters: self.counters,
        }
    }
}

pub struct WorthQuerySessionReadAuthority<'session> {
    binding: &'session WorthQuerySessionBinding,
    plan: &'session WorthQueryProviderExecutionPlanContract,
    provider: &'session WorthQueryGraphProviderAnchor,
    session: WorthQueryProviderSessionView<'session>,
    _invariant: PhantomData<fn(&'session mut ()) -> &'session mut ()>,
}

pub struct WorthQuerySessionEffectAuthority<'session> {
    binding: &'session WorthQuerySessionBinding,
    plan: &'session WorthQueryProviderExecutionPlanContract,
    _invariant: PhantomData<fn(&'session mut ()) -> &'session mut ()>,
}

impl WorthQuerySessionBoundReadsAndEffects<'_> {
    pub fn read_authority(&self) -> WorthQuerySessionReadAuthority<'_> {
        WorthQuerySessionReadAuthority {
            binding: &self.binding,
            plan: &self.contract,
            provider: self.session.provider(),
            session: self.session.token().view(),
            _invariant: PhantomData,
        }
    }

    pub fn effect_authority(&self) -> WorthQuerySessionEffectAuthority<'_> {
        WorthQuerySessionEffectAuthority {
            binding: &self.binding,
            plan: &self.contract,
            _invariant: PhantomData,
        }
    }

    pub fn plan(&self) -> &WorthQueryProviderExecutionPlanContract {
        &self.contract
    }

    pub fn token_identity(&self) -> &str {
        self.binding.token_identity()
    }

    pub fn token_generation(&self) -> u64 {
        self.binding.token_generation()
    }

    pub fn counters(&self) -> WorthQueryProviderSessionProtocolCounters {
        self.counters
    }

    pub(crate) fn provisional_binding_identity(&self) -> &str {
        self.binding.canonical_identity()
    }

    pub(crate) fn provisional_provider(&self) -> &WorthQueryGraphProviderAnchor {
        self.session.provider()
    }

    pub(crate) fn provisional_provider_arc(&self) -> Arc<WorthQueryGraphProviderAnchor> {
        self.session.provider_arc()
    }

    pub(crate) fn provider_session_view(&self) -> WorthQueryProviderSessionView<'_> {
        self.session.token().view()
    }
}

impl WorthQuerySessionReadAuthority<'_> {
    pub fn token_identity(&self) -> &str {
        self.binding.token_identity()
    }

    pub fn token_generation(&self) -> u64 {
        self.binding.token_generation()
    }

    pub(crate) fn binding(&self) -> &WorthQuerySessionBinding {
        self.binding
    }

    pub(crate) fn plan(&self) -> &WorthQueryProviderExecutionPlanContract {
        self.plan
    }

    pub(crate) fn provider(&self) -> &WorthQueryGraphProviderAnchor {
        self.provider
    }

    pub(crate) fn session(&self) -> WorthQueryProviderSessionView<'_> {
        self.session
    }
}

impl WorthQuerySessionEffectAuthority<'_> {
    pub fn token_identity(&self) -> &str {
        self.binding.token_identity()
    }

    pub fn token_generation(&self) -> u64 {
        self.binding.token_generation()
    }

    pub(crate) fn binding(&self) -> &WorthQuerySessionBinding {
        self.binding
    }

    pub(crate) fn plan(&self) -> &WorthQueryProviderExecutionPlanContract {
        self.plan
    }
}
