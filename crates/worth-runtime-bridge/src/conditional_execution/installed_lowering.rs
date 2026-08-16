use std::sync::Arc;

use worth_signal::facade::InstalledSignalConditionalContract;

use super::{BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeConditionalProviderSet};
use crate::correspondence::BridgeInstalledSemanticCorrespondence;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BridgeInstalledConditionalLoweringCounters {
    pub contract_admission_checks: usize,
    pub correspondence_registrations_inspected: usize,
    pub correspondence_targets_inspected: usize,
    pub signal_graph_checks: usize,
    pub signal_node_ownership_checks: usize,
    pub dependency_registry_compilations: usize,
    pub dependency_registry_existing_key_lookups: usize,
    pub dependency_registry_batch_key_lookups: usize,
    pub dependency_registry_commits: usize,
    pub unrelated_dependency_registry_scans: usize,
    pub semantic_observation_plan_compilations: usize,
    pub signal_node_admissions: usize,
    pub correspondence_batch_preparations: usize,
    pub signal_contract_lowerings: usize,
    pub correspondence_admissions: usize,
    pub signal_targets_joined: usize,
    pub provider_checks: usize,
    pub signal_contract_installations: usize,
}

pub struct BridgeInstalledConditionalLowering {
    pub(crate) bridge_runtime_key: u64,
    pub(super) _authority:
        super::lowering_authority::BridgeInstalledConditionalLoweringAuthorityIdentity,
    pub(crate) projection: super::BridgeConditionalLoweringProjectionIdentity,
    pub(crate) contract: super::BridgeConditionalContract,
    pub(crate) location: super::BridgeConditionalLocation,
    pub(crate) correspondences: Vec<BridgeInstalledSemanticCorrespondence>,
    pub(super) semantic_observation_plan:
        Option<super::semantic_observation_plan::BridgeConditionalSemanticObservationPlan>,
    pub(crate) signal_contract: InstalledSignalConditionalContract,
    pub(crate) providers: BridgeConditionalProviderSet,
    pub(super) provider_admission: super::provider_admission::BridgeConditionalProviderAdmission,
    pub(super) lease: Arc<super::liveness::BridgeConditionalLoweringLease>,
    pub(crate) counters: BridgeInstalledConditionalLoweringCounters,
}

impl BridgeInstalledConditionalLowering {
    pub fn projection(&self) -> &super::BridgeConditionalLoweringProjectionIdentity {
        &self.projection
    }
    pub fn location(&self) -> &super::BridgeConditionalLocation {
        &self.location
    }
    pub fn contract(&self) -> &super::BridgeConditionalContract {
        &self.contract
    }
    pub fn signal_graph_instance_id(&self) -> u64 {
        self.signal_contract.graph_instance_id()
    }
    pub fn signal_node(&self) -> worth_signal::facade::NodeId {
        self.signal_contract.node()
    }
    pub fn counters(&self) -> BridgeInstalledConditionalLoweringCounters {
        self.counters
    }
    pub fn correspondence_count(&self) -> usize {
        self.correspondences.len()
    }

    /// Installed semantic dependencies retained by this lowering.
    ///
    /// Consumers may use these declarations to narrow their own candidate
    /// selection. The declarations remain non-authoritative without this
    /// current installed lowering.
    pub fn semantic_dependencies(
        &self,
    ) -> impl ExactSizeIterator<Item = &crate::correspondence::BridgeSemanticDependencyCandidate>
    {
        self.correspondences
            .iter()
            .map(|correspondence| correspondence.dependency())
    }

    pub fn dependency_locality(
        &self,
        ordinal: usize,
    ) -> Option<&crate::correspondence::BridgeSemanticLocality> {
        self.correspondences
            .iter()
            .find(|correspondence| correspondence.dependency().dependency_ordinal() == ordinal)
            .map(|correspondence| correspondence.dependency().locality())
    }

    pub fn validate_signal_decision_contract(
        &self,
        evidence: &worth_signal::facade::SignalConditionalDecisionEvidence,
    ) -> Result<(), BridgeConditionalDenial> {
        if !self.signal_contract.retains_decision(evidence) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SignalContractMismatch,
                "Signal decision evidence does not retain the installed conditional contract",
            ));
        }
        Ok(())
    }
}
