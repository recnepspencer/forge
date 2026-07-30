#[path = "schedule/decision.rs"]
mod decision;
#[path = "schedule/seed.rs"]
mod seed;
#[path = "schedule/trace.rs"]
mod trace;

pub(super) use decision::ScheduleDecision;
#[cfg(test)]
use seed::revision_schedule_seeds;
pub(super) use seed::{RevisionScheduleSeeds, ScheduleSeed};
pub(super) use trace::ScheduleDecisionTrace;

pub(super) fn parse_executed_trace(encoded: &str) -> Result<[ScheduleDecision; 4], String> {
    decision::parse_trace(encoded)
}

pub(super) const CANONICAL_SCHEDULE_SEED: ScheduleSeed = ScheduleSeed::new(0x6c4f_7363_6865_6475);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SchedulePerturbationPlan {
    seed: ScheduleSeed,
    trace: ScheduleDecisionTrace,
}

impl SchedulePerturbationPlan {
    pub(super) fn derive(seed: ScheduleSeed) -> Self {
        let decisions = decision::derive(seed);
        Self {
            seed,
            trace: ScheduleDecisionTrace::new(seed, decisions),
        }
    }

    pub(super) fn canonical() -> Self {
        Self::derive(CANONICAL_SCHEDULE_SEED)
    }

    pub(super) const fn seed(&self) -> ScheduleSeed {
        self.seed
    }

    pub(super) const fn trace(&self) -> &ScheduleDecisionTrace {
        &self.trace
    }

    pub(super) fn child_argument(&self) -> String {
        self.trace
            .decisions()
            .iter()
            .map(ScheduleDecision::encoded)
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[cfg(test)]
mod tests {
    use super::{revision_schedule_seeds, SchedulePerturbationPlan, ScheduleSeed};

    #[test]
    fn all_default_decisions_are_one_lawful_seed_selected_schedule() {
        let plan = SchedulePerturbationPlan::derive(ScheduleSeed::new(0));
        assert_eq!(plan.trace().decisions()[0].choice(), "first-then-second");
        assert_eq!(plan.trace().decisions()[1].choice(), "first-owner");
        assert_eq!(plan.trace().decisions()[2].choice(), "owner-then-waiter");
        assert_eq!(
            plan.trace().decisions()[3].choice(),
            "first-worker-then-second"
        );
    }

    #[test]
    fn schedule_seed_changes_the_closed_decision_trace() {
        let first = SchedulePerturbationPlan::derive(ScheduleSeed::new(1));
        let second = SchedulePerturbationPlan::derive(ScheduleSeed::new(2));
        if first.trace().decisions() == second.trace().decisions() {
            panic!("MUTANT_PREDICATE:schedule-seed-ignored");
        }
        assert_ne!(first.trace().digest(), second.trace().digest());
    }

    #[test]
    fn one_revision_bijects_sixteen_lanes_to_every_decision_trace() {
        let seeds = revision_schedule_seeds("659c5775de9637a2f8a7c1c7d7205d0851ada683")
            .unwrap_or_else(|_| panic!("MUTANT_PREDICATE:revision-schedule-lanes-collapse"));
        assert_eq!(seeds.len(), 16);
        let distinct_seeds = seeds
            .iter()
            .map(|seed| seed.value())
            .collect::<std::collections::BTreeSet<_>>();
        let distinct_traces = seeds
            .iter()
            .map(|seed| {
                SchedulePerturbationPlan::derive(*seed)
                    .trace()
                    .decisions()
                    .map(|decision| decision.choice())
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(distinct_seeds.len(), 16);
        assert_eq!(distinct_traces.len(), 16);
    }
}
