use super::QualifiedFilesystemMedia;

impl QualifiedFilesystemMedia {
    /// Derives the move-only Store durability admission basis from this exact
    /// qualified media generation and its sealed C.4 capability witness.
    #[cfg(feature = "store-runtime-owner")]
    pub fn physical_durability_admission_basis(
        &self,
    ) -> Result<crate::PhysicalDurabilityAdmissionBasis, crate::BackendCapabilityAdmissionDenial>
    {
        let evidence = crate::CapabilityEvidenceClass::EstablishedByFilesystemAdmission;
        let file_sync = self
            .execution_capability()
            .require(crate::BackendCapabilityKind::Fsync, evidence)?;
        let directory_sync = self
            .execution_capability()
            .require(crate::BackendCapabilityKind::DirectorySync, evidence)?;
        let durable_rename = self
            .execution_capability()
            .require(crate::BackendCapabilityKind::DurableRename, evidence)?;
        let binding = self.basis().binding();
        Ok(
            crate::PhysicalDurabilityAdmissionBasis::from_qualified_media(
                crate::durability_profile::QualifiedDurabilityBasisInput {
                    store: self.store_identity(),
                    qualification_contract_version: binding.contract_version,
                    root_identity: binding.root_identity,
                    volume_identity: binding.volume_identity,
                    profile_digest: binding.profile_digest,
                    backend_build_identity: binding.backend_build_identity,
                    target: self.execution_capability().profile(),
                    file_sync,
                    directory_sync,
                    durable_rename,
                },
            ),
        )
    }

    #[cfg(feature = "store-runtime-owner")]
    pub fn physical_durability_admission_identity(
        &self,
    ) -> Result<crate::PhysicalDurabilityAdmissionIdentity, crate::BackendCapabilityAdmissionDenial>
    {
        Ok(self.physical_durability_admission_basis()?.identity())
    }
}
