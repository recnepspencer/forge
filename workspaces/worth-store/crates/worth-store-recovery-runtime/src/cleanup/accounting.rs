use worth_store::physical_runtime::{
    PhysicalRecoveryCleanupFreshnessReadDenialKind, PhysicalRecoveryCleanupRemovalDenialKind,
    StoreRecoveryCleanupFreshnessFailure, StoreRecoveryCleanupFreshnessSample,
};

use crate::handoff::{
    RecoveryCleanupCounters, RecoveryCleanupDeferralEvidence, RecoveryCleanupEvidence,
    RecoveryCleanupEvidenceParts, RecoveryCleanupPosture,
};

use super::{PerformedRecoveryCleanupRemoval, RecoveryCleanupDispositionKind, RecoveryCleanupPlan};

pub(super) struct RecoveryCleanupAccounting {
    plan_identity: [u8; 32],
    published_generation: u64,
    freshness: Vec<StoreRecoveryCleanupFreshnessSample>,
    performed: Vec<PerformedRecoveryCleanupRemoval>,
    deferrals: Vec<RecoveryCleanupDeferralEvidence>,
    counters: RecoveryCleanupCounters,
}

impl RecoveryCleanupAccounting {
    pub(super) fn begin(plan: &RecoveryCleanupPlan) -> Self {
        let bytes_planned = plan
            .candidates()
            .iter()
            .map(|candidate| candidate.byte_count())
            .sum();
        Self {
            plan_identity: plan.identity(),
            published_generation: plan.published_generation(),
            freshness: Vec::new(),
            performed: Vec::new(),
            deferrals: Vec::new(),
            counters: RecoveryCleanupCounters {
                dispositions: plan.dispositions().len() as u64,
                actions_planned: plan.candidates().len() as u64,
                bytes_planned,
                ..RecoveryCleanupCounters::default()
            },
        }
    }

    pub(super) fn record_freshness_sample(&mut self, sample: StoreRecoveryCleanupFreshnessSample) {
        self.counters.freshness_evaluations += 1;
        self.counters.freshness_reads_completed += 1;
        self.counters.freshness_bytes_read +=
            worth_store_physical_format::ROOT_SELECTOR_BYTES as u64;
        self.record_freshness_scheduler(true, false, false, true);
        self.freshness.push(sample);
    }

    pub(super) fn record_freshness_failure(
        &mut self,
        failure: &StoreRecoveryCleanupFreshnessFailure,
    ) {
        self.counters.freshness_evaluations += 1;
        let Some(read) = failure.read() else {
            return;
        };
        if read.completed().is_some() {
            self.counters.freshness_reads_completed += 1;
            self.counters.freshness_bytes_read +=
                worth_store_physical_format::ROOT_SELECTOR_BYTES as u64;
        } else if read.denied().is_some() {
            self.counters.freshness_reads_denied += 1;
        }
        match read.kind() {
            PhysicalRecoveryCleanupFreshnessReadDenialKind::Admission(admission) => {
                self.record_freshness_scheduler(
                    admission.submission_recorded(),
                    admission.scheduler_deferred(),
                    admission.cancellation_recorded(),
                    false,
                );
            }
            PhysicalRecoveryCleanupFreshnessReadDenialKind::Execution(_) => {
                self.record_freshness_scheduler(true, false, true, false);
            }
            PhysicalRecoveryCleanupFreshnessReadDenialKind::Media
            | PhysicalRecoveryCleanupFreshnessReadDenialKind::SchedulerSettlement(_)
            | PhysicalRecoveryCleanupFreshnessReadDenialKind::SignalSettlement(_)
            | PhysicalRecoveryCleanupFreshnessReadDenialKind::InvalidSelector => {
                self.record_freshness_scheduler(true, false, false, true);
            }
        }
    }

    pub(super) fn record_completed(
        &mut self,
        bytes: u64,
        performed: PerformedRecoveryCleanupRemoval,
    ) {
        self.counters.actions_attempted += 1;
        self.counters.actions_completed += 1;
        self.counters.bytes_completed += bytes;
        self.counters.performed_effects += 1;
        self.record_removal_scheduler(true, false, false, true);
        self.performed.push(performed);
    }

    pub(super) fn record_deferral(&mut self, evidence: RecoveryCleanupDeferralEvidence) {
        match &evidence {
            RecoveryCleanupDeferralEvidence::Freshness { failure, .. } => {
                self.record_freshness_failure(failure);
            }
            RecoveryCleanupDeferralEvidence::DeniedBeforeEffect { denial, .. } => {
                self.counters.actions_attempted += 1;
                self.counters.denied_before_effect += 1;
                match denial.kind() {
                    PhysicalRecoveryCleanupRemovalDenialKind::InvalidCommand => {}
                    PhysicalRecoveryCleanupRemovalDenialKind::Admission(admission) => {
                        self.record_removal_scheduler(
                            admission.submission_recorded(),
                            admission.scheduler_deferred(),
                            admission.cancellation_recorded(),
                            false,
                        );
                    }
                    PhysicalRecoveryCleanupRemovalDenialKind::Execution(_) => {
                        self.record_removal_scheduler(true, false, true, false);
                    }
                    PhysicalRecoveryCleanupRemovalDenialKind::Media => {
                        self.record_removal_scheduler(true, false, false, true);
                    }
                }
            }
            RecoveryCleanupDeferralEvidence::IndeterminateEffect { .. } => {
                self.counters.actions_attempted += 1;
                self.counters.indeterminate_effects += 1;
                self.record_removal_scheduler(true, false, false, true);
            }
            RecoveryCleanupDeferralEvidence::PublishedGenerationChanged { .. }
            | RecoveryCleanupDeferralEvidence::EligibilityChanged { .. } => {}
        }
        self.deferrals.push(evidence);
    }

    fn record_freshness_scheduler(
        &mut self,
        submitted: bool,
        deferred: bool,
        cancelled: bool,
        settled: bool,
    ) {
        self.counters.freshness_scheduler_submitted += u64::from(submitted);
        self.counters.freshness_scheduler_deferred += u64::from(deferred);
        self.counters.freshness_scheduler_cancelled += u64::from(cancelled);
        self.counters.freshness_scheduler_settled += u64::from(settled);
        self.record_scheduler(submitted, deferred, cancelled, settled);
    }

    fn record_removal_scheduler(
        &mut self,
        submitted: bool,
        deferred: bool,
        cancelled: bool,
        settled: bool,
    ) {
        self.counters.removal_scheduler_submitted += u64::from(submitted);
        self.counters.removal_scheduler_deferred += u64::from(deferred);
        self.counters.removal_scheduler_cancelled += u64::from(cancelled);
        self.counters.removal_scheduler_settled += u64::from(settled);
        self.record_scheduler(submitted, deferred, cancelled, settled);
    }

    fn record_scheduler(
        &mut self,
        submitted: bool,
        deferred: bool,
        cancelled: bool,
        settled: bool,
    ) {
        self.counters.scheduler_commands_submitted += u64::from(submitted);
        self.counters.scheduler_commands_deferred += u64::from(deferred);
        self.counters.scheduler_commands_cancelled += u64::from(cancelled);
        self.counters.scheduler_commands_settled += u64::from(settled);
    }

    pub(super) fn finish(mut self, plan: RecoveryCleanupPlan) -> RecoveryCleanupPosture {
        for disposition in plan.dispositions() {
            match disposition.kind() {
                RecoveryCleanupDispositionKind::Current => self.counters.current += 1,
                RecoveryCleanupDispositionKind::Retained => self.counters.retained += 1,
                RecoveryCleanupDispositionKind::Eligible => {
                    self.counters.eligible_after_cleanup += 1;
                }
                RecoveryCleanupDispositionKind::Deferred(_) => {
                    self.counters.actions_deferred += 1;
                    self.counters.bytes_deferred += disposition.byte_count();
                }
                RecoveryCleanupDispositionKind::QuarantinedOrUnsupported => {
                    self.counters.quarantined_or_unsupported += 1;
                }
                RecoveryCleanupDispositionKind::SafelyRemoved => {
                    self.counters.safely_removed += 1;
                }
            }
        }
        let evidence = RecoveryCleanupEvidence::new(RecoveryCleanupEvidenceParts {
            plan_identity: self.plan_identity,
            published_generation: self.published_generation,
            dispositions: plan.into_dispositions(),
            freshness: self.freshness.into_boxed_slice(),
            performed: self.performed.into_boxed_slice(),
            deferrals: self.deferrals.into_boxed_slice(),
            counters: self.counters,
        });
        RecoveryCleanupPosture::from_evidence(evidence)
    }
}
