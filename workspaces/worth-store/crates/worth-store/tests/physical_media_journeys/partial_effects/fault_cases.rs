use worth_store_physical_backend::MediaOperationRole;

#[derive(Clone, Copy)]
pub(super) struct FaultCase {
    pub(super) id: &'static str,
    pub(super) expected_outcome: &'static str,
    pub(super) manifest: ManifestPosture,
    pub(super) abrupt: bool,
    pub(super) expected_fault: Option<FaultExpectation>,
}

#[derive(Clone, Copy)]
pub(super) enum ManifestPosture {
    Absent,
    ScaffoldOnly,
    ScaffoldWithLock,
    EmptyStagedIdentity,
    FullStagedIdentity,
    Published,
    PublishedWithQualificationResidue,
}

impl ManifestPosture {
    pub(super) const fn identity_visible(self) -> bool {
        matches!(
            self,
            Self::Published | Self::PublishedWithQualificationResidue
        )
    }
}

#[derive(Clone, Copy)]
pub(super) struct FaultExpectation {
    pub(super) role: MediaOperationRole,
    pub(super) ordinal: u64,
    pub(super) terminal: &'static str,
    pub(super) operation_bound: bool,
}

const fn fault(
    id: &'static str,
    expected_outcome: &'static str,
    manifest: ManifestPosture,
    role: MediaOperationRole,
    ordinal: u64,
    terminal: &'static str,
    operation_bound: bool,
) -> FaultCase {
    FaultCase {
        id,
        expected_outcome,
        manifest,
        abrupt: false,
        expected_fault: Some(FaultExpectation {
            role,
            ordinal,
            terminal,
            operation_bound,
        }),
    }
}

const fn abrupt(
    id: &'static str,
    manifest: ManifestPosture,
    role: MediaOperationRole,
    ordinal: u64,
    operation_bound: bool,
) -> FaultCase {
    FaultCase {
        id,
        expected_outcome: "abrupt-death",
        manifest,
        abrupt: true,
        expected_fault: Some(FaultExpectation {
            role,
            ordinal,
            terminal: "abrupt",
            operation_bound,
        }),
    }
}

pub(super) const CI_CASES: [FaultCase; 5] = [
    FaultCase {
        id: "pass-through",
        expected_outcome: "success",
        manifest: ManifestPosture::Published,
        abrupt: false,
        expected_fault: None,
    },
    fault(
        "before-root-creation",
        "denied",
        ManifestPosture::Absent,
        MediaOperationRole::CreateDirectory,
        1,
        "denied",
        false,
    ),
    fault(
        "short-identity-prefix",
        "success",
        ManifestPosture::Published,
        MediaOperationRole::PositionedWrite,
        1,
        "partial",
        true,
    ),
    fault(
        "directory-barrier-indeterminate",
        "failed",
        ManifestPosture::Published,
        MediaOperationRole::SynchronizeDirectoryPublication,
        1,
        "denied",
        true,
    ),
    abrupt(
        "abrupt-after-replacement",
        ManifestPosture::Published,
        MediaOperationRole::AtomicReplace,
        1,
        true,
    ),
];

pub(super) const RELEASE_CASES: [FaultCase; 17] = [
    fault(
        "after-fixed-directories",
        "failed",
        ManifestPosture::ScaffoldOnly,
        MediaOperationRole::CreateDirectory,
        4,
        "indeterminate",
        false,
    ),
    fault(
        "before-staged-identity-create",
        "failed",
        ManifestPosture::ScaffoldWithLock,
        MediaOperationRole::CreateNew,
        1,
        "denied",
        true,
    ),
    fault(
        "after-staged-identity-create",
        "failed",
        ManifestPosture::EmptyStagedIdentity,
        MediaOperationRole::PositionedWrite,
        1,
        "denied",
        true,
    ),
    fault(
        "after-complete-identity-write",
        "failed",
        ManifestPosture::FullStagedIdentity,
        MediaOperationRole::SynchronizeFileState,
        1,
        "denied",
        true,
    ),
    fault(
        "file-barrier-denial",
        "failed",
        ManifestPosture::FullStagedIdentity,
        MediaOperationRole::SynchronizeFileState,
        1,
        "denied",
        true,
    ),
    fault(
        "after-identity-file-sync",
        "failed",
        ManifestPosture::FullStagedIdentity,
        MediaOperationRole::AtomicReplace,
        1,
        "denied",
        true,
    ),
    abrupt(
        "after-directory-sync-before-observation",
        ManifestPosture::Published,
        MediaOperationRole::SynchronizeRootParentPublication,
        1,
        true,
    ),
    fault(
        "qualification-positioned-write",
        "failed",
        ManifestPosture::Published,
        MediaOperationRole::PositionedWrite,
        2,
        "partial",
        true,
    ),
    fault(
        "qualification-append",
        "failed",
        ManifestPosture::Published,
        MediaOperationRole::Append,
        1,
        "denied",
        true,
    ),
    fault(
        "qualification-truncate",
        "failed",
        ManifestPosture::Published,
        MediaOperationRole::Truncate,
        1,
        "denied",
        true,
    ),
    fault(
        "qualification-allocation",
        "failed",
        ManifestPosture::Published,
        MediaOperationRole::Allocate,
        1,
        "denied",
        true,
    ),
    fault(
        "qualification-metadata",
        "failed",
        ManifestPosture::Published,
        MediaOperationRole::ReadMetadata,
        1,
        "denied",
        true,
    ),
    fault(
        "qualification-list",
        "failed",
        ManifestPosture::Published,
        MediaOperationRole::ListDirectory,
        1,
        "denied",
        true,
    ),
    fault(
        "cleanup-delete",
        "failed",
        ManifestPosture::PublishedWithQualificationResidue,
        MediaOperationRole::Delete,
        1,
        "denied",
        true,
    ),
    fault(
        "cleanup-directory-barrier",
        "failed",
        ManifestPosture::Published,
        MediaOperationRole::SynchronizeDirectoryPublication,
        3,
        "denied",
        true,
    ),
    abrupt(
        "before-lock-release",
        ManifestPosture::Published,
        MediaOperationRole::ReleaseMutationLease,
        1,
        false,
    ),
    abrupt(
        "after-lock-release",
        ManifestPosture::Published,
        MediaOperationRole::ReleaseMutationLease,
        1,
        false,
    ),
];
