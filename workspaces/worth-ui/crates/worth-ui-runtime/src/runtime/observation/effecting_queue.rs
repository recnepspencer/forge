pub(crate) struct UiEffectingObservationQueue {
    capacity: usize,
    admitted_observations: usize,
    sets: Vec<super::UiAdmittedObservationSet>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEffectingObservationQueueAdmissionReceipt {
    admitted_observations: usize,
    total_queued_observations: usize,
    remaining_capacity: usize,
}

#[must_use = "a capacity stop returns the exact admitted observation set"]
pub struct UiEffectingObservationQueueCapacityStop {
    configured: usize,
    observed: usize,
    attempted: usize,
    set: Box<super::UiAdmittedObservationSet>,
}

impl UiEffectingObservationQueue {
    pub(crate) fn new(capacity: usize) -> Self {
        debug_assert!(capacity > 0, "validated observation capacity is nonzero");
        Self {
            capacity,
            admitted_observations: 0,
            sets: Vec::new(),
        }
    }

    pub(crate) fn admit(
        &mut self,
        set: super::UiAdmittedObservationSet,
    ) -> Result<UiEffectingObservationQueueAdmissionReceipt, UiEffectingObservationQueueCapacityStop>
    {
        let admitted = set.summary().admitted_count();
        let attempted = self
            .admitted_observations
            .checked_add(admitted)
            .expect("bounded effecting observation count does not exhaust");
        if attempted > self.capacity {
            return Err(UiEffectingObservationQueueCapacityStop {
                configured: self.capacity,
                observed: self.admitted_observations,
                attempted,
                set: Box::new(set),
            });
        }
        self.admitted_observations = attempted;
        self.sets.push(set);
        Ok(UiEffectingObservationQueueAdmissionReceipt {
            admitted_observations: admitted,
            total_queued_observations: attempted,
            remaining_capacity: self.capacity - attempted,
        })
    }

    pub(crate) fn into_sets(self) -> Box<[super::UiAdmittedObservationSet]> {
        self.sets.into_boxed_slice()
    }

    pub(crate) fn admitted_observation_count(&self) -> usize {
        self.admitted_observations
    }
}

impl UiEffectingObservationQueueAdmissionReceipt {
    pub const fn admitted_observations(self) -> usize {
        self.admitted_observations
    }

    pub const fn total_queued_observations(self) -> usize {
        self.total_queued_observations
    }

    pub const fn remaining_capacity(self) -> usize {
        self.remaining_capacity
    }
}

impl UiEffectingObservationQueueCapacityStop {
    pub const fn configured(&self) -> usize {
        self.configured
    }

    pub const fn observed(&self) -> usize {
        self.observed
    }

    pub const fn attempted(&self) -> usize {
        self.attempted
    }

    pub fn into_observation_set(self) -> super::UiAdmittedObservationSet {
        *self.set
    }
}
