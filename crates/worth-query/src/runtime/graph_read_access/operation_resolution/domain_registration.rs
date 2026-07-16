use super::requirement::WorthQueryGraphReadOperationCapabilityRequirementDeclaration;
use super::WorthQueryGraphReadRegistryAdmissionError;
use crate::authoring::{
    RelationName, WorthQueryAdmittedGraphReadDomainOperationReference,
    WorthQueryGraphReadDomainOperationDeclaration, WorthQueryGraphReadOperationKey,
};
use crate::runtime::{
    WorthQueryGraphReadTraversalOperator, WorthQueryInstalledDomainSubstrateProvenance,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainRegisteredGraphReadOperation {
    operation_name: String,
    operation_version: u32,
    domain_owner: String,
    accepted_relations: Vec<RelationName>,
    traversal_operator: WorthQueryGraphReadTraversalOperator,
    capability_requirements: Vec<WorthQueryGraphReadOperationCapabilityRequirementDeclaration>,
    installed_provenance: Option<WorthQueryInstalledDomainSubstrateProvenance>,
}

impl WorthQueryDomainRegisteredGraphReadOperation {
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

    #[cfg(test)]
    pub(crate) fn installed_provenance(
        &self,
    ) -> Option<&WorthQueryInstalledDomainSubstrateProvenance> {
        self.installed_provenance.as_ref()
    }

    pub fn traversal_operator(&self) -> &WorthQueryGraphReadTraversalOperator {
        &self.traversal_operator
    }

    pub fn capability_requirements(
        &self,
    ) -> &[WorthQueryGraphReadOperationCapabilityRequirementDeclaration] {
        &self.capability_requirements
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "domain_operation:{}:{}:{}:{}:{}:{}",
            self.domain_owner,
            self.operation_name,
            self.operation_version,
            self.traversal_operator.as_str(),
            self.accepted_relations
                .iter()
                .map(RelationName::as_str)
                .collect::<Vec<_>>()
                .join(","),
            self.installed_provenance
                .as_ref()
                .map_or("uninstalled", |provenance| provenance.identity().as_str()),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryGraphReadOperationRegistration {
    operation_name: String,
    operation_version: u32,
    domain_owner: String,
    accepted_relations: Vec<RelationName>,
    traversal_operator: WorthQueryGraphReadTraversalOperator,
    capability_requirements: Vec<WorthQueryGraphReadOperationCapabilityRequirementDeclaration>,
    installed_provenance: Option<WorthQueryInstalledDomainSubstrateProvenance>,
}

impl WorthQueryGraphReadOperationRegistration {
    pub(crate) fn domain(
        operation_name: impl Into<String>,
        operation_version: u32,
        domain_owner: impl Into<String>,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_version,
            domain_owner: domain_owner.into(),
            accepted_relations: Vec::new(),
            traversal_operator: WorthQueryGraphReadTraversalOperator::DeclarationTraversal,
            capability_requirements: Vec::new(),
            installed_provenance: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_declared_operation(
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
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
            traversal_operator: WorthQueryGraphReadTraversalOperator::DeclarationTraversal,
            capability_requirements: declaration
                .support_families()
                .iter()
                .map(|support_family| {
                    WorthQueryGraphReadOperationCapabilityRequirementDeclaration::registration_required(
                        declaration.key().name().as_str(),
                        declaration.key().owner().as_str(),
                        support_family,
                    )
                })
                .collect(),
            installed_provenance: None,
        };
        registration.accepted_relations.sort();
        registration.accepted_relations.dedup();
        registration
            .capability_requirements
            .sort_by_key(|requirement| requirement.digest_part());
        registration.capability_requirements.dedup();
        registration
    }

    pub(crate) fn accepts_relation(
        mut self,
        relation: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphReadRegistryAdmissionError> {
        self.accepted_relations.push(
            RelationName::new(relation).map_err(|_| {
                WorthQueryGraphReadRegistryAdmissionError::MissingAdmittedReferences
            })?,
        );
        self.accepted_relations.sort();
        self.accepted_relations.dedup();
        Ok(self)
    }

    pub(crate) fn lowers_to_traversal_operator(
        mut self,
        operator: WorthQueryGraphReadTraversalOperator,
    ) -> Self {
        self.traversal_operator = operator;
        self
    }

    pub(crate) fn requires_capability(
        mut self,
        requirement: WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
    ) -> Self {
        self.capability_requirements.push(requirement);
        self.capability_requirements
            .sort_by_key(|requirement| requirement.digest_part());
        self.capability_requirements.dedup();
        self
    }

    pub(crate) fn authorized_by_installed_domain(
        mut self,
        provenance: WorthQueryInstalledDomainSubstrateProvenance,
    ) -> Self {
        debug_assert_eq!(self.domain_owner, provenance.domain_owner());
        self.installed_provenance = Some(provenance);
        self
    }

    pub(crate) fn accepted_relation_names(&self) -> &[RelationName] {
        &self.accepted_relations
    }

    pub(crate) fn installed_provenance(
        &self,
    ) -> Option<&WorthQueryInstalledDomainSubstrateProvenance> {
        self.installed_provenance.as_ref()
    }

    pub(crate) fn operation_key(
        &self,
    ) -> Result<WorthQueryGraphReadOperationKey, WorthQueryGraphReadRegistryAdmissionError> {
        WorthQueryGraphReadOperationKey::new(
            self.operation_name.clone(),
            self.operation_version,
            self.domain_owner.clone(),
        )
        .map_err(WorthQueryGraphReadRegistryAdmissionError::from)
    }

    pub(crate) fn matches_declared_operation(
        &self,
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> bool {
        self.operation_name == declaration.key().name().as_str()
            && self.operation_version == declaration.key().version().value()
            && self.domain_owner == declaration.key().owner().as_str()
            && self.accepted_relations == declared_relation_names(declaration.admitted_references())
            && self
                .capability_requirements
                .iter()
                .map(|requirement| requirement.support_family())
                .eq(declaration.support_families().iter().map(String::as_str))
    }

    pub(crate) fn admitted(&self) -> WorthQueryDomainRegisteredGraphReadOperation {
        WorthQueryDomainRegisteredGraphReadOperation {
            operation_name: self.operation_name.clone(),
            operation_version: self.operation_version,
            domain_owner: self.domain_owner.clone(),
            accepted_relations: self.accepted_relations.clone(),
            traversal_operator: self.traversal_operator.clone(),
            capability_requirements: self.capability_requirements.clone(),
            installed_provenance: self.installed_provenance.clone(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        self.admitted().digest_part()
    }
}

fn declared_relation_names(
    references: &[WorthQueryAdmittedGraphReadDomainOperationReference],
) -> Vec<RelationName> {
    let mut relation_names = references
        .iter()
        .map(|reference| reference.relation_name().clone())
        .collect::<Vec<_>>();
    relation_names.sort();
    relation_names.dedup();
    relation_names
}
