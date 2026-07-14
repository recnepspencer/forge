use crate::{
    CheckpointPublicationRoot, CurrentPhysicalRoot, ManifestLocatorRoot, RecoveryRoot,
    RootKindMismatchDenial,
};

pub fn reject_checkpoint_root_as_current_read_authority(
    _: CheckpointPublicationRoot,
) -> RootKindMismatchDenial {
    RootKindMismatchDenial::CheckpointPublicationRootCannotAdmitCurrentReadPlan
}

pub const fn reject_recovery_root_as_current_read_authority(
    _: RecoveryRoot,
) -> RootKindMismatchDenial {
    RootKindMismatchDenial::RecoveryRootRequiresEntryReadmission
}

pub const fn reject_manifest_locator_root_as_current_read_authority(
    _: ManifestLocatorRoot,
) -> RootKindMismatchDenial {
    RootKindMismatchDenial::ManifestLocatorRootCannotAdmitCurrentReadPlan
}

pub const fn readmit_current_root_for_read_plan(root: CurrentPhysicalRoot) -> CurrentPhysicalRoot {
    root
}
