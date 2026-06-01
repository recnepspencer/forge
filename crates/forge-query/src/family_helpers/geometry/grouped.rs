use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationSupportsNeighborhoodGrouping, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::grouped_authoring::{
    forge_query_grouped_declaration_checked_on_handle,
    forge_query_grouped_orchestration_checked_on_handle,
    forge_query_grouped_orchestration_proof_on_handle,
    ordinary_outcome_from_grouped_orchestration_checked, ForgeQueryGroupedContributionComposition,
    ForgeQueryGroupedContributionInput, ForgeQueryGroupedContributionStop,
    ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedDeclarationInput, ForgeQueryGroupedDeclarationStop,
    ForgeQueryGroupedEnvelopeChecked, ForgeQueryGroupedEnvelopeTranscript,
    ForgeQueryGroupedOrchestration, ForgeQueryGroupedOrchestrationChecked,
    ForgeQueryGroupedOrchestrationStop, ForgeQueryGroupedOrchestrationTranscript,
    ForgeQueryGroupedReceiptChecked, ForgeQueryGroupedReceiptTranscript,
    ForgeQueryGroupedRouteChecked, ForgeQueryGroupedRouteTranscript,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;

use super::{ForgeQueryGeometryFamilyHelpers, ForgeQueryGeometryNeighborhoodHelperFamily};

impl<'a, D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryGeometryFamilyHelpers<'a, D, C>
{
    pub fn local_neighborhood_for_active_face_selection<I>(
        &self,
        seed_member: I,
    ) -> ForgeQueryGroupedDeclarationInput<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        local_neighborhood_for_active_face_selection(seed_member)
    }

    pub fn declare_local_neighborhood_for_active_face_selection<I>(
        &self,
        input: ForgeQueryGroupedDeclarationInput<D, I>,
    ) -> Result<ForgeQueryGroupedDeclarationArtifact<D, I>, ForgeQueryGroupedDeclarationStop>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        declare_local_neighborhood_for_active_face_selection(self.handle, input)
    }

    pub fn declare_local_neighborhood_for_active_face_selection_checked<I>(
        &self,
        input: ForgeQueryGroupedDeclarationInput<D, I>,
    ) -> ForgeQueryGroupedDeclarationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        declare_local_neighborhood_for_active_face_selection_checked(self.handle, input)
    }

    pub fn orchestrate_local_neighborhood_for_active_face_selection<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> Result<ForgeQueryGroupedOrchestration<D, I>, ForgeQueryGroupedOrchestrationStop<D, I>>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        orchestrate_local_neighborhood_for_active_face_selection(self.handle, declaration)
    }

    pub fn orchestrate_local_neighborhood_for_active_face_selection_outcome<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryGroupedOrchestration<D, I>>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        orchestrate_local_neighborhood_for_active_face_selection_outcome(self.handle, declaration)
    }

    pub fn orchestrate_local_neighborhood_for_active_face_selection_checked<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        orchestrate_local_neighborhood_for_active_face_selection_checked(self.handle, declaration)
    }

    pub fn orchestrate_local_neighborhood_for_active_face_selection_proof<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        orchestrate_local_neighborhood_for_active_face_selection_proof(self.handle, declaration)
    }

    pub fn grouped_routes_for_active_face_selection_checked<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedRouteChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        self.handle.grouped_route_checked(declaration)
    }

    pub fn grouped_routes_for_active_face_selection_proof<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedRouteTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        self.handle.grouped_route_proof(declaration)
    }

    pub fn grouped_receipt_for_active_face_selection_checked<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedReceiptChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        self.handle.grouped_receipt_checked(declaration)
    }

    pub fn grouped_receipt_for_active_face_selection_proof<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedReceiptTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        self.handle.grouped_receipt_proof(declaration)
    }

    pub fn grouped_envelope_for_active_face_selection_checked<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedEnvelopeChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        self.handle.grouped_envelope_checked(declaration)
    }

    pub fn grouped_envelope_for_active_face_selection_proof<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedEnvelopeTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        self.handle.grouped_envelope_proof(declaration)
    }

    pub fn grouped_contributions_for_active_face_selection_checked<I>(
        &self,
        input: ForgeQueryGroupedContributionInput<D, I>,
    ) -> Result<
        ForgeQueryGroupedContributionComposition<D, I>,
        ForgeQueryGroupedContributionStop<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
        I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
            + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
    {
        self.handle.grouped_contributions_checked(input)
    }
}

pub(super) fn local_neighborhood_for_active_face_selection<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    seed_member: I,
) -> ForgeQueryGroupedDeclarationInput<D, I>
where
    I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
        + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    ForgeQueryGroupedDeclarationInput::local_neighborhood(seed_member)
}

pub(super) fn declare_local_neighborhood_for_active_face_selection<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: ForgeQueryGroupedDeclarationInput<D, I>,
) -> Result<ForgeQueryGroupedDeclarationArtifact<D, I>, ForgeQueryGroupedDeclarationStop>
where
    I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
        + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    match declare_local_neighborhood_for_active_face_selection_checked(handle, input) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => Ok(value),
        ForgeQueryGroupedDeclarationChecked::MemberStopped(stop) => Err(stop),
    }
}

pub(super) fn declare_local_neighborhood_for_active_face_selection_checked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: ForgeQueryGroupedDeclarationInput<D, I>,
) -> ForgeQueryGroupedDeclarationChecked<D, I>
where
    I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
        + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    forge_query_grouped_declaration_checked_on_handle(handle, input)
}

pub(super) fn orchestrate_local_neighborhood_for_active_face_selection<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> Result<ForgeQueryGroupedOrchestration<D, I>, ForgeQueryGroupedOrchestrationStop<D, I>>
where
    I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
        + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    match orchestrate_local_neighborhood_for_active_face_selection_checked(handle, declaration) {
        ForgeQueryGroupedOrchestrationChecked::Bound(value) => Ok(value),
        ForgeQueryGroupedOrchestrationChecked::MemberStopped(stop) => {
            Err(ForgeQueryGroupedOrchestrationStop::MemberStopped(stop))
        }
        ForgeQueryGroupedOrchestrationChecked::WrongWorld(stop) => {
            Err(ForgeQueryGroupedOrchestrationStop::WrongWorld(stop))
        }
        ForgeQueryGroupedOrchestrationChecked::WrongHandle(stop) => {
            Err(ForgeQueryGroupedOrchestrationStop::WrongHandle(stop))
        }
    }
}

pub(super) fn orchestrate_local_neighborhood_for_active_face_selection_outcome<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQueryGroupedOrchestration<D, I>>
where
    I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
        + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    ordinary_outcome_from_grouped_orchestration_checked(
        orchestrate_local_neighborhood_for_active_face_selection_checked(handle, declaration),
    )
}

pub(super) fn orchestrate_local_neighborhood_for_active_face_selection_checked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedOrchestrationChecked<D, I>
where
    I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
        + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    forge_query_grouped_orchestration_checked_on_handle(handle, declaration)
}

pub(super) fn orchestrate_local_neighborhood_for_active_face_selection_proof<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedOrchestrationTranscript<D, I>
where
    I::Family: ForgeQueryGeometryNeighborhoodHelperFamily<D>
        + ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    forge_query_grouped_orchestration_proof_on_handle(handle, declaration)
}
