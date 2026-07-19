use crate::application::{
    worth_query_declaration_entry_crossing_inventory,
    worth_query_declaration_entry_readiness_report, WorthQueryDeclarationEntryCrossingInventory,
    WorthQueryDeclarationEntryReadinessReport, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};
#[cfg(test)]
use crate::application::{
    worth_query_declaration_entry_inspection_on_handle,
    worth_query_declaration_entry_readiness_report_with_request,
    WorthQueryDeclarationEntryContributionCompositionError, WorthQueryDeclarationEntryInspection,
    WorthQueryDeclarationEntryInspectionError, WorthQueryDeclarationEntryInspectionInput,
    WorthQueryDeclarationEntryReadinessRequest,
};

use super::super::WorthQueryInstalledDomainDeclarationContext;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryInstalledDomainDeclarationContext<D, C>
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

    #[cfg(test)]
    pub(crate) fn try_declaration_entry_readiness<I: WorthQueryDeclarationInput<D>>(
        &self,
        request: WorthQueryDeclarationEntryReadinessRequest<D, I>,
    ) -> Result<
        WorthQueryDeclarationEntryReadinessReport<D, I>,
        WorthQueryDeclarationEntryContributionCompositionError<D, I>,
    > {
        worth_query_declaration_entry_readiness_report_with_request::<D, C, I>(self, request)
    }

    #[cfg(test)]
    pub(crate) fn inspect_declaration_entry<I: WorthQueryDeclarationInput<D>>(
        &self,
        subject: WorthQueryDeclarationEntryInspectionInput<D, I>,
    ) -> Result<
        WorthQueryDeclarationEntryInspection<D, I>,
        WorthQueryDeclarationEntryInspectionError<D, I>,
    > {
        worth_query_declaration_entry_inspection_on_handle::<D, C, I>(self, subject)
    }
}
