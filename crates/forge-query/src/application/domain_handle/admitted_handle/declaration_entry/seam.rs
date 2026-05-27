use crate::application::{
    forge_query_declaration_entry_crossing_inventory,
    forge_query_declaration_entry_inspection_on_handle,
    forge_query_declaration_entry_readiness_report,
    forge_query_declaration_entry_readiness_report_with_request,
    ForgeQueryDeclarationEntryContributionCompositionError,
    ForgeQueryDeclarationEntryCrossingInventory, ForgeQueryDeclarationEntryInspection,
    ForgeQueryDeclarationEntryInspectionError, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDeclarationEntryReadinessReport, ForgeQueryDeclarationEntryReadinessRequest,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn declaration_entry_crossing_inventory<I: ForgeQueryDeclarationInput<D>>(
        &self,
    ) -> ForgeQueryDeclarationEntryCrossingInventory<D, I> {
        forge_query_declaration_entry_crossing_inventory::<D, C, I>(self)
    }

    pub fn declaration_entry_readiness<I: ForgeQueryDeclarationInput<D>>(
        &self,
    ) -> ForgeQueryDeclarationEntryReadinessReport<D, I> {
        forge_query_declaration_entry_readiness_report::<D, C, I>(self)
    }

    pub fn try_declaration_entry_readiness<I: ForgeQueryDeclarationInput<D>>(
        &self,
        request: ForgeQueryDeclarationEntryReadinessRequest<D, I>,
    ) -> Result<
        ForgeQueryDeclarationEntryReadinessReport<D, I>,
        ForgeQueryDeclarationEntryContributionCompositionError<D, I>,
    > {
        forge_query_declaration_entry_readiness_report_with_request::<D, C, I>(self, request)
    }

    pub fn inspect_declaration_entry<I: ForgeQueryDeclarationInput<D>>(
        &self,
        subject: ForgeQueryDeclarationEntryInspectionInput<D, I>,
    ) -> Result<
        ForgeQueryDeclarationEntryInspection<D, I>,
        ForgeQueryDeclarationEntryInspectionError<D, I>,
    > {
        forge_query_declaration_entry_inspection_on_handle::<D, C, I>(self, subject)
    }
}
