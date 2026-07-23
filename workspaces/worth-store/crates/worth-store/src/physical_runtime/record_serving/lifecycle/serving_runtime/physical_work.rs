use super::ServingPhysicalRuntime;

impl ServingPhysicalRuntime {
    pub const fn physical_signal_runtime_identity(
        &self,
    ) -> crate::physical_runtime::PhysicalSignalRuntimeIdentity {
        self.parts.signal_owner.runtime_identity()
    }

    pub const fn physical_signal_profile_identity(
        &self,
    ) -> crate::physical_runtime::PhysicalSignalProfileIdentity {
        self.parts.signal_owner.profile()
    }

    pub fn physical_signal_clock_observation(
        &self,
    ) -> Result<
        crate::physical_runtime::PhysicalSignalClockObservation,
        crate::physical_runtime::PhysicalSignalClockObservationFailure,
    > {
        self.parts.signal_owner.clock_observation()
    }

    pub fn physical_signal_observation(
        &self,
    ) -> Result<
        crate::physical_runtime::PhysicalSignalObservation,
        crate::physical_runtime::PhysicalSignalClockObservationFailure,
    > {
        self.parts.signal_owner.observation()
    }

    pub fn apply_physical_aspect_delta(
        &self,
        delta: crate::physical_runtime::PhysicalWorkAspectDelta,
    ) -> Result<(), crate::physical_runtime::PhysicalSignalDeltaApplicationFailure> {
        self.parts.signal_owner.apply_delta(delta)
    }

    pub fn physical_read_submission(&self) -> crate::physical_runtime::PhysicalReadSubmission {
        self.parts.work_submission.read_submission()
    }

    pub fn physical_mutation_submission(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationSubmission {
        self.parts.work_submission.mutation_submission()
    }

    pub fn physical_work_observer(&self) -> crate::physical_runtime::PhysicalWorkObservation {
        self.parts.work_submission.observation()
    }

    pub fn admit_physical_work(
        &self,
        receipt: crate::physical_runtime::PhysicalWorkSubmissionReceipt,
    ) -> Result<
        crate::physical_runtime::AdmittedPhysicalWork,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        crate::physical_runtime::PhysicalWorkAdmission::admit(
            &self.parts.work_submission,
            receipt,
            &self.parts.work_admission,
            &self.parts.health,
        )
    }

    pub fn request_physical_work(
        &self,
        admitted: crate::physical_runtime::AdmittedPhysicalWork,
    ) -> Result<
        crate::physical_runtime::PhysicalWorkReadiness,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        crate::physical_runtime::PhysicalWorkAdmission::require_current(
            &self.parts.work_submission,
            admitted.intent(),
            &self.parts.health,
        )?;
        self.parts.signal_owner.request(admitted)
    }

    pub fn revalidate_physical_work(
        &self,
        ready: crate::physical_runtime::ReadyPhysicalWork,
    ) -> Result<
        crate::physical_runtime::PhysicalWorkReadiness,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        crate::physical_runtime::PhysicalWorkAdmission::require_current(
            &self.parts.work_submission,
            ready.intent(),
            &self.parts.health,
        )?;
        self.parts.signal_owner.revalidate(ready)
    }

    pub fn revalidate_blocked_physical_work(
        &self,
        blocked: crate::physical_runtime::BlockedPhysicalWork,
    ) -> Result<
        crate::physical_runtime::PhysicalWorkReadiness,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        crate::physical_runtime::PhysicalWorkAdmission::require_current(
            &self.parts.work_submission,
            blocked.intent(),
            &self.parts.health,
        )?;
        self.parts.signal_owner.revalidate_blocked(blocked)
    }

    pub fn admit_physical_scheduler_capability(
        &self,
        requirement: worth_store_io_scheduler::IoSchedulerBackendCapabilityRequirement,
    ) -> Result<
        worth_store_io_scheduler::IoSchedulerBackendCapabilityAdmission,
        worth_store_io_scheduler::IoSchedulerBackendCapabilityDenial,
    > {
        self.parts
            .scheduler_admission
            .admit(self.parts.executor.record_serving_media(), requirement)
    }

    pub fn admit_physical_scheduler_demand(
        &self,
        demand: crate::physical_runtime::PhysicalSchedulerDemand,
        backend: &worth_store_io_scheduler::IoSchedulerBackendCapabilityAdmission,
        policy: worth_foundational::FoundationalPolicyAdmissionReceipt,
    ) -> Result<
        crate::physical_runtime::ResourceAdmittedPhysicalWork,
        crate::physical_runtime::PhysicalSchedulerDenial,
    > {
        crate::physical_runtime::PhysicalWorkAdmission::require_current(
            &self.parts.work_submission,
            demand.intent(),
            &self.parts.health,
        )
        .map_err(crate::physical_runtime::PhysicalSchedulerDenial::PreEffect)?;
        crate::physical_runtime::PhysicalWorkScheduler::admit(demand, backend, policy)
    }
}
