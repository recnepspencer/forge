use worth_proof::{CanonicalVec, NonEmpty, UniqueVec};

use super::{PhysicalActorId, PhysicalActorStep, ScheduleReplayDenial};

#[derive(Debug, PartialEq, Eq)]
pub struct PhysicalActorStepSequence {
    steps: NonEmpty<PhysicalActorStep>,
    canonical_steps: CanonicalVec<PhysicalActorStep>,
    unique_actor_ids: UniqueVec<PhysicalActorId>,
}

impl Clone for PhysicalActorStepSequence {
    fn clone(&self) -> Self {
        Self::from_steps(self.steps.as_slice().to_vec())
            .expect("existing actor step sequence preserves canonical proofs")
    }
}

impl PhysicalActorStepSequence {
    pub(crate) fn from_steps(steps: Vec<PhysicalActorStep>) -> Result<Self, ScheduleReplayDenial> {
        let non_empty_steps = NonEmpty::try_from_vec(steps.clone())
            .map_err(|_| ScheduleReplayDenial::EmptyActorStepSchedule)?;
        let canonical_steps = CanonicalVec::try_from_sorted(steps.clone())
            .map_err(|_| ScheduleReplayDenial::NonCanonicalActorStepOrder)?;
        let actor_ids = steps
            .iter()
            .map(|step| step.actor_id_proof().clone())
            .collect();
        let unique_actor_ids = UniqueVec::try_from_unique(actor_ids)
            .map_err(|_| ScheduleReplayDenial::DuplicateActorStepActorId)?;
        Ok(Self {
            steps: non_empty_steps,
            canonical_steps,
            unique_actor_ids,
        })
    }

    pub fn as_slice(&self) -> &[PhysicalActorStep] {
        self.steps.as_slice()
    }

    pub fn canonical_steps(&self) -> &[PhysicalActorStep] {
        self.canonical_steps.as_slice()
    }

    pub fn unique_actor_ids(&self) -> &[PhysicalActorId] {
        self.unique_actor_ids.as_slice()
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}
