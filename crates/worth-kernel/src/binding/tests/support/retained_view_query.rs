use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationInput, ForgeQueryDomainOperatingContext, LowerRuntimeBasisEvidence,
    ScopedInspectionBasis,
};
use worth_spatial::facade::bindings::{
    primitive_rebinding_retained_fact_source, PrimitiveRebindingDeclarationEntry,
    PrimitiveRebindingQueryDomain,
};
use worth_spatial::facade::inspection::{
    branch_local_geometry_inspection_entry, geometry_replay_parity_entry,
    historical_geometry_inspection_entry, primitive_rebinding_retained_subject,
    PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingBranchLocalInspectionError,
    PrimitiveRebindingHistoricalInspection, PrimitiveRebindingHistoricalInspectionError,
    PrimitiveRebindingReplayParity, PrimitiveRebindingReplayParityError,
    PrimitiveRebindingReplaySource,
};

pub(crate) trait PrimitiveRebindingKernelQueryExt {
    fn historical_inspection_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        subject: ForgeQueryDeclarationEnvelopeChecked<PrimitiveRebindingQueryDomain, Self>,
    ) -> Result<PrimitiveRebindingHistoricalInspection, PrimitiveRebindingHistoricalInspectionError>
    where
        Self: Sized + Clone + ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain>,
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>;

    fn branch_local_inspection_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        scoped_basis: &ScopedInspectionBasis,
        branch_basis_evidence: LowerRuntimeBasisEvidence,
        subject: ForgeQueryDeclarationEnvelopeChecked<PrimitiveRebindingQueryDomain, Self>,
    ) -> Result<PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingBranchLocalInspectionError>
    where
        Self: Sized + Clone + ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain>,
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>;

    fn replay_parity_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        replay_source: PrimitiveRebindingReplaySource,
        other_source: PrimitiveRebindingReplaySource,
    ) -> Result<PrimitiveRebindingReplayParity, PrimitiveRebindingReplayParityError>
    where
        Self: Sized,
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>;
}

impl PrimitiveRebindingKernelQueryExt for PrimitiveRebindingDeclarationEntry {
    fn historical_inspection_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        subject: ForgeQueryDeclarationEnvelopeChecked<PrimitiveRebindingQueryDomain, Self>,
    ) -> Result<PrimitiveRebindingHistoricalInspection, PrimitiveRebindingHistoricalInspectionError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        let source = primitive_rebinding_retained_fact_source(self, handle)
            .map_err(PrimitiveRebindingHistoricalInspectionError::RetainedFactSource)?;
        let retained_subject = primitive_rebinding_retained_subject(self.binding_kind(), &subject);
        historical_geometry_inspection_entry(source, retained_subject)
            .inspect_checked(handle, subject)
    }

    fn branch_local_inspection_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        scoped_basis: &ScopedInspectionBasis,
        branch_basis_evidence: LowerRuntimeBasisEvidence,
        subject: ForgeQueryDeclarationEnvelopeChecked<PrimitiveRebindingQueryDomain, Self>,
    ) -> Result<PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingBranchLocalInspectionError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        let source = primitive_rebinding_retained_fact_source(self, handle)
            .map_err(PrimitiveRebindingBranchLocalInspectionError::RetainedFactSource)?;
        let retained_subject = primitive_rebinding_retained_subject(self.binding_kind(), &subject);
        branch_local_geometry_inspection_entry(
            source,
            scoped_basis.clone(),
            branch_basis_evidence,
            retained_subject,
        )
        .inspect_checked(handle, subject)
    }

    fn replay_parity_with_query<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
        replay_source: PrimitiveRebindingReplaySource,
        other_source: PrimitiveRebindingReplaySource,
    ) -> Result<PrimitiveRebindingReplayParity, PrimitiveRebindingReplayParityError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        geometry_replay_parity_entry(replay_source, other_source).compare(handle)
    }
}
