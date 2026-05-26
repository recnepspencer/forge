use crate::application::{
    forge_query_checked_declaration_receipt, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceipt, ForgeQueryDeclarationReceiptChecked,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationReceiptTerminalError,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDomainEntryMarker,
};

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn receipt_routes<I>(
        &self,
        subject: ForgeQueryDeclarationReceiptInput<D, I>,
    ) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.receipt_routes_checked(subject) {
            ForgeQueryDeclarationReceiptChecked::Issued(receipt) => Ok(receipt),
            ForgeQueryDeclarationReceiptChecked::Deferred(receipt) => {
                Err(ForgeQueryDeclarationReceiptTerminalError::Deferred(receipt))
            }
            ForgeQueryDeclarationReceiptChecked::Denied(receipt) => {
                Err(ForgeQueryDeclarationReceiptTerminalError::Denied(receipt))
            }
            ForgeQueryDeclarationReceiptChecked::Failed(receipt) => {
                Err(ForgeQueryDeclarationReceiptTerminalError::Failed(receipt))
            }
        }
    }

    pub fn receipt_routes_checked<I>(
        &self,
        subject: ForgeQueryDeclarationReceiptInput<D, I>,
    ) -> ForgeQueryDeclarationReceiptChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_receipt(subject)
    }

    pub fn receipt_routes_from_progressed<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let checked = self.plan_routes_checked(
            crate::application::ForgeQueryDeclarationRoutePlanInput::admitted(
                progressed.clone(),
                self.describe_foundational(
                    crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                        progressed,
                    ),
                )
                .unwrap_or_else(|_| {
                    panic!("same-handle admitted progression should always describe foundational evidence")
                }),
            ),
        );
        self.receipt_routes(ForgeQueryDeclarationReceiptInput::route_checked(checked))
    }

    pub fn receipt_routes_from_progressed_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: ForgeQueryDeclarationRouteIntent,
    ) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let checked = self.plan_routes_checked(
            crate::application::ForgeQueryDeclarationRoutePlanInput::with_intent(
                progressed.clone(),
                self.describe_foundational(
                    crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                        progressed,
                    ),
                )
                .unwrap_or_else(|_| {
                    panic!("same-handle admitted progression should always describe foundational evidence")
                }),
                intent,
            ),
        );
        self.receipt_routes(ForgeQueryDeclarationReceiptInput::route_checked(checked))
    }

    pub fn declare_review_progress_describe_plan_and_receipt<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryDeclarationReceipt<D, I>,
        crate::application::ForgeQueryDeclarationEntryReceiptError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let progressed = self
            .declare_review_and_progress(input)
            .map_err(crate::application::ForgeQueryDeclarationEntryReceiptError::Entry)?;
        self.receipt_routes_from_progressed(progressed)
            .map_err(crate::application::ForgeQueryDeclarationEntryReceiptError::Receipt)
    }
}
