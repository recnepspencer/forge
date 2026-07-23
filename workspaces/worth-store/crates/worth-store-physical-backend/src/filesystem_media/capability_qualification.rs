use crate::io_capability::BackendCapabilityClaimOutcome;

/// Checked capability progression consumed by the later root-specific owner.
///
/// This is an alias of the existing backend claim lane, not a second witness
/// family. Its success value is evidence for qualification and is deliberately
/// not an operational capability handle.
pub type MediaCapabilityQualificationOutcome = BackendCapabilityClaimOutcome;

pub(super) fn qualify_backend_claims(
    owner: &super::FilesystemMediaOwner,
    identity: &super::namespace_identity_admission::AdmittedStoreIdentity,
    profile: &super::FilesystemBackendProfile,
) -> Result<
    (
        crate::AdmittedBackendCapabilityWitness,
        crate::BackendCapabilityClaimWitness,
        crate::BackendCapabilityClaimWitness,
        crate::BackendCapabilityClaimWitness,
        crate::BackendCapabilityClaimWitness,
    ),
    crate::BackendCapabilityAdmissionDenial,
> {
    use crate::io_capability::{
        BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
        BackendCapabilitySupportPosture, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
        BackendRebindTriggers, BackendTargetProfile, CapabilityEvidenceClass,
        PhysicalBackendCapabilityAdmissionAuthority,
    };
    const FILESYSTEM_ADMISSION_CONFIDENCE_LIMITS: u8 = 3;
    #[cfg(windows)]
    let target = BackendTargetProfile::WindowsFlushFileBuffers;
    #[cfg(not(windows))]
    let target = BackendTargetProfile::PosixFileFsyncDirSync;
    if !identity.belongs_to(owner.identity()) || owner.begin_mutation().is_err() {
        return Err(
            crate::BackendCapabilityAdmissionDenial::FilesystemAdmissionEvidenceUnavailable,
        );
    }
    let support = |capability| match profile.support(capability) {
        super::CapabilitySupport::Supported => BackendCapabilitySupportPosture::Supported,
        super::CapabilitySupport::Unsupported => BackendCapabilitySupportPosture::Unsupported,
        super::CapabilitySupport::Indeterminate => BackendCapabilitySupportPosture::Unknown,
    };
    let support = BackendCapabilitySupportSet::for_filesystem_observation(
        support(super::MediaCapability::OrdinaryFile),
        support(super::MediaCapability::FileStateSynchronization),
        support(super::MediaCapability::DirectorySynchronization),
        support(super::MediaCapability::AtomicSameNamespaceReplacement),
    );
    let admitted = PhysicalBackendCapabilityAdmissionAuthority::for_filesystem_qualification()
        .admit_backend_capability(
            BackendCapabilityAdmissionRequest::for_filesystem_qualification(
                target,
                BackendCapabilityEvidenceBasis::established_by_filesystem_admission(
                    FILESYSTEM_ADMISSION_CONFIDENCE_LIMITS,
                ),
                support,
                BackendMediaAssumptionSet::for_established_filesystem_admission(),
                BackendRebindTriggers::for_filesystem_qualification(),
            ),
        )?;
    let evidence = CapabilityEvidenceClass::EstablishedByFilesystemAdmission;
    Ok((
        admitted,
        admitted.require(BackendCapabilityKind::BufferedFile, evidence)?,
        admitted.require(BackendCapabilityKind::Fsync, evidence)?,
        admitted.require(BackendCapabilityKind::DirectorySync, evidence)?,
        admitted.require(BackendCapabilityKind::DurableRename, evidence)?,
    ))
}
