use std::path::PathBuf;

use worth_store_authority::{ControlStoreFencingAuthority, ControlStoreSelectionCoordinates};
use worth_store_physical_backend::{
    observe_filesystem_failure_domain, ControlMediaFault, ControlMediaIdentity,
    ControlMediaLocation, ControlRecoveryObjectHandle, FilesystemFailureDomainIdentity,
    PhysicalControlAppendReceipt, PhysicalOperationalControlStore,
};

use super::operational_media_path::resolve_operational_media_path;
use super::{
    encode_control_record, ControlStoreTrustPosture, OperationalControlEncodingDenial,
    OperationalControlLocation, OperationalControlRecord, OperationalControlRecordKind,
    ProtectedOperationalMediaLocation, ProtectedOperationalMediaRole,
};

#[derive(Debug)]
pub enum OperationalControlStoreOpenDenial {
    Media(ControlMediaFault),
    SharedFailureDomain {
        control: PathBuf,
        protected: PathBuf,
        failure_domain: String,
    },
    SharedObservedFilesystem {
        control: PathBuf,
        protected: PathBuf,
        filesystem: FilesystemFailureDomainIdentity,
    },
    AllocationFailed,
}

impl From<ControlMediaFault> for OperationalControlStoreOpenDenial {
    fn from(value: ControlMediaFault) -> Self {
        Self::Media(value)
    }
}

#[derive(Debug)]
pub enum OperationalControlAppendDenial {
    Media(ControlMediaFault),
    Encoding(OperationalControlEncodingDenial),
    ControlMediaOverlap { target: PathBuf, control: PathBuf },
    UnconfiguredMaterializationTarget { target: PathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonCurrentRecoveryTargetDenial {
    Unavailable,
    ProtectedMediaOverlap { target: PathBuf, protected: PathBuf },
    ControlMediaOverlap { target: PathBuf, control: PathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NonCurrentRecoveryTargetAdmission {
    _target_parent: PathBuf,
    _control_media_identity: [u8; 32],
}

pub trait OperationalControlStorePort {
    fn publish_recovery_object(
        &self,
        content: &[u8],
    ) -> Result<ControlRecoveryObjectHandle, OperationalControlAppendDenial>;

    fn append(
        &self,
        record: &OperationalControlRecord,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial>;
}

#[derive(Debug)]
pub struct OperationalControlStore {
    physical: PhysicalOperationalControlStore,
    media_surfaces: [PathBuf; 3],
    backup_target_roots: Vec<PathBuf>,
    protected_media_roots: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObservedFilesystemSeparation {
    Enforce,
    #[cfg(test)]
    CertifiedDistinct,
}

impl OperationalControlStore {
    pub fn open(
        control_location: OperationalControlLocation,
        protected_locations: impl IntoIterator<Item = ProtectedOperationalMediaLocation>,
    ) -> Result<Self, OperationalControlStoreOpenDenial> {
        Self::open_with_topology_policy(
            control_location,
            protected_locations,
            ObservedFilesystemSeparation::Enforce,
        )
    }

    #[cfg(test)]
    pub(crate) fn open_with_certified_topology(
        control_location: OperationalControlLocation,
        protected_locations: impl IntoIterator<Item = ProtectedOperationalMediaLocation>,
    ) -> Result<Self, OperationalControlStoreOpenDenial> {
        Self::open_with_topology_policy(
            control_location,
            protected_locations,
            ObservedFilesystemSeparation::CertifiedDistinct,
        )
    }

    fn open_with_topology_policy(
        control_location: OperationalControlLocation,
        protected_locations: impl IntoIterator<Item = ProtectedOperationalMediaLocation>,
        filesystem_separation: ObservedFilesystemSeparation,
    ) -> Result<Self, OperationalControlStoreOpenDenial> {
        let control = resolve_operational_media_path(control_location.path())
            .map_err(|error| OperationalControlStoreOpenDenial::Media(error.into()))?;
        let control_filesystem = observe_filesystem_failure_domain(&control)
            .map_err(|error| OperationalControlStoreOpenDenial::Media(error.into()))?;
        let physical_location = ControlMediaLocation::new(&control);
        let recovery_objects =
            resolve_operational_media_path(&physical_location.recovery_object_root())
                .map_err(|error| OperationalControlStoreOpenDenial::Media(error.into()))?;
        let identity = resolve_operational_media_path(&physical_location.identity_path())
            .map_err(|error| OperationalControlStoreOpenDenial::Media(error.into()))?;
        let mut backup_target_roots = Vec::new();
        let mut protected_media_roots = Vec::new();
        for protected in protected_locations {
            let protected_path = resolve_operational_media_path(protected.path())
                .map_err(|error| OperationalControlStoreOpenDenial::Media(error.into()))?;
            let overlaps_control_surface =
                [&control, &recovery_objects, &identity]
                    .iter()
                    .any(|surface| {
                        surface.starts_with(&protected_path) || protected_path.starts_with(surface)
                    });
            if control_location.failure_domain() == protected.failure_domain()
                || overlaps_control_surface
            {
                return Err(OperationalControlStoreOpenDenial::SharedFailureDomain {
                    control,
                    protected: protected_path,
                    failure_domain: control_location.failure_domain().as_str().to_owned(),
                });
            }
            let protected_filesystem = observe_filesystem_failure_domain(&protected_path)
                .map_err(|error| OperationalControlStoreOpenDenial::Media(error.into()))?;
            if filesystem_separation == ObservedFilesystemSeparation::Enforce
                && control_filesystem == protected_filesystem
            {
                return Err(
                    OperationalControlStoreOpenDenial::SharedObservedFilesystem {
                        control,
                        protected: protected_path,
                        filesystem: control_filesystem,
                    },
                );
            }
            if protected.role() == ProtectedOperationalMediaRole::BackupTarget {
                backup_target_roots
                    .try_reserve(1)
                    .map_err(|_| OperationalControlStoreOpenDenial::AllocationFailed)?;
                backup_target_roots.push(protected_path.clone());
            }
            protected_media_roots
                .try_reserve(1)
                .map_err(|_| OperationalControlStoreOpenDenial::AllocationFailed)?;
            protected_media_roots.push(protected_path);
        }
        Ok(Self {
            physical: PhysicalOperationalControlStore::open(physical_location)?,
            media_surfaces: [control, recovery_objects, identity],
            backup_target_roots,
            protected_media_roots,
        })
    }

    pub const fn media_identity(&self) -> ControlMediaIdentity {
        self.physical.identity()
    }

    pub fn observe_selection_coordinates(
        &self,
    ) -> Result<Option<ControlStoreSelectionCoordinates>, ControlMediaFault> {
        self.physical.observe_current_prefix().map(|current| {
            current.map(|(generation, prefix_digest)| {
                ControlStoreSelectionCoordinates::new(
                    self.media_identity().fingerprint(),
                    generation,
                    prefix_digest,
                )
            })
        })
    }

    pub fn inspect_generations(
        &self,
        fencing_authority: &ControlStoreFencingAuthority<'_>,
    ) -> ControlStoreTrustPosture {
        super::inspect_control_store_copies(&[self], fencing_authority)
    }

    pub fn inspect_generations_with_budget(
        &self,
        fencing_authority: &ControlStoreFencingAuthority<'_>,
        budget: super::OperationalControlReplayBudget,
    ) -> ControlStoreTrustPosture {
        super::inspect_control_store_copies_with_budget(&[self], fencing_authority, budget)
    }

    pub(crate) const fn physical(&self) -> &PhysicalOperationalControlStore {
        &self.physical
    }

    pub(crate) fn admit_non_current_recovery_target(
        &self,
        target_parent: &std::path::Path,
    ) -> Result<NonCurrentRecoveryTargetAdmission, NonCurrentRecoveryTargetDenial> {
        let target = resolve_operational_media_path(target_parent)
            .map_err(|_| NonCurrentRecoveryTargetDenial::Unavailable)?;
        if let Some(control) = self
            .media_surfaces
            .iter()
            .find(|control| control.starts_with(&target) || target.starts_with(control))
        {
            return Err(NonCurrentRecoveryTargetDenial::ControlMediaOverlap {
                target,
                control: control.clone(),
            });
        }
        if let Some(protected) = self.protected_media_roots.iter().find(|protected| {
            protected.starts_with(&target) || target.starts_with(protected.as_path())
        }) {
            return Err(NonCurrentRecoveryTargetDenial::ProtectedMediaOverlap {
                target,
                protected: protected.clone(),
            });
        }
        Ok(NonCurrentRecoveryTargetAdmission {
            _target_parent: target,
            _control_media_identity: self.media_identity().fingerprint(),
        })
    }
}

impl OperationalControlStorePort for OperationalControlStore {
    fn publish_recovery_object(
        &self,
        content: &[u8],
    ) -> Result<ControlRecoveryObjectHandle, OperationalControlAppendDenial> {
        self.physical
            .publish_recovery_object(content)
            .map_err(OperationalControlAppendDenial::Media)
    }

    fn append(
        &self,
        record: &OperationalControlRecord,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial> {
        self.reject_control_media_overlap(record)?;
        let payload =
            encode_control_record(record).map_err(OperationalControlAppendDenial::Encoding)?;
        let transition_identity = format!(
            "{}\0{}",
            record.operation_id().as_str(),
            record.transition_id().as_str()
        );
        self.physical
            .append_at_current_tail(&transition_identity, &payload)
            .map_err(OperationalControlAppendDenial::Media)
    }
}

impl OperationalControlStore {
    fn reject_control_media_overlap(
        &self,
        record: &OperationalControlRecord,
    ) -> Result<(), OperationalControlAppendDenial> {
        let OperationalControlRecordKind::BackupMaterializationOpened { plan } = record.kind()
        else {
            return Ok(());
        };
        for control in &self.media_surfaces {
            if control.starts_with(plan.target_parent())
                || plan.target_parent().starts_with(control)
            {
                return Err(OperationalControlAppendDenial::ControlMediaOverlap {
                    target: plan.target_parent().to_path_buf(),
                    control: control.clone(),
                });
            }
        }
        if !self
            .backup_target_roots
            .iter()
            .any(|target| plan.target_parent().starts_with(target))
        {
            return Err(
                OperationalControlAppendDenial::UnconfiguredMaterializationTarget {
                    target: plan.target_parent().to_path_buf(),
                },
            );
        }
        Ok(())
    }
}
