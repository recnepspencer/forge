use crate::authoring::RelationName;
use crate::runtime::{
    WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
    WorthQueryGraphReadOperationRegistration, WorthQueryGraphReadTraversalOperator,
};

use super::super::{
    WorthQueryDomainIdentityName, WorthQueryDomainIdentityNamespace,
    WorthQueryInstalledDomainAuthorityWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainGraphReadOperationDefinition {
    name: WorthQueryDomainIdentityName,
    version: u32,
    accepted_relations: Vec<RelationName>,
    traversal_operator: WorthQueryGraphReadTraversalOperator,
    support_families: Vec<WorthQueryDomainIdentityNamespace>,
}

impl WorthQueryDomainGraphReadOperationDefinition {
    pub fn new(name: WorthQueryDomainIdentityName, version: u32) -> Self {
        Self {
            name,
            version,
            accepted_relations: Vec::new(),
            traversal_operator: WorthQueryGraphReadTraversalOperator::DeclarationTraversal,
            support_families: Vec::new(),
        }
    }

    pub fn accepts_relation(mut self, relation: RelationName) -> Self {
        self.accepted_relations.push(relation);
        self.accepted_relations.sort();
        self.accepted_relations.dedup();
        self
    }

    pub fn lowers_to(mut self, operator: WorthQueryGraphReadTraversalOperator) -> Self {
        self.traversal_operator = operator;
        self
    }

    pub fn requires_support_family(
        mut self,
        support_family: WorthQueryDomainIdentityNamespace,
    ) -> Self {
        self.support_families.push(support_family);
        self.support_families.sort();
        self.support_families.dedup();
        self
    }

    pub fn name(&self) -> &WorthQueryDomainIdentityName {
        &self.name
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn accepted_relations(&self) -> &[RelationName] {
        &self.accepted_relations
    }

    pub(crate) fn slot_key(&self) -> String {
        format!("{}:{}", self.name.as_str(), self.version)
    }

    pub(crate) fn canonical_part(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.name.as_str(),
            self.version,
            self.traversal_operator.as_str(),
            self.accepted_relations
                .iter()
                .map(RelationName::as_str)
                .collect::<Vec<_>>()
                .join(","),
            self.support_families
                .iter()
                .map(WorthQueryDomainIdentityNamespace::as_str)
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    pub(crate) fn lower_with_owner(
        &self,
        owner: &str,
        provenance: crate::runtime::WorthQueryInstalledDomainSubstrateProvenance,
    ) -> WorthQueryGraphReadOperationRegistration {
        let mut registration = WorthQueryGraphReadOperationRegistration::domain(
            self.name.as_str(),
            self.version,
            owner,
        )
        .lowers_to_traversal_operator(self.traversal_operator.clone());
        for relation in &self.accepted_relations {
            registration = registration
                .accepts_relation(relation.as_str())
                .expect("typed relation names remain valid while lowering a domain package");
        }
        for support_family in &self.support_families {
            registration = registration.requires_capability(
                WorthQueryGraphReadOperationCapabilityRequirementDeclaration::registration_required(
                    self.name.as_str(),
                    owner,
                    support_family.as_str(),
                ),
            );
        }
        registration.authorized_by_installed_domain(provenance)
    }

    pub(crate) fn declare_for_installed_authority(
        &self,
        authority: &WorthQueryInstalledDomainAuthorityWitness,
    ) -> crate::authoring::WorthQueryGraphReadDomainOperationDeclaration {
        let owner = authority.authority().domain_owner();
        let mut declaration = crate::authoring::WorthQueryGraphReadDomainOperationDeclaration::new(
            self.name.as_str(),
            self.version,
            owner,
        )
        .expect("validated installed operation definitions have canonical identity parts");
        for relation in &self.accepted_relations {
            declaration = declaration
                .admit_relation_reference(relation.as_str())
                .expect("typed relation names remain valid in installed operation declarations");
        }
        for support_family in &self.support_families {
            declaration = declaration
                .requires_support_family(support_family.as_str())
                .expect("typed support families remain valid in installed operation declarations");
        }
        declaration
    }
}
