use super::error::ConflictBatchAdmissionInventoryError;
use super::row::{
    ConflictBatchAdmissionAuthorityKind, ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionCostPosture, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionOwner,
    ConflictBatchAdmissionQuerySurface, ConflictBatchAdmissionReplacementPhase,
    ConflictBatchAdmissionRowScope, ConflictBatchAdmissionSurfaceIdentity,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ConflictBatchAdmissionInventoryRowBuilder {
    surface_identity: Option<ConflictBatchAdmissionSurfaceIdentity>,
    source_path: Option<String>,
    surface_name: Option<String>,
    owner: Option<ConflictBatchAdmissionOwner>,
    current_caller: Option<String>,
    authority_kind: Option<ConflictBatchAdmissionAuthorityKind>,
    disposition: Option<ConflictBatchAdmissionDisposition>,
    replacement_phase: Option<ConflictBatchAdmissionReplacementPhase>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
    certification_posture: Option<ConflictBatchAdmissionCertificationPosture>,
    cost_posture: Option<ConflictBatchAdmissionCostPosture>,
    query_surface: Option<ConflictBatchAdmissionQuerySurface>,
    row_scope: Option<ConflictBatchAdmissionRowScope>,
}

impl ConflictBatchAdmissionInventoryRowBuilder {
    pub(crate) const fn surface_identity(
        mut self,
        value: ConflictBatchAdmissionSurfaceIdentity,
    ) -> Self {
        self.surface_identity = Some(value);
        self
    }

    pub(crate) fn source_path(mut self, value: impl Into<String>) -> Self {
        self.source_path = Some(value.into());
        self
    }

    pub(crate) fn surface_name(mut self, value: impl Into<String>) -> Self {
        self.surface_name = Some(value.into());
        self
    }

    pub(crate) const fn owner(mut self, value: ConflictBatchAdmissionOwner) -> Self {
        self.owner = Some(value);
        self
    }

    pub(crate) fn current_caller(mut self, value: impl Into<String>) -> Self {
        self.current_caller = Some(value.into());
        self
    }

    pub(crate) const fn authority_kind(
        mut self,
        value: ConflictBatchAdmissionAuthorityKind,
    ) -> Self {
        self.authority_kind = Some(value);
        self
    }

    pub(crate) const fn disposition(mut self, value: ConflictBatchAdmissionDisposition) -> Self {
        self.disposition = Some(value);
        self
    }

    pub(crate) const fn replacement_phase(
        mut self,
        value: ConflictBatchAdmissionReplacementPhase,
    ) -> Self {
        self.replacement_phase = Some(value);
        self
    }

    pub(crate) fn blocker(mut self, value: impl Into<String>) -> Self {
        self.blocker = Some(value.into());
        self
    }

    pub(crate) fn removal_trigger(mut self, value: impl Into<String>) -> Self {
        self.removal_trigger = Some(value.into());
        self
    }

    pub(crate) const fn certification_posture(
        mut self,
        value: ConflictBatchAdmissionCertificationPosture,
    ) -> Self {
        self.certification_posture = Some(value);
        self
    }

    pub(crate) const fn cost_posture(mut self, value: ConflictBatchAdmissionCostPosture) -> Self {
        self.cost_posture = Some(value);
        self
    }

    pub(crate) const fn query_surface(mut self, value: ConflictBatchAdmissionQuerySurface) -> Self {
        self.query_surface = Some(value);
        self
    }

    pub(crate) const fn row_scope(mut self, value: ConflictBatchAdmissionRowScope) -> Self {
        self.row_scope = Some(value);
        self
    }

    pub(crate) fn build(
        self,
    ) -> Result<ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionInventoryError> {
        let surface_identity = self
            .surface_identity
            .ok_or(ConflictBatchAdmissionInventoryError::MissingSurfaceIdentity)?;
        let source_path = non_empty(
            self.source_path,
            ConflictBatchAdmissionInventoryError::MissingSourcePath,
        )?;
        let surface_name = non_empty(
            self.surface_name,
            ConflictBatchAdmissionInventoryError::MissingSurfaceName,
        )?;
        let owner = self
            .owner
            .ok_or(ConflictBatchAdmissionInventoryError::MissingOwner)?;
        let current_caller = non_empty(
            self.current_caller,
            ConflictBatchAdmissionInventoryError::MissingCurrentCaller,
        )?;
        let authority_kind = self
            .authority_kind
            .ok_or(ConflictBatchAdmissionInventoryError::MissingAuthorityKind)?;
        let disposition = self
            .disposition
            .ok_or(ConflictBatchAdmissionInventoryError::MissingDisposition)?;
        let replacement_phase = self
            .replacement_phase
            .ok_or(ConflictBatchAdmissionInventoryError::MissingReplacementPhase)?;
        let blocker = non_empty(
            self.blocker,
            ConflictBatchAdmissionInventoryError::MissingBlocker,
        )?;
        let removal_trigger = non_empty(
            self.removal_trigger,
            ConflictBatchAdmissionInventoryError::MissingRemovalTrigger,
        )?;
        let certification_posture = self
            .certification_posture
            .ok_or(ConflictBatchAdmissionInventoryError::MissingCertificationPosture)?;
        let cost_posture = self
            .cost_posture
            .ok_or(ConflictBatchAdmissionInventoryError::MissingCostPosture)?;
        let query_surface = self
            .query_surface
            .ok_or(ConflictBatchAdmissionInventoryError::MissingQuerySurface)?;
        let row_scope = self
            .row_scope
            .ok_or(ConflictBatchAdmissionInventoryError::MissingRowScope)?;

        validate_row_contract(
            surface_identity,
            authority_kind,
            disposition,
            certification_posture,
            query_surface,
        )?;

        Ok(ConflictBatchAdmissionInventoryRow {
            surface_identity,
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
    surface_identity: ConflictBatchAdmissionSurfaceIdentity,
    authority_kind: ConflictBatchAdmissionAuthorityKind,
    disposition: ConflictBatchAdmissionDisposition,
    certification_posture: ConflictBatchAdmissionCertificationPosture,
    query_surface: ConflictBatchAdmissionQuerySurface,
) -> Result<(), ConflictBatchAdmissionInventoryError> {
    let query_kind =
        authority_kind == ConflictBatchAdmissionAuthorityKind::QuerySupportProofSurface;
    if query_kind && query_surface == ConflictBatchAdmissionQuerySurface::NotQuery {
        return Err(ConflictBatchAdmissionInventoryError::QuerySurfaceRequired(
            surface_identity,
        ));
    }
    if !query_kind && query_surface != ConflictBatchAdmissionQuerySurface::NotQuery {
        return Err(
            ConflictBatchAdmissionInventoryError::QuerySurfaceCannotMintAuthority(surface_identity),
        );
    }
    if disposition == ConflictBatchAdmissionDisposition::CertificationOnly
        && certification_posture
            == ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable
    {
        return Err(
            ConflictBatchAdmissionInventoryError::CertificationOnlyOrdinaryReachable(
                surface_identity,
            ),
        );
    }
    if disposition == ConflictBatchAdmissionDisposition::Cap
        && certification_posture
            != ConflictBatchAdmissionCertificationPosture::NonOrdinaryResidueDeniedAsOrdinaryProof
    {
        return Err(
            ConflictBatchAdmissionInventoryError::CappedResidueWithoutResiduePosture(
                surface_identity,
            ),
        );
    }
    if disposition == ConflictBatchAdmissionDisposition::QueryGap
        && certification_posture
            != ConflictBatchAdmissionCertificationPosture::QuerySupportOnlyCannotMintConflictAuthority
    {
        return Err(
            ConflictBatchAdmissionInventoryError::QuerySurfaceCannotMintAuthority(
                surface_identity,
            ),
        );
    }
    Ok(())
}

fn non_empty(
    value: Option<String>,
    error: ConflictBatchAdmissionInventoryError,
) -> Result<String, ConflictBatchAdmissionInventoryError> {
    let value = value.ok_or(error.clone())?;
    if value.trim().is_empty() {
        return Err(error);
    }
    Ok(value)
}
