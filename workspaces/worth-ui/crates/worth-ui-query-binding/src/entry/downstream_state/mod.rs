mod reference_catalog;
mod settled_snapshot;

use std::collections::BTreeMap;
use std::sync::Arc;

use reference_catalog::WorthUiInstalledReferenceCatalog;
use settled_snapshot::WorthUiSettledSnapshotRetention;

use crate::compatibility::managed_live::{
    WorthUiExactManagedLiveResourceEvidence, WorthUiManagedLiveCompatibilityObservation,
    WorthUiManagedLiveCompatibilityRetention,
};

use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryLiveAdmissionDenial,
    WorthUiQueryLiveAdmissionStop, WorthUiQueryLiveProjectionOutcome, WorthUiQueryLiveResource,
    WorthUiQueryMeasurementFactSettlement, WorthUiQueryViewExecutionEvidenceDenial,
    WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewIdentity,
};

/// Downstream-only state. Query progression values cannot enter this owner;
/// each retained compatibility lifecycle has a separate physical owner.
#[derive(Debug)]
pub struct WorthUiInstalledDownstreamQueryState {
    references: WorthUiInstalledReferenceCatalog,
    managed_live: Option<WorthUiManagedLiveCompatibilityRetention>,
    settled_snapshot: WorthUiSettledSnapshotRetention,
}

impl WorthUiInstalledDownstreamQueryState {
    pub(super) fn new(
        references: BTreeMap<WorthUiQueryViewIdentity, WorthUiInstalledQueryBindingReference>,
    ) -> Self {
        let references = WorthUiInstalledReferenceCatalog::new(references);
        let has_live_reference = references.has_live_reference();
        Self {
            references,
            managed_live: has_live_reference.then(Default::default),
            settled_snapshot: Default::default(),
        }
    }

    pub(super) fn swap_retained_state_with(&mut self, other: &mut Self) {
        match (&mut self.managed_live, &mut other.managed_live) {
            (Some(left), Some(right)) => left.swap_with(right),
            _ => std::mem::swap(&mut self.managed_live, &mut other.managed_live),
        }
        self.settled_snapshot.swap_with(&mut other.settled_snapshot);
    }

    pub(super) fn admit_settled_snapshot(
        &mut self,
        projection: crate::WorthUiSettledSnapshotProjection,
    ) -> Result<crate::WorthUiSettledSnapshotFact, crate::WorthUiSettledSnapshotAdmissionStop> {
        let reference = projection.installed_reference().clone();
        if self.references.validate(&reference).is_err() {
            return Err(crate::WorthUiSettledSnapshotAdmissionStop::new(
                crate::WorthUiSettledSnapshotAdmissionDenial::ForeignInstalledReference,
                projection,
            ));
        }
        self.settled_snapshot.admit(projection, &reference)
    }

    pub(super) fn refresh_settled_snapshot(
        &mut self,
        projection: crate::WorthUiSettledSnapshotProjection,
    ) -> Result<crate::WorthUiSettledSnapshotFact, crate::WorthUiSettledSnapshotAdmissionStop> {
        let reference = projection.installed_reference().clone();
        if self.references.validate(&reference).is_err() {
            return Err(crate::WorthUiSettledSnapshotAdmissionStop::new(
                crate::WorthUiSettledSnapshotAdmissionDenial::ForeignInstalledReference,
                projection,
            ));
        }
        self.settled_snapshot.refresh(projection, &reference)
    }

    pub(super) fn settled_snapshot_fact_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<&crate::WorthUiSettledSnapshotFact, WorthUiQueryViewExecutionEvidenceDenial> {
        self.validate_reference(reference)?;
        self.settled_snapshot
            .fact_for(reference)
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)
    }

    pub(super) fn exact_settled_snapshot_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<
        Option<crate::WorthUiExactSettledSnapshotEvidence>,
        WorthUiQueryViewExecutionEvidenceDenial,
    > {
        self.validate_reference(reference)?;
        Ok(self.settled_snapshot.exact_evidence_for(reference))
    }

    #[cfg(test)]
    pub(super) fn rebuild_settled_snapshot_index(&mut self) {
        self.settled_snapshot.rebuild_index();
    }

    pub(super) fn admit_live(
        &mut self,
        resource: WorthUiQueryLiveResource,
        outcome: WorthUiQueryLiveProjectionOutcome,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryLiveAdmissionStop> {
        let (definition, outcome, installed_execution) = outcome.into_transfer().into_parts();
        let reference = match self.references.reference_for_projection(&definition) {
            Ok(reference) => reference,
            Err(denial) => {
                return Err(WorthUiQueryLiveAdmissionStop::new(
                    WorthUiQueryLiveAdmissionDenial::Projection(denial),
                    resource,
                ));
            }
        };
        let resource =
            validate_live_resource(reference, &definition, &installed_execution, resource)?;
        let Some(managed_live) = self.managed_live.as_mut() else {
            return Err(WorthUiQueryLiveAdmissionStop::new(
                WorthUiQueryLiveAdmissionDenial::CompatibilityNotInstalled,
                resource,
            ));
        };
        if managed_live.contains(reference) {
            return Err(WorthUiQueryLiveAdmissionStop::new(
                WorthUiQueryLiveAdmissionDenial::LiveResourceAlreadyAdmitted,
                resource,
            ));
        }
        let identity = definition.identity().clone();
        let settlement = match managed_live.admit_projection(
            definition,
            outcome,
            installed_execution,
            reference,
        ) {
            Ok(settlement) => settlement,
            Err(denial) => {
                return Err(WorthUiQueryLiveAdmissionStop::new(
                    WorthUiQueryLiveAdmissionDenial::Projection(denial),
                    resource,
                ));
            }
        };
        managed_live.insert_after_vacancy_check(identity, resource);
        Ok(settlement)
    }

    pub(super) fn validate_reference(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<(), WorthUiQueryViewExecutionEvidenceDenial> {
        self.references.validate(reference)
    }

    pub(super) fn installation_is_current(&self) -> bool {
        self.references.installation_is_current()
    }

    pub(super) fn execution_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewExecutionEvidenceDenial>
    {
        self.validate_reference(reference)?;
        if let Some(fact) = self.settled_snapshot.shared_fact_for(reference) {
            return Ok(
                WorthUiQueryViewExecutionEvidenceReference::from_settled_snapshot(
                    reference.clone(),
                    fact,
                ),
            );
        }
        self.managed_live
            .as_ref()
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)?
            .execution_evidence_for(reference)
    }

    pub(super) fn frame_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<crate::WorthUiQueryFrameEvidence, WorthUiQueryViewExecutionEvidenceDenial> {
        self.validate_reference(reference)?;
        if let Some(fact) = self.settled_snapshot.fact_for(reference) {
            return Ok(crate::WorthUiQueryFrameEvidence::from_settled_snapshot(
                reference, fact,
            ));
        }
        self.managed_live
            .as_ref()
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)?
            .frame_evidence_for(reference)
    }

    pub(super) fn exact_live_resource_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<
        Option<WorthUiExactManagedLiveResourceEvidence>,
        WorthUiQueryViewExecutionEvidenceDenial,
    > {
        self.validate_reference(reference)?;
        Ok(self
            .managed_live
            .as_ref()
            .and_then(|managed_live| managed_live.exact_evidence(reference)))
    }

    pub(super) fn retains_live_resource_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<bool, WorthUiQueryViewExecutionEvidenceDenial> {
        self.exact_live_resource_evidence_for(reference)
            .map(|evidence| evidence.is_some())
    }

    pub(super) fn take_settlement(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<Arc<WorthUiQueryMeasurementFactSettlement>> {
        self.managed_live
            .as_mut()
            .and_then(|managed_live| managed_live.take_settlement(reference))
    }

    pub(super) fn replace_settlement(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
        settlement: Arc<WorthUiQueryMeasurementFactSettlement>,
    ) {
        self.managed_live
            .get_or_insert_with(Default::default)
            .replace_settlement(reference, settlement);
    }

    pub(super) fn take_live_resource(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiQueryLiveResource> {
        self.managed_live
            .as_mut()
            .and_then(|managed_live| managed_live.take(reference))
    }

    pub(super) fn replace_live_resource(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
        resource: WorthUiQueryLiveResource,
    ) -> Option<WorthUiQueryLiveResource> {
        match self.managed_live.as_mut() {
            Some(managed_live) => managed_live.replace(reference, resource),
            None if self.references.has_live_reference() => {
                let mut managed_live = WorthUiManagedLiveCompatibilityRetention::default();
                let displaced = managed_live.replace(reference, resource);
                self.managed_live = Some(managed_live);
                displaced
            }
            None => Some(resource),
        }
    }

    pub(super) fn drain_live_resources_into(
        &mut self,
        retirement: &mut Vec<WorthUiQueryLiveResource>,
    ) {
        if let Some(managed_live) = self.managed_live.as_mut() {
            managed_live.drain_into(retirement);
        }
    }

    pub(super) fn retain_only_live_resources_for(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
        retirement: &mut Vec<WorthUiQueryLiveResource>,
    ) {
        if let Some(managed_live) = self.managed_live.as_mut() {
            managed_live.retain_only(references, retirement);
        }
    }

    pub(super) fn finish_managed_live_succession(
        &mut self,
        retirement: &mut Vec<WorthUiQueryLiveResource>,
    ) {
        if self.references.has_live_reference() {
            return;
        }
        if let Some(mut managed_live) = self.managed_live.take() {
            managed_live.drain_into(retirement);
        }
    }

    pub(super) fn retain_only_settlements_for(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
    ) {
        if let Some(managed_live) = self.managed_live.as_mut() {
            managed_live.retain_only_settlements(references);
        }
        self.settled_snapshot.retain_only(references);
    }

    pub(super) fn take_settled_snapshot(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<crate::WorthUiSettledSnapshotProjection> {
        self.settled_snapshot.take(reference)
    }

    pub(super) fn replace_settled_snapshot(
        &mut self,
        projection: crate::WorthUiSettledSnapshotProjection,
    ) {
        self.settled_snapshot.replace(projection);
    }

    pub(super) fn managed_live_compatibility_observation(
        &self,
    ) -> WorthUiManagedLiveCompatibilityObservation {
        self.managed_live
            .as_ref()
            .map_or_else(Default::default, |managed_live| managed_live.observation())
    }

    pub(super) fn query_state_observation(&self) -> crate::WorthUiRuntimeQueryStateObservation {
        let (settled_snapshot_count, orphan_settled_snapshot_count) = self
            .settled_snapshot
            .observation_counts(|reference| self.references.validate(reference).is_ok());
        crate::WorthUiRuntimeQueryStateObservation::installed(
            self.references.len(),
            self.references.stale_reference_count(),
            settled_snapshot_count,
            orphan_settled_snapshot_count,
            self.managed_live_compatibility_observation(),
        )
    }
}

fn validate_live_resource(
    reference: &WorthUiInstalledQueryBindingReference,
    definition: &crate::WorthUiQueryViewDefinition,
    installed_execution: &worth_query::facade::domain::WorthQueryInstalledDomainExecutionReceipt,
    resource: WorthUiQueryLiveResource,
) -> Result<WorthUiQueryLiveResource, WorthUiQueryLiveAdmissionStop> {
    if resource.installed_authority() != &reference.installed_domain().handle().authority_witness()
    {
        return Err(WorthUiQueryLiveAdmissionStop::new(
            WorthUiQueryLiveAdmissionDenial::InstalledAuthorityMismatch,
            resource,
        ));
    }
    if resource.definition() != definition {
        return Err(WorthUiQueryLiveAdmissionStop::new(
            WorthUiQueryLiveAdmissionDenial::ViewDefinitionMismatch,
            resource,
        ));
    }
    if !resource.matches_projection_resource(installed_execution) {
        return Err(WorthUiQueryLiveAdmissionStop::new(
            WorthUiQueryLiveAdmissionDenial::ProjectionResourceMismatch,
            resource,
        ));
    }
    Ok(resource)
}
