use super::DomainGraphOperationDeclarationError;
use crate::authoring::RelationName;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryAdmittedGraphReadDomainOperationReference {
    relation_name: RelationName,
}

impl WorthQueryAdmittedGraphReadDomainOperationReference {
    pub fn relation(
        relation_name: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        let relation_name = relation_name.into();
        let relation_name = RelationName::new(relation_name)
            .map_err(|_| DomainGraphOperationDeclarationError::EmptyAdmittedReference)?;
        Ok(Self { relation_name })
    }

    pub fn relation_name(&self) -> &RelationName {
        &self.relation_name
    }

    pub fn terminal_relation_projection_for_boundary(&self) -> &str {
        self.relation_name.as_str()
    }
}
