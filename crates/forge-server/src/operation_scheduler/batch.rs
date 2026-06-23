use crate::{ForgeServerLoweredOperationPlan, ForgeServerResponseFacade};

use super::{
    mutation_executor::execute_slots as execute_ordered_slots,
    shared_read_executor::execute_slots as execute_shared_read_slots,
    ForgeServerExecutedOperationBatch, ForgeServerOperationExecutionSlot,
    ForgeServerSchedulerCancellationDirective, ForgeServerSchedulerCertificationSabotage,
    ForgeServerSchedulerConflictDenial, ForgeServerSchedulerConflictDenialFacts,
    ForgeServerSchedulerLane,
};

#[derive(Debug)]
pub struct ForgeServerScheduledOperationBatch {
    responses: ForgeServerResponseFacade,
    slots: Vec<ForgeServerOperationExecutionSlot>,
}

impl ForgeServerScheduledOperationBatch {
    pub(crate) fn new(
        responses: ForgeServerResponseFacade,
        plans: impl IntoIterator<Item = ForgeServerLoweredOperationPlan>,
    ) -> Result<Self, ForgeServerSchedulerConflictDenial> {
        let plans = plans.into_iter().collect::<Vec<_>>();
        let planned_batch_width = plans.len();
        let slots = plans
            .into_iter()
            .enumerate()
            .map(|(ordinal, plan)| {
                ForgeServerOperationExecutionSlot::from_lowered_plan(ordinal, plan)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|denial| denial.attach_batch_scheduler_counters(planned_batch_width))?;
        validate_batch_conflicts(&slots)
            .map_err(|denial| denial.attach_batch_scheduler_counters(planned_batch_width))?;
        validate_batch_execution_mode(&slots)
            .map_err(|denial| denial.attach_batch_scheduler_counters(planned_batch_width))?;
        Ok(Self { responses, slots })
    }

    pub fn slots(&self) -> &[ForgeServerOperationExecutionSlot] {
        &self.slots
    }

    pub fn execute(self) -> ForgeServerExecutedOperationBatch {
        self.execute_with_cancellation(std::iter::empty())
    }

    pub fn execute_serialized_replay(self) -> ForgeServerExecutedOperationBatch {
        execute_shared_read_slots(
            self.responses,
            self.slots,
            std::iter::empty(),
            std::iter::empty(),
            false,
        )
    }

    pub fn execute_with_cancellation(
        self,
        directives: impl IntoIterator<Item = ForgeServerSchedulerCancellationDirective>,
    ) -> ForgeServerExecutedOperationBatch {
        let ordered = self
            .slots
            .iter()
            .any(|slot| slot.scheduler_lane_kind() != &ForgeServerSchedulerLane::SharedRead);
        if ordered {
            execute_ordered_slots(self.responses, self.slots, directives)
        } else {
            execute_shared_read_slots(
                self.responses,
                self.slots,
                directives,
                std::iter::empty(),
                true,
            )
        }
    }

    pub fn execute_with_certification_sabotage(
        self,
        sabotage: impl IntoIterator<Item = ForgeServerSchedulerCertificationSabotage>,
    ) -> ForgeServerExecutedOperationBatch {
        execute_shared_read_slots(
            self.responses,
            self.slots,
            std::iter::empty(),
            sabotage,
            true,
        )
    }
}

fn validate_batch_execution_mode(
    slots: &[ForgeServerOperationExecutionSlot],
) -> Result<(), ForgeServerSchedulerConflictDenial> {
    let contains_shared_read = slots
        .iter()
        .any(|slot| slot.scheduler_lane_kind() == &ForgeServerSchedulerLane::SharedRead);
    let contains_ordered = slots
        .iter()
        .any(|slot| slot.scheduler_lane_kind() != &ForgeServerSchedulerLane::SharedRead);
    if contains_shared_read && contains_ordered {
        return Err(
            ForgeServerSchedulerConflictDenial::unsupported_ordered_operation(
                "scheduler batches may not mix shared-read and ordered mutation lanes in phase 8",
            ),
        );
    }
    Ok(())
}

fn validate_batch_conflicts(
    slots: &[ForgeServerOperationExecutionSlot],
) -> Result<(), ForgeServerSchedulerConflictDenial> {
    for (left_index, left_slot) in slots.iter().enumerate() {
        if left_slot.scheduler_lane_kind() == &ForgeServerSchedulerLane::SharedRead {
            continue;
        }
        let Some(left_basis_digest) = left_slot.slot_basis_digest().map(ToString::to_string) else {
            continue;
        };
        for right_slot in &slots[left_index + 1..] {
            if left_slot.scheduler_lane_kind() != right_slot.scheduler_lane_kind() {
                continue;
            }
            let Some(right_basis_digest) = right_slot.slot_basis_digest() else {
                continue;
            };
            if left_basis_digest == right_basis_digest {
                return Err(ForgeServerSchedulerConflictDenial::conflicting_mutation_plan(
                    format!(
                        "ordered mutation scheduler denied slots {} and {} on lane `{}` because both are bound to caller basis `{left_basis_digest}`",
                        left_slot.ordinal(),
                        right_slot.ordinal(),
                        left_slot.scheduler_lane()
                    ),
                )
                .with_conflict_facts(
                    ForgeServerSchedulerConflictDenialFacts::conflicting_mutation_plan(
                        left_slot.scheduler_lane(),
                        Some(left_basis_digest.clone()),
                        left_slot.ordinal(),
                        right_slot.ordinal(),
                    ),
                ));
            }
        }
    }
    Ok(())
}
