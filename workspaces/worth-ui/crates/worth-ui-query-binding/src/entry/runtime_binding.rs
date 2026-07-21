use std::sync::Arc;

use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryLiveAdmissionDenial,
    WorthUiQueryLiveAdmissionStop, WorthUiQueryLiveProjectionOutcome, WorthUiQueryLiveResource,
    WorthUiQueryMeasurementFactSettlement, WorthUiQueryViewExecutionEvidenceReference,
};

use super::WorthUiInstalledDownstreamQueryState;

/// Runtime binding posture. The installed variant owns downstream UI state,
/// never a Query operating world or move-only Query progression value.
#[derive(Debug)]
pub enum WorthUiRuntimeQueryBinding {
    QueryFree,
    Installed(Box<WorthUiInstalledDownstreamQueryState>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewExecutionEvidenceDenial {
    QueryNotInstalled,
    ForeignInstalledReference,
    ProjectionNotAdmitted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSettledSnapshotAdmissionDenial {
    QueryNotInstalled,
    ForeignInstalledReference,
    DuplicateSettlement,
    MissingPredecessorSettlement,
    SourceGenerationExhausted,
    SourceOrderExhausted,
}

pub struct WorthUiSettledSnapshotAdmissionStop {
    denial: WorthUiSettledSnapshotAdmissionDenial,
    projection: Box<crate::WorthUiSettledSnapshotProjection>,
}

impl std::fmt::Debug for WorthUiSettledSnapshotAdmissionStop {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiSettledSnapshotAdmissionStop")
            .field("denial", &self.denial)
            .field("projection", &"returned exact Query projection")
            .finish()
    }
}

impl WorthUiSettledSnapshotAdmissionStop {
    pub(crate) fn new(
        denial: WorthUiSettledSnapshotAdmissionDenial,
        projection: crate::WorthUiSettledSnapshotProjection,
    ) -> Self {
        Self {
            denial,
            projection: Box::new(projection),
        }
    }

    pub fn denial(&self) -> WorthUiSettledSnapshotAdmissionDenial {
        self.denial
    }
    pub fn into_projection(self) -> crate::WorthUiSettledSnapshotProjection {
        *self.projection
    }
}

impl WorthUiRuntimeQueryBinding {
    pub fn state_observation(&self) -> crate::WorthUiRuntimeQueryStateObservation {
        match self {
            Self::QueryFree => Default::default(),
            Self::Installed(binding) => binding.query_state_observation(),
        }
    }

    pub fn reference_membership_observation(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> crate::WorthUiQueryReferenceMembershipObservation {
        match self {
            Self::QueryFree => crate::WorthUiQueryReferenceMembershipObservation::QueryFree,
            Self::Installed(binding) if binding.validate_reference(reference).is_ok() => {
                crate::WorthUiQueryReferenceMembershipObservation::ExactInstalledReference
            }
            Self::Installed(_) => {
                crate::WorthUiQueryReferenceMembershipObservation::ForeignInstalledReference
            }
        }
    }

    pub fn managed_live_compatibility_observation(
        &self,
    ) -> crate::compatibility::managed_live::WorthUiManagedLiveCompatibilityObservation {
        match self {
            Self::QueryFree => Default::default(),
            Self::Installed(binding) => binding.managed_live_compatibility_observation(),
        }
    }

    pub fn admit_settled_snapshot(
        &mut self,
        projection: crate::WorthUiSettledSnapshotProjection,
    ) -> Result<crate::WorthUiSettledSnapshotFact, WorthUiSettledSnapshotAdmissionStop> {
        match self {
            Self::QueryFree => Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::QueryNotInstalled,
                projection,
            )),
            Self::Installed(binding) => binding.admit_settled_snapshot(projection),
        }
    }

    pub fn refresh_settled_snapshot(
        &mut self,
        projection: crate::WorthUiSettledSnapshotProjection,
    ) -> Result<crate::WorthUiSettledSnapshotFact, WorthUiSettledSnapshotAdmissionStop> {
        match self {
            Self::QueryFree => Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::QueryNotInstalled,
                projection,
            )),
            Self::Installed(binding) => binding.refresh_settled_snapshot(projection),
        }
    }

    pub fn settled_snapshot_fact_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<&crate::WorthUiSettledSnapshotFact, WorthUiQueryViewExecutionEvidenceDenial> {
        match self {
            Self::QueryFree => Err(WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.settled_snapshot_fact_for(reference),
        }
    }

    pub fn exact_settled_snapshot_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<
        Option<crate::WorthUiExactSettledSnapshotEvidence>,
        WorthUiQueryViewExecutionEvidenceDenial,
    > {
        match self {
            Self::QueryFree => Err(WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.exact_settled_snapshot_evidence_for(reference),
        }
    }

    #[cfg(test)]
    pub(crate) fn rebuild_settled_snapshot_index_for_test(&mut self) {
        if let Self::Installed(binding) = self {
            binding.rebuild_settled_snapshot_index();
        }
    }
    pub(crate) fn swap_runtime_state_with(&mut self, other: &mut Self) {
        let (Self::Installed(left), Self::Installed(right)) = (self, other) else {
            return;
        };
        left.swap_retained_state_with(right);
    }

    pub(crate) fn admits_reference(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> bool {
        match self {
            Self::QueryFree => false,
            Self::Installed(binding) => binding.validate_reference(reference).is_ok(),
        }
    }

    pub(crate) fn installation_is_current(&self) -> bool {
        match self {
            Self::QueryFree => true,
            Self::Installed(binding) => binding.installation_is_current(),
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
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<WorthUiQueryViewExecutionEvidenceReference, WorthUiQueryViewExecutionEvidenceDenial>
    {
        match self {
            Self::QueryFree => Err(WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.execution_evidence_for(reference),
        }
    }

    pub fn frame_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<crate::WorthUiQueryFrameEvidence, WorthUiQueryViewExecutionEvidenceDenial> {
        match self {
            Self::QueryFree => Err(WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.frame_evidence_for(reference),
        }
    }

    pub fn exact_live_resource_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<
        Option<crate::WorthUiExactManagedLiveResourceEvidence>,
        WorthUiQueryViewExecutionEvidenceDenial,
    > {
        match self {
            Self::QueryFree => Err(WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.exact_live_resource_evidence_for(reference),
        }
    }

    pub fn retains_live_resource_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<bool, WorthUiQueryViewExecutionEvidenceDenial> {
        match self {
            Self::QueryFree => Err(WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.retains_live_resource_for(reference),
        }
    }

    pub(crate) fn take_settlement(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<Arc<WorthUiQueryMeasurementFactSettlement>> {
        match self {
            Self::QueryFree => None,
            Self::Installed(binding) => binding.take_settlement(reference),
        }
    }

    pub(crate) fn replace_settlement(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
        settlement: Arc<WorthUiQueryMeasurementFactSettlement>,
    ) {
        let Self::Installed(binding) = self else {
            unreachable!("validated successor reference requires an installed binding")
        };
        binding.replace_settlement(reference, settlement);
    }

    pub(crate) fn take_settled_snapshot(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<crate::WorthUiSettledSnapshotProjection> {
        match self {
            Self::QueryFree => None,
            Self::Installed(binding) => binding.take_settled_snapshot(reference),
        }
    }

    pub(crate) fn replace_settled_snapshot(
        &mut self,
        projection: crate::WorthUiSettledSnapshotProjection,
    ) {
        let Self::Installed(binding) = self else {
            unreachable!("validated exact Query settlement requires an installed binding")
        };
        binding.replace_settled_snapshot(projection);
    }

    pub(crate) fn take_live_resource(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiQueryLiveResource> {
        match self {
            Self::QueryFree => None,
            Self::Installed(binding) => binding.take_live_resource(reference),
        }
    }

    pub(crate) fn replace_live_resource(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
        resource: WorthUiQueryLiveResource,
    ) -> Option<WorthUiQueryLiveResource> {
        let Self::Installed(binding) = self else {
            unreachable!("validated successor reference requires an installed binding")
        };
        binding.replace_live_resource(reference, resource)
    }

    pub(crate) fn drain_live_resources_into(
        &mut self,
        retirement: &mut Vec<WorthUiQueryLiveResource>,
    ) {
        if let Self::Installed(binding) = self {
            binding.drain_live_resources_into(retirement);
        }
    }

    pub(crate) fn retain_only_live_resources_for(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
        retirement: &mut Vec<WorthUiQueryLiveResource>,
    ) {
        if let Self::Installed(binding) = self {
            binding.retain_only_live_resources_for(references, retirement);
        }
    }

    pub(crate) fn finish_managed_live_succession(
        &mut self,
        retirement: &mut Vec<WorthUiQueryLiveResource>,
    ) {
        if let Self::Installed(binding) = self {
            binding.finish_managed_live_succession(retirement);
        }
    }

    pub(crate) fn retain_only_settlements_for(
        &mut self,
        references: &[WorthUiInstalledQueryBindingReference],
    ) {
        if let Self::Installed(binding) = self {
            binding.retain_only_settlements_for(references);
        }
    }
}
