use super::{
    DomainGraphOperationDeclarationError, ForgeQueryAdmittedGraphReadDomainOperationReference,
    ForgeQueryGraphReadOperationKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadDomainOperationDeclaration {
    key: ForgeQueryGraphReadOperationKey,
    admitted_references: Vec<ForgeQueryAdmittedGraphReadDomainOperationReference>,
    support_families: Vec<String>,
}

impl ForgeQueryGraphReadDomainOperationDeclaration {
    pub fn new(
        name: impl Into<String>,
        version: u32,
        owner: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        Ok(Self {
            key: ForgeQueryGraphReadOperationKey::new(name, version, owner)?,
            admitted_references: Vec::new(),
            support_families: Vec::new(),
        })
    }

    pub fn admit_relation_reference(
        mut self,
        relation_name: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        self.admitted_references
            .push(ForgeQueryAdmittedGraphReadDomainOperationReference::relation(relation_name)?);
        self.admitted_references.sort();
        self.admitted_references.dedup();
        Ok(self)
    }

    pub fn requires_support_family(
        mut self,
        support_family: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        let support_family = support_family.into();
        if support_family.trim().is_empty() {
            return Err(DomainGraphOperationDeclarationError::EmptySupportFamily);
        }
        self.support_families.push(support_family);
        self.support_families.sort();
        self.support_families.dedup();
        Ok(self)
    }

    pub fn key(&self) -> &ForgeQueryGraphReadOperationKey {
        &self.key
    }

    pub fn admitted_references(&self) -> &[ForgeQueryAdmittedGraphReadDomainOperationReference] {
        &self.admitted_references
    }

    pub fn support_families(&self) -> &[String] {
        &self.support_families
    }

    pub fn digest_part(&self) -> String {
        let references = self
            .admitted_references
            .iter()
            .map(|reference| reference.terminal_relation_projection_for_boundary())
            .collect::<Vec<_>>()
            .join(",");
        let support = self.support_families.join(",");
        format!(
            "domain_declaration:{}:{}:{}",
            self.key.digest_part(),
            references,
            support
        )
    }
}
