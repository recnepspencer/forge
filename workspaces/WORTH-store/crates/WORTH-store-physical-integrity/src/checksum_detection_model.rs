#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCorruptionClass {
    AccidentalPhysicalByteCorruption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCollisionPosture {
    NonCryptographicCollisionPossible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAuthenticityPosture {
    DoesNotProveAuthenticity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAuthorizationPosture {
    DoesNotProveAuthorization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChecksumDetectionModel {
    corruption_class: ChecksumCorruptionClass,
    collision_posture: ChecksumCollisionPosture,
    authenticity_posture: ChecksumAuthenticityPosture,
    authorization_posture: ChecksumAuthorizationPosture,
}

impl ChecksumDetectionModel {
    pub const fn crc32c_physical_bytes() -> Self {
        Self {
            corruption_class: ChecksumCorruptionClass::AccidentalPhysicalByteCorruption,
            collision_posture: ChecksumCollisionPosture::NonCryptographicCollisionPossible,
            authenticity_posture: ChecksumAuthenticityPosture::DoesNotProveAuthenticity,
            authorization_posture: ChecksumAuthorizationPosture::DoesNotProveAuthorization,
        }
    }

    pub const fn crc64_nvme_physical_bytes() -> Self {
        Self {
            corruption_class: ChecksumCorruptionClass::AccidentalPhysicalByteCorruption,
            collision_posture: ChecksumCollisionPosture::NonCryptographicCollisionPossible,
            authenticity_posture: ChecksumAuthenticityPosture::DoesNotProveAuthenticity,
            authorization_posture: ChecksumAuthorizationPosture::DoesNotProveAuthorization,
        }
    }

    pub const fn corruption_class(self) -> ChecksumCorruptionClass {
        self.corruption_class
    }

    pub const fn collision_posture(self) -> ChecksumCollisionPosture {
        self.collision_posture
    }

    pub const fn authenticity_posture(self) -> ChecksumAuthenticityPosture {
        self.authenticity_posture
    }

    pub const fn authorization_posture(self) -> ChecksumAuthorizationPosture {
        self.authorization_posture
    }
}
