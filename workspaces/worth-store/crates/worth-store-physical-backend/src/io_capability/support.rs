use super::vocabulary::{BackendCapabilityKind, BackendCapabilitySupportPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendCapabilitySupportSet {
    buffered_file: BackendCapabilitySupportPosture,
    direct_io: BackendCapabilitySupportPosture,
    mmap: BackendCapabilitySupportPosture,
    async_io: BackendCapabilitySupportPosture,
    fsync: BackendCapabilitySupportPosture,
    directory_sync: BackendCapabilitySupportPosture,
    durable_rename: BackendCapabilitySupportPosture,
    secure_frame_io: BackendCapabilitySupportPosture,
}

impl BackendCapabilitySupportSet {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn all_supported() -> Self {
        Self {
            buffered_file: BackendCapabilitySupportPosture::Supported,
            direct_io: BackendCapabilitySupportPosture::Supported,
            mmap: BackendCapabilitySupportPosture::Supported,
            async_io: BackendCapabilitySupportPosture::Supported,
            fsync: BackendCapabilitySupportPosture::Supported,
            directory_sync: BackendCapabilitySupportPosture::Supported,
            durable_rename: BackendCapabilitySupportPosture::Supported,
            secure_frame_io: BackendCapabilitySupportPosture::Supported,
        }
    }

    pub(crate) const fn for_filesystem_observation(
        buffered_file: BackendCapabilitySupportPosture,
        fsync: BackendCapabilitySupportPosture,
        directory_sync: BackendCapabilitySupportPosture,
        durable_rename: BackendCapabilitySupportPosture,
    ) -> Self {
        Self {
            buffered_file,
            direct_io: BackendCapabilitySupportPosture::Unsupported,
            mmap: BackendCapabilitySupportPosture::Unsupported,
            async_io: BackendCapabilitySupportPosture::Unsupported,
            fsync,
            directory_sync,
            durable_rename,
            secure_frame_io: BackendCapabilitySupportPosture::Unsupported,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn buffered_durable_only() -> Self {
        Self::buffered_durable()
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    const fn buffered_durable() -> Self {
        Self {
            buffered_file: BackendCapabilitySupportPosture::Supported,
            direct_io: BackendCapabilitySupportPosture::Unsupported,
            mmap: BackendCapabilitySupportPosture::Unsupported,
            async_io: BackendCapabilitySupportPosture::Unsupported,
            fsync: BackendCapabilitySupportPosture::Supported,
            directory_sync: BackendCapabilitySupportPosture::Supported,
            durable_rename: BackendCapabilitySupportPosture::Supported,
            secure_frame_io: BackendCapabilitySupportPosture::Unsupported,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_posture(
        mut self,
        kind: BackendCapabilityKind,
        posture: BackendCapabilitySupportPosture,
    ) -> Self {
        match kind {
            BackendCapabilityKind::BufferedFile => self.buffered_file = posture,
            BackendCapabilityKind::DirectIo => self.direct_io = posture,
            BackendCapabilityKind::Mmap => self.mmap = posture,
            BackendCapabilityKind::AsyncIo => self.async_io = posture,
            BackendCapabilityKind::Fsync => self.fsync = posture,
            BackendCapabilityKind::DirectorySync => self.directory_sync = posture,
            BackendCapabilityKind::DurableRename => self.durable_rename = posture,
            BackendCapabilityKind::SecureFrameIo => self.secure_frame_io = posture,
        }
        self
    }

    pub const fn posture(self, kind: BackendCapabilityKind) -> BackendCapabilitySupportPosture {
        match kind {
            BackendCapabilityKind::BufferedFile => self.buffered_file,
            BackendCapabilityKind::DirectIo => self.direct_io,
            BackendCapabilityKind::Mmap => self.mmap,
            BackendCapabilityKind::AsyncIo => self.async_io,
            BackendCapabilityKind::Fsync => self.fsync,
            BackendCapabilityKind::DirectorySync => self.directory_sync,
            BackendCapabilityKind::DurableRename => self.durable_rename,
            BackendCapabilityKind::SecureFrameIo => self.secure_frame_io,
        }
    }
}
