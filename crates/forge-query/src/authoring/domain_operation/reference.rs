use super::DomainGraphOperationDeclarationError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryAdmittedGraphReadDomainOperationReference {
    relation_name: String,
}

impl ForgeQueryAdmittedGraphReadDomainOperationReference {
    pub fn relation(
        relation_name: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        let relation_name = relation_name.into();
        if relation_name.trim().is_empty() {
            return Err(DomainGraphOperationDeclarationError::EmptyAdmittedReference);
        }
        Ok(Self { relation_name })
    }

    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }
}
