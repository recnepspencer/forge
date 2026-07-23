use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;

use worth_store_physical_format::store_namespace::StoreNamespaceRelativeRole;

use super::super::{
    ArtifactFamilyDirectory, MediaFaultSchedule, MutationOwnershipDenial, MutationOwnershipLease,
    StagingDirectory,
};
use super::{
    FilesystemMediaAdmissionAuthority, FilesystemMediaOwner, FilesystemMediaOwnerAdmissionDenial,
    ObservedMediaOwnerAdmissionFailure,
};

impl FilesystemMediaOwner {
    pub fn admit(
        root: &Path,
        _authority: FilesystemMediaAdmissionAuthority,
    ) -> Result<Self, FilesystemMediaOwnerAdmissionDenial> {
        Self::admit_with_schedule(root, MediaFaultSchedule::default())
    }

    pub(in crate::filesystem_media) fn admit_with_schedule(
        root: &Path,
        schedule: MediaFaultSchedule,
    ) -> Result<Self, FilesystemMediaOwnerAdmissionDenial> {
        Self::admit_with_observation(root, schedule).map_err(|failure| failure.denial)
    }

    pub(in crate::filesystem_media) fn admit_with_observation(
        root: &Path,
        schedule: MediaFaultSchedule,
    ) -> Result<Self, ObservedMediaOwnerAdmissionFailure> {
        let counters = Arc::new(super::super::operation_counters::MediaCounterCells::default());
        let boundary = super::super::fault_interposition::MediaFaultInterposer::new(
            schedule,
            Arc::clone(&counters),
        );
        Self::admit_with_boundary(root, boundary, counters)
    }

    pub(in crate::filesystem_media) fn admit_with_boundary(
        root: &Path,
        boundary: super::super::fault_interposition::MediaFaultInterposer,
        counters: Arc<super::super::operation_counters::MediaCounterCells>,
    ) -> Result<Self, ObservedMediaOwnerAdmissionFailure> {
        use super::super::owner_admission_effect::MediaOwnerAdmissionEffectFate;

        counters.ownership_attempt();
        let mut namespace = match super::super::namespace_admission::create_or_open(root, &boundary)
        {
            Ok(namespace) => namespace,
            Err(failure) => {
                return Err(ObservedMediaOwnerAdmissionFailure {
                    denial: FilesystemMediaOwnerAdmissionDenial::Confinement(failure.denial()),
                    counters: counters.snapshot(),
                    effect_fate: failure.effect_fate(),
                    release: None,
                });
            }
        };
        let namespace_effect_fate = namespace.admission_effect_fate();
        let namespace_directory = match namespace
            .open_role_directory(StoreNamespaceRelativeRole::NamespaceDirectory, &boundary)
        {
            Ok(directory) => directory,
            Err(denial) => {
                drop(namespace);
                return Err(ObservedMediaOwnerAdmissionFailure {
                    denial: FilesystemMediaOwnerAdmissionDenial::Confinement(denial),
                    counters: counters.snapshot(),
                    effect_fate: namespace_effect_fate,
                    release: None,
                });
            }
        };
        let mutation_lease = match MutationOwnershipLease::try_acquire(
            namespace.owner_identity(),
            &namespace_directory,
            &boundary,
        ) {
            Ok(lease) => lease,
            Err(failure) => {
                let denial = failure.denial();
                if denial == MutationOwnershipDenial::Contended {
                    counters.ownership_contended();
                }
                let effect_fate = namespace_effect_fate.combine(failure.effect_fate());
                let release = failure.release();
                drop(namespace_directory);
                drop(namespace);
                return Err(ObservedMediaOwnerAdmissionFailure {
                    denial: FilesystemMediaOwnerAdmissionDenial::Ownership(denial),
                    counters: counters.snapshot(),
                    effect_fate,
                    release,
                });
            }
        };
        let owner_effect_fate = MediaOwnerAdmissionEffectFate::EffectPossible;
        let families = match namespace
            .open_role_directory(StoreNamespaceRelativeRole::FamiliesDirectory, &boundary)
            .map(ArtifactFamilyDirectory::new)
        {
            Ok(families) => families,
            Err(denial) => {
                drop(namespace_directory);
                drop(namespace);
                let release = mutation_lease.release(&boundary);
                return Err(ObservedMediaOwnerAdmissionFailure {
                    denial: FilesystemMediaOwnerAdmissionDenial::Confinement(denial),
                    counters: counters.snapshot(),
                    effect_fate: owner_effect_fate,
                    release: Some(release),
                });
            }
        };
        let staging = match namespace
            .open_role_directory(StoreNamespaceRelativeRole::StagingDirectory, &boundary)
            .map(StagingDirectory::new)
        {
            Ok(staging) => staging,
            Err(denial) => {
                drop(families);
                drop(namespace_directory);
                drop(namespace);
                let release = mutation_lease.release(&boundary);
                return Err(ObservedMediaOwnerAdmissionFailure {
                    denial: FilesystemMediaOwnerAdmissionDenial::Confinement(denial),
                    counters: counters.snapshot(),
                    effect_fate: owner_effect_fate,
                    release: Some(release),
                });
            }
        };
        let store_root_publication_required = namespace.store_root_publication_required();
        let root_parent_publication_required = namespace.root_parent_publication_required();
        Ok(Self {
            boundary,
            namespace,
            namespace_directory,
            families,
            staging,
            mutation_lease,
            namespace_mutation_sequence: std::sync::Mutex::new(()),
            next_file_handle_generation: AtomicU64::new(5),
            next_operation_identity: AtomicU64::new(1),
            file_mutation_sequences: Default::default(),
            artifact_mutations: Arc::new(Default::default()),
            store_root_publication_required: AtomicBool::new(store_root_publication_required),
            root_parent_publication_required: AtomicBool::new(root_parent_publication_required),
        })
    }
}
