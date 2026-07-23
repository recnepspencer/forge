mod reference_catalog;
mod settled_snapshot;

use std::collections::BTreeMap;

use crate::operation_live::WorthUiOperationLiveRetention;
use reference_catalog::WorthUiInstalledReferenceCatalog;
use settled_snapshot::WorthUiSettledSnapshotRetention;

use crate::{
    WorthUiExactOperationLiveResourceEvidence, WorthUiInstalledQueryBindingReference,
    WorthUiOperationLiveAdmissionDenial, WorthUiOperationLiveAdmissionStop,
    WorthUiOperationLiveResource, WorthUiQueryViewExecutionEvidenceDenial,
    WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewIdentity,
};

/// Downstream-only state. Query progression values cannot enter this owner;
/// each retained compatibility lifecycle has a separate physical owner.
#[derive(Debug)]
pub struct WorthUiInstalledDownstreamQueryState {
    references: WorthUiInstalledReferenceCatalog,
    operation_live: Option<WorthUiOperationLiveRetention>,
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
            operation_live: has_live_reference.then(Default::default),
            settled_snapshot: Default::default(),
        }
    }

    pub(super) fn swap_retained_state_with(&mut self, other: &mut Self) {
        match (&mut self.operation_live, &mut other.operation_live) {
            (Some(left), Some(right)) => left.swap_with(right),
            _ => std::mem::swap(&mut self.operation_live, &mut other.operation_live),
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

    pub(super) fn admit_operation_live(
        &mut self,
        resource: WorthUiOperationLiveResource,
    ) -> Result<(), WorthUiOperationLiveAdmissionStop> {
        let reference = resource.installed_reference().clone();
        if self.references.validate(&reference).is_err() {
            return Err(WorthUiOperationLiveAdmissionStop::new(
                WorthUiOperationLiveAdmissionDenial::ForeignInstalledReference,
                resource,
            ));
        };
        let Some(operation_live) = self.operation_live.as_mut() else {
            return Err(WorthUiOperationLiveAdmissionStop::new(
                WorthUiOperationLiveAdmissionDenial::ForeignInstalledReference,
                resource,
            ));
        };
        if operation_live.contains(&reference) {
            return Err(WorthUiOperationLiveAdmissionStop::new(
                WorthUiOperationLiveAdmissionDenial::DuplicateResource,
                resource,
            ));
        }
        operation_live.admit(resource)
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
        let resource = self
            .operation_live
            .as_ref()
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)?
            .resource(reference)
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)?;
        Ok(
            WorthUiQueryViewExecutionEvidenceReference::from_settled_snapshot(
                reference.clone(),
                resource.shared_fact(),
            ),
        )
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
        let resource = self
            .operation_live
            .as_ref()
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)?
            .resource(reference)
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)?;
        Ok(crate::WorthUiQueryFrameEvidence::from_settled_snapshot(
            reference,
            resource.fact(),
        ))
    }

    pub(super) fn exact_operation_live_resource_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<
        Option<WorthUiExactOperationLiveResourceEvidence>,
        WorthUiQueryViewExecutionEvidenceDenial,
    > {
        self.validate_reference(reference)?;
        Ok(self
            .operation_live
            .as_ref()
            .and_then(|operation_live| operation_live.exact_evidence(reference)))
    }

    pub(super) fn retains_operation_live_resource_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<bool, WorthUiQueryViewExecutionEvidenceDenial> {
        self.exact_operation_live_resource_evidence_for(reference)
            .map(|evidence| evidence.is_some())
    }

    pub(super) fn take_operation_live_resource(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiOperationLiveResource> {
        self.operation_live
            .as_mut()
            .and_then(|operation_live| operation_live.take(reference))
    }

    pub(super) fn replace_operation_live_resource(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
        resource: WorthUiOperationLiveResource,
    ) -> Option<WorthUiOperationLiveResource> {
        debug_assert_eq!(resource.installed_reference(), reference);
        match self.operation_live.as_mut() {
            Some(operation_live) => operation_live.insert(resource),
            None if self.references.has_live_reference() => {
                let mut operation_live = WorthUiOperationLiveRetention::default();
                let displaced = operation_live.insert(resource);
                self.operation_live = Some(operation_live);
                displaced
            }
            None => Some(resource),
        }
    }

    pub(super) fn drain_operation_live_resources_into(
        &mut self,
        retirement: &mut Vec<WorthUiOperationLiveResource>,
    ) {
        if let Some(operation_live) = self.operation_live.as_mut() {
            operation_live.drain_into(retirement);
        }
    }

    pub(super) fn retain_only_operation_live_resources_for(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
        retirement: &mut Vec<WorthUiOperationLiveResource>,
    ) {
        if let Some(operation_live) = self.operation_live.as_mut() {
            operation_live.retain_only(references, retirement);
        }
    }

    pub(super) fn finish_operation_live_succession(
        &mut self,
        retirement: &mut Vec<WorthUiOperationLiveResource>,
    ) {
        if self.references.has_live_reference() {
            return;
        }
        if let Some(mut operation_live) = self.operation_live.take() {
            operation_live.drain_into(retirement);
        }
    }

    pub(super) fn retain_only_settlements_for(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
    ) {
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

    pub(super) fn operation_live_observation(&self) -> crate::WorthUiOperationLiveObservation {
        self.operation_live
            .as_ref()
            .map_or_else(Default::default, |operation_live| {
                operation_live.observation(|reference| self.references.validate(reference).is_ok())
            })
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
            self.operation_live_observation(),
        )
    }
}
