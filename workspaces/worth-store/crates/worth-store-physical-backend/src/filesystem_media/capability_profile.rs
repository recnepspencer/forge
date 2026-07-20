macro_rules! define_media_capabilities {
    ($($capability:ident),+ $(,)?) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum MediaCapability {
            $($capability),+
        }

        impl MediaCapability {
            pub const ALL: [Self; define_media_capabilities!(@count $($capability),+)] = [
                $(Self::$capability),+
            ];

            const fn index(self) -> usize {
                self as usize
            }
        }
    };
    (@count $($capability:ident),+) => {
        <[()]>::len(&[$(define_media_capabilities!(@unit $capability)),+])
    };
    (@unit $capability:ident) => { () };
}

define_media_capabilities!(
    OrdinaryFile,
    Directory,
    PositionedTransfer,
    Append,
    Metadata,
    DirectoryListing,
    Deletion,
    FileStateSynchronization,
    DirectorySynchronization,
    AtomicSameNamespaceReplacement,
    DataOnlySynchronization,
    SparseAllocation,
    EagerAllocation,
    MemoryMap,
    DirectIo,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilitySupport {
    Supported,
    Unsupported,
    Indeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaCapabilityObservation {
    capability: MediaCapability,
    support: CapabilitySupport,
}

impl MediaCapabilityObservation {
    pub const fn new(capability: MediaCapability, support: CapabilitySupport) -> Self {
        Self {
            capability,
            support,
        }
    }

    pub const fn capability(self) -> MediaCapability {
        self.capability
    }

    pub const fn support(self) -> CapabilitySupport {
        self.support
    }
}

/// Complete support observation for one backend profile.
///
/// This is diagnostic input to later qualification. It grants no capability
/// and cannot be promoted into an operation handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesystemBackendProfile {
    support_by_capability: [CapabilitySupport; MediaCapability::ALL.len()],
    root_identity: [u8; 32],
    volume_identity: [u8; 32],
    filesystem_type: Box<str>,
    allocation_granularity: core::num::NonZeroU64,
    location: FilesystemLocation,
    removable: bool,
    read_only: bool,
}

pub(super) struct ObservedFilesystemProfile {
    pub(super) support_by_capability: [CapabilitySupport; MediaCapability::ALL.len()],
    pub(super) root_identity: [u8; 32],
    pub(super) volume_identity: [u8; 32],
    pub(super) filesystem_type: Box<str>,
    pub(super) allocation_granularity: core::num::NonZeroU64,
    pub(super) location: FilesystemLocation,
    pub(super) removable: bool,
    pub(super) read_only: bool,
}

impl FilesystemBackendProfile {
    pub fn from_observations(
        observations: &[MediaCapabilityObservation],
    ) -> Result<Self, CapabilityProfileError> {
        let mut seen = [false; MediaCapability::ALL.len()];
        let mut support_by_capability =
            [CapabilitySupport::Indeterminate; MediaCapability::ALL.len()];

        for observation in observations {
            let index = observation.capability().index();
            if seen[index] {
                return Err(CapabilityProfileError::Duplicate(observation.capability()));
            }
            seen[index] = true;
            support_by_capability[index] = observation.support();
        }

        for capability in MediaCapability::ALL {
            if !seen[capability.index()] {
                return Err(CapabilityProfileError::Missing(capability));
            }
        }

        Ok(Self {
            support_by_capability,
            root_identity: [0; 32],
            volume_identity: [0; 32],
            filesystem_type: "unbound-observation".into(),
            allocation_granularity: core::num::NonZeroU64::MIN,
            location: FilesystemLocation::Unknown,
            removable: false,
            read_only: false,
        })
    }

    pub const fn support(&self, capability: MediaCapability) -> CapabilitySupport {
        self.support_by_capability[capability.index()]
    }

    pub const fn root_identity(&self) -> [u8; 32] {
        self.root_identity
    }

    pub const fn volume_identity(&self) -> [u8; 32] {
        self.volume_identity
    }

    pub fn filesystem_type(&self) -> &str {
        &self.filesystem_type
    }

    pub const fn allocation_granularity(&self) -> core::num::NonZeroU64 {
        self.allocation_granularity
    }

    pub const fn location(&self) -> FilesystemLocation {
        self.location
    }

    pub const fn is_removable(&self) -> bool {
        self.removable
    }

    pub const fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub(super) fn from_root_observation(observed: ObservedFilesystemProfile) -> Self {
        Self {
            support_by_capability: observed.support_by_capability,
            root_identity: observed.root_identity,
            volume_identity: observed.volume_identity,
            filesystem_type: observed.filesystem_type,
            allocation_granularity: observed.allocation_granularity,
            location: observed.location,
            removable: observed.removable,
            read_only: observed.read_only,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemLocation {
    Local,
    Remote,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityProfileError {
    Missing(MediaCapability),
    Duplicate(MediaCapability),
}
