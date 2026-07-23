use super::*;
use crate::domain_computation::*;

pub struct WorthQueryPortableArtifactContractBuilder {
    family: WorthQueryArtifactFamilyIdentity,
    schema_version: WorthQueryArtifactSchemaVersion,
    protocol_version: WorthQueryArtifactProtocolVersion,
    content_identity: Option<WorthQueryArtifactContentIdentityContract>,
    ownership: Option<WorthQueryArtifactOwnershipContract>,
    occurrence: Option<WorthQueryArtifactOccurrenceContract>,
    evidence: Option<WorthQueryArtifactEvidenceContract>,
    reproducibility: Option<WorthQueryArtifactReproducibilityContract>,
    search: Option<WorthQueryCandidateSearchContract>,
    convergence: Option<WorthQueryConvergenceContract>,
    transformation: Option<WorthQueryTransformationEvidenceContract>,
    carriage: Option<WorthQueryArtifactCarriageContract>,
    lifecycle: Option<WorthQueryArtifactLifecycleContract>,
    counters: Option<WorthQueryStructuralCounterContract>,
    governance: Option<WorthQueryArtifactGovernanceContract>,
    compatibility: Option<WorthQueryArtifactCompatibilityContract>,
    producer_roles: Vec<String>,
    consumer_roles: Vec<String>,
}

impl WorthQueryPortableArtifactContractBuilder {
    pub(crate) fn new(
        family: WorthQueryArtifactFamilyIdentity,
        schema_version: WorthQueryArtifactSchemaVersion,
        protocol_version: WorthQueryArtifactProtocolVersion,
    ) -> Self {
        Self {
            family,
            schema_version,
            protocol_version,
            content_identity: None,
            ownership: None,
            occurrence: None,
            evidence: None,
            reproducibility: None,
            search: None,
            convergence: None,
            transformation: None,
            carriage: None,
            lifecycle: None,
            counters: None,
            governance: None,
            compatibility: None,
            producer_roles: Vec::new(),
            consumer_roles: Vec::new(),
        }
    }

    pub fn identity(mut self, value: WorthQueryArtifactContentIdentityContract) -> Self {
        self.content_identity = Some(value);
        self
    }

    pub fn ownership(mut self, value: WorthQueryArtifactOwnershipContract) -> Self {
        self.ownership = Some(value);
        self
    }

    pub fn occurrence(mut self, value: WorthQueryArtifactOccurrenceContract) -> Self {
        self.occurrence = Some(value);
        self
    }

    pub fn evidence(mut self, value: WorthQueryArtifactEvidenceContract) -> Self {
        self.evidence = Some(value);
        self
    }

    pub fn reproducibility(mut self, value: WorthQueryArtifactReproducibilityContract) -> Self {
        self.reproducibility = Some(value);
        self
    }

    pub fn search(mut self, value: WorthQueryCandidateSearchContract) -> Self {
        self.search = Some(value);
        self
    }

    pub fn convergence(mut self, value: WorthQueryConvergenceContract) -> Self {
        self.convergence = Some(value);
        self
    }

    pub fn transformation(mut self, value: WorthQueryTransformationEvidenceContract) -> Self {
        self.transformation = Some(value);
        self
    }

    pub fn carriage(mut self, value: WorthQueryArtifactCarriageContract) -> Self {
        self.carriage = Some(value);
        self
    }

    pub fn lifecycle(mut self, value: WorthQueryArtifactLifecycleContract) -> Self {
        self.lifecycle = Some(value);
        self
    }

    pub fn counters(mut self, value: WorthQueryStructuralCounterContract) -> Self {
        self.counters = Some(value);
        self
    }

    pub fn governance(mut self, value: WorthQueryArtifactGovernanceContract) -> Self {
        self.governance = Some(value);
        self
    }

    pub fn compatibility(mut self, value: WorthQueryArtifactCompatibilityContract) -> Self {
        self.compatibility = Some(value);
        self
    }

    pub fn produced_by(mut self, roles: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.producer_roles
            .extend(roles.into_iter().map(Into::into));
        self
    }

    pub fn consumed_by(mut self, roles: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.consumer_roles
            .extend(roles.into_iter().map(Into::into));
        self
    }

    pub fn finish(
        self,
    ) -> Result<WorthQueryPortableArtifactContract, WorthQueryArtifactContractValidationDenial>
    {
        let missing = |field| {
            WorthQueryArtifactContractValidationDenial::new(
                WorthQueryArtifactContractValidationDenialKind::MissingRequiredContract,
                field,
            )
        };
        let mut contract = WorthQueryPortableArtifactContract {
            family: self.family,
            schema_version: self.schema_version,
            protocol_version: self.protocol_version,
            content_identity: self.content_identity.ok_or_else(|| missing("identity"))?,
            ownership: self.ownership.ok_or_else(|| missing("ownership"))?,
            occurrence: self.occurrence.ok_or_else(|| missing("occurrence"))?,
            evidence: self.evidence.ok_or_else(|| missing("evidence"))?,
            reproducibility: self
                .reproducibility
                .ok_or_else(|| missing("reproducibility"))?,
            search: self.search.ok_or_else(|| missing("search"))?,
            convergence: self.convergence.ok_or_else(|| missing("convergence"))?,
            transformation: self
                .transformation
                .ok_or_else(|| missing("transformation"))?,
            carriage: self.carriage.ok_or_else(|| missing("carriage"))?,
            lifecycle: self.lifecycle.ok_or_else(|| missing("lifecycle"))?,
            counters: self.counters.ok_or_else(|| missing("counters"))?,
            governance: self.governance.ok_or_else(|| missing("governance"))?,
            compatibility: self.compatibility.ok_or_else(|| missing("compatibility"))?,
            producer_roles: self.producer_roles,
            consumer_roles: self.consumer_roles,
            identity: WorthQueryArtifactContractIdentity::minted(String::new()),
        };
        contract.producer_roles.sort();
        contract.producer_roles.dedup();
        contract.consumer_roles.sort();
        contract.consumer_roles.dedup();
        contract.occurrence.canonicalize();
        contract.reproducibility.canonicalize();
        contract.governance.canonicalize();
        validate_artifact_contract(&contract)?;
        contract.identity = canonical_artifact_contract_identity(&contract);
        Ok(contract)
    }
}
