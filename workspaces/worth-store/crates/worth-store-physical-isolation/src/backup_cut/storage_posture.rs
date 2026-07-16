use worth_store_physical_backend::{
    BackendCapabilityClaimWitness, BackendCapabilityKind, BackendTargetProfile,
    CapabilityEvidenceClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupCutStoragePosture {
    backend_profile: BackendTargetProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupCutStoragePostureDenial {
    WrongCapability,
    ProfileMismatch,
    EvidenceTooWeak,
    UnsupportedProfile,
}

impl BackupCutStoragePosture {
    #[cfg(any(test, feature = "certification-authority"))]
    pub const fn for_certification_test() -> Self {
        Self {
            backend_profile: BackendTargetProfile::PosixFileFsyncDirSync,
        }
    }

    pub fn from_capability_claims(
        file_sync: BackendCapabilityClaimWitness,
        directory_sync: BackendCapabilityClaimWitness,
        durable_rename: BackendCapabilityClaimWitness,
    ) -> Result<Self, BackupCutStoragePostureDenial> {
        if file_sync.kind() != BackendCapabilityKind::Fsync
            || directory_sync.kind() != BackendCapabilityKind::DirectorySync
            || durable_rename.kind() != BackendCapabilityKind::DurableRename
        {
            return Err(BackupCutStoragePostureDenial::WrongCapability);
        }
        if file_sync.profile() != directory_sync.profile()
            || file_sync.profile() != durable_rename.profile()
        {
            return Err(BackupCutStoragePostureDenial::ProfileMismatch);
        }
        if [file_sync, directory_sync, durable_rename]
            .iter()
            .any(|claim| claim.evidence_class() != CapabilityEvidenceClass::CertifiedBackendProfile)
        {
            return Err(BackupCutStoragePostureDenial::EvidenceTooWeak);
        }
        if !matches!(
            file_sync.profile(),
            BackendTargetProfile::SimulatedStrictDurable
                | BackendTargetProfile::PosixFileFsyncDirSync
                | BackendTargetProfile::WindowsFlushFileBuffers
        ) {
            return Err(BackupCutStoragePostureDenial::UnsupportedProfile);
        }
        Ok(Self {
            backend_profile: file_sync.profile(),
        })
    }

    pub const fn format_profile(self) -> &'static str {
        "worth-physical-format-v1"
    }

    pub const fn backend_profile(self) -> &'static str {
        match self.backend_profile {
            BackendTargetProfile::SimulatedStrictDurable => "simulated-strict-durable",
            BackendTargetProfile::PosixFileFsyncDirSync => "posix-file-fsync-dir-sync",
            BackendTargetProfile::WindowsFlushFileBuffers => "windows-flush-file-buffers",
            BackendTargetProfile::MmapFlushNotDurabilityCertified => {
                "mmap-flush-not-durability-certified"
            }
            BackendTargetProfile::AdversarialLostFlush => "adversarial-lost-flush",
            BackendTargetProfile::AdversarialReorderedFlush => "adversarial-reordered-flush",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BackupCutStoragePosture, BackupCutStoragePostureDenial};
    use worth_store_physical_backend::{
        BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
        BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
        BackendTargetProfile, CapabilityEvidenceClass, PhysicalBackendCapabilityAdmissionAuthority,
    };

    #[test]
    fn storage_posture_requires_three_certified_supported_capabilities() {
        let certified = admitted(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
        );
        let posture = BackupCutStoragePosture::from_capability_claims(
            claim(
                &certified,
                BackendCapabilityKind::Fsync,
                CapabilityEvidenceClass::CertifiedBackendProfile,
            ),
            claim(
                &certified,
                BackendCapabilityKind::DirectorySync,
                CapabilityEvidenceClass::CertifiedBackendProfile,
            ),
            claim(
                &certified,
                BackendCapabilityKind::DurableRename,
                CapabilityEvidenceClass::CertifiedBackendProfile,
            ),
        )
        .expect("certified file durability posture");
        assert_eq!(posture.backend_profile(), "posix-file-fsync-dir-sync");

        let external = admitted(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::externally_guaranteed(1),
        );
        assert_eq!(
            BackupCutStoragePosture::from_capability_claims(
                claim(
                    &external,
                    BackendCapabilityKind::Fsync,
                    CapabilityEvidenceClass::ExternallyGuaranteed
                ),
                claim(
                    &external,
                    BackendCapabilityKind::DirectorySync,
                    CapabilityEvidenceClass::ExternallyGuaranteed
                ),
                claim(
                    &external,
                    BackendCapabilityKind::DurableRename,
                    CapabilityEvidenceClass::ExternallyGuaranteed
                ),
            ),
            Err(BackupCutStoragePostureDenial::EvidenceTooWeak)
        );
    }

    #[test]
    fn adversarial_backend_profile_cannot_back_a_backup_cut() {
        let adversarial = admitted(
            BackendTargetProfile::AdversarialLostFlush,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
        );
        assert_eq!(
            BackupCutStoragePosture::from_capability_claims(
                claim(
                    &adversarial,
                    BackendCapabilityKind::Fsync,
                    CapabilityEvidenceClass::CertifiedBackendProfile
                ),
                claim(
                    &adversarial,
                    BackendCapabilityKind::DirectorySync,
                    CapabilityEvidenceClass::CertifiedBackendProfile
                ),
                claim(
                    &adversarial,
                    BackendCapabilityKind::DurableRename,
                    CapabilityEvidenceClass::CertifiedBackendProfile
                ),
            ),
            Err(BackupCutStoragePostureDenial::UnsupportedProfile)
        );
    }

    fn admitted(
        profile: BackendTargetProfile,
        basis: BackendCapabilityEvidenceBasis,
    ) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
        PhysicalBackendCapabilityAdmissionAuthority::store_owned()
            .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
                profile,
                basis,
                BackendCapabilitySupportSet::all_supported(),
                BackendMediaAssumptionSet::platform_file_defaults(),
                BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
            ))
            .expect("backend capability admission")
    }

    fn claim(
        witness: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
        kind: BackendCapabilityKind,
        evidence: CapabilityEvidenceClass,
    ) -> worth_store_physical_backend::BackendCapabilityClaimWitness {
        witness.require(kind, evidence).expect("capability claim")
    }
}
