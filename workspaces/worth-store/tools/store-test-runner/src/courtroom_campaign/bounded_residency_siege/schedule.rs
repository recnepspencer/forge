#[path = "schedule/decision.rs"]
mod decision;

pub(super) use decision::{DurabilityCheckpointOrder, ScheduleDecision};
pub(super) use worth_store_physical_certification::{
    C7DurabilityCrashSeam, SchedulePerturbationSeed, SchedulePerturbationTrace,
    SourceClosureScheduleSeeds,
};

pub(super) fn parse_executed_trace(encoded: &str) -> Result<[ScheduleDecision; 4], String> {
    decision::parse_trace(encoded)
}

pub(super) const CANONICAL_SCHEDULE_SEED: SchedulePerturbationSeed =
    SchedulePerturbationSeed::from_u64(0x6c4f_7363_6865_6475);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SchedulePerturbationPlan {
    seed: SchedulePerturbationSeed,
    decisions: [ScheduleDecision; 5],
    trace: SchedulePerturbationTrace,
}

impl SchedulePerturbationPlan {
    pub(super) fn derive(seed: SchedulePerturbationSeed) -> Self {
        let decisions = decision::derive(seed);
        let trace = SchedulePerturbationTrace::new(
            seed,
            decisions
                .iter()
                .copied()
                .map(ScheduleDecision::canonical_trace_decision),
        )
        .expect("the closed Courtroom C decision vocabulary is canonical");
        Self {
            seed,
            decisions,
            trace,
        }
    }

    pub(super) fn canonical() -> Self {
        Self::derive(CANONICAL_SCHEDULE_SEED)
    }

    pub(super) const fn seed(&self) -> SchedulePerturbationSeed {
        self.seed
    }

    pub(super) const fn trace(&self) -> &SchedulePerturbationTrace {
        &self.trace
    }

    pub(super) fn serving_decisions(&self) -> &[ScheduleDecision; 4] {
        self.decisions[..4]
            .try_into()
            .expect("the canonical schedule begins with four serving decisions")
    }

    pub(super) const fn durability_checkpoint_order(&self) -> DurabilityCheckpointOrder {
        match self.decisions[4] {
            ScheduleDecision::DurabilityCheckpointOrder(order) => order,
            _ => unreachable!(),
        }
    }

    pub(super) fn child_argument(&self) -> String {
        self.decisions
            .iter()
            .take(4)
            .map(ScheduleDecision::encoded)
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        C7DurabilityCrashSeam, SchedulePerturbationPlan, SchedulePerturbationSeed,
        SourceClosureScheduleSeeds,
    };

    #[test]
    fn all_default_decisions_are_one_lawful_seed_selected_schedule() {
        let plan = SchedulePerturbationPlan::derive(SchedulePerturbationSeed::from_u64(0));
        assert_eq!(plan.trace().decisions()[0].choice(), "first-then-second");
        assert_eq!(plan.trace().decisions()[1].choice(), "first-owner");
        assert_eq!(plan.trace().decisions()[2].choice(), "owner-then-waiter");
        assert_eq!(
            plan.trace().decisions()[3].choice(),
            "first-worker-then-second"
        );
        assert_eq!(
            plan.trace().decisions()[4].choice(),
            "checkpoint-before-target"
        );
    }

    #[test]
    fn schedule_seed_changes_the_closed_decision_trace() {
        let first = SchedulePerturbationPlan::derive(SchedulePerturbationSeed::from_u64(1));
        let second = SchedulePerturbationPlan::derive(SchedulePerturbationSeed::from_u64(2));
        if first.trace().decisions() == second.trace().decisions() {
            panic!("MUTANT_PREDICATE:schedule-seed-ignored");
        }
        assert_ne!(first.trace().digest(), second.trace().digest());
    }

    #[test]
    fn one_source_closure_derives_sixteen_distinct_replayable_lane_seeds() {
        let seeds = SourceClosureScheduleSeeds::derive([0x65; 32])
            .unwrap_or_else(|_| panic!("MUTANT_PREDICATE:source-closure-schedule-lanes-collapse"));
        let distinct_seeds = seeds
            .seeds()
            .iter()
            .map(|seed| seed.value())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(distinct_seeds.len(), 16);
        for seed in seeds.seeds() {
            let first = SchedulePerturbationPlan::derive(*seed);
            let replay = SchedulePerturbationPlan::derive(*seed);
            assert_eq!(first.trace(), replay.trace());
        }
    }

    #[test]
    fn source_closure_seed_domain_is_c7_specific_and_stable() {
        let seeds = SourceClosureScheduleSeeds::derive([0x65; 32]).unwrap();
        if seeds.seed(0).map(SchedulePerturbationSeed::value) != Some(4_683_625_316_395_142_225) {
            panic!("MUTANT_PREDICATE:c7-source-schedule-domain-drifted");
        }
    }

    #[test]
    fn sixteen_ci_lanes_cover_every_explicit_c7_crash_seam_twice() {
        let lanes = SourceClosureScheduleSeeds::derive([0x65; 32]).unwrap();
        for seam in C7DurabilityCrashSeam::ALL {
            let count = (0..16)
                .filter(|lane| lanes.crash_seam(*lane) == Some(seam))
                .count();
            if count != 2 {
                panic!("MUTANT_PREDICATE:c7-crash-seam-rotation-collapsed");
            }
        }
    }
}
