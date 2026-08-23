use super::protocol_world::{
    UiNativeLifecycleEvent, UiNativeLifecycleObservation, UiNativeLifecycleState,
    UiNativeLifecycleWorld,
};

pub(super) struct CausalEventMutation<'a> {
    pub(super) initial: UiNativeLifecycleState,
    pub(super) schedule: &'a [UiNativeLifecycleEvent],
    pub(super) action_index: usize,
    pub(super) replacement: UiNativeLifecycleEvent,
}

pub(super) struct CausalMutationResult {
    baseline: Vec<UiNativeLifecycleObservation>,
    mutant: Vec<UiNativeLifecycleObservation>,
    first_divergence: usize,
}

impl CausalEventMutation<'_> {
    pub(super) fn run(self) -> CausalMutationResult {
        assert!(self.action_index < self.schedule.len());
        assert_ne!(self.schedule[self.action_index], self.replacement);
        let baseline = execute(self.initial, self.schedule, None);
        let mutant = execute(
            self.initial,
            self.schedule,
            Some((self.action_index, self.replacement)),
        );
        let first_divergence = baseline
            .iter()
            .zip(&mutant)
            .position(|(expected, observed)| expected != observed)
            .expect("a causal action replacement must be observable");
        assert_eq!(first_divergence, self.action_index);
        CausalMutationResult {
            baseline,
            mutant,
            first_divergence,
        }
    }
}

impl CausalMutationResult {
    pub(super) fn baseline_at_divergence(&self) -> UiNativeLifecycleObservation {
        self.baseline[self.first_divergence]
    }

    pub(super) fn mutant_at_divergence(&self) -> UiNativeLifecycleObservation {
        self.mutant[self.first_divergence]
    }

    pub(super) fn baseline_final(&self) -> UiNativeLifecycleObservation {
        *self.baseline.last().expect("non-empty causal schedule")
    }

    pub(super) fn mutant_final(&self) -> UiNativeLifecycleObservation {
        *self.mutant.last().expect("non-empty causal schedule")
    }
}

fn execute(
    initial: UiNativeLifecycleState,
    schedule: &[UiNativeLifecycleEvent],
    replacement: Option<(usize, UiNativeLifecycleEvent)>,
) -> Vec<UiNativeLifecycleObservation> {
    let mut world = UiNativeLifecycleWorld::new(initial);
    schedule
        .iter()
        .copied()
        .enumerate()
        .map(|(index, event)| {
            let event = replacement
                .filter(|(replacement_index, _)| *replacement_index == index)
                .map_or(event, |(_, replacement)| replacement);
            world.apply(event)
        })
        .collect()
}
