use std::collections::{BTreeMap, BTreeSet};

use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiOperationLiveAdmissionDenial,
    WorthUiOperationLiveAdmissionStop, WorthUiOperationLiveResource, WorthUiQueryViewIdentity,
    WorthUiSettledSnapshotSourceGeneration, WorthUiSettledSnapshotSourceOrder,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiOperationLiveObservation {
    subsystem_construction_count: usize,
    retained_resource_count: usize,
    orphan_resource_count: usize,
    resource_registration_count: usize,
    succession_operation_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExactOperationLiveResourceEvidence {
    installed_reference: WorthUiInstalledQueryBindingReference,
    lease_identity: String,
}

#[derive(Default)]
pub(crate) struct WorthUiOperationLiveRetention {
    resources: BTreeMap<WorthUiQueryViewIdentity, WorthUiOperationLiveResource>,
    resource_registration_count: usize,
    succession_operation_count: usize,
    next_source_generation: u64,
    next_source_order: u64,
}

impl std::fmt::Debug for WorthUiOperationLiveRetention {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiOperationLiveRetention")
            .field("retained_resource_count", &self.resources.len())
            .field(
                "resource_registration_count",
                &self.resource_registration_count,
            )
            .field(
                "succession_operation_count",
                &self.succession_operation_count,
            )
            .finish()
    }
}

impl WorthUiOperationLiveRetention {
    pub(crate) fn contains(&self, reference: &WorthUiInstalledQueryBindingReference) -> bool {
        self.resources
            .contains_key(reference.definition().identity())
    }

    pub(crate) fn insert(
        &mut self,
        resource: WorthUiOperationLiveResource,
    ) -> Option<WorthUiOperationLiveResource> {
        let identity = resource.definition().identity().clone();
        let replaced = self.resources.insert(identity, resource);
        if replaced.is_none() {
            self.resource_registration_count += 1;
        }
        replaced
    }

    pub(crate) fn admit(
        &mut self,
        mut resource: WorthUiOperationLiveResource,
    ) -> Result<(), WorthUiOperationLiveAdmissionStop> {
        let generation = match self.next_source_generation.checked_add(1) {
            Some(value) => value,
            None => {
                return Err(WorthUiOperationLiveAdmissionStop::new(
                    WorthUiOperationLiveAdmissionDenial::SourceGenerationExhausted,
                    resource,
                ));
            }
        };
        let order = match self.next_source_order.checked_add(1) {
            Some(value) => value,
            None => {
                return Err(WorthUiOperationLiveAdmissionStop::new(
                    WorthUiOperationLiveAdmissionDenial::SourceOrderExhausted,
                    resource,
                ));
            }
        };
        resource.attach_source_coordinates(
            WorthUiSettledSnapshotSourceGeneration::new(generation),
            WorthUiSettledSnapshotSourceOrder::new(order),
        );
        self.next_source_generation = generation;
        self.next_source_order = order;
        debug_assert!(self.insert(resource).is_none());
        Ok(())
    }

    pub(crate) fn exact_evidence(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiExactOperationLiveResourceEvidence> {
        self.resources
            .get(reference.definition().identity())
            .map(|resource| WorthUiExactOperationLiveResourceEvidence {
                installed_reference: resource.installed_reference().clone(),
                lease_identity: resource.lease_identity().to_owned(),
            })
    }

    pub(crate) fn resource(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<&WorthUiOperationLiveResource> {
        self.resources.get(reference.definition().identity())
    }

    pub(crate) fn take(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiOperationLiveResource> {
        self.succession_operation_count += 1;
        self.resources.remove(reference.definition().identity())
    }

    pub(crate) fn drain_into(&mut self, retirement: &mut Vec<WorthUiOperationLiveResource>) {
        self.succession_operation_count += 1;
        retirement.extend(std::mem::take(&mut self.resources).into_values());
    }

    pub(crate) fn retain_only(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
        retirement: &mut Vec<WorthUiOperationLiveResource>,
    ) {
        self.succession_operation_count += 1;
        let retained = references
            .iter()
            .map(|reference| reference.definition().identity())
            .collect::<BTreeSet<_>>();
        let resources = std::mem::take(&mut self.resources);
        for (identity, resource) in resources {
            if retained.contains(&identity) {
                self.resources.insert(identity, resource);
            } else {
                retirement.push(resource);
            }
        }
    }

    pub(crate) fn swap_with(&mut self, other: &mut Self) {
        self.succession_operation_count += 1;
        other.succession_operation_count += 1;
        std::mem::swap(&mut self.resources, &mut other.resources);
        std::mem::swap(
            &mut self.next_source_generation,
            &mut other.next_source_generation,
        );
        std::mem::swap(&mut self.next_source_order, &mut other.next_source_order);
    }

    pub(crate) fn observation(
        &self,
        reference_is_valid: impl Fn(&WorthUiInstalledQueryBindingReference) -> bool,
    ) -> WorthUiOperationLiveObservation {
        WorthUiOperationLiveObservation {
            subsystem_construction_count: 1,
            retained_resource_count: self.resources.len(),
            orphan_resource_count: self
                .resources
                .values()
                .filter(|resource| !reference_is_valid(resource.installed_reference()))
                .count(),
            resource_registration_count: self.resource_registration_count,
            succession_operation_count: self.succession_operation_count,
        }
    }
}

impl WorthUiOperationLiveObservation {
    pub fn subsystem_construction_count(self) -> usize {
        self.subsystem_construction_count
    }

    pub fn retained_resource_count(self) -> usize {
        self.retained_resource_count
    }

    pub fn orphan_resource_count(self) -> usize {
        self.orphan_resource_count
    }

    pub fn resource_registration_count(self) -> usize {
        self.resource_registration_count
    }

    pub fn succession_operation_count(self) -> usize {
        self.succession_operation_count
    }
}

impl WorthUiExactOperationLiveResourceEvidence {
    pub fn installed_reference(&self) -> &WorthUiInstalledQueryBindingReference {
        &self.installed_reference
    }

    pub fn lease_identity(&self) -> &str {
        &self.lease_identity
    }
}
