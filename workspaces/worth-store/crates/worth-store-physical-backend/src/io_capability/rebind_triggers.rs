#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendRebindTriggers {
    bits: u16,
}

impl BackendRebindTriggers {
    const KERNEL: u16 = 1 << 0;
    const FILESYSTEM: u16 = 1 << 1;
    const MOUNT: u16 = 1 << 2;
    const FIRMWARE: u16 = 1 << 3;
    const BACKEND: u16 = 1 << 4;
    #[cfg(any(test, feature = "certification-test-authority"))]
    const MEDIA: u16 = 1 << 5;
    #[cfg(any(test, feature = "certification-test-authority"))]
    const CLOUD_VOLUME: u16 = 1 << 6;
    #[cfg(any(test, feature = "certification-test-authority"))]
    const SECTOR_ALIGNMENT: u16 = 1 << 7;
    #[cfg(any(test, feature = "certification-test-authority"))]
    const SECURITY_POSTURE: u16 = 1 << 8;

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn none() -> Self {
        Self { bits: 0 }
    }

    pub(crate) const fn for_filesystem_qualification() -> Self {
        Self::platform_and_backend()
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn kernel_filesystem_mount_firmware_and_backend() -> Self {
        Self::platform_and_backend()
    }

    const fn platform_and_backend() -> Self {
        Self {
            bits: Self::KERNEL | Self::FILESYSTEM | Self::MOUNT | Self::FIRMWARE | Self::BACKEND,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_media(self) -> Self {
        Self {
            bits: self.bits | Self::MEDIA,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_cloud_volume(self) -> Self {
        Self {
            bits: self.bits | Self::CLOUD_VOLUME,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_sector_alignment(self) -> Self {
        Self {
            bits: self.bits | Self::SECTOR_ALIGNMENT,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub const fn with_security_posture(self) -> Self {
        Self {
            bits: self.bits | Self::SECURITY_POSTURE,
        }
    }

    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }
}
