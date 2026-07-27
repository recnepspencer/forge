use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::{
    WorthQueryAdmittedInvariantStateLoadPlan, WorthQueryInvariantExecutionDenialKind,
    WorthQueryInvariantExecutionFailure,
};
use crate::execution_digest::hash_parts;

static NEXT_INVARIANT_EVIDENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryInvariantStructuralCounters {
    loaded_facts: usize,
    load_work_units: u64,
    execution_work_units: u64,
}

impl WorthQueryInvariantStructuralCounters {
    pub fn new(loaded_facts: usize, load_work_units: u64, execution_work_units: u64) -> Self {
        Self {
            loaded_facts,
            load_work_units,
            execution_work_units,
        }
    }

    pub fn loaded_facts(self) -> usize {
        self.loaded_facts
    }

    pub fn load_work_units(self) -> u64 {
        self.load_work_units
    }

    pub fn execution_work_units(self) -> u64 {
        self.execution_work_units
    }
}

#[derive(Clone, Copy)]
pub struct WorthQueryInvariantStateLoadRequestView<'a> {
    plan: &'a WorthQueryAdmittedInvariantStateLoadPlan,
}

impl<'a> WorthQueryInvariantStateLoadRequestView<'a> {
    pub(crate) fn new(plan: &'a WorthQueryAdmittedInvariantStateLoadPlan) -> Self {
        Self { plan }
    }

    pub fn plan_identity(self) -> &'a str {
        self.plan.identity()
    }

    pub fn locators(self) -> &'a [super::WorthQueryInvariantStateLocator] {
        self.plan.locators()
    }
}

pub struct WorthQueryInvariantStateLoadAdmission {
    binding: WorthQueryInvariantStateLoadBinding,
    expected_locators: Arc<[super::WorthQueryInvariantStateLocator]>,
    max_state_facts: usize,
    max_work_units: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryInvariantStateLoadBinding {
    pub(super) session_binding_identity: Arc<str>,
    pub(super) requirement_identity: Arc<str>,
    pub(super) proposed_state_identity: Arc<str>,
    pub(super) attempt_generation: u64,
    pub(super) load_plan_identity: Arc<str>,
}

impl WorthQueryInvariantStateLoadAdmission {
    pub(super) fn new(
        binding: WorthQueryInvariantStateLoadBinding,
        plan: &WorthQueryAdmittedInvariantStateLoadPlan,
        max_state_facts: usize,
        max_work_units: u64,
    ) -> Self {
        Self {
            binding,
            expected_locators: plan.locators().into(),
            max_state_facts,
            max_work_units,
        }
    }

    pub fn admit(
        self,
        physical_load_evidence: impl Into<Arc<str>>,
        loaded_fact_locators: impl IntoIterator<Item = super::WorthQueryInvariantStateLocator>,
        counters: WorthQueryInvariantStructuralCounters,
    ) -> Result<WorthQueryInvariantStateLoadEvidence, WorthQueryInvariantExecutionFailure> {
        let physical_load_evidence = canonical(physical_load_evidence)?;
        let mut loaded = loaded_fact_locators.into_iter().collect::<Vec<_>>();
        loaded.sort();
        loaded.dedup();
        if loaded.is_empty() || counters.loaded_facts() != loaded.len() {
            return Err(failure(
                WorthQueryInvariantExecutionDenialKind::EmptyStateLoad,
            ));
        }
        if loaded.as_slice() != self.expected_locators.as_ref() {
            return Err(failure(
                WorthQueryInvariantExecutionDenialKind::StateLoadClosureMismatch,
            ));
        }
        if loaded.len() > self.max_state_facts {
            return Err(exhausted(
                WorthQueryInvariantExecutionDenialKind::StateLoadBudgetExceeded,
            ));
        }
        if counters.load_work_units() > self.max_work_units {
            return Err(exhausted(
                WorthQueryInvariantExecutionDenialKind::ExecutionBudgetExceeded,
            ));
        }
        let occurrence = NEXT_INVARIANT_EVIDENCE.fetch_add(1, Ordering::Relaxed);
        let identity = hash_parts(
            &[
                vec![
                    "worth_query_invariant_state_load_evidence_v1".to_owned(),
                    self.binding.session_binding_identity.to_string(),
                    self.binding.requirement_identity.to_string(),
                    self.binding.proposed_state_identity.to_string(),
                    self.binding.attempt_generation.to_string(),
                    self.binding.load_plan_identity.to_string(),
                    physical_load_evidence.to_string(),
                    counters.loaded_facts().to_string(),
                    counters.load_work_units().to_string(),
                    occurrence.to_string(),
                ],
                std::iter::once(loaded.len().to_string())
                    .chain(loaded.iter().flat_map(|locator| {
                        [locator.family().to_owned(), locator.identity().to_owned()]
                    }))
                    .collect(),
            ]
            .concat(),
        );
        Ok(WorthQueryInvariantStateLoadEvidence {
            identity: identity.into(),
            binding: self.binding,
            physical_load_evidence,
            loaded_fact_locators: loaded.into(),
            counters,
        })
    }
}

pub struct WorthQueryInvariantStateLoadEvidence {
    identity: Arc<str>,
    binding: WorthQueryInvariantStateLoadBinding,
    physical_load_evidence: Arc<str>,
    loaded_fact_locators: Arc<[super::WorthQueryInvariantStateLocator]>,
    counters: WorthQueryInvariantStructuralCounters,
}

impl WorthQueryInvariantStateLoadEvidence {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn view(&self) -> WorthQueryInvariantStateLoadEvidenceView<'_> {
        WorthQueryInvariantStateLoadEvidenceView { evidence: self }
    }

    pub(super) fn belongs_to(&self, expected: &WorthQueryInvariantStateLoadBinding) -> bool {
        &self.binding == expected
    }

    pub(crate) fn counters(&self) -> WorthQueryInvariantStructuralCounters {
        self.counters
    }
}

#[derive(Clone, Copy)]
pub struct WorthQueryInvariantStateLoadEvidenceView<'a> {
    evidence: &'a WorthQueryInvariantStateLoadEvidence,
}

impl<'a> WorthQueryInvariantStateLoadEvidenceView<'a> {
    pub fn identity(self) -> &'a str {
        self.evidence.identity()
    }

    pub fn loaded_fact_locators(self) -> &'a [super::WorthQueryInvariantStateLocator] {
        &self.evidence.loaded_fact_locators
    }

    pub fn physical_load_evidence(self) -> &'a str {
        &self.evidence.physical_load_evidence
    }
}

pub trait WorthQueryInvariantExecutionProvider: Send + Sync + 'static {
    fn load_invariant_state(
        &self,
        session: crate::domain_computation::provider_session::WorthQueryProviderSessionView<'_>,
        request: WorthQueryInvariantStateLoadRequestView<'_>,
        admission: WorthQueryInvariantStateLoadAdmission,
    ) -> Result<WorthQueryInvariantStateLoadEvidence, WorthQueryInvariantExecutionFailure>;

    fn execute_invariant(
        &self,
        session: crate::domain_computation::provider_session::WorthQueryProviderSessionView<'_>,
        execution: super::WorthQueryBoundInvariantExecutionView<'_>,
        admission: super::WorthQueryInvariantVerdictAdmission,
    ) -> Result<super::WorthQueryInvariantProviderVerdict, WorthQueryInvariantExecutionFailure>;
}

fn canonical(value: impl Into<Arc<str>>) -> Result<Arc<str>, WorthQueryInvariantExecutionFailure> {
    let value = value.into();
    if value.trim().is_empty() || value.trim() != value.as_ref() {
        Err(failure(
            WorthQueryInvariantExecutionDenialKind::ProviderRejected,
        ))
    } else {
        Ok(value)
    }
}

fn failure(kind: WorthQueryInvariantExecutionDenialKind) -> WorthQueryInvariantExecutionFailure {
    WorthQueryInvariantExecutionFailure::new(kind, "invariant provider evidence denied")
}

fn exhausted(kind: WorthQueryInvariantExecutionDenialKind) -> WorthQueryInvariantExecutionFailure {
    WorthQueryInvariantExecutionFailure::exhausted(kind, "invariant budget exhausted")
}
