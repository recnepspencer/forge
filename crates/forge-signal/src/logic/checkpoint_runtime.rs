//! Tier-0 checkpoint runtime for batched domain refresh.

use forge_core::KernelError;

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
use crate::data::effect_mapping::EffectMapping;
use crate::data::evaluator::CheckpointEvaluator;

/// Runtime state for batched Tier-0 signal scheduling.
#[derive(Debug, Clone)]
pub struct CheckpointRuntime<D: Copy + Ord, I: Copy + Ord> {
    dirty: BatchedDirtySet<D, I>,
    policy: CheckpointPolicy<D>,
}

impl<D: Copy + Ord, I: Copy + Ord> CheckpointRuntime<D, I> {
    /// Create a new runtime with a per-domain checkpoint policy.
    pub fn new(policy: CheckpointPolicy<D>) -> Self {
        Self {
            dirty: BatchedDirtySet::new(),
            policy,
        }
    }

    /// Read-only dirty state.
    pub fn dirty(&self) -> &BatchedDirtySet<D, I> {
        &self.dirty
    }

    /// Mutable dirty state.
    pub fn dirty_mut(&mut self) -> &mut BatchedDirtySet<D, I> {
        &mut self.dirty
    }

    /// Read-only checkpoint policy.
    pub fn policy(&self) -> &CheckpointPolicy<D> {
        &self.policy
    }

    /// Mutable checkpoint policy.
    pub fn policy_mut(&mut self) -> &mut CheckpointPolicy<D> {
        &mut self.policy
    }

    /// Route one effect into this runtime's dirty set.
    pub fn record_effect<M>(&mut self, effect: &M::Effect)
    where
        M: EffectMapping<Domain = D, Impact = I>,
    {
        M::route(effect, &mut self.dirty);
    }

    /// Mark one domain globally dirty.
    pub fn mark_domain_global(&mut self, domain: D) {
        self.dirty.mark_domain_global(domain);
    }

    /// Mark one scoped impact dirty.
    pub fn mark_domain_scoped(&mut self, domain: D, impact: I) {
        self.dirty.mark_domain_scoped(domain, impact);
    }

    /// Flush domains scheduled for this barrier.
    ///
    /// Returns number of refreshed domains.
    pub fn flush<E>(
        &mut self,
        barrier: CheckpointBarrier,
        evaluator: &mut E,
        ctx: &mut E::Context,
    ) -> Result<usize, KernelError>
    where
        E: CheckpointEvaluator<Domain = D, Impact = I>,
    {
        let domains: Vec<D> = self
            .dirty
            .dirty_domains()
            .filter(|domain| self.policy.barrier_for(*domain) == barrier)
            .collect();

        for domain in &domains {
            let impact = self
                .dirty
                .take_domain_impact(*domain)
                .unwrap_or_else(DomainImpact::empty);
            evaluator.refresh(*domain, impact, ctx)?;
        }

        Ok(domains.len())
    }

    /// Ensure one domain is refreshed immediately regardless of barrier policy.
    ///
    /// Returns `Ok(true)` if a refresh ran.
    pub fn ensure_fresh<E>(
        &mut self,
        domain: D,
        evaluator: &mut E,
        ctx: &mut E::Context,
    ) -> Result<bool, KernelError>
    where
        E: CheckpointEvaluator<Domain = D, Impact = I>,
    {
        let Some(impact) = self.dirty.take_domain_impact(domain) else {
            return Ok(false);
        };
        evaluator.refresh(domain, impact, ctx)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckpointRuntime;
    use crate::data::checkpoint::CheckpointBarrier;
    use crate::data::checkpoint_policy::CheckpointPolicy;
    use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
    use crate::data::effect_mapping::EffectMapping;
    use crate::data::evaluator::CheckpointEvaluator;
    use forge_core::KernelError;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Domain {
        Alpha,
        Beta,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Impact {
        One,
        Two,
    }

    #[derive(Debug, Clone)]
    enum Effect {
        TouchAlpha(Impact),
        DirtyBetaGlobal,
    }

    struct Router;

    impl EffectMapping for Router {
        type Domain = Domain;
        type Effect = Effect;
        type Impact = Impact;

        fn route(effect: &Self::Effect, sink: &mut BatchedDirtySet<Self::Domain, Self::Impact>) {
            match effect {
                Effect::TouchAlpha(i) => sink.mark_domain_scoped(Domain::Alpha, *i),
                Effect::DirtyBetaGlobal => sink.mark_domain_global(Domain::Beta),
            }
        }
    }

    #[derive(Default)]
    struct Recorder {
        refreshed: Vec<(Domain, bool, Vec<Impact>)>,
    }

    impl CheckpointEvaluator for Recorder {
        type Domain = Domain;
        type Impact = Impact;
        type Context = ();

        fn refresh(
            &mut self,
            domain: Self::Domain,
            impact: DomainImpact<Self::Impact>,
            _ctx: &mut Self::Context,
        ) -> Result<(), KernelError> {
            self.refreshed
                .push((domain, impact.is_global(), impact.scoped().collect()));
            Ok(())
        }
    }

    #[test]
    fn flush_respects_barrier_policy_and_order() {
        let mut policy = CheckpointPolicy::new(CheckpointBarrier::PerOperation);
        policy.set_barrier(Domain::Beta, CheckpointBarrier::PerCommit);
        let mut runtime = CheckpointRuntime::new(policy);
        let mut evaluator = Recorder::default();
        let mut ctx = ();

        runtime.record_effect::<Router>(&Effect::TouchAlpha(Impact::Two));
        runtime.record_effect::<Router>(&Effect::TouchAlpha(Impact::One));
        runtime.record_effect::<Router>(&Effect::DirtyBetaGlobal);

        let refreshed = runtime
            .flush(CheckpointBarrier::PerOperation, &mut evaluator, &mut ctx)
            .unwrap();
        assert_eq!(refreshed, 1);
        assert_eq!(evaluator.refreshed.len(), 1);
        assert_eq!(evaluator.refreshed[0].0, Domain::Alpha);
        assert!(!evaluator.refreshed[0].1);
        assert_eq!(evaluator.refreshed[0].2, vec![Impact::One, Impact::Two]);

        let refreshed = runtime
            .flush(CheckpointBarrier::PerCommit, &mut evaluator, &mut ctx)
            .unwrap();
        assert_eq!(refreshed, 1);
        assert_eq!(evaluator.refreshed.len(), 2);
        assert_eq!(evaluator.refreshed[1].0, Domain::Beta);
        assert!(evaluator.refreshed[1].1);
    }

    #[test]
    fn ensure_fresh_forces_single_domain_refresh() {
        let policy = CheckpointPolicy::new(CheckpointBarrier::PerCommit);
        let mut runtime = CheckpointRuntime::new(policy);
        let mut evaluator = Recorder::default();
        let mut ctx = ();

        runtime.record_effect::<Router>(&Effect::TouchAlpha(Impact::One));
        let did_refresh = runtime
            .ensure_fresh(Domain::Alpha, &mut evaluator, &mut ctx)
            .unwrap();
        assert!(did_refresh);
        assert_eq!(evaluator.refreshed.len(), 1);
        assert_eq!(evaluator.refreshed[0].0, Domain::Alpha);
    }
}
