mod availability;
mod graph;
mod outcome;
mod route;
mod wake;
mod worker;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::physical_runtime::{
    work::{
        PhysicalSignalAspectBindingDigest, PhysicalSignalAspectBindingSet,
        PhysicalWorkAspectDelta,
    },
    LifecycleGeneration, PhysicalSignalProfileIdentity, PhysicalWorkProfileDeclaration,
};

pub use outcome::{
    PhysicalSignalClockObservation, PhysicalSignalClockObservationFailure,
    PhysicalSignalConstructionFailure, PhysicalSignalDeltaApplicationFailure,
    PhysicalSignalObservation, PhysicalSignalRuntimeIdentity, PhysicalSignalShutdownOutcome,
};
use route::PhysicalSignalRouteOwner;
use worker::PhysicalSignalGraphWorker;

pub(in crate::physical_runtime) use availability::PhysicalSignalAdmissionStatus;

pub(in crate::physical_runtime) struct PhysicalWorkSignalOwner {
    runtime_identity: PhysicalSignalRuntimeIdentity,
    profile: PhysicalSignalProfileIdentity,
    bindings: Arc<PhysicalSignalAspectBindingSet>,
    graph_worker: PhysicalSignalGraphWorker,
    admission_status: PhysicalSignalAdmissionStatus,
    certification_failure: AtomicBool,
    _lifecycle_generation: LifecycleGeneration,
}

impl PhysicalWorkSignalOwner {
    pub(super) fn build_foundation(
        lifecycle_generation: LifecycleGeneration,
        profile: PhysicalWorkProfileDeclaration,
    ) -> Result<Self, PhysicalSignalConstructionFailure> {
        let runtime_identity = new_runtime_identity()?;
        let bindings = Arc::new(PhysicalSignalAspectBindingSet::install(profile));
        let profile = bindings.profile();
        let admission_status = PhysicalSignalAdmissionStatus::available();
        let graph_worker =
            PhysicalSignalGraphWorker::spawn(Arc::clone(&bindings), admission_status.clone())?;
        Ok(Self {
            runtime_identity,
            profile,
            bindings,
            graph_worker,
            admission_status,
            certification_failure: AtomicBool::new(false),
            _lifecycle_generation: lifecycle_generation,
        })
    }

    pub(in crate::physical_runtime) const fn profile(&self) -> PhysicalSignalProfileIdentity {
        self.profile
    }

    pub(in crate::physical_runtime) fn bindings(&self) -> Arc<PhysicalSignalAspectBindingSet> {
        Arc::clone(&self.bindings)
    }

    pub(in crate::physical_runtime) fn admission_status(&self) -> PhysicalSignalAdmissionStatus {
        self.admission_status.clone()
    }

    pub(in crate::physical_runtime) const fn runtime_identity(
        &self,
    ) -> PhysicalSignalRuntimeIdentity {
        self.runtime_identity
    }

    pub(in crate::physical_runtime) fn clock_observation(
        &self,
    ) -> Result<PhysicalSignalClockObservation, PhysicalSignalClockObservationFailure> {
        self.require_available()
            .map_err(|_| PhysicalSignalClockObservationFailure::OwnerUnavailable)?;
        let basis = self
            .graph_worker
            .clock_basis()
            .ok_or(PhysicalSignalClockObservationFailure::OwnerUnavailable)?;
        Ok(PhysicalSignalClockObservation::new(
            basis.current_tick().get(),
            basis.last_advance_ordinal().get(),
        ))
    }

    pub(in crate::physical_runtime) fn apply_delta(
        &self,
        delta: PhysicalWorkAspectDelta,
    ) -> Result<(), PhysicalSignalDeltaApplicationFailure> {
        self.require_available()
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::OwnerUnavailable)?;
        let route = self
            .route(delta.binding())
            .ok_or(PhysicalSignalDeltaApplicationFailure::BindingNotInstalled)?;
        route.apply_delta(delta)
    }

    pub(in crate::physical_runtime) fn observation(
        &self,
    ) -> Result<PhysicalSignalObservation, PhysicalSignalClockObservationFailure> {
        Ok(PhysicalSignalObservation::new(
            self.profile,
            1,
            u16::try_from(self.bindings.len()).expect("Signal aspect capacity fits u16"),
            u16::try_from(self.graph_worker.len())
                .expect("Signal locality owner capacity fits u16"),
            crate::physical_runtime::work::PHYSICAL_ASYNC_CAPABILITIES.len() as u8,
            self.clock_observation()?,
        ))
    }

    pub(in crate::physical_runtime) fn request(
        &self,
        admitted: crate::physical_runtime::AdmittedPhysicalWork,
    ) -> Result<
        crate::physical_runtime::PhysicalWorkReadiness,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        self.require_available()?;
        let route = admitted.authority().binding();
        self.route(route)
            .ok_or(crate::physical_runtime::PhysicalWorkPreEffectDenial::CapabilityAbsent)?
            .request(admitted)
    }

    pub(in crate::physical_runtime) fn revalidate(
        &self,
        ready: crate::physical_runtime::ReadyPhysicalWork,
    ) -> Result<
        crate::physical_runtime::PhysicalWorkReadiness,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        self.require_available()?;
        let route = ready.authority().binding();
        let (admitted, active) = ready.into_signal_parts();
        self.revalidate_parts(route, admitted, active)
    }

    pub(in crate::physical_runtime) fn revalidate_blocked(
        &self,
        blocked: crate::physical_runtime::BlockedPhysicalWork,
    ) -> Result<
        crate::physical_runtime::PhysicalWorkReadiness,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        self.require_available()?;
        let route = blocked.authority().binding();
        let (admitted, active) = blocked
            .into_revalidation_parts()
            .ok_or(crate::physical_runtime::PhysicalWorkPreEffectDenial::DependencyBlocked)?;
        self.revalidate_parts(route, admitted, active)
    }

    pub(in crate::physical_runtime) fn dispose(mut self) -> PhysicalSignalShutdownOutcome {
        self.admission_status.revoke();
        self.graph_worker.stop();
        if self.certification_failure.load(Ordering::Acquire) {
            PhysicalSignalShutdownOutcome::OwnerRevoked
        } else {
            PhysicalSignalShutdownOutcome::Disposed
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime) fn fail_worker_for_certification(&self) {
        self.certification_failure.store(true, Ordering::Release);
        self.graph_worker.fail_for_certification();
    }

    fn revalidate_parts(
        &self,
        route: PhysicalSignalAspectBindingDigest,
        admitted: crate::physical_runtime::AdmittedPhysicalWork,
        active: worth_signal::facade::ResourceRequestHandle,
    ) -> Result<
        crate::physical_runtime::PhysicalWorkReadiness,
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        self.route(route)
            .ok_or(crate::physical_runtime::PhysicalWorkPreEffectDenial::CapabilityAbsent)?
            .revalidate(admitted, active)
    }

    fn route(
        &self,
        route: PhysicalSignalAspectBindingDigest,
    ) -> Option<&PhysicalSignalRouteOwner> {
        self.graph_worker.route(route)
    }

    fn require_available(
        &self,
    ) -> Result<(), crate::physical_runtime::PhysicalWorkPreEffectDenial> {
        self.admission_status
            .is_available()
            .then_some(())
            .ok_or(crate::physical_runtime::PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)
    }
}

impl Drop for PhysicalWorkSignalOwner {
    fn drop(&mut self) {
        self.admission_status.revoke();
    }
}

fn new_runtime_identity() -> Result<PhysicalSignalRuntimeIdentity, PhysicalSignalConstructionFailure>
{
    let mut bytes = [0_u8; 16];
    getrandom::fill(&mut bytes)
        .map_err(|_| PhysicalSignalConstructionFailure::IdentityEntropyUnavailable)?;
    Ok(PhysicalSignalRuntimeIdentity::new(bytes))
}
