use super::{qualification_basis::RootProfileBinding, AdmittedFilesystemMedia};
use super::{MediaQualificationRebindRequired, MediaQualificationStale};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaQualificationBasisDrift {
    RootIdentity,
    VolumeIdentity,
    BackendProfile,
    BackendBuild,
    QualificationContract,
}

pub(super) fn basis_drift(
    expected: &RootProfileBinding,
    observed: &RootProfileBinding,
) -> Option<MediaQualificationBasisDrift> {
    if expected.contract_version != observed.contract_version {
        Some(MediaQualificationBasisDrift::QualificationContract)
    } else if expected.root_identity != observed.root_identity {
        Some(MediaQualificationBasisDrift::RootIdentity)
    } else if expected.volume_identity != observed.volume_identity {
        Some(MediaQualificationBasisDrift::VolumeIdentity)
    } else if expected.backend_build_identity != observed.backend_build_identity {
        Some(MediaQualificationBasisDrift::BackendBuild)
    } else if expected.profile_digest != observed.profile_digest {
        Some(MediaQualificationBasisDrift::BackendProfile)
    } else {
        None
    }
}

pub(super) fn pre_ownership_drift(
    drift: MediaQualificationBasisDrift,
    observed_root: [u8; 32],
    counters: super::MediaCounterSnapshot,
) -> AdmittedFilesystemMedia {
    match drift {
        MediaQualificationBasisDrift::RootIdentity => {
            worth_proof::TransitionOutcome::stale(MediaQualificationStale::RootIdentityChanged {
                observed: observed_root,
                counters,
            })
            .into()
        }
        MediaQualificationBasisDrift::VolumeIdentity => {
            worth_proof::TransitionOutcome::rebind_required(
                MediaQualificationRebindRequired::VolumeChanged { counters },
            )
            .into()
        }
        MediaQualificationBasisDrift::BackendProfile
        | MediaQualificationBasisDrift::BackendBuild => {
            worth_proof::TransitionOutcome::rebind_required(
                MediaQualificationRebindRequired::BackendProfileChanged { counters },
            )
            .into()
        }
        MediaQualificationBasisDrift::QualificationContract => {
            worth_proof::TransitionOutcome::rebind_required(
                MediaQualificationRebindRequired::QualificationContractChanged { counters },
            )
            .into()
        }
    }
}
