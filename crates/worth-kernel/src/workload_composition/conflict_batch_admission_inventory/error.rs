use super::row::ConflictBatchAdmissionSurfaceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConflictBatchAdmissionInventoryError {
    MissingSurfaceIdentity,
    MissingSourcePath,
    MissingSurfaceName,
    MissingOwner,
    MissingCurrentCaller,
    MissingAuthorityKind,
    MissingDisposition,
    MissingReplacementPhase,
    MissingBlocker,
    MissingRemovalTrigger,
    MissingCertificationPosture,
    MissingCostPosture,
    MissingQuerySurface,
    MissingRowScope,
    DuplicateSurface(ConflictBatchAdmissionSurfaceIdentity),
    MissingRequiredSurface(ConflictBatchAdmissionSurfaceIdentity),
    QuerySurfaceRequired(ConflictBatchAdmissionSurfaceIdentity),
    QuerySurfaceCannotMintAuthority(ConflictBatchAdmissionSurfaceIdentity),
    CertificationOnlyOrdinaryReachable(ConflictBatchAdmissionSurfaceIdentity),
    CappedResidueWithoutResiduePosture(ConflictBatchAdmissionSurfaceIdentity),
    UnclassifiedDiscoveredSurface(String),
    SourceFirewallViolation(String),
}
