use crate::domain_installation::operation_authority_chain::{
    WorthQueryLineageBoundOperationPhase, WorthQueryOperationPhaseProof,
};
use crate::domain_installation::operation_identity_basis::lineage_outcome_material;
use crate::identity_evolution::InstalledIdentityEvolutionOutcome;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryStageLineageDeclaration {
    stage_identity: String,
    outcomes: Vec<InstalledIdentityEvolutionOutcome>,
}

impl WorthQueryStageLineageDeclaration {
    pub(super) fn from_execution(
        stage_identity: String,
        outcomes: Vec<InstalledIdentityEvolutionOutcome>,
    ) -> Self {
        debug_assert!(!stage_identity.trim().is_empty());
        debug_assert!(!outcomes.is_empty());
        Self {
            stage_identity,
            outcomes,
        }
    }

    pub fn stage_identity(&self) -> &str {
        &self.stage_identity
    }
    pub fn outcomes(&self) -> &[InstalledIdentityEvolutionOutcome] {
        &self.outcomes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTraceLineageEvidence {
    stage_identity: String,
    stage_receipt_identity: String,
    effect_receipt_identities: Vec<String>,
    outcome: InstalledIdentityEvolutionOutcome,
    foundational_lineage: crate::identity_evolution::WorthQueryFoundationalLineageAttachment,
}

impl WorthQueryTraceLineageEvidence {
    pub fn stage_identity(&self) -> &str {
        &self.stage_identity
    }
    pub fn stage_receipt_identity(&self) -> &str {
        &self.stage_receipt_identity
    }
    pub fn effect_receipt_identities(&self) -> &[String] {
        &self.effect_receipt_identities
    }
    pub fn outcome(&self) -> &InstalledIdentityEvolutionOutcome {
        &self.outcome
    }
    pub fn foundational_lineage(
        &self,
    ) -> &crate::identity_evolution::WorthQueryFoundationalLineageAttachment {
        &self.foundational_lineage
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryTraceLineageCounters {
    pub indexed_trace_stages: usize,
    pub indexed_effect_receipts: usize,
    pub stage_lookups: usize,
    pub outcome_contract_checks: usize,
    pub outcome_width: usize,
    pub effect_receipt_attachments: usize,
    pub conditional_path_checks: usize,
    pub unrelated_trace_scans: usize,
    pub unrelated_identity_scans: usize,
}

pub struct WorthQueryTraceLineageReport {
    pub(super) identity: String,
    pub(super) trace_identity: String,
    pub(super) evidence: Vec<WorthQueryTraceLineageEvidence>,
    pub(super) counters: WorthQueryTraceLineageCounters,
    pub(super) proof: WorthQueryOperationPhaseProof<WorthQueryLineageBoundOperationPhase>,
}

impl WorthQueryTraceLineageReport {
    pub fn identity(&self) -> &str {
        debug_assert_eq!(self.proof.payload().identity(), self.identity);
        &self.identity
    }
    pub fn trace_identity(&self) -> &str {
        &self.trace_identity
    }
    pub fn evidence(&self) -> &[WorthQueryTraceLineageEvidence] {
        &self.evidence
    }
    pub fn counters(&self) -> WorthQueryTraceLineageCounters {
        self.counters
    }

    pub(crate) fn semantic_part(&self) -> String {
        self.evidence
            .iter()
            .map(|evidence| {
                format!(
                    "{}:{}:{}:{}",
                    evidence.stage_identity,
                    evidence.stage_receipt_identity,
                    evidence.effect_receipt_identities.join(","),
                    lineage_outcome_material(&evidence.outcome)
                )
            })
            .collect::<Vec<_>>()
            .join("|")
    }
}

pub(super) fn lineage_evidence(
    stage_identity: String,
    stage_receipt_identity: String,
    effect_receipt_identities: Vec<String>,
    outcome: InstalledIdentityEvolutionOutcome,
    foundational_lineage: crate::identity_evolution::WorthQueryFoundationalLineageAttachment,
) -> WorthQueryTraceLineageEvidence {
    WorthQueryTraceLineageEvidence {
        stage_identity,
        stage_receipt_identity,
        effect_receipt_identities,
        outcome,
        foundational_lineage,
    }
}
