use std::collections::{BTreeMap, BTreeSet};

use super::projection_facts::WorthUiManagedLiveProjectionRetention;
use super::WorthUiQueryLiveResource;
use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryMeasurementFactSettlement,
    WorthUiQueryMeasurementFactSettlementDenial, WorthUiQueryViewDefinition,
    WorthUiQueryViewExecutionEvidenceDenial, WorthUiQueryViewExecutionEvidenceReference,
    WorthUiQueryViewIdentity,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiManagedLiveCompatibilityObservation {
    subsystem_construction_count: usize,
    retained_resource_count: usize,
    retained_projection_count: usize,
    resource_without_projection_count: usize,
    projection_without_resource_count: usize,
    resource_registration_count: usize,
    succession_operation_count: usize,
}

#[derive(Debug, Default)]
pub(crate) struct WorthUiManagedLiveCompatibilityRetention {
    projections: WorthUiManagedLiveProjectionRetention,
    resources: BTreeMap<WorthUiQueryViewIdentity, WorthUiQueryLiveResource>,
    resource_registration_count: usize,
    succession_operation_count: usize,
}

/// Observes one exact compatibility resource without exposing it or granting
/// lifecycle authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiExactManagedLiveResourceEvidence {
    installed_reference: WorthUiInstalledQueryBindingReference,
}

impl WorthUiManagedLiveCompatibilityRetention {
    pub(crate) fn admit_projection(
        &mut self,
        definition: WorthUiQueryViewDefinition,
        outcome: worth_query::facade::read::WorthQueryProjectionOutcome,
        installed_execution: worth_query::facade::domain::WorthQueryInstalledDomainExecutionReceipt,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        self.projections
            .admit(definition, outcome, installed_execution, reference)
    }

    pub(crate) fn execution_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewExecutionEvidenceDenial>
    {
        self.projections.execution_evidence_for(reference)
    }

    pub(crate) fn frame_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<crate::WorthUiQueryFrameEvidence, WorthUiQueryViewExecutionEvidenceDenial> {
        self.projections.frame_evidence_for(reference)
    }

    pub(crate) fn take_settlement(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<Arc<WorthUiQueryMeasurementFactSettlement>> {
        self.projections.take(reference)
    }

    pub(crate) fn replace_settlement(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
        settlement: Arc<WorthUiQueryMeasurementFactSettlement>,
    ) {
        self.projections.replace(reference, settlement);
    }

    pub(crate) fn retain_only_settlements(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
    ) {
        self.projections.retain_only(references);
    }

    pub(crate) fn contains(&self, reference: &WorthUiInstalledQueryBindingReference) -> bool {
        self.resources
            .contains_key(reference.definition().identity())
    }

    pub(crate) fn exact_evidence(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiExactManagedLiveResourceEvidence> {
        self.contains(reference)
            .then(|| WorthUiExactManagedLiveResourceEvidence {
                installed_reference: reference.clone(),
            })
    }

    pub(crate) fn insert_after_vacancy_check(
        &mut self,
        identity: WorthUiQueryViewIdentity,
        resource: WorthUiQueryLiveResource,
    ) {
        let replaced = self.resources.insert(identity, resource);
        debug_assert!(replaced.is_none(), "vacancy was proven before settlement");
        self.resource_registration_count += 1;
    }

    pub(crate) fn take(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiQueryLiveResource> {
        self.succession_operation_count += 1;
        self.resources.remove(reference.definition().identity())
    }

    pub(crate) fn replace(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
        resource: WorthUiQueryLiveResource,
    ) -> Option<WorthUiQueryLiveResource> {
        self.succession_operation_count += 1;
        self.resources
            .insert(reference.definition().identity().clone(), resource)
    }

    pub(crate) fn drain_into(&mut self, retirement: &mut Vec<WorthUiQueryLiveResource>) {
        self.succession_operation_count += 1;
        retirement.extend(std::mem::take(&mut self.resources).into_values());
    }

    pub(crate) fn retain_only(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
        retirement: &mut Vec<WorthUiQueryLiveResource>,
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
        self.projections.swap_with(&mut other.projections);
        std::mem::swap(&mut self.resources, &mut other.resources);
    }

    pub(crate) fn observation(&self) -> WorthUiManagedLiveCompatibilityObservation {
        let resource_without_projection_count = self
            .resources
            .keys()
            .filter(|identity| !self.projections.contains(identity))
            .count();
        let projection_without_resource_count = self
            .projections
            .len()
            .saturating_sub(self.resources.len() - resource_without_projection_count);
        WorthUiManagedLiveCompatibilityObservation {
            subsystem_construction_count: 1,
            retained_resource_count: self.resources.len(),
            retained_projection_count: self.projections.len(),
            resource_without_projection_count,
            projection_without_resource_count,
            resource_registration_count: self.resource_registration_count,
            succession_operation_count: self.succession_operation_count,
        }
    }
}

impl WorthUiManagedLiveCompatibilityObservation {
    pub fn subsystem_construction_count(self) -> usize {
        self.subsystem_construction_count
    }

    pub fn retained_resource_count(self) -> usize {
        self.retained_resource_count
    }

    pub fn retained_projection_count(self) -> usize {
        self.retained_projection_count
    }

    pub fn resource_without_projection_count(self) -> usize {
        self.resource_without_projection_count
    }

    pub fn projection_without_resource_count(self) -> usize {
        self.projection_without_resource_count
    }

    pub fn resource_registration_count(self) -> usize {
        self.resource_registration_count
    }

    pub fn succession_operation_count(self) -> usize {
        self.succession_operation_count
    }
}

impl WorthUiExactManagedLiveResourceEvidence {
    pub fn installed_reference(&self) -> &WorthUiInstalledQueryBindingReference {
        &self.installed_reference
    }
}
