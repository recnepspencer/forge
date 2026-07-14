use worth_relational::facade::identity::KindId;

use crate::authoring::RelationName;
use crate::runtime::{
    WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
    WorthQueryGraphReadOperationRegistration, WorthQueryGraphReadTraversalOperator,
};

use super::{WorthQueryDomainIdentityComponentError, WorthQueryDomainIdentityName};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainInvariantPredicate {
    RequiresOutgoingRelations {
        relevant_entity_kinds: Vec<KindId>,
        required_relation_kinds: Vec<KindId>,
        traversal_depth: u16,
    },
}

impl WorthQueryDomainInvariantPredicate {
    pub fn requires_outgoing_relations(
        mut relevant_entity_kinds: Vec<KindId>,
        mut required_relation_kinds: Vec<KindId>,
        traversal_depth: u16,
    ) -> Self {
        relevant_entity_kinds.sort();
        relevant_entity_kinds.dedup();
        required_relation_kinds.sort();
        required_relation_kinds.dedup();
        Self::RequiresOutgoingRelations {
            relevant_entity_kinds,
            required_relation_kinds,
            traversal_depth,
        }
    }

    pub(crate) fn canonical_part(&self) -> String {
        match self {
            Self::RequiresOutgoingRelations {
                relevant_entity_kinds,
                required_relation_kinds,
                traversal_depth,
            } => format!(
                "requires-outgoing:{}:{}:{}",
                relevant_entity_kinds
                    .iter()
                    .map(|kind| kind.as_u32().to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                required_relation_kinds
                    .iter()
                    .map(|kind| kind.as_u32().to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                traversal_depth
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainInvariantDefinition {
    name: WorthQueryDomainIdentityName,
    semantic_version: super::WorthQueryDomainSemanticVersion,
    predicate: WorthQueryDomainInvariantPredicate,
}

impl WorthQueryDomainInvariantDefinition {
    pub fn new(
        name: WorthQueryDomainIdentityName,
        semantic_version: super::WorthQueryDomainSemanticVersion,
        predicate: WorthQueryDomainInvariantPredicate,
    ) -> Self {
        Self {
            name,
            semantic_version,
            predicate,
        }
    }

    pub fn name(&self) -> &WorthQueryDomainIdentityName {
        &self.name
    }
    pub fn semantic_version(&self) -> super::WorthQueryDomainSemanticVersion {
        self.semantic_version
    }
    pub fn predicate(&self) -> &WorthQueryDomainInvariantPredicate {
        &self.predicate
    }

    pub(crate) fn slot_key(&self) -> String {
        format!("{}:{}", self.name.as_str(), self.semantic_version.major())
    }

    pub(crate) fn canonical_part(&self) -> String {
        format!(
            "{}:{}.{}:{}",
            self.name.as_str(),
            self.semantic_version.major(),
            self.semantic_version.minor(),
            self.predicate.canonical_part()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainGraphReadOperationDefinition {
    name: WorthQueryDomainIdentityName,
    version: u32,
    accepted_relations: Vec<RelationName>,
    traversal_operator: WorthQueryGraphReadTraversalOperator,
    capability_requirements: Vec<WorthQueryGraphReadOperationCapabilityRequirementDeclaration>,
}

impl WorthQueryDomainGraphReadOperationDefinition {
    pub fn new(name: WorthQueryDomainIdentityName, version: u32) -> Self {
        Self {
            name,
            version,
            accepted_relations: Vec::new(),
            traversal_operator: WorthQueryGraphReadTraversalOperator::DeclarationTraversal,
            capability_requirements: Vec::new(),
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

    pub fn requires_capability(
        mut self,
        requirement: WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
    ) -> Self {
        self.capability_requirements.push(requirement);
        self.capability_requirements
            .sort_by_key(|item| item.digest_part());
        self.capability_requirements.dedup();
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
            self.capability_requirements
                .iter()
                .map(|item| item.digest_part())
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    pub(crate) fn lower_with_owner(&self, owner: &str) -> WorthQueryGraphReadOperationRegistration {
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
        for requirement in &self.capability_requirements {
            registration = registration.requires_capability(requirement.clone());
        }
        registration
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainDeclarationFamilyDefinition {
    name: WorthQueryDomainIdentityName,
    version: u32,
}

impl WorthQueryDomainDeclarationFamilyDefinition {
    pub fn new(
        name: impl Into<String>,
        version: u32,
    ) -> Result<Self, WorthQueryDomainIdentityComponentError> {
        Ok(Self {
            name: WorthQueryDomainIdentityName::new(name)?,
            version,
        })
    }

    pub fn name(&self) -> &WorthQueryDomainIdentityName {
        &self.name
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub(crate) fn slot_key(&self) -> &str {
        self.name.as_str()
    }
    pub(crate) fn canonical_part(&self) -> String {
        format!("{}:{}", self.name.as_str(), self.version)
    }
}
