//! Closed descriptive material for publication consumers.

use worth_query_installation::facade::{
    PublishedAftermathPosture, WorthQueryCanonicalWorkEvidence,
};

use super::WorthQueryApplicationCommitReceipt;
use crate::domain_computation::application_aftermath::{
    ExternalRailTransportFault, WorthQueryExternalDispatchPostureKind,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitTerminalKind, WorthQueryExternalDispatchPreparationDenial,
    WorthQueryPrimaryMutationWorkEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationCommitPublicationExternalEffect {
    NotDeclared,
    PendingDispatch,
    Completed,
    Acknowledged,
    Unresolved(Option<ExternalRailTransportFault>),
    PreparationDenied(WorthQueryExternalDispatchPreparationDenial),
}

/// Execution-owned, non-authoritative description of one commit terminal.
///
/// It intentionally carries no receipt, runtime, branch, session, record, or
/// causal identity and cannot be converted back into commit authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationCommitPublicationSource {
    aftermath_posture: Option<PublishedAftermathPosture>,
    external_effect: WorthQueryApplicationCommitPublicationExternalEffect,
    terminal_kind: WorthQueryApplicationCommitTerminalKind,
    mutation_work: Option<WorthQueryPrimaryMutationWorkEvidence>,
    changed_record_count: usize,
    emitted_effect_count: usize,
    publication_work: WorthQueryCanonicalWorkEvidence,
    attempt_resources_released: Option<bool>,
}

impl WorthQueryApplicationCommitPublicationSource {
    pub(super) fn from_receipt(receipt: &WorthQueryApplicationCommitReceipt) -> Self {
        Self {
            aftermath_posture: receipt.published_aftermath_posture(),
            external_effect: publication_external_effect(receipt),
            terminal_kind: receipt.terminal().kind(),
            mutation_work: receipt.mutation_work().cloned(),
            changed_record_count: receipt.changed_record_count(),
            emitted_effect_count: receipt.emitted_effect_count(),
            publication_work: receipt.canonical_work().publication(),
            attempt_resources_released: receipt.terminal().attempt_resources_released(),
        }
    }

    pub const fn aftermath_posture(&self) -> Option<PublishedAftermathPosture> {
        self.aftermath_posture
    }

    pub const fn external_effect(&self) -> WorthQueryApplicationCommitPublicationExternalEffect {
        self.external_effect
    }

    pub const fn terminal_kind(&self) -> WorthQueryApplicationCommitTerminalKind {
        self.terminal_kind
    }

    pub const fn mutation_work(&self) -> Option<&WorthQueryPrimaryMutationWorkEvidence> {
        self.mutation_work.as_ref()
    }

    pub const fn changed_record_count(&self) -> usize {
        self.changed_record_count
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.emitted_effect_count
    }

    pub const fn publication_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.publication_work
    }

    pub const fn attempt_resources_released(&self) -> Option<bool> {
        self.attempt_resources_released
    }
}

fn publication_external_effect(
    receipt: &WorthQueryApplicationCommitReceipt,
) -> WorthQueryApplicationCommitPublicationExternalEffect {
    if let Some(dispatch) = receipt.external_dispatch() {
        return match dispatch.posture().kind() {
            WorthQueryExternalDispatchPostureKind::Completed => {
                WorthQueryApplicationCommitPublicationExternalEffect::Completed
            }
            WorthQueryExternalDispatchPostureKind::Acknowledged => {
                WorthQueryApplicationCommitPublicationExternalEffect::Acknowledged
            }
            WorthQueryExternalDispatchPostureKind::Unresolved => {
                WorthQueryApplicationCommitPublicationExternalEffect::Unresolved(
                    dispatch
                        .posture()
                        .classification()
                        .map(|value| value.fault()),
                )
            }
        };
    }
    if let Some(denial) = receipt.external_dispatch_preparation_denial() {
        return WorthQueryApplicationCommitPublicationExternalEffect::PreparationDenied(denial);
    }
    if receipt.dispatch_outbox().is_some() {
        WorthQueryApplicationCommitPublicationExternalEffect::PendingDispatch
    } else {
        WorthQueryApplicationCommitPublicationExternalEffect::NotDeclared
    }
}
