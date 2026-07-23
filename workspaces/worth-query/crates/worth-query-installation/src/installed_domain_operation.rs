use crate::domain_operation::{
    WorthQueryConditionalNodeLocation, WorthQueryPortableConditionalNodeDeclaration,
    WorthQueryPortableDomainOperationDefinition, WorthQuerySemanticTruthDependency,
    WorthQueryValidatedDomainOperation,
};
use crate::generation::WorthQueryInstallationGeneration;
use crate::package::WorthQueryPortableDomainPackageIdentity;
use sha2::{Digest, Sha256};

/// Opaque proof that one structured domain operation belongs to an exact
/// installed package, runtime, and generation.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainOperationAuthority {
    pub(crate) runtime_ordinal: u64,
    pub(crate) generation: WorthQueryInstallationGeneration,
    pub(crate) owner: String,
    pub(crate) package_identity: WorthQueryPortableDomainPackageIdentity,
    pub(crate) admission_identity: String,
    pub(crate) package_authority_nonce: [u8; 32],
    pub(crate) validated: WorthQueryValidatedDomainOperation,
}

impl WorthQueryInstalledDomainOperationAuthority {
    pub fn runtime_ordinal(&self) -> u64 {
        self.runtime_ordinal
    }

    pub fn generation(&self) -> WorthQueryInstallationGeneration {
        self.generation
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn operation_slot(&self) -> String {
        self.definition().identity().slot()
    }

    pub fn definition(&self) -> &WorthQueryPortableDomainOperationDefinition {
        self.validated.definition()
    }

    pub fn package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.package_identity
    }

    pub fn conditional_dependency(
        &self,
        location: WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
    ) -> Result<
        WorthQueryInstalledConditionalDependencyAuthority,
        WorthQueryConditionalDependencyLookupDenial,
    > {
        let node = resolve_node(self.definition().semantics(), &location)
            .ok_or(WorthQueryConditionalDependencyLookupDenial::NodeNotDeclared)?;
        let dependency = node
            .dependencies()
            .get(dependency_ordinal)
            .cloned()
            .ok_or(WorthQueryConditionalDependencyLookupDenial::DependencyNotDeclared)?;
        Ok(WorthQueryInstalledConditionalDependencyAuthority {
            runtime_ordinal: self.runtime_ordinal,
            generation: self.generation,
            owner: self.owner.clone(),
            operation_slot: self.operation_slot(),
            operation_canonical_identity: self.definition().canonical_identity().to_string(),
            package_authority_nonce: self.package_authority_nonce,
            location,
            node: node.clone(),
            dependency_ordinal,
            dependency,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalDependencyLookupDenial {
    NodeNotDeclared,
    DependencyNotDeclared,
}

/// Opaque installation proof for one dependency declared by one exact
/// operation-level or workflow-stage conditional node.
#[derive(Debug)]
pub struct WorthQueryInstalledConditionalDependencyAuthority {
    runtime_ordinal: u64,
    generation: WorthQueryInstallationGeneration,
    owner: String,
    operation_slot: String,
    operation_canonical_identity: String,
    package_authority_nonce: [u8; 32],
    location: WorthQueryConditionalNodeLocation,
    node: WorthQueryPortableConditionalNodeDeclaration,
    dependency_ordinal: usize,
    dependency: WorthQuerySemanticTruthDependency,
}

impl WorthQueryInstalledConditionalDependencyAuthority {
    pub fn runtime_ordinal(&self) -> u64 {
        self.runtime_ordinal
    }

    pub fn generation(&self) -> WorthQueryInstallationGeneration {
        self.generation
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn operation_slot(&self) -> &str {
        &self.operation_slot
    }

    pub fn operation_canonical_identity(&self) -> &str {
        &self.operation_canonical_identity
    }

    pub fn location(&self) -> &WorthQueryConditionalNodeLocation {
        &self.location
    }

    pub fn node(&self) -> &WorthQueryPortableConditionalNodeDeclaration {
        &self.node
    }

    pub fn dependency_ordinal(&self) -> usize {
        self.dependency_ordinal
    }

    pub fn dependency(&self) -> &WorthQuerySemanticTruthDependency {
        &self.dependency
    }

    pub fn authority_binding_identity(&self) -> String {
        let mut hash = Sha256::new();
        hash.update(self.package_authority_nonce);
        for field in [
            self.owner.as_str(),
            self.operation_slot.as_str(),
            self.operation_canonical_identity.as_str(),
            self.location.stage_identity().unwrap_or("operation"),
            self.location.node_identity(),
        ] {
            hash.update(field.len().to_le_bytes());
            hash.update(field.as_bytes());
        }
        hash.update(self.runtime_ordinal.to_le_bytes());
        hash.update(self.generation.ordinal().to_le_bytes());
        hash.update(self.dependency_ordinal.to_le_bytes());
        format!("{:x}", hash.finalize())
    }
}

fn resolve_node<'a>(
    semantics: &'a crate::domain_operation::WorthQueryDomainOperationSemanticClosure,
    location: &WorthQueryConditionalNodeLocation,
) -> Option<&'a WorthQueryPortableConditionalNodeDeclaration> {
    match location {
        WorthQueryConditionalNodeLocation::Operation { node_identity } => semantics
            .conditional_nodes
            .iter()
            .find(|node| node.identity() == node_identity),
        WorthQueryConditionalNodeLocation::WorkflowStage {
            stage_identity,
            node_identity,
        } => match &semantics.workflow {
            crate::domain_operation::WorthQueryOperationWorkflowContract::NotRequired => None,
            crate::domain_operation::WorthQueryOperationWorkflowContract::Declared(workflow) => {
                workflow
                    .stages()
                    .iter()
                    .find(|stage| stage.identity() == stage_identity)
                    .and_then(|stage| {
                        stage
                            .semantics()
                            .conditional_nodes
                            .iter()
                            .find(|node| node.identity() == node_identity)
                    })
            }
        },
    }
}
