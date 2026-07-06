#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSecurityMetadataDenial {
    MissingPlatformSecurityMetadata,
    UnsupportedPlatformSecurityMetadata,
    UnavailablePlatformSecurityMetadata,
    LegacyReadmissionRequired,
    AuthenticityResultCannotBePhysicalMetadata,
}
