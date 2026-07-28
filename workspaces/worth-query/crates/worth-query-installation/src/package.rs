mod application_schema_validation;
mod artifact_closure;
mod definition;
mod identity;
mod member_validation;
mod validation_denial;

use worth_proof::{
    Artifact, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

pub use definition::{WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind};
pub use identity::{WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackageIdentity};
pub use validation_denial::{
    WorthQueryPortablePackageValidationDenial, WorthQueryPortablePackageValidationDenialKind,
};

use crate::domain_computation::WorthQueryPortableArtifactContract;
use crate::domain_operation::{
    WorthQueryPortableDomainOperationDefinition, WorthQueryValidatedDomainOperation,
};
use crate::package::{identity::canonical_identity, member_validation::validate_package_members};
use crate::package_requirements::{
    WorthQueryInstallationCapabilityFamily, WorthQueryInstallationConfigSectionFamily,
    WorthQueryInstallationContributionCategory, WorthQueryInstallationOperatingRequirement,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationSchemaDeclaration, ErasedApplicationSchemaDeclaration,
};

#[derive(Clone, Debug)]
pub struct WorthQueryPortableDomainPackage {
    identity: WorthQueryPortableDomainIdentity,
    capabilities: Vec<WorthQueryInstallationCapabilityFamily>,
    configuration: Vec<WorthQueryInstallationConfigSectionFamily>,
    operating: Vec<WorthQueryInstallationOperatingRequirement>,
    definitions: Vec<WorthQueryPortableDefinition>,
    domain_operations: Vec<WorthQueryPortableDomainOperationDefinition>,
    artifact_contracts: Vec<WorthQueryPortableArtifactContract>,
    application_schemas: Vec<ErasedApplicationSchemaDeclaration>,
    contributions: Vec<WorthQueryInstallationContributionCategory>,
}

impl WorthQueryPortableDomainPackage {
    pub fn new(identity: WorthQueryPortableDomainIdentity) -> Self {
        Self {
            identity,
            capabilities: Vec::new(),
            configuration: Vec::new(),
            operating: Vec::new(),
            definitions: Vec::new(),
            domain_operations: Vec::new(),
            artifact_contracts: Vec::new(),
            application_schemas: Vec::new(),
            contributions: Vec::new(),
        }
    }

    pub fn requires_capability(mut self, value: impl Into<String>) -> Self {
        self.capabilities
            .push(WorthQueryInstallationCapabilityFamily::new(value));
        self
    }

    pub fn requires_configuration(mut self, value: impl Into<String>) -> Self {
        self.configuration
            .push(WorthQueryInstallationConfigSectionFamily::new(value));
        self
    }

    pub fn requires_operating_posture(mut self, value: impl Into<String>) -> Self {
        self.operating
            .push(WorthQueryInstallationOperatingRequirement::new(value));
        self
    }

    pub fn definition(mut self, definition: WorthQueryPortableDefinition) -> Self {
        self.definitions.push(definition);
        self
    }

    pub fn domain_operation(
        mut self,
        definition: WorthQueryPortableDomainOperationDefinition,
    ) -> Self {
        self.domain_operations.push(definition);
        self
    }

    pub fn artifact_contract(mut self, contract: WorthQueryPortableArtifactContract) -> Self {
        self.artifact_contracts.push(contract);
        self
    }

    pub fn application_schema<Schema>(
        mut self,
        declaration: ApplicationSchemaDeclaration<Schema>,
    ) -> Self {
        self.application_schemas.push(declaration.into_erased());
        self
    }

    #[doc(hidden)]
    pub fn application_schema_erased(
        mut self,
        declaration: ErasedApplicationSchemaDeclaration,
    ) -> Self {
        self.application_schemas.push(declaration);
        self
    }

    pub fn permits_contribution(mut self, value: impl Into<String>) -> Self {
        self.contributions
            .push(WorthQueryInstallationContributionCategory::new(value));
        self
    }

    pub fn validate(
        mut self,
    ) -> Result<WorthQueryValidatedPortableDomainPackage, WorthQueryPortablePackageValidationDenial>
    {
        validate_package_members(&mut self)?;
        let validated_domain_operations = admit_domain_operations(&self.domain_operations)?;
        let identity = canonical_identity(&self);
        let authority =
            AuthorityWitness::from_authority_marker(PortablePackageValidationAuthority {
                _private: (),
            });
        let artifact = Artifact::with_proofs_and_current_basis(
            self,
            Proof::from_authority_witness(&authority),
            identity.clone(),
            authority,
        );
        Ok(WorthQueryValidatedPortableDomainPackage {
            artifact,
            identity,
            validated_domain_operations,
        })
    }
}

fn admit_domain_operations(
    operations: &[WorthQueryPortableDomainOperationDefinition],
) -> Result<Vec<WorthQueryValidatedDomainOperation>, WorthQueryPortablePackageValidationDenial> {
    let mut validated = Vec::with_capacity(operations.len());
    for operation in operations.iter().cloned() {
        let slot = operation.identity().slot();
        let operation = WorthQueryValidatedDomainOperation::admit(operation).map_err(|reason| {
            WorthQueryPortablePackageValidationDenial::invalid_domain_operation(format!(
                "{slot}:{reason}"
            ))
        })?;
        validated.push(operation);
    }
    Ok(validated)
}

#[derive(Clone, Debug)]
pub struct WorthQueryValidatedPortableDomainPackage {
    artifact: ValidatedPortablePackageArtifact,
    identity: WorthQueryPortableDomainPackageIdentity,
    validated_domain_operations: Vec<WorthQueryValidatedDomainOperation>,
}

#[derive(Clone, Debug)]
struct PortablePackageValidated;
impl PhaseMarker for PortablePackageValidated {}

#[derive(Clone, Debug)]
struct PortablePackageMeaningValidated;
impl ProofMarker for PortablePackageMeaningValidated {}

#[derive(Clone, Debug)]
struct PortablePackageValidationAuthority {
    _private: (),
}
impl AuthorityMarker for PortablePackageValidationAuthority {}
impl AuthorityProves<PortablePackageMeaningValidated> for PortablePackageValidationAuthority {}

type ValidatedPortablePackageArtifact = Artifact<
    PortablePackageValidated,
    WorthQueryPortableDomainPackage,
    Proof<PortablePackageMeaningValidated, PortablePackageValidationAuthority>,
    FreshnessScopedBasis<
        CurrentValidity,
        worth_proof::AssumptionBasis<WorthQueryPortableDomainPackageIdentity>,
    >,
>;

impl WorthQueryValidatedPortableDomainPackage {
    pub(crate) fn has_same_authoritative_meaning(&self, other: &Self) -> bool {
        self.domain_identity() == other.domain_identity()
            && self.capabilities() == other.capabilities()
            && self.configuration() == other.configuration()
            && self.operating_requirements() == other.operating_requirements()
            && self.definitions() == other.definitions()
            && self.domain_operations() == other.domain_operations()
            && self.artifact_contracts() == other.artifact_contracts()
            && self.application_schemas() == other.application_schemas()
            && self.contribution_policy() == other.contribution_policy()
    }

    pub fn domain_identity(&self) -> &WorthQueryPortableDomainIdentity {
        &self.artifact.payload().identity
    }

    pub fn identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.identity
    }

    pub fn capabilities(&self) -> &[WorthQueryInstallationCapabilityFamily] {
        &self.artifact.payload().capabilities
    }

    pub fn configuration(&self) -> &[WorthQueryInstallationConfigSectionFamily] {
        &self.artifact.payload().configuration
    }

    pub fn operating_requirements(&self) -> &[WorthQueryInstallationOperatingRequirement] {
        &self.artifact.payload().operating
    }

    pub fn definitions(&self) -> &[WorthQueryPortableDefinition] {
        &self.artifact.payload().definitions
    }

    pub fn domain_operations(&self) -> &[WorthQueryPortableDomainOperationDefinition] {
        &self.artifact.payload().domain_operations
    }

    pub fn artifact_contracts(&self) -> &[WorthQueryPortableArtifactContract] {
        &self.artifact.payload().artifact_contracts
    }

    pub fn application_schemas(&self) -> &[ErasedApplicationSchemaDeclaration] {
        &self.artifact.payload().application_schemas
    }

    pub(crate) fn validated_domain_operations(&self) -> &[WorthQueryValidatedDomainOperation] {
        &self.validated_domain_operations
    }

    pub fn contribution_policy(&self) -> &[WorthQueryInstallationContributionCategory] {
        &self.artifact.payload().contributions
    }
}
