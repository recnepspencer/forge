#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSecurityMetadataDeclarationKind {
    KeyScope,
    KeyVersionPosture,
    TenantScope,
    AuthenticityRequirement,
    AuthenticityRequirementClass,
    CustodyPosture,
    LegacyPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSecurityMetadataDeclaration {
    kind: PhysicalSecurityMetadataDeclarationKind,
}

impl PhysicalSecurityMetadataDeclaration {
    pub const fn key_scope() -> Self {
        Self::new(PhysicalSecurityMetadataDeclarationKind::KeyScope)
    }

    pub const fn key_version_posture() -> Self {
        Self::new(PhysicalSecurityMetadataDeclarationKind::KeyVersionPosture)
    }

    pub const fn tenant_scope() -> Self {
        Self::new(PhysicalSecurityMetadataDeclarationKind::TenantScope)
    }

    pub const fn authenticity_requirement() -> Self {
        Self::new(PhysicalSecurityMetadataDeclarationKind::AuthenticityRequirement)
    }

    pub const fn authenticity_requirement_class() -> Self {
        Self::new(PhysicalSecurityMetadataDeclarationKind::AuthenticityRequirementClass)
    }

    pub const fn custody_posture() -> Self {
        Self::new(PhysicalSecurityMetadataDeclarationKind::CustodyPosture)
    }

    pub const fn legacy_posture() -> Self {
        Self::new(PhysicalSecurityMetadataDeclarationKind::LegacyPosture)
    }

    pub const fn kind(self) -> PhysicalSecurityMetadataDeclarationKind {
        self.kind
    }

    const fn new(kind: PhysicalSecurityMetadataDeclarationKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSecurityMetadataResultExclusion;

impl PhysicalSecurityMetadataResultExclusion {
    pub const fn authenticity_result_is_not_metadata_declaration() -> Self {
        Self
    }
}
