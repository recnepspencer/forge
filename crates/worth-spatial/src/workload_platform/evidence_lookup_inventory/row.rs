use super::error::{EvidenceLookupInventoryError, EvidenceLookupInventoryErrorKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupAuthorityKind {
    RawEvidenceVectorAccess,
    BroadReceiptScan,
    StageLocalNearbyLookup,
    CopiedDigestSearch,
    PublicEvidenceRowExposure,
    BooleanStageLookupHelper,
    CompatibilityWrapper,
    QueryLookingLocalProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupDisposition {
    Migrate,
    Delete,
    Cap,
    CertificationOnly,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupOwner {
    WorthSpatial,
    WorthKernel,
    WorthTopo,
    ForgeQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupReplacementPhase {
    PhaseTwoFamilyCatalog,
    PhaseThreeAdmission,
    PhaseFourSelection,
    PhaseEightSweep,
    PhaseTenConsumerKit,
    PhaseFourteenDeletion,
    NotReplacedCertificationOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupQuerySurface {
    NotQuery,
    SupportAdmission,
    SupportPinning,
    ProjectionConsumption,
    LowerRuntimeBoundaryEnvelope,
    TypedArtifactIdentity,
    ConsumerKitProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupInventoryRowScope {
    ConcreteSource,
    FamilySummary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupCertificationPosture {
    OrdinaryProductionReachable,
    CertificationOnlyDeniedAsOrdinaryProof,
    TestFixtureDeniedAsOrdinaryProof,
    NonOrdinaryResidueDeniedAsOrdinaryProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupCostPosture {
    LocalTypedLookup,
    BroadEvidenceLedgerScan,
    BroadReceiptLedgerScan,
    SourceInventoryOnly,
    PublicFacadeExposure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupInventoryRow {
    source_path: String,
    surface_name: String,
    owner: EvidenceLookupOwner,
    current_caller: String,
    authority_kind: EvidenceLookupAuthorityKind,
    disposition: EvidenceLookupDisposition,
    replacement_phase: EvidenceLookupReplacementPhase,
    blocker: String,
    removal_trigger: String,
    certification_posture: EvidenceLookupCertificationPosture,
    cost_posture: EvidenceLookupCostPosture,
    query_surface: EvidenceLookupQuerySurface,
    row_scope: EvidenceLookupInventoryRowScope,
}

impl EvidenceLookupInventoryRow {
    pub(crate) fn builder() -> EvidenceLookupInventoryRowBuilder {
        EvidenceLookupInventoryRowBuilder::default()
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub const fn owner(&self) -> EvidenceLookupOwner {
        self.owner
    }

    pub fn current_caller(&self) -> &str {
        &self.current_caller
    }

    pub const fn authority_kind(&self) -> EvidenceLookupAuthorityKind {
        self.authority_kind
    }

    pub const fn disposition(&self) -> EvidenceLookupDisposition {
        self.disposition
    }

    pub const fn replacement_phase(&self) -> EvidenceLookupReplacementPhase {
        self.replacement_phase
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn certification_posture(&self) -> EvidenceLookupCertificationPosture {
        self.certification_posture
    }

    pub const fn cost_posture(&self) -> EvidenceLookupCostPosture {
        self.cost_posture
    }

    pub const fn query_surface(&self) -> EvidenceLookupQuerySurface {
        self.query_surface
    }

    pub const fn row_scope(&self) -> EvidenceLookupInventoryRowScope {
        self.row_scope
    }

    pub const fn claims_lookup_execution_authority(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct EvidenceLookupInventoryRowBuilder {
    source_path: Option<String>,
    surface_name: Option<String>,
    owner: Option<EvidenceLookupOwner>,
    current_caller: Option<String>,
    authority_kind: Option<EvidenceLookupAuthorityKind>,
    disposition: Option<EvidenceLookupDisposition>,
    replacement_phase: Option<EvidenceLookupReplacementPhase>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
    certification_posture: Option<EvidenceLookupCertificationPosture>,
    cost_posture: Option<EvidenceLookupCostPosture>,
    query_surface: Option<EvidenceLookupQuerySurface>,
    row_scope: Option<EvidenceLookupInventoryRowScope>,
}

impl EvidenceLookupInventoryRowBuilder {
    pub fn source_path(mut self, value: impl Into<String>) -> Self {
        self.source_path = Some(value.into());
        self
    }

    pub fn surface_name(mut self, value: impl Into<String>) -> Self {
        self.surface_name = Some(value.into());
        self
    }

    pub const fn owner(mut self, value: EvidenceLookupOwner) -> Self {
        self.owner = Some(value);
        self
    }

    pub fn current_caller(mut self, value: impl Into<String>) -> Self {
        self.current_caller = Some(value.into());
        self
    }

    pub const fn authority_kind(mut self, value: EvidenceLookupAuthorityKind) -> Self {
        self.authority_kind = Some(value);
        self
    }

    pub const fn disposition(mut self, value: EvidenceLookupDisposition) -> Self {
        self.disposition = Some(value);
        self
    }

    pub const fn replacement_phase(mut self, value: EvidenceLookupReplacementPhase) -> Self {
        self.replacement_phase = Some(value);
        self
    }

    pub fn blocker(mut self, value: impl Into<String>) -> Self {
        self.blocker = Some(value.into());
        self
    }

    pub fn removal_trigger(mut self, value: impl Into<String>) -> Self {
        self.removal_trigger = Some(value.into());
        self
    }

    pub const fn certification_posture(
        mut self,
        value: EvidenceLookupCertificationPosture,
    ) -> Self {
        self.certification_posture = Some(value);
        self
    }

    pub const fn cost_posture(mut self, value: EvidenceLookupCostPosture) -> Self {
        self.cost_posture = Some(value);
        self
    }

    pub const fn query_surface(mut self, value: EvidenceLookupQuerySurface) -> Self {
        self.query_surface = Some(value);
        self
    }

    pub const fn row_scope(mut self, value: EvidenceLookupInventoryRowScope) -> Self {
        self.row_scope = Some(value);
        self
    }

    pub fn build(self) -> Result<EvidenceLookupInventoryRow, EvidenceLookupInventoryError> {
        let source_path = require_non_empty(
            self.source_path,
            EvidenceLookupInventoryErrorKind::MissingSourcePath,
        )?;
        let surface_name = require_non_empty(
            self.surface_name,
            EvidenceLookupInventoryErrorKind::MissingSurfaceName,
        )?;
        let owner = self
            .owner
            .ok_or_else(|| error(EvidenceLookupInventoryErrorKind::MissingOwner))?;
        let current_caller = require_non_empty(
            self.current_caller,
            EvidenceLookupInventoryErrorKind::MissingCurrentCaller,
        )?;
        let authority_kind = self
            .authority_kind
            .ok_or_else(|| error(EvidenceLookupInventoryErrorKind::MissingAuthorityKind))?;
        let disposition = self
            .disposition
            .ok_or_else(|| error(EvidenceLookupInventoryErrorKind::MissingDisposition))?;
        let replacement_phase = self
            .replacement_phase
            .ok_or_else(|| error(EvidenceLookupInventoryErrorKind::MissingReplacementPhase))?;
        let blocker = require_non_empty(
            self.blocker,
            EvidenceLookupInventoryErrorKind::MissingBlocker,
        )?;
        let removal_trigger = require_non_empty(
            self.removal_trigger,
            EvidenceLookupInventoryErrorKind::MissingRemovalTrigger,
        )?;
        let certification_posture = self
            .certification_posture
            .ok_or_else(|| error(EvidenceLookupInventoryErrorKind::MissingCertificationPosture))?;
        let cost_posture = self
            .cost_posture
            .ok_or_else(|| error(EvidenceLookupInventoryErrorKind::MissingCostPosture))?;
        let query_surface = self
            .query_surface
            .ok_or_else(|| error(EvidenceLookupInventoryErrorKind::MissingQuerySurface))?;
        let row_scope = self
            .row_scope
            .ok_or_else(|| error(EvidenceLookupInventoryErrorKind::MissingRowScope))?;

        validate_row_contract(
            authority_kind,
            disposition,
            certification_posture,
            query_surface,
        )?;

        Ok(EvidenceLookupInventoryRow {
            source_path,
            surface_name,
            owner,
            current_caller,
            authority_kind,
            disposition,
            replacement_phase,
            blocker,
            removal_trigger,
            certification_posture,
            cost_posture,
            query_surface,
            row_scope,
        })
    }
}

fn validate_row_contract(
    authority_kind: EvidenceLookupAuthorityKind,
    disposition: EvidenceLookupDisposition,
    certification_posture: EvidenceLookupCertificationPosture,
    query_surface: EvidenceLookupQuerySurface,
) -> Result<(), EvidenceLookupInventoryError> {
    validate_query_surface(authority_kind, query_surface)?;
    if disposition == EvidenceLookupDisposition::CertificationOnly
        && certification_posture == EvidenceLookupCertificationPosture::OrdinaryProductionReachable
    {
        return Err(error(
            EvidenceLookupInventoryErrorKind::CertificationOnlyRequiresDenialPosture,
        ));
    }
    if disposition == EvidenceLookupDisposition::Cap
        && certification_posture
            != EvidenceLookupCertificationPosture::NonOrdinaryResidueDeniedAsOrdinaryProof
    {
        return Err(error(
            EvidenceLookupInventoryErrorKind::CappedResidueRequiresBlocker,
        ));
    }
    Ok(())
}

fn validate_query_surface(
    authority_kind: EvidenceLookupAuthorityKind,
    query_surface: EvidenceLookupQuerySurface,
) -> Result<(), EvidenceLookupInventoryError> {
    if authority_kind == EvidenceLookupAuthorityKind::QueryLookingLocalProof
        && query_surface == EvidenceLookupQuerySurface::NotQuery
    {
        return Err(error(
            EvidenceLookupInventoryErrorKind::QuerySurfaceRequired,
        ));
    }
    if authority_kind != EvidenceLookupAuthorityKind::QueryLookingLocalProof
        && query_surface != EvidenceLookupQuerySurface::NotQuery
    {
        return Err(error(
            EvidenceLookupInventoryErrorKind::QuerySurfaceCannotMintLookupAuthority,
        ));
    }
    Ok(())
}

fn require_non_empty(
    value: Option<String>,
    kind: EvidenceLookupInventoryErrorKind,
) -> Result<String, EvidenceLookupInventoryError> {
    let value = value.ok_or_else(|| error(kind))?;
    if value.is_empty() {
        return Err(error(kind));
    }
    Ok(value)
}

const fn error(kind: EvidenceLookupInventoryErrorKind) -> EvidenceLookupInventoryError {
    EvidenceLookupInventoryError::new(kind)
}
