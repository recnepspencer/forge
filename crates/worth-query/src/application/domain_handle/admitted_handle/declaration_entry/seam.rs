use crate::application::{
    worth_query_declaration_entry_crossing_inventory,
    worth_query_declaration_entry_inspection_on_handle,
    worth_query_declaration_entry_readiness_report,
    worth_query_declaration_entry_readiness_report_with_request,
    WorthQueryDeclarationEntryContributionCompositionError,
    WorthQueryDeclarationEntryCrossingInventory, WorthQueryDeclarationEntryInspection,
    WorthQueryDeclarationEntryInspectionError, WorthQueryDeclarationEntryInspectionInput,
    WorthQueryDeclarationEntryReadinessReport, WorthQueryDeclarationEntryReadinessRequest,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::super::WorthQueryAdmittedConfiguredDomainHandle;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn declaration_entry_crossing_inventory<I: WorthQueryDeclarationInput<D>>(
        &self,
    ) -> WorthQueryDeclarationEntryCrossingInventory<D, I> {
        worth_query_declaration_entry_crossing_inventory::<D, C, I>(self)
    }

    pub fn declaration_entry_readiness<I: WorthQueryDeclarationInput<D>>(
        &self,
    ) -> WorthQueryDeclarationEntryReadinessReport<D, I> {
        worth_query_declaration_entry_readiness_report::<D, C, I>(self)
    }

    pub fn try_declaration_entry_readiness<I: WorthQueryDeclarationInput<D>>(
        &self,
        request: WorthQueryDeclarationEntryReadinessRequest<D, I>,
    ) -> Result<
        WorthQueryDeclarationEntryReadinessReport<D, I>,
        WorthQueryDeclarationEntryContributionCompositionError<D, I>,
    > {
        worth_query_declaration_entry_readiness_report_with_request::<D, C, I>(self, request)
    }

    pub fn inspect_declaration_entry<I: WorthQueryDeclarationInput<D>>(
        &self,
        subject: WorthQueryDeclarationEntryInspectionInput<D, I>,
    ) -> Result<
        WorthQueryDeclarationEntryInspection<D, I>,
        WorthQueryDeclarationEntryInspectionError<D, I>,
    > {
        worth_query_declaration_entry_inspection_on_handle::<D, C, I>(self, subject)
    }
}
