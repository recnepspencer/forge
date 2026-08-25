use std::collections::BTreeMap;

#[path = "recovery/external.rs"]
mod external;
pub(crate) use external::prepare_external_recovery;
#[path = "recovery/physical_epoch.rs"]
mod physical_epoch;
use physical_epoch::UiNativePhysicalRecoveryOwner;
pub(crate) use physical_epoch::{
    UiNativePhysicalRecoveryFact, UiNativePhysicalRecoveryPreparation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeRecoveryCause {
    DerivedStateLost,
    PresentationIndeterminate,
    SurfaceLost,
    SurfaceOutdated,
    Resize,
    Dpi,
    DeviceLost,
}

#[must_use]
pub(crate) struct UiNativeRecoveryRequirement {
    binding: u64,
    state: UiNativeBindingRecovery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UiNativeBindingRecovery {
    cause: UiNativeRecoveryCause,
    physical_epoch: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct UiNativeRecoveryLineage {
    host_session: u64,
    semantic_surface: u64,
    host_surface: u64,
}

pub(crate) struct UiNativeRecoveryRegistry {
    required: BTreeMap<u64, UiNativeBindingRecovery>,
    in_flight: BTreeMap<u64, UiNativeBindingRecovery>,
    parked: BTreeMap<UiNativeRecoveryLineage, UiNativeBindingRecovery>,
    physical: UiNativePhysicalRecoveryOwner,
}

impl Default for UiNativeRecoveryRegistry {
    fn default() -> Self {
        Self {
            required: BTreeMap::new(),
            in_flight: BTreeMap::new(),
            parked: BTreeMap::new(),
            physical: UiNativePhysicalRecoveryOwner::default(),
        }
    }
}

impl UiNativeRecoveryRegistry {
    pub(crate) fn require(&mut self, binding: u64, cause: UiNativeRecoveryCause) {
        let incoming_physical_epoch =
            graphics_recovery_cause(cause).map(|physical| self.require_physical(physical));
        let current = self.required.get(&binding).copied();
        let cause = current
            .map(|current| dominant(current.cause, cause))
            .unwrap_or(cause);
        let physical_epoch =
            incoming_physical_epoch.or_else(|| current.and_then(|current| current.physical_epoch));
        self.required.insert(
            binding,
            UiNativeBindingRecovery {
                cause,
                physical_epoch,
            },
        );
    }

    pub(crate) fn requires(&self, binding: u64) -> bool {
        self.required.contains_key(&binding)
    }

    pub(crate) fn cause(&self, binding: u64) -> Option<UiNativeRecoveryCause> {
        self.required.get(&binding).map(|required| required.cause)
    }

    pub(crate) fn ready(&self, binding: u64) -> bool {
        self.required
            .get(&binding)
            .is_some_and(|required| self.binding_ready(*required))
    }

    pub(crate) fn physical_preparation(
        &self,
        binding: u64,
    ) -> Option<UiNativePhysicalRecoveryPreparation> {
        self.physical
            .preparation(self.required.get(&binding)?.physical_epoch?)
    }

    pub(crate) fn commit_physical(
        &mut self,
        preparation: UiNativePhysicalRecoveryPreparation,
        device_generation: u64,
        surface_generation: u64,
    ) -> bool {
        self.physical
            .commit(preparation, device_generation, surface_generation)
    }

    pub(crate) fn physical_fact(&self, binding: u64) -> Option<UiNativePhysicalRecoveryFact> {
        self.physical
            .fact(self.required.get(&binding)?.physical_epoch?)
    }

    pub(crate) fn take(&mut self, binding: u64) -> Option<UiNativeRecoveryRequirement> {
        let state = self.required.get(&binding).copied()?;
        if !self.binding_ready(state) || self.in_flight.contains_key(&binding) {
            return None;
        }
        self.required.remove(&binding);
        self.in_flight.insert(binding, state);
        Some(UiNativeRecoveryRequirement { binding, state })
    }

    pub(crate) fn settle(&mut self, requirement: UiNativeRecoveryRequirement) -> bool {
        if self.in_flight.remove(&requirement.binding) != Some(requirement.state) {
            return false;
        }
        let current_epoch = self.current_physical_epoch();
        if current_epoch.is_some() && requirement.state.physical_epoch != current_epoch {
            let cause = self
                .current_physical_cause()
                .map(|current| dominant(requirement.state.cause, current))
                .unwrap_or(requirement.state.cause);
            self.required.insert(
                requirement.binding,
                UiNativeBindingRecovery {
                    cause,
                    physical_epoch: current_epoch,
                },
            );
            return false;
        }
        self.clear_unreferenced_physical();
        true
    }

    pub(crate) fn resolve(&mut self, binding: u64) -> bool {
        let removed = self.required.remove(&binding).is_some();
        self.clear_unreferenced_physical();
        removed
    }

    pub(crate) fn restore(&mut self, requirement: UiNativeRecoveryRequirement) {
        if self.in_flight.remove(&requirement.binding) != Some(requirement.state) {
            return;
        }
        let current_epoch = self.current_physical_epoch();
        let cause = self
            .current_physical_cause()
            .map(|current| dominant(requirement.state.cause, current))
            .unwrap_or(requirement.state.cause);
        self.required.insert(
            requirement.binding,
            UiNativeBindingRecovery {
                cause,
                physical_epoch: current_epoch.or(requirement.state.physical_epoch),
            },
        );
    }

    pub(crate) fn transfer(&mut self, predecessor: u64, successor: u64) -> bool {
        if predecessor == successor {
            return self.requires(predecessor);
        }
        let Some(requirement) = self.required.remove(&predecessor) else {
            return false;
        };
        self.merge_required(successor, requirement);
        true
    }

    pub(crate) fn park(&mut self, binding: u64, lineage: UiNativeRecoveryLineage) -> bool {
        let Some(requirement) = self.required.remove(&binding) else {
            return false;
        };
        let retained = self
            .parked
            .get(&lineage)
            .copied()
            .map(|current| merge_binding(current, requirement))
            .unwrap_or(requirement);
        self.parked.insert(lineage, retained);
        true
    }

    pub(crate) fn claim(&mut self, lineage: UiNativeRecoveryLineage, successor: u64) -> bool {
        let Some(requirement) = self.parked.remove(&lineage) else {
            return false;
        };
        self.merge_required(successor, requirement);
        true
    }

    pub(crate) fn clear(&mut self) {
        self.required.clear();
        self.in_flight.clear();
        self.parked.clear();
        self.physical.clear();
    }

    pub(crate) fn len(&self) -> usize {
        self.required
            .len()
            .saturating_add(self.in_flight.len())
            .saturating_add(self.parked.len())
    }

    fn require_physical(&mut self, cause: UiNativeRecoveryCause) -> u64 {
        let admission = self.physical.require(cause);
        let epoch = admission.epoch();
        if admission.replaces_references() {
            for required in self.required.values_mut() {
                required.physical_epoch = Some(epoch);
            }
            for parked in self.parked.values_mut() {
                parked.physical_epoch = Some(epoch);
            }
        }
        epoch
    }

    fn binding_ready(&self, required: UiNativeBindingRecovery) -> bool {
        match required.physical_epoch {
            None => true,
            Some(epoch) => self.physical.fact(epoch).is_some(),
        }
    }

    fn current_physical_epoch(&self) -> Option<u64> {
        self.physical.epoch()
    }

    fn current_physical_cause(&self) -> Option<UiNativeRecoveryCause> {
        self.physical.cause()
    }

    fn clear_unreferenced_physical(&mut self) {
        let Some(epoch) = self.current_physical_epoch() else {
            return;
        };
        let referenced = self
            .required
            .values()
            .chain(self.in_flight.values())
            .chain(self.parked.values())
            .any(|required| required.physical_epoch == Some(epoch));
        if !referenced {
            self.physical.clear();
        }
    }

    fn merge_required(&mut self, binding: u64, incoming: UiNativeBindingRecovery) {
        let merged = self
            .required
            .get(&binding)
            .copied()
            .map(|current| merge_binding(current, incoming))
            .unwrap_or(incoming);
        self.required.insert(binding, merged);
    }
}

impl UiNativeRecoveryLineage {
    pub(crate) fn from_registration(
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> Self {
        Self {
            host_session: request.host_session_identity(),
            semantic_surface: request.semantic_surface_identity().diagnostic_value(),
            host_surface: request.host_surface_identity().diagnostic_value(),
        }
    }
}

impl UiNativeRecoveryRequirement {
    pub(crate) const fn binding(&self) -> u64 {
        self.binding
    }

    pub(crate) const fn cause(&self) -> UiNativeRecoveryCause {
        self.state.cause
    }
}

fn merge_binding(
    current: UiNativeBindingRecovery,
    incoming: UiNativeBindingRecovery,
) -> UiNativeBindingRecovery {
    UiNativeBindingRecovery {
        cause: dominant(current.cause, incoming.cause),
        physical_epoch: incoming.physical_epoch.or(current.physical_epoch),
    }
}

const fn graphics_recovery_cause(cause: UiNativeRecoveryCause) -> Option<UiNativeRecoveryCause> {
    match cause {
        UiNativeRecoveryCause::SurfaceOutdated
        | UiNativeRecoveryCause::SurfaceLost
        | UiNativeRecoveryCause::DeviceLost => Some(cause),
        UiNativeRecoveryCause::DerivedStateLost
        | UiNativeRecoveryCause::PresentationIndeterminate
        | UiNativeRecoveryCause::Resize
        | UiNativeRecoveryCause::Dpi => None,
    }
}

const fn dominant(
    current: UiNativeRecoveryCause,
    successor: UiNativeRecoveryCause,
) -> UiNativeRecoveryCause {
    if recovery_rank(successor) > recovery_rank(current) {
        successor
    } else {
        current
    }
}

const fn recovery_rank(cause: UiNativeRecoveryCause) -> u8 {
    match cause {
        UiNativeRecoveryCause::Resize => 0,
        UiNativeRecoveryCause::Dpi => 1,
        UiNativeRecoveryCause::SurfaceOutdated => 2,
        UiNativeRecoveryCause::DerivedStateLost => 3,
        UiNativeRecoveryCause::SurfaceLost => 4,
        UiNativeRecoveryCause::PresentationIndeterminate => 5,
        UiNativeRecoveryCause::DeviceLost => 6,
    }
}

#[cfg(test)]
#[path = "recovery/tests.rs"]
mod tests;
