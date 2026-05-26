use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::checked::ForgeQueryDeclarationEntryOrchestrationChecked;
use super::proof::{
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord,
};

mod entry;
mod reason;
mod route;

pub(crate) struct ForgeQueryLoweredDeclarationEntryOrchestration<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    pub(crate) checked: ForgeQueryDeclarationEntryOrchestrationChecked<D, I>,
    pub(crate) stage_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
}

pub(crate) fn forge_query_lower_declaration_entry_orchestration_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
) -> ForgeQueryLoweredDeclarationEntryOrchestration<D, I> {
    let mut stage_records = vec![ForgeQueryDeclarationEntryOrchestrationStageRecord::reached(
        ForgeQueryDeclarationEntryOrchestrationStage::AdmittedHandle,
        Some(handle.handle_identity_digest().to_string()),
    )];
    let checked = entry::lower_from_declaration_checked(handle, &mut stage_records, input);
    ForgeQueryLoweredDeclarationEntryOrchestration {
        checked,
        stage_records,
    }
}

pub(super) fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
