use super::requirement::ForgeQueryGraphReadOperationCapabilityRequirementDeclaration;
use super::ForgeQueryGraphReadRegistryAdmissionError;
use crate::authoring::{
    ForgeQueryAdmittedGraphReadDomainOperationReference,
    ForgeQueryGraphReadDomainOperationDeclaration, ForgeQueryGraphReadOperationKey, RelationName,
};
use crate::runtime::ForgeQueryGraphReadTraversalOperator;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainRegisteredGraphReadOperation {
    operation_name: String,
    operation_version: u32,
    domain_owner: String,
    accepted_relations: Vec<RelationName>,
    traversal_operator: ForgeQueryGraphReadTraversalOperator,
    capability_requirements: Vec<ForgeQueryGraphReadOperationCapabilityRequirementDeclaration>,
}

impl ForgeQueryDomainRegisteredGraphReadOperation {
    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn operation_version(&self) -> u32 {
        self.operation_version
    }

    pub fn domain_owner(&self) -> &str {
        &self.domain_owner
    }

    pub fn accepted_relation_names(&self) -> &[RelationName] {
        &self.accepted_relations
    }

    pub fn traversal_operator(&self) -> &ForgeQueryGraphReadTraversalOperator {
        &self.traversal_operator
    }

    pub fn capability_requirements(
        &self,
    ) -> &[ForgeQueryGraphReadOperationCapabilityRequirementDeclaration] {
        &self.capability_requirements
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "domain_operation:{}:{}:{}:{}:{}",
            self.domain_owner,
            self.operation_name,
            self.operation_version,
            self.traversal_operator.as_str(),
            self.accepted_relations
                .iter()
                .map(RelationName::as_str)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadOperationRegistration {
    operation_name: String,
    operation_version: u32,
    domain_owner: String,
    accepted_relations: Vec<RelationName>,
    traversal_operator: ForgeQueryGraphReadTraversalOperator,
    capability_requirements: Vec<ForgeQueryGraphReadOperationCapabilityRequirementDeclaration>,
}

impl ForgeQueryGraphReadOperationRegistration {
    pub fn domain(
        operation_name: impl Into<String>,
        operation_version: u32,
        domain_owner: impl Into<String>,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_version,
            domain_owner: domain_owner.into(),
            accepted_relations: Vec::new(),
            traversal_operator: ForgeQueryGraphReadTraversalOperator::DeclarationTraversal,
            capability_requirements: Vec::new(),
        }
    }

    pub fn for_declared_operation(
        declaration: &ForgeQueryGraphReadDomainOperationDeclaration,
    ) -> Self {
        let mut registration = Self {
            operation_name: declaration.key().name().as_str().to_string(),
            operation_version: declaration.key().version().value(),
            domain_owner: declaration.key().owner().as_str().to_string(),
            accepted_relations: declaration
                .admitted_references()
                .iter()
                .map(|reference| reference.relation_name().clone())
                .collect(),
            traversal_operator: ForgeQueryGraphReadTraversalOperator::DeclarationTraversal,
            capability_requirements: declaration
                .support_families()
                .iter()
                .map(|support_family| {
                    ForgeQueryGraphReadOperationCapabilityRequirementDeclaration::registration_required(
                        declaration.key().name().as_str(),
                        declaration.key().owner().as_str(),
                        support_family,
                    )
                })
                .collect(),
        };
        registration.accepted_relations.sort();
        registration.accepted_relations.dedup();
        registration
            .capability_requirements
            .sort_by_key(|requirement| requirement.digest_part());
        registration.capability_requirements.dedup();
        registration
    }

    pub fn accepts_relation(
        mut self,
        relation: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphReadRegistryAdmissionError> {
        self.accepted_relations.push(
            RelationName::new(relation).map_err(|_| {
                ForgeQueryGraphReadRegistryAdmissionError::MissingAdmittedReferences
            })?,
        );
        self.accepted_relations.sort();
        self.accepted_relations.dedup();
        Ok(self)
    }

    pub fn lowers_to_traversal_operator(
        mut self,
        operator: ForgeQueryGraphReadTraversalOperator,
    ) -> Self {
        self.traversal_operator = operator;
        self
    }

    pub fn requires_capability(
        mut self,
        requirement: ForgeQueryGraphReadOperationCapabilityRequirementDeclaration,
    ) -> Self {
        self.capability_requirements.push(requirement);
        self.capability_requirements
            .sort_by_key(|requirement| requirement.digest_part());
        self.capability_requirements.dedup();
        self
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn operation_version(&self) -> u32 {
        self.operation_version
    }

    pub fn domain_owner(&self) -> &str {
        &self.domain_owner
    }

    pub fn accepted_relation_names(&self) -> &[RelationName] {
        &self.accepted_relations
    }

    pub fn operation_key(
        &self,
    ) -> Result<ForgeQueryGraphReadOperationKey, ForgeQueryGraphReadRegistryAdmissionError> {
        ForgeQueryGraphReadOperationKey::new(
            self.operation_name.clone(),
            self.operation_version,
            self.domain_owner.clone(),
        )
        .map_err(ForgeQueryGraphReadRegistryAdmissionError::from)
    }

    pub(crate) fn matches_declared_operation(
        &self,
        declaration: &ForgeQueryGraphReadDomainOperationDeclaration,
    ) -> bool {
        self.operation_name == declaration.key().name().as_str()
            && self.operation_version == declaration.key().version().value()
            && self.domain_owner == declaration.key().owner().as_str()
            && self.accepted_relations == declared_relation_names(declaration.admitted_references())
    }

    pub(crate) fn admitted(&self) -> ForgeQueryDomainRegisteredGraphReadOperation {
        ForgeQueryDomainRegisteredGraphReadOperation {
            operation_name: self.operation_name.clone(),
            operation_version: self.operation_version,
            domain_owner: self.domain_owner.clone(),
            accepted_relations: self.accepted_relations.clone(),
            traversal_operator: self.traversal_operator.clone(),
            capability_requirements: self.capability_requirements.clone(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        self.admitted().digest_part()
    }
}

fn declared_relation_names(
    references: &[ForgeQueryAdmittedGraphReadDomainOperationReference],
) -> Vec<RelationName> {
    let mut relation_names = references
        .iter()
        .map(|reference| reference.relation_name().clone())
        .collect::<Vec<_>>();
    relation_names.sort();
    relation_names.dedup();
    relation_names
}
