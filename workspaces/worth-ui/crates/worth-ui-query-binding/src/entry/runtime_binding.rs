use std::collections::BTreeMap;
use std::sync::Arc;

use crate::prerequisites::WorthUiQueryAllocationSourceAuthority;
use crate::{
    WorthUiInstalledQueryDomain, WorthUiQueryLiveAdmissionDenial, WorthUiQueryLiveAdmissionStop,
    WorthUiQueryLiveProjectionOutcome, WorthUiQueryLiveResource,
    WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial,
    WorthUiQuerySnapshotProjectionOutcome, WorthUiQueryViewDefinition,
    WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewIdentity,
};

/// Runtime binding state. Only the installed variant can consume a projection.
#[derive(Debug)]
pub enum WorthUiRuntimeQueryBinding {
    QueryFree,
    Installed(Box<WorthUiQueryBindingSubsystem>),
}

#[derive(Debug)]
pub struct WorthUiQueryBindingSubsystem {
    installed_domain: WorthUiInstalledQueryDomain,
    definitions: BTreeMap<WorthUiQueryViewIdentity, WorthUiQueryViewDefinition>,
    allocation_source_authority: WorthUiQueryAllocationSourceAuthority,
    latest_settlements:
        BTreeMap<WorthUiQueryViewIdentity, Arc<WorthUiQueryMeasurementFactSettlement>>,
    live_resources: BTreeMap<WorthUiQueryViewIdentity, WorthUiQueryLiveResource>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewExecutionEvidenceDenial {
    QueryNotInstalled,
    ForeignInstalledReference,
    ProjectionNotAdmitted,
}

impl WorthUiRuntimeQueryBinding {
    pub(crate) fn swap_runtime_state_with(&mut self, other: &mut Self) {
        let (Self::Installed(left), Self::Installed(right)) = (self, other) else {
            return;
        };
        std::mem::swap(&mut left.latest_settlements, &mut right.latest_settlements);
        std::mem::swap(&mut left.live_resources, &mut right.live_resources);
    }

    pub(crate) fn admits_reference(
        &self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> bool {
        match self {
            Self::QueryFree => false,
            Self::Installed(binding) => binding.validate_reference(reference).is_ok(),
        }
    }

    pub fn admit(
        &mut self,
        outcome: WorthUiQuerySnapshotProjectionOutcome,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        match self {
            Self::QueryFree => Err(WorthUiQueryMeasurementFactSettlementDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.admit(outcome),
        }
    }

    pub fn admit_live(
        &mut self,
        resource: WorthUiQueryLiveResource,
        outcome: WorthUiQueryLiveProjectionOutcome,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryLiveAdmissionStop> {
        match self {
            Self::QueryFree => Err(WorthUiQueryLiveAdmissionStop::new(
                WorthUiQueryLiveAdmissionDenial::QueryNotInstalled,
                resource,
            )),
            Self::Installed(binding) => binding.admit_live(resource, outcome),
        }
    }

    pub fn execution_evidence_for(
        &self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> Result<WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewExecutionEvidenceDenial>
    {
        match self {
            Self::QueryFree => Err(WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.execution_evidence_for(reference),
        }
    }

    pub fn retains_live_resource_for(
        &self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> Result<bool, WorthUiQueryViewExecutionEvidenceDenial> {
        match self {
            Self::QueryFree => Err(WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.retains_live_resource_for(reference),
        }
    }

    pub(crate) fn take_settlement(
        &mut self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> Option<Arc<WorthUiQueryMeasurementFactSettlement>> {
        match self {
            Self::QueryFree => None,
            Self::Installed(binding) => binding
                .latest_settlements
                .remove(reference.definition().identity()),
        }
    }

    pub(crate) fn replace_settlement(
        &mut self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
        settlement: Arc<WorthUiQueryMeasurementFactSettlement>,
    ) {
        let Self::Installed(binding) = self else {
            unreachable!("validated successor reference requires an installed binding")
        };
        binding
            .latest_settlements
            .insert(reference.definition().identity().clone(), settlement);
    }

    pub(crate) fn take_live_resource(
        &mut self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiQueryLiveResource> {
        match self {
            Self::QueryFree => None,
            Self::Installed(binding) => binding
                .live_resources
                .remove(reference.definition().identity()),
        }
    }

    pub(crate) fn replace_live_resource(
        &mut self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
        resource: WorthUiQueryLiveResource,
    ) -> Option<WorthUiQueryLiveResource> {
        let Self::Installed(binding) = self else {
            unreachable!("validated successor reference requires an installed binding")
        };
        binding
            .live_resources
            .insert(reference.definition().identity().clone(), resource)
    }

    pub(crate) fn drain_live_resources_into(
        &mut self,
        retirement: &mut Vec<WorthUiQueryLiveResource>,
    ) {
        if let Self::Installed(binding) = self {
            retirement.extend(std::mem::take(&mut binding.live_resources).into_values());
        }
    }

    pub(crate) fn retain_only_live_resources_for(
        &mut self,
        references: &[crate::WorthUiInstalledQueryBindingReference],
        retirement: &mut Vec<WorthUiQueryLiveResource>,
    ) {
        let Self::Installed(binding) = self else {
            return;
        };
        let retained = references
            .iter()
            .map(|reference| reference.definition().identity())
            .collect::<std::collections::BTreeSet<_>>();
        let resources = std::mem::take(&mut binding.live_resources);
        for (identity, resource) in resources {
            if retained.contains(&identity) {
                binding.live_resources.insert(identity, resource);
            } else {
                retirement.push(resource);
            }
        }
    }

    pub(crate) fn retain_only_settlements_for(
        &mut self,
        references: &[crate::WorthUiInstalledQueryBindingReference],
    ) {
        let Self::Installed(binding) = self else {
            return;
        };
        let retained = references
            .iter()
            .map(|reference| reference.definition().identity())
            .collect::<std::collections::BTreeSet<_>>();
        binding
            .latest_settlements
            .retain(|identity, _| retained.contains(identity));
    }
}

impl WorthUiQueryBindingSubsystem {
    pub(super) fn new(
        installed_domain: WorthUiInstalledQueryDomain,
        definitions: BTreeMap<WorthUiQueryViewIdentity, WorthUiQueryViewDefinition>,
    ) -> Self {
        Self {
            installed_domain,
            definitions,
            allocation_source_authority: Default::default(),
            latest_settlements: BTreeMap::new(),
            live_resources: BTreeMap::new(),
        }
    }

    fn admit(
        &mut self,
        outcome: WorthUiQuerySnapshotProjectionOutcome,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        let (definition, outcome, installed_execution) = outcome.into_transfer().into_parts();
        self.admit_projection_parts(definition, outcome, installed_execution)
    }

    fn admit_projection_parts(
        &mut self,
        definition: WorthUiQueryViewDefinition,
        outcome: worth_query::facade::read::WorthQueryProjectionOutcome,
        installed_execution: worth_query::facade::domain::WorthQueryInstalledDomainExecutionReceipt,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        if installed_execution.installed_authority()
            != &self.installed_domain.handle().authority_witness()
        {
            return Err(WorthUiQueryMeasurementFactSettlementDenial::InstalledAuthorityMismatch);
        }
        if self.definitions.get(definition.identity()) != Some(&definition) {
            return Err(WorthUiQueryMeasurementFactSettlementDenial::UnregisteredView);
        }
        let identity = definition.identity().clone();
        let settlement =
            self.allocation_source_authority
                .admit(definition, outcome, installed_execution)?;
        self.latest_settlements
            .insert(identity, Arc::new(settlement.clone()));
        Ok(settlement)
    }

    fn admit_live(
        &mut self,
        resource: WorthUiQueryLiveResource,
        outcome: WorthUiQueryLiveProjectionOutcome,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryLiveAdmissionStop> {
        let (definition, outcome, installed_execution) = outcome.into_transfer().into_parts();
        if resource.installed_authority() != &self.installed_domain.handle().authority_witness() {
            return Err(WorthUiQueryLiveAdmissionStop::new(
                WorthUiQueryLiveAdmissionDenial::InstalledAuthorityMismatch,
                resource,
            ));
        }
        if resource.definition() != &definition {
            return Err(WorthUiQueryLiveAdmissionStop::new(
                WorthUiQueryLiveAdmissionDenial::ViewDefinitionMismatch,
                resource,
            ));
        }
        if !resource.matches_projection_resource(&installed_execution) {
            return Err(WorthUiQueryLiveAdmissionStop::new(
                WorthUiQueryLiveAdmissionDenial::ProjectionResourceMismatch,
                resource,
            ));
        }
        if self.live_resources.contains_key(definition.identity()) {
            return Err(WorthUiQueryLiveAdmissionStop::new(
                WorthUiQueryLiveAdmissionDenial::LiveResourceAlreadyAdmitted,
                resource,
            ));
        }
        let identity = definition.identity().clone();
        let settlement = match self.admit_projection_parts(definition, outcome, installed_execution)
        {
            Ok(settlement) => settlement,
            Err(denial) => {
                return Err(WorthUiQueryLiveAdmissionStop::new(
                    WorthUiQueryLiveAdmissionDenial::Projection(denial),
                    resource,
                ));
            }
        };
        self.live_resources.insert(identity, resource);
        Ok(settlement)
    }

    fn execution_evidence_for(
        &self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> Result<WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewExecutionEvidenceDenial>
    {
        self.validate_reference(reference)?;
        self.latest_settlements
            .get(reference.definition().identity())
            .cloned()
            .map(WorthUiQueryViewExecutionEvidenceReference::new)
            .ok_or(WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted)
    }

    fn retains_live_resource_for(
        &self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> Result<bool, WorthUiQueryViewExecutionEvidenceDenial> {
        self.validate_reference(reference)?;
        Ok(self
            .live_resources
            .contains_key(reference.definition().identity()))
    }

    fn validate_reference(
        &self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> Result<(), WorthUiQueryViewExecutionEvidenceDenial> {
        if !self
            .installed_domain
            .shares_authority_with(reference.installed_domain())
            || self.definitions.get(reference.definition().identity())
                != Some(reference.definition())
        {
            return Err(WorthUiQueryViewExecutionEvidenceDenial::ForeignInstalledReference);
        }
        Ok(())
    }
}
