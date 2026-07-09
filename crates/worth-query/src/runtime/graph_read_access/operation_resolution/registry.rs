use super::{
    WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
    WorthQueryGraphReadOperationRegistration,
    WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
    WorthQueryGraphReadRegistryAdmissionError,
};
use crate::authoring::{
    WorthQueryGraphReadDomainOperationDeclaration, WorthQueryGraphReadOperationKey,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryGraphReadOperationRegistry {
    registrations: Vec<WorthQueryGraphReadOperationRegistration>,
    required_capabilities: Vec<RequiredCapabilityRule>,
    unsupported_shapes: Vec<UnsupportedShapeRule>,
    unsupported_operations: Vec<UnsupportedOperationRule>,
}

impl WorthQueryGraphReadOperationRegistry {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn define(
        registrations: impl IntoIterator<Item = WorthQueryGraphReadOperationRegistration>,
    ) -> Self {
        let mut registry = Self::empty();
        for registration in registrations {
            registry = registry.with_registration(registration);
        }
        registry
    }

    pub fn admit(
        registrations: impl IntoIterator<Item = WorthQueryGraphReadOperationRegistration>,
    ) -> Result<Self, WorthQueryGraphReadRegistryAdmissionError> {
        let mut registry = Self::empty();
        for registration in registrations {
            registry = registry.admit_registration(registration)?;
        }
        Ok(registry)
    }

    pub fn with_registration(
        mut self,
        registration: WorthQueryGraphReadOperationRegistration,
    ) -> Self {
        self.registrations.push(registration);
        self.registrations
            .sort_by_key(|registration| registration.digest_part());
        self.registrations.dedup();
        self
    }

    pub fn admit_registration(
        mut self,
        registration: WorthQueryGraphReadOperationRegistration,
    ) -> Result<Self, WorthQueryGraphReadRegistryAdmissionError> {
        let key = registration.operation_key()?;
        if registration.accepted_relation_names().is_empty() {
            return Err(WorthQueryGraphReadRegistryAdmissionError::MissingAdmittedReferences);
        }
        if self
            .registrations
            .iter()
            .any(|existing| existing.operation_key().ok().as_ref() == Some(&key))
        {
            return Err(WorthQueryGraphReadRegistryAdmissionError::DuplicateOperationKey);
        }
        if self.registrations.iter().any(|existing| {
            existing.accepted_relation_names() == registration.accepted_relation_names()
        }) {
            return Err(
                WorthQueryGraphReadRegistryAdmissionError::AmbiguousDomainReferenceAdmission,
            );
        }
        self.registrations.push(registration);
        self.registrations
            .sort_by_key(|registration| registration.digest_part());
        Ok(self)
    }

    pub fn with_required_capability_for_relations(
        mut self,
        relations: impl IntoIterator<Item = impl Into<String>>,
        requirement: WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
    ) -> Self {
        self.required_capabilities
            .push(RequiredCapabilityRule::new(relations, requirement));
        self.required_capabilities.sort();
        self.required_capabilities.dedup();
        self
    }

    pub fn with_unsupported_shape_for_relations(
        mut self,
        relations: impl IntoIterator<Item = impl Into<String>>,
        denial: WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
    ) -> Self {
        self.unsupported_shapes
            .push(UnsupportedShapeRule::new(relations, denial));
        self.unsupported_shapes.sort();
        self.unsupported_shapes.dedup();
        self
    }

    pub fn with_unsupported_shape_for_operation(
        mut self,
        operation_key: WorthQueryGraphReadOperationKey,
        denial: WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
    ) -> Self {
        self.unsupported_operations
            .push(UnsupportedOperationRule::new(operation_key, denial));
        self.unsupported_operations.sort();
        self.unsupported_operations.dedup();
        self
    }

    pub fn registrations(&self) -> &[WorthQueryGraphReadOperationRegistration] {
        &self.registrations
    }

    pub(crate) fn matching_declared_operation(
        &self,
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationRegistration> {
        self.registrations
            .iter()
            .find(|registration| registration.matches_declared_operation(declaration))
    }

    pub(crate) fn matching_unsupported_declared_operation(
        &self,
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationUnsupportedShapeDeclaration> {
        self.unsupported_operations
            .iter()
            .find(|rule| rule.operation_key == *declaration.key())
            .map(|rule| &rule.denial)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct RequiredCapabilityRule {
    relations: Vec<String>,
    requirement: WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
}

impl RequiredCapabilityRule {
    fn new(
        relations: impl IntoIterator<Item = impl Into<String>>,
        requirement: WorthQueryGraphReadOperationCapabilityRequirementDeclaration,
    ) -> Self {
        Self {
            relations: normalized_relations(relations),
            requirement,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct UnsupportedShapeRule {
    relations: Vec<String>,
    denial: WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct UnsupportedOperationRule {
    operation_key: WorthQueryGraphReadOperationKey,
    denial: WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
}

impl UnsupportedOperationRule {
    fn new(
        operation_key: WorthQueryGraphReadOperationKey,
        denial: WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
    ) -> Self {
        Self {
            operation_key,
            denial,
        }
    }
}

impl UnsupportedShapeRule {
    fn new(
        relations: impl IntoIterator<Item = impl Into<String>>,
        denial: WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
    ) -> Self {
        Self {
            relations: normalized_relations(relations),
            denial,
        }
    }
}

fn normalized_relations(relations: impl IntoIterator<Item = impl Into<String>>) -> Vec<String> {
    let mut relations = relations
        .into_iter()
        .map(Into::into)
        .collect::<Vec<String>>();
    relations.sort();
    relations.dedup();
    relations
}
