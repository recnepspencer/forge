use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use super::WorthUiQueryAllocationSourceAuthority;
use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryMeasurementFactSettlement,
    WorthUiQueryMeasurementFactSettlementDenial, WorthUiQueryViewDefinition,
    WorthUiQueryViewExecutionEvidenceDenial, WorthUiQueryViewExecutionEvidenceReference,
    WorthUiQueryViewIdentity,
};

/// Retains projection settlement only for the transitional managed-live lane.
/// Ordinary snapshots use Query's exact settled projection owner.
#[derive(Debug, Default)]
pub(in crate::compatibility::managed_live) struct WorthUiManagedLiveProjectionRetention {
    allocation_source_authority: WorthUiQueryAllocationSourceAuthority,
    settlements: BTreeMap<WorthUiQueryViewIdentity, Arc<WorthUiQueryMeasurementFactSettlement>>,
}

impl WorthUiManagedLiveProjectionRetention {
    pub(in crate::compatibility::managed_live) fn len(&self) -> usize {
        self.settlements.len()
    }

    pub(in crate::compatibility::managed_live) fn contains(
        &self,
        identity: &WorthUiQueryViewIdentity,
    ) -> bool {
        self.settlements.contains_key(identity)
    }

    pub(in crate::compatibility::managed_live) fn admit(
        &mut self,
        definition: WorthUiQueryViewDefinition,
        outcome: worth_query::facade::read::WorthQueryProjectionOutcome,
        installed_execution: worth_query::facade::domain::WorthQueryInstalledDomainExecutionReceipt,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        if installed_execution.installed_authority()
            != &reference.installed_domain().handle().authority_witness()
        {
            return Err(WorthUiQueryMeasurementFactSettlementDenial::InstalledAuthorityMismatch);
        }
        let identity = definition.identity().clone();
        let settlement =
            self.allocation_source_authority
                .admit(definition, outcome, installed_execution)?;
        self.settlements
            .insert(identity, Arc::new(settlement.clone()));
        Ok(settlement)
    }

    pub(in crate::compatibility::managed_live) fn execution_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewExecutionEvidenceDenial>
    {
        self.settlements
            .get(reference.definition().identity())
            .cloned()
            .map(WorthUiQueryViewExecutionEvidenceReference::from_managed_live_compatibility)
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)
    }

    pub(in crate::compatibility::managed_live) fn frame_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<crate::WorthUiQueryFrameEvidence, WorthUiQueryViewExecutionEvidenceDenial> {
        self.settlements
            .get(reference.definition().identity())
            .map(|settlement| {
                crate::WorthUiQueryFrameEvidence::from_managed_live_compatibility(settlement)
            })
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)
    }

    pub(in crate::compatibility::managed_live) fn take(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<Arc<WorthUiQueryMeasurementFactSettlement>> {
        self.settlements.remove(reference.definition().identity())
    }

    pub(in crate::compatibility::managed_live) fn replace(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
        settlement: Arc<WorthUiQueryMeasurementFactSettlement>,
    ) {
        self.settlements
            .insert(reference.definition().identity().clone(), settlement);
    }

    pub(in crate::compatibility::managed_live) fn retain_only(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
    ) {
        let retained = references
            .iter()
            .map(|reference| reference.definition().identity())
            .collect::<BTreeSet<_>>();
        self.settlements
            .retain(|identity, _| retained.contains(identity));
    }

    pub(in crate::compatibility::managed_live) fn swap_with(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.settlements, &mut other.settlements);
    }
}
