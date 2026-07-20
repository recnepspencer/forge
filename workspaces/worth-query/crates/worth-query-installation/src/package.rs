mod definition;
mod identity;
mod validation_denial;

use std::collections::BTreeMap;
use worth_proof::{
    Artifact, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

pub use definition::{WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind};
pub use identity::{WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackageIdentity};
pub use validation_denial::{
    WorthQueryPortablePackageValidationDenial, WorthQueryPortablePackageValidationDenialKind,
};

use crate::domain_operation::{
    WorthQueryPortableDomainOperationDefinition, WorthQueryValidatedDomainOperation,
};
use crate::package::identity::canonical_identity;
use crate::package_requirements::{
    WorthQueryInstallationCapabilityFamily, WorthQueryInstallationConfigSectionFamily,
    WorthQueryInstallationContributionCategory, WorthQueryInstallationOperatingRequirement,
};

#[derive(Clone, Debug)]
pub struct WorthQueryPortableDomainPackage {
    identity: WorthQueryPortableDomainIdentity,
    capabilities: Vec<WorthQueryInstallationCapabilityFamily>,
    configuration: Vec<WorthQueryInstallationConfigSectionFamily>,
    operating: Vec<WorthQueryInstallationOperatingRequirement>,
    definitions: Vec<WorthQueryPortableDefinition>,
    domain_operations: Vec<WorthQueryPortableDomainOperationDefinition>,
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

    pub fn permits_contribution(mut self, value: impl Into<String>) -> Self {
        self.contributions
            .push(WorthQueryInstallationContributionCategory::new(value));
        self
    }

    pub fn validate(
        mut self,
    ) -> Result<WorthQueryValidatedPortableDomainPackage, WorthQueryPortablePackageValidationDenial>
    {
        validate_required_meaning(&self)?;
        canonicalize(&mut self.capabilities);
        canonicalize(&mut self.configuration);
        canonicalize(&mut self.operating);
        self.contributions.sort();
        reject_duplicate_contributions(&self.contributions)?;
        self.definitions.sort();
        reject_definition_conflicts(&self.definitions)?;
        self.domain_operations
            .sort_by(|left, right| left.identity().cmp(right.identity()));
        reject_domain_operation_conflicts(&self.domain_operations)?;
        let mut validated_domain_operations = Vec::with_capacity(self.domain_operations.len());
        for operation in self.domain_operations.iter().cloned() {
            let slot = operation.identity().slot();
            let validated =
                WorthQueryValidatedDomainOperation::admit(operation).map_err(|reason| {
                    WorthQueryPortablePackageValidationDenial::invalid_domain_operation(format!(
                        "{slot}:{reason}"
                    ))
                })?;
            validated_domain_operations.push(validated);
        }

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

    pub(crate) fn validated_domain_operations(&self) -> &[WorthQueryValidatedDomainOperation] {
        &self.validated_domain_operations
    }

    pub fn contribution_policy(&self) -> &[WorthQueryInstallationContributionCategory] {
        &self.artifact.payload().contributions
    }
}

fn validate_required_meaning(
    package: &WorthQueryPortableDomainPackage,
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    if package.identity.owner().trim().is_empty() {
        return Err(WorthQueryPortablePackageValidationDenial::empty_domain_owner());
    }
    if let Some(definition) = package
        .definitions
        .iter()
        .find(|definition| definition.slot().trim().is_empty())
    {
        return Err(
            WorthQueryPortablePackageValidationDenial::empty_definition_slot(definition.kind()),
        );
    }
    if let Some(definition) = package
        .definitions
        .iter()
        .find(|definition| definition.semantics().trim().is_empty())
    {
        return Err(
            WorthQueryPortablePackageValidationDenial::empty_definition_semantics(
                definition.kind(),
                definition.slot(),
            ),
        );
    }
    if let Some(requirement) = empty_requirement(package) {
        return Err(WorthQueryPortablePackageValidationDenial::empty_requirement(requirement));
    }
    Ok(())
}

fn reject_duplicate_contributions(
    contributions: &[WorthQueryInstallationContributionCategory],
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    if contributions.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(WorthQueryPortablePackageValidationDenial::duplicate_contribution_category());
    }
    Ok(())
}

fn reject_definition_conflicts(
    definitions: &[WorthQueryPortableDefinition],
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    let mut slots = BTreeMap::new();
    for definition in definitions {
        let key = (definition.kind(), definition.slot().to_string());
        if let Some(existing) = slots.insert(key, definition.semantics().to_string()) {
            return Err(if existing == definition.semantics() {
                WorthQueryPortablePackageValidationDenial::duplicate_definition(
                    definition.kind(),
                    definition.slot(),
                )
            } else {
                WorthQueryPortablePackageValidationDenial::conflicting_definition(
                    definition.kind(),
                    definition.slot(),
                )
            });
        }
    }
    Ok(())
}

fn reject_domain_operation_conflicts(
    operations: &[WorthQueryPortableDomainOperationDefinition],
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    for pair in operations.windows(2) {
        if pair[0].identity() != pair[1].identity() {
            continue;
        }
        let slot = pair[1].identity().slot();
        return Err(if pair[0] == pair[1] {
            WorthQueryPortablePackageValidationDenial::duplicate_definition(
                WorthQueryPortableDefinitionKind::DomainOperation,
                slot,
            )
        } else {
            WorthQueryPortablePackageValidationDenial::conflicting_definition(
                WorthQueryPortableDefinitionKind::DomainOperation,
                slot,
            )
        });
    }
    Ok(())
}

fn empty_requirement(package: &WorthQueryPortableDomainPackage) -> Option<&'static str> {
    if package
        .capabilities
        .iter()
        .any(|value| value.as_str().trim().is_empty())
    {
        return Some("capability");
    }
    if package
        .configuration
        .iter()
        .any(|value| value.as_str().trim().is_empty())
    {
        return Some("configuration");
    }
    if package
        .operating
        .iter()
        .any(|value| value.as_str().trim().is_empty())
    {
        return Some("operating");
    }
    if package
        .contributions
        .iter()
        .any(|value| value.as_str().trim().is_empty())
    {
        return Some("contribution");
    }
    None
}

fn canonicalize<T: Ord>(values: &mut Vec<T>) {
    values.sort();
    values.dedup();
}
