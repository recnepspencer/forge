use super::{
    WorthQueryArtifactCarriageContract, WorthQueryArtifactCompatibilityContract,
    WorthQueryArtifactContentIdentityContract, WorthQueryArtifactContractIdentity,
    WorthQueryArtifactContractReference, WorthQueryArtifactEvidenceContract,
    WorthQueryArtifactFamily, WorthQueryArtifactFamilyIdentity,
    WorthQueryArtifactGovernanceContract, WorthQueryArtifactLifecycleContract,
    WorthQueryArtifactOwnershipContract, WorthQueryArtifactProtocolVersion,
    WorthQueryArtifactSchemaVersion, WorthQueryPortableArtifactContractBuilder,
};
use crate::domain_computation::{
    WorthQueryArtifactAccessPathContract, WorthQueryArtifactOccurrenceContract,
    WorthQueryArtifactReproducibilityContract, WorthQueryCandidateSearchContract,
    WorthQueryConvergenceContract, WorthQueryDecisionRecordContract,
    WorthQueryStructuralCounterContract, WorthQueryTransformationEvidenceContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortableArtifactContract {
    pub(crate) family: WorthQueryArtifactFamilyIdentity,
    pub(crate) schema_version: WorthQueryArtifactSchemaVersion,
    pub(crate) protocol_version: WorthQueryArtifactProtocolVersion,
    pub(crate) content_identity: WorthQueryArtifactContentIdentityContract,
    pub(crate) ownership: WorthQueryArtifactOwnershipContract,
    pub(crate) occurrence: WorthQueryArtifactOccurrenceContract,
    pub(crate) evidence: WorthQueryArtifactEvidenceContract,
    pub(crate) reproducibility: WorthQueryArtifactReproducibilityContract,
    pub(crate) search: WorthQueryCandidateSearchContract,
    pub(crate) convergence: WorthQueryConvergenceContract,
    pub(crate) transformation: WorthQueryTransformationEvidenceContract,
    pub(crate) access_path: WorthQueryArtifactAccessPathContract,
    pub(crate) carriage: WorthQueryArtifactCarriageContract,
    pub(crate) lifecycle: WorthQueryArtifactLifecycleContract,
    pub(crate) counters: WorthQueryStructuralCounterContract,
    pub(crate) decisions: WorthQueryDecisionRecordContract,
    pub(crate) governance: WorthQueryArtifactGovernanceContract,
    pub(crate) compatibility: WorthQueryArtifactCompatibilityContract,
    pub(crate) producer_roles: Vec<String>,
    pub(crate) consumer_roles: Vec<String>,
    pub(crate) identity: WorthQueryArtifactContractIdentity,
}

impl WorthQueryPortableArtifactContract {
    pub fn declare<F: WorthQueryArtifactFamily>(
        schema_version: WorthQueryArtifactSchemaVersion,
        protocol_version: WorthQueryArtifactProtocolVersion,
    ) -> WorthQueryPortableArtifactContractBuilder {
        WorthQueryPortableArtifactContractBuilder::new(
            WorthQueryArtifactFamilyIdentity::declared::<F>(),
            schema_version,
            protocol_version,
        )
    }

    pub fn family(&self) -> &WorthQueryArtifactFamilyIdentity {
        &self.family
    }

    pub const fn schema_version(&self) -> WorthQueryArtifactSchemaVersion {
        self.schema_version
    }

    pub const fn protocol_version(&self) -> WorthQueryArtifactProtocolVersion {
        self.protocol_version
    }

    pub fn identity(&self) -> &WorthQueryArtifactContractIdentity {
        &self.identity
    }

    pub fn content_identity(&self) -> &WorthQueryArtifactContentIdentityContract {
        &self.content_identity
    }

    pub fn reference(&self) -> WorthQueryArtifactContractReference {
        WorthQueryArtifactContractReference::new(
            self.family.clone(),
            self.schema_version,
            self.protocol_version,
        )
    }

    pub fn ownership(&self) -> &WorthQueryArtifactOwnershipContract {
        &self.ownership
    }

    pub fn occurrence(&self) -> &WorthQueryArtifactOccurrenceContract {
        &self.occurrence
    }

    pub fn evidence(&self) -> &WorthQueryArtifactEvidenceContract {
        &self.evidence
    }

    pub fn reproducibility(&self) -> &WorthQueryArtifactReproducibilityContract {
        &self.reproducibility
    }

    pub fn search(&self) -> &WorthQueryCandidateSearchContract {
        &self.search
    }

    pub fn convergence(&self) -> &WorthQueryConvergenceContract {
        &self.convergence
    }

    pub fn transformation(&self) -> &WorthQueryTransformationEvidenceContract {
        &self.transformation
    }

    pub fn access_path(&self) -> &WorthQueryArtifactAccessPathContract {
        &self.access_path
    }

    pub const fn carriage(&self) -> WorthQueryArtifactCarriageContract {
        self.carriage
    }

    pub const fn lifecycle(&self) -> WorthQueryArtifactLifecycleContract {
        self.lifecycle
    }

    pub fn counters(&self) -> &WorthQueryStructuralCounterContract {
        &self.counters
    }

    pub fn decisions(&self) -> &WorthQueryDecisionRecordContract {
        &self.decisions
    }

    pub fn governance(&self) -> &WorthQueryArtifactGovernanceContract {
        &self.governance
    }

    pub fn compatibility(&self) -> &WorthQueryArtifactCompatibilityContract {
        &self.compatibility
    }

    pub fn producer_roles(&self) -> &[String] {
        &self.producer_roles
    }

    pub fn consumer_roles(&self) -> &[String] {
        &self.consumer_roles
    }
}
