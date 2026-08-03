use worth_store::physical_runtime::{
    PhysicalWorkCourtroomRunBinding, PhysicalWorkFreshReopenEvidence,
    PhysicalWorkMutantLocalization, PhysicalWorkOracleEvidence, PhysicalWorkSourceBinding,
};

use super::super::{
    c7_crash_campaign::C7CrashCampaignEvidence,
    execution::BoundedResidencyProducerObservation,
    offline_protocol::OfflineObservation,
    protocol::BoundedResidencySiegeObservation,
    schedule::{SchedulePerturbationPlan, SourceClosureScheduleSeeds},
};

pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyCourtroomEvidence
{
    pub(super) run: PhysicalWorkCourtroomRunBinding,
    pub(super) runner: PhysicalWorkSourceBinding,
    pub(super) observer: PhysicalWorkSourceBinding,
    pub(super) producer: BoundedResidencyProducerObservation,
    pub(super) child: BoundedResidencySiegeObservation,
    pub(super) offline: OfflineObservation,
    pub(super) reopen: PhysicalWorkFreshReopenEvidence,
    pub(super) oracle: PhysicalWorkOracleEvidence,
    pub(super) mutants: Box<[PhysicalWorkMutantLocalization]>,
    pub(super) workload_seed: u64,
    pub(super) schedule: SchedulePerturbationPlan,
    pub(super) source_schedule: SourceClosureScheduleSeeds,
    pub(super) crash_campaign: C7CrashCampaignEvidence,
}

impl BoundedResidencyCourtroomEvidence {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn source(
        &self,
    ) -> &PhysicalWorkSourceBinding {
        self.run.source()
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn writer(
        &self,
    ) -> &PhysicalWorkSourceBinding {
        self.run.binary()
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn runner(
        &self,
    ) -> &PhysicalWorkSourceBinding {
        &self.runner
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn observer(
        &self,
    ) -> &PhysicalWorkSourceBinding {
        &self.observer
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn child(
        &self,
    ) -> &BoundedResidencySiegeObservation {
        &self.child
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn producer(
        &self,
    ) -> BoundedResidencyProducerObservation {
        self.producer
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn offline(
        &self,
    ) -> &OfflineObservation {
        &self.offline
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn reopen(
        &self,
    ) -> PhysicalWorkFreshReopenEvidence {
        self.reopen
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn oracle(
        &self,
    ) -> &PhysicalWorkOracleEvidence {
        &self.oracle
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn mutants(
        &self,
    ) -> &[PhysicalWorkMutantLocalization] {
        &self.mutants
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn run(
        &self,
    ) -> &PhysicalWorkCourtroomRunBinding {
        &self.run
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn workload_seed(
        &self,
    ) -> u64 {
        self.workload_seed
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn schedule(
        &self,
    ) -> &SchedulePerturbationPlan {
        &self.schedule
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn source_schedule(
        &self,
    ) -> &SourceClosureScheduleSeeds {
        &self.source_schedule
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn crash_campaign(
        &self,
    ) -> &C7CrashCampaignEvidence {
        &self.crash_campaign
    }
}
