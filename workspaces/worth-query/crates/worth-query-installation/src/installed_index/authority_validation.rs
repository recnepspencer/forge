use super::{
    WorthQueryInstalledPackageAuthority, WorthQueryInstalledPackageIndex,
    WorthQueryInstalledPackageIndexDenial, WorthQueryInstalledPackageIndexDenialKind,
};
use crate::domain_computation::WorthQueryInstalledArtifactContractAuthority;
use crate::installed_domain_operation::WorthQueryInstalledDomainOperationAuthority;
use crate::installed_operation::WorthQueryInstalledOperationAuthority;

impl WorthQueryInstalledPackageIndex {
    pub fn validate(
        &self,
        authority: &WorthQueryInstalledPackageAuthority,
    ) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
        if authority.runtime_ordinal != self.runtime.ordinal() {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::ForeignRuntime,
                &authority.owner,
            ));
        }
        if authority.generation != self.generation {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::StaleGeneration,
                &authority.owner,
            ));
        }
        let record = self.packages.get(&authority.owner).ok_or_else(|| {
            WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::DomainNotInstalled,
                &authority.owner,
            )
        })?;
        if record.package.package().identity() != &authority.package_identity {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::PackageIdentityChanged,
                &authority.owner,
            ));
        }
        if record.package.admission_identity() != &authority.admission_identity {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::AdmissionIdentityChanged,
                &authority.owner,
            ));
        }
        if !record.authority_key.matches(&authority.authority_key) {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::AuthorityMismatch,
                &authority.owner,
            ));
        }
        Ok(())
    }

    pub fn validate_operation(
        &self,
        authority: &WorthQueryInstalledOperationAuthority,
    ) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
        self.validate(&WorthQueryInstalledPackageAuthority {
            runtime_ordinal: authority.runtime_ordinal,
            generation: authority.generation,
            owner: authority.owner.clone(),
            package_identity: authority.package_identity.clone(),
            admission_identity: authority.admission_identity.clone(),
            authority_key: authority.package_authority_key.clone(),
        })?;
        let current = self.operation(&authority.owner, &authority.operation_slot)?;
        if current.operation_semantics != authority.operation_semantics {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::OperationSemanticsChanged,
                &authority.operation_slot,
            ));
        }
        Ok(())
    }

    pub fn validate_domain_operation(
        &self,
        authority: &WorthQueryInstalledDomainOperationAuthority,
    ) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
        self.validate(&WorthQueryInstalledPackageAuthority {
            runtime_ordinal: authority.runtime_ordinal,
            generation: authority.generation,
            owner: authority.owner.clone(),
            package_identity: authority.package_identity.clone(),
            admission_identity: authority.admission_identity.clone(),
            authority_key: authority.package_authority_key.clone(),
        })?;
        let slot = authority.definition().identity().slot();
        let current = self
            .domain_operations
            .get(&(authority.owner.clone(), slot.clone()))
            .ok_or_else(|| {
                WorthQueryInstalledPackageIndexDenial::new(
                    WorthQueryInstalledPackageIndexDenialKind::OperationNotInstalled,
                    &slot,
                )
            })?;
        if current != &authority.validated {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::OperationSemanticsChanged,
                slot,
            ));
        }
        Ok(())
    }

    pub fn validate_artifact_contract(
        &self,
        authority: &WorthQueryInstalledArtifactContractAuthority,
    ) -> Result<(), WorthQueryInstalledPackageIndexDenial> {
        self.validate(&WorthQueryInstalledPackageAuthority {
            runtime_ordinal: authority.runtime_ordinal,
            generation: authority.generation,
            owner: authority.owner.clone(),
            package_identity: authority.package_identity.clone(),
            admission_identity: authority.admission_identity.clone(),
            authority_key: authority.package_authority_key.clone(),
        })?;
        let current = self.artifact_contract(
            &authority.owner,
            authority.contract.family().as_str(),
            authority.contract.schema_version(),
            authority.contract.protocol_version(),
        )?;
        if current.contract != authority.contract {
            return Err(WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::ArtifactContractSemanticsChanged,
                authority.contract.family().as_str(),
            ));
        }
        Ok(())
    }
}
