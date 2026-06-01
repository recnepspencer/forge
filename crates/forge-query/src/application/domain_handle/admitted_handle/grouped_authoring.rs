use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::grouped_authoring::{
    forge_query_grouped_contribution_checked_on_handle,
    forge_query_grouped_declaration_checked_on_handle,
    forge_query_grouped_envelope_checked_on_handle, forge_query_grouped_envelope_proof_on_handle,
    forge_query_grouped_orchestration_checked_on_handle,
    forge_query_grouped_orchestration_proof_on_handle,
    forge_query_grouped_receipt_checked_on_handle, forge_query_grouped_receipt_proof_on_handle,
    forge_query_grouped_route_checked_on_handle, forge_query_grouped_route_proof_on_handle,
    forge_query_grouped_support_report, ordinary_outcome_from_grouped_orchestration_checked,
    ForgeQueryGroupedContributionComposition, ForgeQueryGroupedContributionInput,
    ForgeQueryGroupedContributionStop, ForgeQueryGroupedDeclarationArtifact,
    ForgeQueryGroupedDeclarationChecked, ForgeQueryGroupedDeclarationInput,
    ForgeQueryGroupedDeclarationStop, ForgeQueryGroupedEnvelopeChecked,
    ForgeQueryGroupedEnvelopeTranscript, ForgeQueryGroupedOrchestration,
    ForgeQueryGroupedOrchestrationChecked, ForgeQueryGroupedOrchestrationStop,
    ForgeQueryGroupedOrchestrationTranscript, ForgeQueryGroupedReceiptChecked,
    ForgeQueryGroupedReceiptTranscript, ForgeQueryGroupedRouteChecked,
    ForgeQueryGroupedRouteTranscript, ForgeQueryGroupedSupportReport,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn declare_grouped_checked<I>(
        &self,
        input: ForgeQueryGroupedDeclarationInput<D, I>,
    ) -> ForgeQueryGroupedDeclarationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_declaration_checked_on_handle(self, input)
    }

    pub fn declare_grouped<I>(
        &self,
        input: ForgeQueryGroupedDeclarationInput<D, I>,
    ) -> Result<ForgeQueryGroupedDeclarationArtifact<D, I>, ForgeQueryGroupedDeclarationStop>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        match self.declare_grouped_checked(input) {
            ForgeQueryGroupedDeclarationChecked::Bound(value) => Ok(value),
            ForgeQueryGroupedDeclarationChecked::MemberStopped(stop) => Err(stop),
        }
    }

    pub fn orchestrate_grouped_checked<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_orchestration_checked_on_handle(self, declaration)
    }

    pub fn orchestrate_grouped<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> Result<ForgeQueryGroupedOrchestration<D, I>, ForgeQueryGroupedOrchestrationStop<D, I>>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        match self.orchestrate_grouped_checked(declaration) {
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

    pub fn orchestrate_grouped_outcome<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryGroupedOrchestration<D, I>>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        ordinary_outcome_from_grouped_orchestration_checked(
            self.orchestrate_grouped_checked(declaration),
        )
    }

    pub fn orchestrate_grouped_proof<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_orchestration_proof_on_handle(self, declaration)
    }

    pub fn grouped_route_checked<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedRouteChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_route_checked_on_handle(self, declaration)
    }

    pub fn grouped_route_proof<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedRouteTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_route_proof_on_handle(self, declaration)
    }

    pub fn grouped_receipt_checked<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedReceiptChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_receipt_checked_on_handle(self, declaration)
    }

    pub fn grouped_receipt_proof<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedReceiptTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_receipt_proof_on_handle(self, declaration)
    }

    pub fn grouped_envelope_checked<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedEnvelopeChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_envelope_checked_on_handle(self, declaration)
    }

    pub fn grouped_envelope_proof<I>(
        &self,
        declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedEnvelopeTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_envelope_proof_on_handle(self, declaration)
    }

    pub fn grouped_contributions_checked<I>(
        &self,
        input: ForgeQueryGroupedContributionInput<D, I>,
    ) -> Result<
        ForgeQueryGroupedContributionComposition<D, I>,
        ForgeQueryGroupedContributionStop<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D> + Clone,
    {
        forge_query_grouped_contribution_checked_on_handle(self, input)
    }

    pub fn grouped_support_report<I>(
        &self,
        declaration: &ForgeQueryGroupedDeclarationArtifact<D, I>,
    ) -> ForgeQueryGroupedSupportReport
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let _ = self;
        forge_query_grouped_support_report(declaration)
    }
}
