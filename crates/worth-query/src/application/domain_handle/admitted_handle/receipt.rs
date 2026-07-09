use crate::application::{
    checked_route_plan_from_progressed_with_profile, worth_query_checked_declaration_receipt,
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceipt, WorthQueryDeclarationReceiptChecked,
    WorthQueryDeclarationReceiptInput, WorthQueryDeclarationReceiptTerminalError,
    WorthQueryDeclarationRouteIntent, WorthQueryDomainEntryMarker,
};
use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use super::WorthQueryAdmittedConfiguredDomainHandle;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn receipt_routes<I>(
        &self,
        subject: WorthQueryDeclarationReceiptInput<D, I>,
    ) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        match self.receipt_routes_checked(subject) {
            WorthQueryDeclarationReceiptChecked::Issued(receipt) => Ok(receipt),
            WorthQueryDeclarationReceiptChecked::Deferred(receipt) => {
                Err(WorthQueryDeclarationReceiptTerminalError::Deferred(receipt))
            }
            WorthQueryDeclarationReceiptChecked::Denied(receipt) => {
                Err(WorthQueryDeclarationReceiptTerminalError::Denied(receipt))
            }
            WorthQueryDeclarationReceiptChecked::Failed(receipt) => {
                Err(WorthQueryDeclarationReceiptTerminalError::Failed(receipt))
            }
        }
    }

    pub fn receipt_routes_checked<I>(
        &self,
        subject: WorthQueryDeclarationReceiptInput<D, I>,
    ) -> WorthQueryDeclarationReceiptChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_receipt(subject)
    }

    pub fn receipt_routes_from_progressed<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        let checked = checked_route_plan_from_progressed_with_profile(
            self,
            progressed,
            None,
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        );
        self.receipt_routes(WorthQueryDeclarationReceiptInput::route_checked(checked))
    }

    pub fn receipt_routes_from_progressed_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: WorthQueryDeclarationRouteIntent,
    ) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        let checked = checked_route_plan_from_progressed_with_profile(
            self,
            progressed,
            Some(intent),
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        );
        self.receipt_routes(WorthQueryDeclarationReceiptInput::route_checked(checked))
    }

    pub fn declare_review_progress_describe_plan_and_receipt<I>(
        &self,
        input: I,
    ) -> Result<
        WorthQueryDeclarationReceipt<D, I>,
        crate::application::WorthQueryDeclarationEntryReceiptError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        let progressed = self
            .declare_review_and_progress(input)
            .map_err(crate::application::WorthQueryDeclarationEntryReceiptError::Entry)?;
        self.receipt_routes_from_progressed(progressed)
            .map_err(crate::application::WorthQueryDeclarationEntryReceiptError::Receipt)
    }
}
