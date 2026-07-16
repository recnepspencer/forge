use super::{
    DomainGraphOperationDeclarationError, WorthQueryAdmittedGraphReadDomainOperationReference,
    WorthQueryGraphReadOperationKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadDomainOperationDeclaration {
    key: WorthQueryGraphReadOperationKey,
    admitted_references: Vec<WorthQueryAdmittedGraphReadDomainOperationReference>,
    support_families: Vec<String>,
    installed_authority:
        Option<crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness>,
}

impl WorthQueryGraphReadDomainOperationDeclaration {
    pub(crate) fn new(
        name: impl Into<String>,
        version: u32,
        owner: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        Ok(Self {
            key: WorthQueryGraphReadOperationKey::new(name, version, owner)?,
            admitted_references: Vec::new(),
            support_families: Vec::new(),
            installed_authority: None,
        })
    }

    pub(crate) fn admit_relation_reference(
        mut self,
        relation_name: impl Into<String>,
    ) -> Result<Self, DomainGraphOperationDeclarationError> {
        self.admitted_references
            .push(WorthQueryAdmittedGraphReadDomainOperationReference::relation(relation_name)?);
        self.admitted_references.sort();
        self.admitted_references.dedup();
        Ok(self)
    }

    pub(crate) fn requires_support_family(
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

    pub fn key(&self) -> &WorthQueryGraphReadOperationKey {
        &self.key
    }

    pub fn admitted_references(&self) -> &[WorthQueryAdmittedGraphReadDomainOperationReference] {
        &self.admitted_references
    }

    pub fn support_families(&self) -> &[String] {
        &self.support_families
    }

    pub(crate) fn authorized_by_installed_domain(
        mut self,
        authority: crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness,
    ) -> Self {
        debug_assert_eq!(
            self.key.owner().as_str(),
            authority.authority().domain_owner()
        );
        self.installed_authority = Some(authority);
        self
    }

    pub(crate) fn installed_authority(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness> {
        self.installed_authority.as_ref()
    }

    pub fn digest_part(&self) -> String {
        let references = self
            .admitted_references
            .iter()
            .map(|reference| reference.terminal_relation_projection_for_boundary())
            .collect::<Vec<_>>()
            .join(",");
        let support = self.support_families.join(",");
        let mut digest = format!(
            "domain_declaration:{}:{}:{}",
            self.key.digest_part(),
            references,
            support
        );
        if let Some(authority) = &self.installed_authority {
            digest.push_str(":installed_authority=");
            digest.push_str(authority.witness_identity().as_str());
        }
        digest
    }
}
