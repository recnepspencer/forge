use std::path::PathBuf;

use worth_proof::Binding;
use worth_store::physical_runtime::{
    MediaOwnerIdentity, PhysicalRecoveryMediaGeneration, QualifiedPhysicalBackendProfile,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{
    PhysicalRecoveryLimits, PhysicalRecoverySessionIdentity, PhysicalRecoveryStaticConfiguration,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PhysicalRecoveryRootOwnershipAxis {
    declared_root: PathBuf,
    owner: MediaOwnerIdentity,
}

worth_proof::binding_axes! {
    pub(crate) struct PhysicalRecoveryEntryAxes {
        pub(crate) root_ownership: PhysicalRecoveryRootOwnershipAxis => RootOwnership,
        pub(crate) recovery_session: PhysicalRecoverySessionIdentity => RecoverySession,
        pub(crate) backend_profile: QualifiedPhysicalBackendProfile => BackendProfile,
        pub(crate) qualified_media_generation: PhysicalRecoveryMediaGeneration => QualifiedMediaGeneration,
        pub(crate) static_configuration: [u8; 32] => StaticConfiguration,
        pub(crate) recovery_limits: [u8; 32] => RecoveryLimits,
    }
    drift pub enum PhysicalRecoveryEntryBindingDrift;
}

worth_proof::binding_axes! {
    pub(crate) struct AdmittedRecoveryWorldAxes {
        pub(crate) root_ownership: PhysicalRecoveryRootOwnershipAxis => RootOwnership,
        pub(crate) stable_store: StableStoreIdentity => StableStore,
        pub(crate) recovery_session: PhysicalRecoverySessionIdentity => RecoverySession,
        pub(crate) backend_profile: QualifiedPhysicalBackendProfile => BackendProfile,
        pub(crate) qualified_media_generation: PhysicalRecoveryMediaGeneration => QualifiedMediaGeneration,
        pub(crate) static_configuration: [u8; 32] => StaticConfiguration,
        pub(crate) recovery_limits: [u8; 32] => RecoveryLimits,
    }
    drift pub(crate) enum AdmittedRecoveryWorldBindingDrift;
}

pub(crate) type PhysicalRecoveryEntryBinding = Binding<PhysicalRecoveryEntryAxes>;
pub(crate) type AdmittedRecoveryWorldBinding = Binding<AdmittedRecoveryWorldAxes>;

pub(crate) struct PhysicalRecoveryEntryPresentation(PhysicalRecoveryEntryBinding);

impl PhysicalRecoveryEntryPresentation {
    pub(crate) fn compare_with(
        &self,
        retained: &PhysicalRecoveryEntryBinding,
    ) -> Result<(), PhysicalRecoveryEntryBindingDrift> {
        retained.ensure_matches(&self.0)
    }
}

pub(crate) fn entry_binding(
    root: PathBuf,
    root_owner: MediaOwnerIdentity,
    session: PhysicalRecoverySessionIdentity,
    profile: &QualifiedPhysicalBackendProfile,
    media_generation: PhysicalRecoveryMediaGeneration,
    configuration: &PhysicalRecoveryStaticConfiguration,
    limits: PhysicalRecoveryLimits,
) -> PhysicalRecoveryEntryBinding {
    Binding::new(PhysicalRecoveryEntryAxes {
        root_ownership: PhysicalRecoveryRootOwnershipAxis {
            declared_root: root,
            owner: root_owner,
        },
        recovery_session: session,
        backend_profile: profile.clone(),
        qualified_media_generation: media_generation,
        static_configuration: configuration.identity(),
        recovery_limits: limits.identity(),
    })
}

pub(crate) fn entry_presentation(
    root: PathBuf,
    root_owner: MediaOwnerIdentity,
    session: PhysicalRecoverySessionIdentity,
    profile: &QualifiedPhysicalBackendProfile,
    media_generation: PhysicalRecoveryMediaGeneration,
    configuration: &PhysicalRecoveryStaticConfiguration,
    limits: PhysicalRecoveryLimits,
) -> PhysicalRecoveryEntryPresentation {
    PhysicalRecoveryEntryPresentation(entry_binding(
        root,
        root_owner,
        session,
        profile,
        media_generation,
        configuration,
        limits,
    ))
}

pub(crate) fn admitted_world_binding(
    entry: &PhysicalRecoveryEntryBinding,
    store_identity: StableStoreIdentity,
) -> AdmittedRecoveryWorldBinding {
    let axes = entry.axes();
    Binding::new(AdmittedRecoveryWorldAxes {
        root_ownership: axes.root_ownership.clone(),
        stable_store: store_identity,
        recovery_session: axes.recovery_session,
        backend_profile: axes.backend_profile.clone(),
        qualified_media_generation: axes.qualified_media_generation,
        static_configuration: axes.static_configuration,
        recovery_limits: axes.recovery_limits,
    })
}
