#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthServerOperationSchedulerCounters {
    planned_batch_width: usize,
    conflicting_mutation_plan_denial_count: usize,
    admitted_read_slot_count: usize,
    queued_read_slot_count: usize,
    completed_read_slot_count: usize,
    admitted_submission_slot_count: usize,
    completed_submission_slot_count: usize,
    admitted_mutation_slot_count: usize,
    completed_mutation_slot_count: usize,
    cancelled_before_admission_count: usize,
    cancelled_after_admission_before_execution_count: usize,
    cancelled_during_execution_count: usize,
    isolated_failure_count: usize,
    dependent_failure_count: usize,
    stale_basis_stop_count: usize,
    queue_closed_slot_count: usize,
    forbidden_global_lock_acquisition_count: usize,
}

impl WorthServerOperationSchedulerCounters {
    pub(crate) fn set_planned_batch_width(&mut self, width: usize) {
        self.planned_batch_width = width;
    }

    pub(crate) fn increment_admitted_read_slot_count(&mut self) {
        self.admitted_read_slot_count += 1;
    }

    pub(crate) fn increment_admitted_read_slot_count_by(&mut self, count: usize) {
        self.admitted_read_slot_count += count;
    }

    pub(crate) fn increment_conflicting_mutation_plan_denial_count(&mut self) {
        self.conflicting_mutation_plan_denial_count += 1;
    }

    pub(crate) fn increment_queued_read_slot_count(&mut self) {
        self.queued_read_slot_count += 1;
    }

    pub(crate) fn increment_queued_read_slot_count_by(&mut self, count: usize) {
        self.queued_read_slot_count += count;
    }

    pub(crate) fn increment_completed_read_slot_count(&mut self) {
        self.completed_read_slot_count += 1;
    }

    pub(crate) fn increment_completed_read_slot_count_by(&mut self, count: usize) {
        self.completed_read_slot_count += count;
    }

    pub(crate) fn increment_admitted_submission_slot_count(&mut self) {
        self.admitted_submission_slot_count += 1;
    }

    pub(crate) fn increment_completed_submission_slot_count(&mut self) {
        self.completed_submission_slot_count += 1;
    }

    pub(crate) fn increment_admitted_mutation_slot_count(&mut self) {
        self.admitted_mutation_slot_count += 1;
    }

    pub(crate) fn increment_completed_mutation_slot_count(&mut self) {
        self.completed_mutation_slot_count += 1;
    }

    pub(crate) fn increment_cancelled_before_admission_count(&mut self) {
        self.cancelled_before_admission_count += 1;
    }

    pub(crate) fn increment_cancelled_after_admission_before_execution_count(&mut self) {
        self.cancelled_after_admission_before_execution_count += 1;
    }

    pub(crate) fn increment_cancelled_during_execution_count(&mut self) {
        self.cancelled_during_execution_count += 1;
    }

    pub(crate) fn increment_isolated_failure_count(&mut self) {
        self.isolated_failure_count += 1;
    }

    pub(crate) fn increment_dependent_failure_count(&mut self) {
        self.dependent_failure_count += 1;
    }

    pub(crate) fn increment_stale_basis_stop_count(&mut self) {
        self.stale_basis_stop_count += 1;
    }

    pub(crate) fn increment_queue_closed_slot_count(&mut self) {
        self.queue_closed_slot_count += 1;
    }

    pub(crate) fn add_forbidden_global_lock_acquisitions(&mut self, count: usize) {
        self.forbidden_global_lock_acquisition_count += count;
    }

    pub(crate) fn absorb(&mut self, other: &Self) {
        self.conflicting_mutation_plan_denial_count += other.conflicting_mutation_plan_denial_count;
        self.admitted_read_slot_count += other.admitted_read_slot_count;
        self.queued_read_slot_count += other.queued_read_slot_count;
        self.completed_read_slot_count += other.completed_read_slot_count;
        self.admitted_submission_slot_count += other.admitted_submission_slot_count;
        self.completed_submission_slot_count += other.completed_submission_slot_count;
        self.admitted_mutation_slot_count += other.admitted_mutation_slot_count;
        self.completed_mutation_slot_count += other.completed_mutation_slot_count;
        self.cancelled_before_admission_count += other.cancelled_before_admission_count;
        self.cancelled_after_admission_before_execution_count +=
            other.cancelled_after_admission_before_execution_count;
        self.cancelled_during_execution_count += other.cancelled_during_execution_count;
        self.isolated_failure_count += other.isolated_failure_count;
        self.dependent_failure_count += other.dependent_failure_count;
        self.stale_basis_stop_count += other.stale_basis_stop_count;
        self.queue_closed_slot_count += other.queue_closed_slot_count;
        self.forbidden_global_lock_acquisition_count +=
            other.forbidden_global_lock_acquisition_count;
    }

    pub fn planned_batch_width(&self) -> usize {
        self.planned_batch_width
    }

    pub fn conflicting_mutation_plan_denial_count(&self) -> usize {
        self.conflicting_mutation_plan_denial_count
    }

    pub fn admitted_read_slot_count(&self) -> usize {
        self.admitted_read_slot_count
    }

    pub fn queued_read_slot_count(&self) -> usize {
        self.queued_read_slot_count
    }

    pub fn completed_read_slot_count(&self) -> usize {
        self.completed_read_slot_count
    }

    pub fn admitted_submission_slot_count(&self) -> usize {
        self.admitted_submission_slot_count
    }

    pub fn completed_submission_slot_count(&self) -> usize {
        self.completed_submission_slot_count
    }

    pub fn admitted_mutation_slot_count(&self) -> usize {
        self.admitted_mutation_slot_count
    }

    pub fn completed_mutation_slot_count(&self) -> usize {
        self.completed_mutation_slot_count
    }

    pub fn cancelled_before_admission_count(&self) -> usize {
        self.cancelled_before_admission_count
    }

    pub fn cancelled_after_admission_before_execution_count(&self) -> usize {
        self.cancelled_after_admission_before_execution_count
    }

    pub fn cancelled_during_execution_count(&self) -> usize {
        self.cancelled_during_execution_count
    }

    pub fn isolated_failure_count(&self) -> usize {
        self.isolated_failure_count
    }

    pub fn dependent_failure_count(&self) -> usize {
        self.dependent_failure_count
    }

    pub fn stale_basis_stop_count(&self) -> usize {
        self.stale_basis_stop_count
    }

    pub fn queue_closed_slot_count(&self) -> usize {
        self.queue_closed_slot_count
    }

    pub fn forbidden_global_lock_acquisition_count(&self) -> usize {
        self.forbidden_global_lock_acquisition_count
    }
}
