use worth_store::physical_runtime::{
    PhysicalWorkCounterSnapshot, PhysicalWorkCounterStage, PhysicalWorkIdentity,
    PhysicalWorkOperationFamily, RecordReadObservation, ServingPhysicalRuntime,
};

#[derive(Default)]
pub(super) struct ReadWorkIdentitySpan {
    count: u64,
    first: Option<PhysicalWorkIdentity>,
    last: Option<PhysicalWorkIdentity>,
}

pub(super) struct WorkCounterDelta {
    before: PhysicalWorkCounterSnapshot,
    after: PhysicalWorkCounterSnapshot,
}

impl ReadWorkIdentitySpan {
    pub(super) fn observe(
        &mut self,
        serving: &ServingPhysicalRuntime,
        observation: RecordReadObservation,
    ) -> Result<(), String> {
        let count = observation.physical_work_count();
        let endpoints = (
            observation.first_physical_work(),
            observation.last_physical_work(),
        );
        let (Some(first), Some(last)) = endpoints else {
            return if count == 0 {
                Ok(())
            } else {
                Err("C.6 read work omitted an identity endpoint".to_owned())
            };
        };
        if count == 0
            || !work_belongs_to_runtime(serving, first)
            || !work_belongs_to_runtime(serving, last)
            || operation_span(first, last) != Some(count)
            || self.last.is_some_and(|previous| {
                previous.operation().get().checked_add(1) != Some(first.operation().get())
            })
        {
            return Err("C.6 read work identities were foreign or discontinuous".to_owned());
        }
        self.count = self.count.saturating_add(count);
        self.first = self.first.or(Some(first));
        self.last = Some(last);
        Ok(())
    }

    pub(super) const fn count(&self) -> u64 {
        self.count
    }

    pub(super) const fn first(&self) -> Option<PhysicalWorkIdentity> {
        self.first
    }

    pub(super) const fn last(&self) -> Option<PhysicalWorkIdentity> {
        self.last
    }
}

impl WorkCounterDelta {
    pub(super) const fn new(
        before: PhysicalWorkCounterSnapshot,
        after: PhysicalWorkCounterSnapshot,
    ) -> Self {
        Self { before, after }
    }

    pub(super) fn count(
        &self,
        family: PhysicalWorkOperationFamily,
        stage: PhysicalWorkCounterStage,
    ) -> Result<u64, String> {
        self.after
            .count(family, stage)
            .checked_sub(self.before.count(family, stage))
            .ok_or_else(|| format!("C.6 {family:?} {stage:?} counter regressed"))
    }
}

pub(super) fn work_belongs_to_runtime(
    serving: &ServingPhysicalRuntime,
    identity: PhysicalWorkIdentity,
) -> bool {
    identity.store() == serving.store_identity()
        && identity.runtime() == serving.runtime_identity()
        && identity.generation().lifecycle() == serving.residency_observation().store_generation()
}

fn operation_span(first: PhysicalWorkIdentity, last: PhysicalWorkIdentity) -> Option<u64> {
    last.operation()
        .get()
        .checked_sub(first.operation().get())
        .and_then(|difference| difference.checked_add(1))
}
