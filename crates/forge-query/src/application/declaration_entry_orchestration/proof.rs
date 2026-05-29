use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::checked::ForgeQueryDeclarationEntryOrchestrationChecked;
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationStage {
    AdmittedHandle,
    DeclarationReviewed,
    LegalityEstablished,
    ProgressionResolved,
    FoundationalDescribed,
    RoutePlanned,
    ReceiptIssued,
    EnvelopeConstructed,
}

impl ForgeQueryDeclarationEntryOrchestrationStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedHandle => "admitted_handle",
            Self::DeclarationReviewed => "declaration_reviewed",
            Self::LegalityEstablished => "legality_established",
            Self::ProgressionResolved => "progression_resolved",
            Self::FoundationalDescribed => "foundational_described",
            Self::RoutePlanned => "route_planned",
            Self::ReceiptIssued => "receipt_issued",
            Self::EnvelopeConstructed => "envelope_constructed",
        }
    }
}

pub struct ForgeQueryDeclarationEntryOrchestrationStageRecord {
    stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reached: bool,
    retained_digest: Option<String>,
}

impl ForgeQueryDeclarationEntryOrchestrationStageRecord {
    pub(crate) fn reached(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
    ) -> Self {
        Self {
            stage,
            reached: true,
            retained_digest,
        }
    }

    pub(crate) fn stopped(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
    ) -> Self {
        Self {
            stage,
            reached: false,
            retained_digest,
        }
    }

    pub fn stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.stage
    }

    pub fn is_reached(&self) -> bool {
        self.reached
    }

    pub fn is_stop(&self) -> bool {
        !self.reached
    }

    pub fn retained_digest(&self) -> Option<&str> {
        self.retained_digest.as_deref()
    }
}

pub struct ForgeQueryDeclarationEntryOrchestrationProof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    outcome: ForgeQueryDeclarationEntryOrchestrationChecked<D, I>,
    stage_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    orchestration_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationProof<D, I>
{
    pub(crate) fn new(
        outcome: ForgeQueryDeclarationEntryOrchestrationChecked<D, I>,
        stage_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    ) -> Self {
        let parts = stage_records
            .iter()
            .map(|record| {
                format!(
                    "{}:{}:{}",
                    record.stage().as_str(),
                    if record.is_reached() {
                        "reached"
                    } else {
                        "stopped"
                    },
                    record.retained_digest().unwrap_or("none")
                )
            })
            .collect::<Vec<_>>();
        let orchestration_digest = hash_parts(&[
            format!("outcome:{}", outcome.outcome_identity()),
            format!("stages:{}", parts.join("|")),
        ]);
        Self {
            outcome,
            stage_records,
            orchestration_digest,
        }
    }

    pub fn outcome(&self) -> &ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
        &self.outcome
    }

    pub fn stage_records(&self) -> &[ForgeQueryDeclarationEntryOrchestrationStageRecord] {
        &self.stage_records
    }

    pub fn orchestration_digest(&self) -> &str {
        &self.orchestration_digest
    }
}

pub(crate) fn forge_query_declaration_entry_orchestration_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
) -> ForgeQueryDeclarationEntryOrchestrationProof<D, I> {
    let lowered =
        super::lower::forge_query_lower_declaration_entry_orchestration_on_handle(handle, input);
    ForgeQueryDeclarationEntryOrchestrationProof::new(lowered.checked, lowered.stage_records)
}
