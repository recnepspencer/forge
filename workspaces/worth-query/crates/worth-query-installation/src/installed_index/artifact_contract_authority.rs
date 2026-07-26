use crate::domain_computation::{
    WorthQueryArtifactProtocolVersion, WorthQueryArtifactSchemaVersion,
    WorthQueryInstalledArtifactContractAuthority,
};

use super::{
    WorthQueryInstalledPackageIndex, WorthQueryInstalledPackageIndexDenial,
    WorthQueryInstalledPackageIndexDenialKind,
};

impl WorthQueryInstalledPackageIndex {
    pub fn artifact_contract(
        &self,
        owner: &str,
        family: &str,
        schema_version: WorthQueryArtifactSchemaVersion,
        protocol_version: WorthQueryArtifactProtocolVersion,
    ) -> Result<WorthQueryInstalledArtifactContractAuthority, WorthQueryInstalledPackageIndexDenial>
    {
        let record = self.packages.get(owner).ok_or_else(|| {
            WorthQueryInstalledPackageIndexDenial::new(
                WorthQueryInstalledPackageIndexDenialKind::DomainNotInstalled,
                owner,
            )
        })?;
        let contract = self
            .artifact_contracts
            .get(&(
                owner.to_string(),
                family.to_string(),
                schema_version.get(),
                protocol_version.get(),
            ))
            .cloned()
            .ok_or_else(|| {
                WorthQueryInstalledPackageIndexDenial::new(
                    WorthQueryInstalledPackageIndexDenialKind::ArtifactContractNotInstalled,
                    family,
                )
            })?;
        Ok(WorthQueryInstalledArtifactContractAuthority {
            runtime_ordinal: self.runtime.ordinal(),
            generation: self.generation,
            owner: owner.to_string(),
            package_identity: record.package.package().identity().clone(),
            admission_identity: record.package.admission_identity().to_string(),
            package_authority_nonce: record.authority_nonce,
            contract,
        })
    }
}
