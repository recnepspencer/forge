use std::path::PathBuf;

use worth_proof::AuthorityWitness;
use worth_store::physical_runtime::{
    AdmittedRecoveryFilesystemMedia, PhysicalRecoveryFreshnessPort,
    PhysicalRecoveryRegisteredSessionAuthority, QualifiedPhysicalBackendProfile,
    QualifiedRecoveryFilesystemMedia, RecoveryFilesystemQualificationError,
};

use super::authority_binding::{
    admitted_world_binding, entry_binding, entry_presentation, AdmittedRecoveryWorldBinding,
    PhysicalRecoveryEntryBinding, PhysicalRecoveryEntryPresentation,
};
use super::{
    counters, PhysicalRecoveryAdmissionCounters, PhysicalRecoveryEntryBindingDrift,
    PhysicalRecoveryLimits, PhysicalRecoverySessionIdentity, PhysicalRecoveryStaticConfiguration,
    RecoverySession,
};

worth_proof::authority_marker!(pub PhysicalRecoveryPlatformMarker);

pub struct PhysicalRecoveryPlatformAuthority {
    _witness: AuthorityWitness<PhysicalRecoveryPlatformMarker>,
    media: QualifiedRecoveryFilesystemMedia,
    registered_session: PhysicalRecoveryRegisteredSessionAuthority,
    session: RecoverySession,
    binding: PhysicalRecoveryEntryBinding,
    limits: PhysicalRecoveryLimits,
}

#[cfg(test)]
mod binding_contract_tests {
    use worth_proof::Binding;
    use worth_proof::TransitionOutcome;
    use worth_store::physical_runtime::{
        FilesystemAccessPosture, FilesystemMediaAdmission, PhysicalRuntimeAdmission, PhysicalStore,
    };

    use super::*;
    use crate::entry::PhysicalRecoveryLimitDeclaration;

    #[test]
    fn semantic_comparator_rejects_each_axis_without_mutating_an_authority() {
        let parent = tempfile::tempdir().expect("test parent");
        let primary_root = parent.path().join("primary");
        let alternate_root = parent.path().join("alternate");
        initialize_store(&primary_root);
        initialize_store(&alternate_root);
        let primary = PhysicalRecoveryPlatformAuthority::acquire(
            primary_root,
            PhysicalRecoveryStaticConfiguration::current(),
            limits(8),
        )
        .expect("primary authority");
        let alternate = PhysicalRecoveryPlatformAuthority::acquire(
            alternate_root,
            PhysicalRecoveryStaticConfiguration::current(),
            limits(9),
        )
        .expect("alternate authority");
        let base = primary.binding.axes().clone();
        let other = alternate.binding.axes();
        assert_ne!(base.backend_profile, other.backend_profile);

        let mut candidates = Vec::new();
        let mut axes = base.clone();
        axes.root_ownership = other.root_ownership.clone();
        candidates.push((axes, PhysicalRecoveryEntryBindingDrift::RootOwnership));
        let mut axes = base.clone();
        axes.recovery_session = other.recovery_session;
        candidates.push((axes, PhysicalRecoveryEntryBindingDrift::RecoverySession));
        let mut axes = base.clone();
        axes.backend_profile = other.backend_profile.clone();
        candidates.push((axes, PhysicalRecoveryEntryBindingDrift::BackendProfile));
        let mut axes = base.clone();
        axes.qualified_media_generation = other.qualified_media_generation;
        candidates.push((
            axes,
            PhysicalRecoveryEntryBindingDrift::QualifiedMediaGeneration,
        ));
        let mut axes = base.clone();
        axes.static_configuration[0] ^= 1;
        candidates.push((axes, PhysicalRecoveryEntryBindingDrift::StaticConfiguration));
        let mut axes = base;
        axes.recovery_limits = other.recovery_limits;
        candidates.push((axes, PhysicalRecoveryEntryBindingDrift::RecoveryLimits));

        for (axes, expected) in candidates {
            assert_eq!(
                primary.binding.ensure_matches(&Binding::new(axes)),
                Err(expected)
            );
        }
        primary.refuse();
        alternate.refuse();
    }

    fn initialize_store(root: &std::path::Path) {
        let runtime = PhysicalStore::admit(
            PhysicalRuntimeAdmission::new(root.to_path_buf()).expect("declared root"),
        )
        .expect("ordinary runtime admission");
        let admission = FilesystemMediaAdmission::production(
            FilesystemAccessPosture::CoordinatedServiceAccount,
        );
        let media = match runtime.try_admit_filesystem_media(admission).into_raw() {
            TransitionOutcome::Success(media) => media,
            _ => panic!("ordinary media initialization failed"),
        };
        let _ = media.close();
    }

    fn limits(scale: u64) -> PhysicalRecoveryLimits {
        PhysicalRecoveryLimits::admit(PhysicalRecoveryLimitDeclaration {
            selector_candidates: 2,
            checkpoint_candidates: scale,
            manifest_bytes: scale * 1024,
            manifest_entries: scale,
            wal_segments: scale,
            wal_frames: scale * 8,
            wal_bytes: scale * 4096,
            redo_targets: scale,
            redo_bytes: scale * 4096,
            distinct_pages_and_extents: scale,
            operation_bindings: scale,
            staging_bytes: scale * 4096,
            dirty_frames: scale,
            concurrent_commands: scale,
            publication_effects: 2,
            cleanup_candidates: scale,
            cleanup_bytes: scale * 4096,
            observation_bytes: scale * 1024,
        })
        .expect("bounded limits")
    }
}

impl PhysicalRecoveryPlatformAuthority {
    pub fn acquire(
        root: PathBuf,
        configuration: PhysicalRecoveryStaticConfiguration,
        limits: PhysicalRecoveryLimits,
    ) -> Result<Self, PhysicalRecoveryPlatformAdmissionError> {
        let media = QualifiedRecoveryFilesystemMedia::qualify_existing(&root)
            .map_err(PhysicalRecoveryPlatformAdmissionError::BackendQualification)?;
        Self::from_qualified_media(root, configuration, limits, media)
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn acquire_for_certification(
        root: PathBuf,
        configuration: PhysicalRecoveryStaticConfiguration,
        limits: PhysicalRecoveryLimits,
        schedule: worth_store::physical_runtime::certification::MediaFaultSchedule,
    ) -> Result<Self, PhysicalRecoveryPlatformAdmissionError> {
        let media =
            QualifiedRecoveryFilesystemMedia::qualify_existing_for_certification(&root, schedule)
                .map_err(PhysicalRecoveryPlatformAdmissionError::BackendQualification)?;
        Self::from_qualified_media(root, configuration, limits, media)
    }

    fn from_qualified_media(
        root: PathBuf,
        configuration: PhysicalRecoveryStaticConfiguration,
        limits: PhysicalRecoveryLimits,
        media: QualifiedRecoveryFilesystemMedia,
    ) -> Result<Self, PhysicalRecoveryPlatformAdmissionError> {
        let Some(freshness) = PhysicalRecoveryFreshnessPort::admit(&media) else {
            drop(media);
            return Err(PhysicalRecoveryPlatformAdmissionError::FreshnessUnavailable);
        };
        let Some(registered_session) = freshness.register_session() else {
            drop(media);
            return Err(PhysicalRecoveryPlatformAdmissionError::SessionIdentityUnavailable);
        };
        let session = match RecoverySession::issue(registered_session.session_identity_bytes()) {
            Ok(session) => session,
            Err(()) => {
                drop(media);
                return Err(PhysicalRecoveryPlatformAdmissionError::SessionIdentityUnavailable);
            }
        };
        let session_identity = session.identity();
        let binding = entry_binding(
            root.clone(),
            media.root_ownership_identity(),
            session_identity,
            media.backend_profile(),
            media.media_generation(),
            &configuration,
            limits,
        );
        Ok(Self {
            _witness: PhysicalRecoveryPlatformMarker::witness(),
            media,
            registered_session,
            session,
            binding,
            limits,
        })
    }

    pub fn qualified_backend_profile(&self) -> &QualifiedPhysicalBackendProfile {
        self.media.backend_profile()
    }

    pub const fn session_identity(&self) -> PhysicalRecoverySessionIdentity {
        self.session.identity()
    }

    pub fn process_counters() -> PhysicalRecoveryAdmissionCounters {
        counters::snapshot(None)
    }

    pub(crate) fn recovery_effect_count(&self) -> u64 {
        self.media.recovery_effect_count()
    }

    pub(crate) fn present_request(
        &self,
        root: PathBuf,
        profile: &QualifiedPhysicalBackendProfile,
        configuration: &PhysicalRecoveryStaticConfiguration,
        limits: PhysicalRecoveryLimits,
    ) -> PhysicalRecoveryEntryPresentation {
        entry_presentation(
            root,
            self.media.root_ownership_identity(),
            self.session.identity(),
            profile,
            self.media.media_generation(),
            configuration,
            limits,
        )
    }

    pub(crate) fn compare_request(
        &self,
        presentation: &PhysicalRecoveryEntryPresentation,
    ) -> Result<(), PhysicalRecoveryEntryBindingDrift> {
        presentation.compare_with(&self.binding)
    }

    pub(crate) fn into_admitted(self) -> Result<AdmittedPlatformAdmission, RefusedAuthority> {
        let Self {
            media,
            registered_session,
            session,
            limits,
            binding,
            ..
        } = self;
        let recovery_effects = media.recovery_effect_count();
        match media.admit_persisted_store() {
            Ok(media) => {
                let world_binding = admitted_world_binding(&binding, media.store_identity());
                Ok(AdmittedPlatformAdmission {
                    authority: AdmittedPlatformAuthority {
                        media,
                        session,
                        _world_binding: world_binding,
                        limits,
                    },
                    registered_session,
                })
            }
            Err(error) => {
                session.refuse();
                Err(RefusedAuthority {
                    error,
                    recovery_effects,
                })
            }
        }
    }

    pub(crate) fn refuse(self) {
        let Self { media, session, .. } = self;
        drop(media);
        session.refuse();
    }
}

pub(crate) struct AdmittedPlatformAuthority {
    pub(crate) media: AdmittedRecoveryFilesystemMedia,
    pub(crate) session: RecoverySession,
    pub(crate) _world_binding: AdmittedRecoveryWorldBinding,
    pub(crate) limits: PhysicalRecoveryLimits,
}

pub(crate) struct AdmittedPlatformAdmission {
    pub(crate) authority: AdmittedPlatformAuthority,
    pub(crate) registered_session: PhysicalRecoveryRegisteredSessionAuthority,
}

impl AdmittedPlatformAuthority {
    pub(crate) fn refuse(self) {
        let Self { media, session, .. } = self;
        drop(media);
        session.refuse();
    }
}

pub(crate) struct RefusedAuthority {
    pub(crate) error: RecoveryFilesystemQualificationError,
    pub(crate) recovery_effects: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoveryPlatformAdmissionError {
    BackendQualification(RecoveryFilesystemQualificationError),
    SessionIdentityUnavailable,
    FreshnessUnavailable,
}
