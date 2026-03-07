use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::dirty_set::{BatchedDirtySet, DomainImpact};
use crate::data::effect_mapping::EffectMapping;
use crate::data::error::SignalError;
use crate::data::evaluator::CheckpointEvaluator;

use super::runtime::CheckpointRuntime;

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
    ) -> Result<(), SignalError> {
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
