use std::num::NonZeroU64;

use worth_store_physical_backend::{
    CapabilitySupport, FilesystemBackendProfile, FilesystemLocation, MediaCapability,
};

use super::{require_text, PhysicalWorkRunProvenanceDenial};

macro_rules! define_capabilities {
    ($($name:ident => $label:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum PhysicalWorkFilesystemCapabilityEvidence {
            $($name),+
        }

        impl PhysicalWorkFilesystemCapabilityEvidence {
            pub const ALL: [Self; define_capabilities!(@count $($name),+)] = [
                $(Self::$name),+
            ];

            pub const fn label(self) -> &'static str {
                match self {
                    $(Self::$name => $label),+
                }
            }
        }
    };
    (@count $($name:ident),+) => {
        <[()]>::len(&[$(define_capabilities!(@unit $name)),+])
    };
    (@unit $name:ident) => { () };
}

define_capabilities!(
    OrdinaryFile => "ordinary-file",
    Directory => "directory",
    PositionedTransfer => "positioned-transfer",
    Append => "append",
    Metadata => "metadata",
    DirectoryListing => "directory-listing",
    Deletion => "deletion",
    FileStateSynchronization => "file-state-synchronization",
    DirectorySynchronization => "directory-synchronization",
    AtomicSameNamespaceReplacement => "atomic-same-namespace-replacement",
    DataOnlySynchronization => "data-only-synchronization",
    SparseAllocation => "sparse-allocation",
    EagerAllocation => "eager-allocation",
    MemoryMap => "memory-map",
    DirectIo => "direct-io",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkFilesystemSupportEvidence {
    Supported,
    Unsupported,
    Indeterminate,
}

impl PhysicalWorkFilesystemSupportEvidence {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Unsupported => "unsupported",
            Self::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkFilesystemCapabilityObservation {
    capability: PhysicalWorkFilesystemCapabilityEvidence,
    support: PhysicalWorkFilesystemSupportEvidence,
}

impl PhysicalWorkFilesystemCapabilityObservation {
    pub const fn new(
        capability: PhysicalWorkFilesystemCapabilityEvidence,
        support: PhysicalWorkFilesystemSupportEvidence,
    ) -> Self {
        Self {
            capability,
            support,
        }
    }

    pub const fn capability(self) -> PhysicalWorkFilesystemCapabilityEvidence {
        self.capability
    }

    pub const fn support(self) -> PhysicalWorkFilesystemSupportEvidence {
        self.support
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkFilesystemLocationEvidence {
    Local,
    Remote,
    Unknown,
}

impl PhysicalWorkFilesystemLocationEvidence {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Unknown => "unknown",
        }
    }
}

pub struct PhysicalWorkFilesystemProfileParts {
    pub root_identity: [u8; 32],
    pub volume_identity: [u8; 32],
    pub filesystem_type: Box<str>,
    pub allocation_granularity: NonZeroU64,
    pub location: PhysicalWorkFilesystemLocationEvidence,
    pub removable: bool,
    pub read_only: bool,
    pub capabilities: Box<[PhysicalWorkFilesystemCapabilityObservation]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkFilesystemProfileEvidence {
    root_identity: [u8; 32],
    volume_identity: [u8; 32],
    filesystem_type: Box<str>,
    allocation_granularity: NonZeroU64,
    location: PhysicalWorkFilesystemLocationEvidence,
    removable: bool,
    read_only: bool,
    capabilities: Box<[PhysicalWorkFilesystemCapabilityObservation]>,
}

impl PhysicalWorkFilesystemProfileEvidence {
    pub fn from_backend(
        profile: &FilesystemBackendProfile,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        let capabilities = MediaCapability::ALL
            .into_iter()
            .map(|capability| {
                PhysicalWorkFilesystemCapabilityObservation::new(
                    lower_capability(capability),
                    lower_support(profile.support(capability)),
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self::from_parts(PhysicalWorkFilesystemProfileParts {
            root_identity: profile.root_identity(),
            volume_identity: profile.volume_identity(),
            filesystem_type: profile.filesystem_type().into(),
            allocation_granularity: profile.allocation_granularity(),
            location: lower_location(profile.location()),
            removable: profile.is_removable(),
            read_only: profile.is_read_only(),
            capabilities,
        })
    }

    pub fn from_parts(
        parts: PhysicalWorkFilesystemProfileParts,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        if parts.root_identity == [0; 32] {
            return Err(PhysicalWorkRunProvenanceDenial::UnqualifiedFilesystemRoot);
        }
        if parts.volume_identity == [0; 32] {
            return Err(PhysicalWorkRunProvenanceDenial::UnqualifiedFilesystemVolume);
        }
        require_text(
            &parts.filesystem_type,
            PhysicalWorkRunProvenanceDenial::EmptyFilesystemType,
        )?;
        validate_capabilities(&parts.capabilities)?;
        Ok(Self {
            root_identity: parts.root_identity,
            volume_identity: parts.volume_identity,
            filesystem_type: parts.filesystem_type,
            allocation_granularity: parts.allocation_granularity,
            location: parts.location,
            removable: parts.removable,
            read_only: parts.read_only,
            capabilities: parts.capabilities,
        })
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

    pub const fn allocation_granularity(&self) -> NonZeroU64 {
        self.allocation_granularity
    }

    pub const fn location(&self) -> PhysicalWorkFilesystemLocationEvidence {
        self.location
    }

    pub const fn is_removable(&self) -> bool {
        self.removable
    }

    pub const fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub const fn capabilities(&self) -> &[PhysicalWorkFilesystemCapabilityObservation] {
        &self.capabilities
    }
}

fn validate_capabilities(
    observations: &[PhysicalWorkFilesystemCapabilityObservation],
) -> Result<(), PhysicalWorkRunProvenanceDenial> {
    for capability in PhysicalWorkFilesystemCapabilityEvidence::ALL {
        let count = observations
            .iter()
            .filter(|observation| observation.capability() == capability)
            .count();
        match count {
            0 => return Err(PhysicalWorkRunProvenanceDenial::MissingFilesystemCapability),
            1 => {}
            _ => return Err(PhysicalWorkRunProvenanceDenial::DuplicateFilesystemCapability),
        }
    }
    if observations.len() != PhysicalWorkFilesystemCapabilityEvidence::ALL.len() {
        return Err(PhysicalWorkRunProvenanceDenial::DuplicateFilesystemCapability);
    }
    Ok(())
}

const fn lower_capability(capability: MediaCapability) -> PhysicalWorkFilesystemCapabilityEvidence {
    match capability {
        MediaCapability::OrdinaryFile => PhysicalWorkFilesystemCapabilityEvidence::OrdinaryFile,
        MediaCapability::Directory => PhysicalWorkFilesystemCapabilityEvidence::Directory,
        MediaCapability::PositionedTransfer => {
            PhysicalWorkFilesystemCapabilityEvidence::PositionedTransfer
        }
        MediaCapability::Append => PhysicalWorkFilesystemCapabilityEvidence::Append,
        MediaCapability::Metadata => PhysicalWorkFilesystemCapabilityEvidence::Metadata,
        MediaCapability::DirectoryListing => {
            PhysicalWorkFilesystemCapabilityEvidence::DirectoryListing
        }
        MediaCapability::Deletion => PhysicalWorkFilesystemCapabilityEvidence::Deletion,
        MediaCapability::FileStateSynchronization => {
            PhysicalWorkFilesystemCapabilityEvidence::FileStateSynchronization
        }
        MediaCapability::DirectorySynchronization => {
            PhysicalWorkFilesystemCapabilityEvidence::DirectorySynchronization
        }
        MediaCapability::AtomicSameNamespaceReplacement => {
            PhysicalWorkFilesystemCapabilityEvidence::AtomicSameNamespaceReplacement
        }
        MediaCapability::DataOnlySynchronization => {
            PhysicalWorkFilesystemCapabilityEvidence::DataOnlySynchronization
        }
        MediaCapability::SparseAllocation => {
            PhysicalWorkFilesystemCapabilityEvidence::SparseAllocation
        }
        MediaCapability::EagerAllocation => {
            PhysicalWorkFilesystemCapabilityEvidence::EagerAllocation
        }
        MediaCapability::MemoryMap => PhysicalWorkFilesystemCapabilityEvidence::MemoryMap,
        MediaCapability::DirectIo => PhysicalWorkFilesystemCapabilityEvidence::DirectIo,
    }
}

const fn lower_support(support: CapabilitySupport) -> PhysicalWorkFilesystemSupportEvidence {
    match support {
        CapabilitySupport::Supported => PhysicalWorkFilesystemSupportEvidence::Supported,
        CapabilitySupport::Unsupported => PhysicalWorkFilesystemSupportEvidence::Unsupported,
        CapabilitySupport::Indeterminate => PhysicalWorkFilesystemSupportEvidence::Indeterminate,
    }
}

const fn lower_location(location: FilesystemLocation) -> PhysicalWorkFilesystemLocationEvidence {
    match location {
        FilesystemLocation::Local => PhysicalWorkFilesystemLocationEvidence::Local,
        FilesystemLocation::Remote => PhysicalWorkFilesystemLocationEvidence::Remote,
        FilesystemLocation::Unknown => PhysicalWorkFilesystemLocationEvidence::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use super::PhysicalWorkRunProvenanceDenial;
    use super::{
        PhysicalWorkFilesystemCapabilityEvidence, PhysicalWorkFilesystemCapabilityObservation,
        PhysicalWorkFilesystemLocationEvidence, PhysicalWorkFilesystemProfileEvidence,
        PhysicalWorkFilesystemProfileParts, PhysicalWorkFilesystemSupportEvidence,
    };

    #[test]
    fn unqualified_root_identity_cannot_enter_courtroom_evidence() {
        let capabilities = PhysicalWorkFilesystemCapabilityEvidence::ALL
            .map(|capability| {
                PhysicalWorkFilesystemCapabilityObservation::new(
                    capability,
                    PhysicalWorkFilesystemSupportEvidence::Supported,
                )
            })
            .into();
        assert_eq!(
            PhysicalWorkFilesystemProfileEvidence::from_parts(PhysicalWorkFilesystemProfileParts {
                root_identity: [0; 32],
                volume_identity: [2; 32],
                filesystem_type: "test".into(),
                allocation_granularity: NonZeroU64::MIN,
                location: PhysicalWorkFilesystemLocationEvidence::Local,
                removable: false,
                read_only: false,
                capabilities,
            }),
            Err(PhysicalWorkRunProvenanceDenial::UnqualifiedFilesystemRoot)
        );
    }
}
