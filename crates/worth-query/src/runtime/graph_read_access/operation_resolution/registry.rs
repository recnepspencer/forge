use super::{
    WorthQueryGraphReadOperationRegistration,
    WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
    WorthQueryGraphReadRegistryAdmissionError,
};
use crate::authoring::{
    WorthQueryGraphReadDomainOperationDeclaration, WorthQueryGraphReadOperationKey,
};

pub(crate) trait WorthQueryGraphReadOperationLookup {
    fn matching_declared_operation(
        &self,
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationRegistration>;

    fn matching_unsupported_declared_operation(
        &self,
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationUnsupportedShapeDeclaration>;
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthQueryGraphReadOperationRegistry {
    registrations: Vec<WorthQueryGraphReadOperationRegistration>,
    unsupported_operations: Vec<UnsupportedOperationRule>,
}

impl WorthQueryGraphReadOperationLookup for WorthQueryGraphReadOperationRegistry {
    fn matching_declared_operation(
        &self,
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationRegistration> {
        self.matching_declared_operation(declaration)
    }

    fn matching_unsupported_declared_operation(
        &self,
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationUnsupportedShapeDeclaration> {
        self.matching_unsupported_declared_operation(declaration)
    }
}

impl WorthQueryGraphReadOperationRegistry {
    pub(crate) fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn admit(
        registrations: impl IntoIterator<Item = WorthQueryGraphReadOperationRegistration>,
    ) -> Result<Self, WorthQueryGraphReadRegistryAdmissionError> {
        let mut registry = Self::empty();
        for registration in registrations {
            registry = registry.admit_registration(registration)?;
        }
        Ok(registry)
    }

    pub(crate) fn admit_registration(
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

    #[cfg(test)]
    pub(crate) fn with_unsupported_shape_for_operation(
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
struct UnsupportedOperationRule {
    operation_key: WorthQueryGraphReadOperationKey,
    denial: WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
}

impl UnsupportedOperationRule {
    #[cfg(test)]
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
