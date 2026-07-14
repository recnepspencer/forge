#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamilyDenial {
    AuthenticityBoundaryDenied,
    ConcreteKeyKindDoesNotMatchPhysicalKeyDomain,
    CourtroomCannotMintAuthority,
    TerminalProjectionCannotMintAuthority,
    CrossKeyScopePartitionDenied,
    CrossTenantScopePartitionDenied,
    CustodyBoundaryDenied,
    HashIdentityRequiresCollisionVerification,
    InexactFamilyCannotSatisfyExactClaim,
    MissingFamilyDeclaration,
    IncompleteProjectionPosture,
    DerivedFamilyCannotMintProductionAuthority,
    DiagnosticFamilyCannotMintProductionAuthority,
    PhysicalKeyDomainNotDeclaredForFamily,
    PhysicalKeyDomainDoesNotSupportPrefixBounds,
    PhysicalKeyDomainDoesNotSupportRangeBounds,
    ReadmissionFamilyCannotEnterStrategyAdmission,
    SecurityAuthorityMismatch,
    VerifierLaneCannotEnterStrategyAdmission,
    TransferBoundaryFamilyCannotEnterStrategyAdmission,
    OfflineImportOnlyFamilyCannotEnterStrategyAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArtifactFamilyDenialKind {
    AuthenticityBoundary,
    ConcreteKeyKindDoesNotMatchPhysicalKeyDomain,
    CourtroomCannotMintAuthority,
    TerminalProjectionCannotMintAuthority,
    CrossKeyScopePartition,
    CrossTenantScopePartition,
    CustodyBoundary,
    HashIdentityRequiresCollisionVerification,
    InexactFamilyCannotSatisfyExactClaim,
    MissingFamilyDeclaration,
    IncompleteProjectionPosture,
    DerivedFamilyCannotMintProductionAuthority,
    DiagnosticFamilyCannotMintProductionAuthority,
    PhysicalKeyDomainNotDeclaredForFamily,
    PhysicalKeyDomainDoesNotSupportPrefixBounds,
    PhysicalKeyDomainDoesNotSupportRangeBounds,
    ReadmissionFamilyCannotEnterStrategyAdmission,
    SecurityAuthorityMismatch,
    VerifierLaneCannotEnterStrategyAdmission,
    TransferBoundaryFamilyCannotEnterStrategyAdmission,
    OfflineImportOnlyFamilyCannotEnterStrategyAdmission,
}

impl ArtifactFamilyDenial {
    pub const fn kind(self) -> ArtifactFamilyDenialKind {
        match self {
            Self::AuthenticityBoundaryDenied => ArtifactFamilyDenialKind::AuthenticityBoundary,
            Self::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain => {
                ArtifactFamilyDenialKind::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain
            }
            Self::CourtroomCannotMintAuthority => {
                ArtifactFamilyDenialKind::CourtroomCannotMintAuthority
            }
            Self::TerminalProjectionCannotMintAuthority => {
                ArtifactFamilyDenialKind::TerminalProjectionCannotMintAuthority
            }
            Self::CrossKeyScopePartitionDenied => ArtifactFamilyDenialKind::CrossKeyScopePartition,
            Self::CrossTenantScopePartitionDenied => {
                ArtifactFamilyDenialKind::CrossTenantScopePartition
            }
            Self::CustodyBoundaryDenied => ArtifactFamilyDenialKind::CustodyBoundary,
            Self::HashIdentityRequiresCollisionVerification => {
                ArtifactFamilyDenialKind::HashIdentityRequiresCollisionVerification
            }
            Self::InexactFamilyCannotSatisfyExactClaim => {
                ArtifactFamilyDenialKind::InexactFamilyCannotSatisfyExactClaim
            }
            Self::MissingFamilyDeclaration => ArtifactFamilyDenialKind::MissingFamilyDeclaration,
            Self::IncompleteProjectionPosture => {
                ArtifactFamilyDenialKind::IncompleteProjectionPosture
            }
            Self::DerivedFamilyCannotMintProductionAuthority => {
                ArtifactFamilyDenialKind::DerivedFamilyCannotMintProductionAuthority
            }
            Self::DiagnosticFamilyCannotMintProductionAuthority => {
                ArtifactFamilyDenialKind::DiagnosticFamilyCannotMintProductionAuthority
            }
            Self::PhysicalKeyDomainNotDeclaredForFamily => {
                ArtifactFamilyDenialKind::PhysicalKeyDomainNotDeclaredForFamily
            }
            Self::PhysicalKeyDomainDoesNotSupportPrefixBounds => {
                ArtifactFamilyDenialKind::PhysicalKeyDomainDoesNotSupportPrefixBounds
            }
            Self::PhysicalKeyDomainDoesNotSupportRangeBounds => {
                ArtifactFamilyDenialKind::PhysicalKeyDomainDoesNotSupportRangeBounds
            }
            Self::ReadmissionFamilyCannotEnterStrategyAdmission => {
                ArtifactFamilyDenialKind::ReadmissionFamilyCannotEnterStrategyAdmission
            }
            Self::SecurityAuthorityMismatch => ArtifactFamilyDenialKind::SecurityAuthorityMismatch,
            Self::VerifierLaneCannotEnterStrategyAdmission => {
                ArtifactFamilyDenialKind::VerifierLaneCannotEnterStrategyAdmission
            }
            Self::TransferBoundaryFamilyCannotEnterStrategyAdmission => {
                ArtifactFamilyDenialKind::TransferBoundaryFamilyCannotEnterStrategyAdmission
            }
            Self::OfflineImportOnlyFamilyCannotEnterStrategyAdmission => {
                ArtifactFamilyDenialKind::OfflineImportOnlyFamilyCannotEnterStrategyAdmission
            }
        }
    }
}
