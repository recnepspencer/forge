use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationProduct, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceipt, ForgeQueryDeclarationReceiptChecked,
    ForgeQueryDeclarationReceiptTerminalError, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

use super::common::{receipt_orchestration_identity, receipt_terminal_from_checked};
use super::transcript::ForgeQueryDeclarationReceiptOrchestrationTranscript;
use crate::application::{
    forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle,
    ForgeQueryDeclarationEntryProductChecked,
};

pub(crate) fn forge_query_declaration_receipt_orchestration_from_progressed_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>> {
    match forge_query_checked_declaration_receipt_orchestration_from_progressed_on_handle(
        handle,
        progressed,
        route_intent,
    ) {
        ForgeQueryDeclarationReceiptChecked::Issued(receipt) => Ok(receipt),
        other => Err(receipt_terminal_from_checked(other)),
    }
}

pub(crate) fn forge_query_checked_declaration_receipt_orchestration_from_progressed_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> ForgeQueryDeclarationReceiptChecked<D, I> {
    match forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
        handle,
        progressed,
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
        route_intent,
    )
    .checked
    {
        ForgeQueryDeclarationEntryProductChecked::Receipt(checked) => checked,
        _ => panic!("receipt orchestration must project the receipt product"),
    }
}

pub(crate) fn forge_query_declaration_receipt_orchestration_from_progressed_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> ForgeQueryDeclarationReceiptOrchestrationTranscript<D, I> {
    let lowered =
        forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
            handle,
            progressed,
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
            ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
            ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
            route_intent,
        );
    let checked = match lowered.checked {
        ForgeQueryDeclarationEntryProductChecked::Receipt(checked) => checked,
        _ => panic!("receipt orchestration proof must project the receipt product"),
    };
    let outcome_identity = receipt_orchestration_identity(&checked);
    ForgeQueryDeclarationReceiptOrchestrationTranscript::new(
        lowered.plan,
        checked,
        lowered.step_records,
        outcome_identity,
    )
}
