use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryGraphObligationExecutionStatus;

use crate::validator_invariant_catalog::relational_invariant_catalog::WorthTopologyRelationalInvariantCatalogCloseout;
use crate::validator_invariant_catalog::selected_graph_obligation_enforcement::{
    WorthTopologyGraphObligationExecutionProofProjection,
    WorthTopologyGraphObligationExecutionRowProjection,
    WorthTopologySelectedGraphObligationDiagnosticWitness,
    WorthTopologySelectedGraphObligationEnforcementCounters,
    WorthTopologySelectedGraphObligationEnforcementDenial,
    WorthTopologySelectedGraphObligationEnforcementDenialKind,
    WorthTopologySelectedGraphObligationEnforcementOutcome,
    WorthTopologySelectedGraphObligationEnforcementPhaseSevenSeed,
    WorthTopologySelectedGraphObligationEnforcementReceipt,
    WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport,
    WorthTopologySelectedGraphObligationExecutionInput,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologySelectedRelationalInvariantFamilyRow,
};

#[derive(Clone, Debug)]
pub struct WorthTopologySelectedGraphObligationEnforcementCloseout {
    phase_six_seed_digest: String,
    selected_plan_digest: String,
    query_execution_envelope_digest: String,
    proof_projection: WorthTopologyGraphObligationExecutionProofProjection,
    query_execution_rows: Vec<WorthTopologyGraphObligationExecutionRowProjection>,
    enforcement_receipts: Vec<WorthTopologySelectedGraphObligationEnforcementReceipt>,
    source_firewall: WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport,
    counters: WorthTopologySelectedGraphObligationEnforcementCounters,
    phase_seven_seed: WorthTopologySelectedGraphObligationEnforcementPhaseSevenSeed,
    closeout_digest: String,
}

impl WorthTopologySelectedGraphObligationEnforcementCloseout {
    pub fn execute_from_relational_invariant_closeout(
        relational_closeout: &WorthTopologyRelationalInvariantCatalogCloseout,
        execution_input: WorthTopologySelectedGraphObligationExecutionInput,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let proof_projection =
            WorthTopologyGraphObligationExecutionProofProjection::from_execution_backed_proof(
                execution_input.execution_backed_adoption_proof(),
            );
        reject_mismatched_execution_envelope(
            &proof_projection,
            execution_input.query_execution_envelope().envelope_digest(),
        )?;
        let source_firewall =
            WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport::current();
        if !source_firewall.is_clean() {
            return Err(selected_graph_denial(
                WorthTopologySelectedGraphObligationEnforcementDenialKind::SourceFirewallViolation,
                source_firewall.report_digest(),
                "Phase 6 source firewall found local graph-obligation ceremony residue",
            ));
        }
        let query_execution_rows = execution_input
            .query_execution_envelope()
            .rows()
            .iter()
            .map(WorthTopologyGraphObligationExecutionRowProjection::from_query_row)
            .collect::<Vec<_>>();
        let rows_by_registration = query_rows_by_registration(&query_execution_rows);
        let validator_receipts = relational_closeout
            .selected_validator_family_rows()
            .iter()
            .map(|selected| {
                receipt_for_selected_obligation(
                    relational_closeout.selected_plan_digest(),
                    execution_input.query_execution_envelope().envelope_digest(),
                    selected.row_digest(),
                    selected.worth_family_identity_digest(),
                    selected.registration_digest(),
                    &rows_by_registration,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let invariant_receipts = relational_closeout
            .selected_invariant_family_rows()
            .iter()
            .map(|selected| {
                receipt_for_selected_invariant(
                    relational_closeout.selected_plan_digest(),
                    execution_input.query_execution_envelope().envelope_digest(),
                    selected,
                    &rows_by_registration,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let enforcement_receipts = validator_receipts
            .into_iter()
            .chain(invariant_receipts)
            .collect::<Vec<_>>();
        let counters = WorthTopologySelectedGraphObligationEnforcementCounters::from_receipts(
            relational_closeout.selected_validator_family_rows().len(),
            relational_closeout.selected_invariant_family_rows().len(),
            query_execution_rows.len(),
            source_firewall.violations().len(),
            usize::from(!proof_projection.support_pin_digest().is_empty()),
            usize::from(!proof_projection.adoption_manifest_digest().is_empty()),
            usize::from(!proof_projection.residue_manifest_digest().is_empty()),
            &enforcement_receipts,
        );
        let phase_seven_seed =
            WorthTopologySelectedGraphObligationEnforcementPhaseSevenSeed::from_parts(
                relational_closeout.phase_six_seed().seed_digest(),
                relational_closeout.selected_plan_digest(),
                relational_closeout
                    .phase_six_seed()
                    .routing_closure_digest(),
                execution_input.query_execution_envelope().envelope_digest(),
                proof_projection.adoption_manifest_digest(),
                proof_projection.support_pin_digest(),
                proof_projection.support_matrix_digest(),
                proof_projection.residue_manifest_digest(),
                proof_projection.local_ceremony_audit_digest(),
                proof_projection.in_memory_proof_digest(),
                proof_projection.execution_proof_digest(),
                &counters,
                source_firewall.report_digest(),
                &enforcement_receipts,
            );
        let closeout_digest = selected_graph_closeout_digest(
            relational_closeout.phase_six_seed().seed_digest(),
            relational_closeout.selected_plan_digest(),
            execution_input.query_execution_envelope().envelope_digest(),
            proof_projection.projection_digest(),
            source_firewall.report_digest(),
            counters.counters_digest(),
            phase_seven_seed.seed_digest(),
            &enforcement_receipts,
        );
        Ok(Self {
            phase_six_seed_digest: relational_closeout
                .phase_six_seed()
                .seed_digest()
                .to_string(),
            selected_plan_digest: relational_closeout.selected_plan_digest().to_string(),
            query_execution_envelope_digest: execution_input
                .query_execution_envelope()
                .envelope_digest()
                .to_string(),
            proof_projection,
            query_execution_rows,
            enforcement_receipts,
            source_firewall,
            counters,
            phase_seven_seed,
            closeout_digest,
        })
    }

    pub fn phase_six_seed_digest(&self) -> &str {
        &self.phase_six_seed_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn query_execution_envelope_digest(&self) -> &str {
        &self.query_execution_envelope_digest
    }

    pub const fn proof_projection(&self) -> &WorthTopologyGraphObligationExecutionProofProjection {
        &self.proof_projection
    }

    pub fn query_execution_rows(&self) -> &[WorthTopologyGraphObligationExecutionRowProjection] {
        &self.query_execution_rows
    }

    pub fn enforcement_receipts(
        &self,
    ) -> &[WorthTopologySelectedGraphObligationEnforcementReceipt] {
        &self.enforcement_receipts
    }

    pub const fn source_firewall(
        &self,
    ) -> &WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport {
        &self.source_firewall
    }

    pub const fn counters(&self) -> &WorthTopologySelectedGraphObligationEnforcementCounters {
        &self.counters
    }

    pub const fn phase_seven_seed(
        &self,
    ) -> &WorthTopologySelectedGraphObligationEnforcementPhaseSevenSeed {
        &self.phase_seven_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn reject_mismatched_execution_envelope(
    proof_projection: &WorthTopologyGraphObligationExecutionProofProjection,
    observed_envelope_digest: &str,
) -> Result<(), WorthTopologyLegalityCatalogError> {
    if proof_projection.execution_envelope_digest() == observed_envelope_digest {
        return Ok(());
    }
    Err(selected_graph_denial(
        WorthTopologySelectedGraphObligationEnforcementDenialKind::ExecutionEnvelopeMismatch,
        observed_envelope_digest,
        "Query execution envelope must match the Consumer Kit execution-backed adoption proof",
    ))
}

fn receipt_for_selected_invariant(
    selected_plan_digest: &str,
    query_execution_envelope_digest: &str,
    selected: &WorthTopologySelectedRelationalInvariantFamilyRow,
    rows_by_registration: &BTreeMap<&str, &WorthTopologyGraphObligationExecutionRowProjection>,
) -> Result<WorthTopologySelectedGraphObligationEnforcementReceipt, WorthTopologyLegalityCatalogError>
{
    receipt_for_selected_obligation(
        selected_plan_digest,
        query_execution_envelope_digest,
        selected.selected_obligation_row_digest(),
        selected.worth_family_identity_digest(),
        selected.registration_digest(),
        rows_by_registration,
    )
}

fn receipt_for_selected_obligation(
    selected_plan_digest: &str,
    query_execution_envelope_digest: &str,
    selected_obligation_row_digest: &str,
    worth_family_identity_digest: &str,
    registration_digest: &str,
    rows_by_registration: &BTreeMap<&str, &WorthTopologyGraphObligationExecutionRowProjection>,
) -> Result<WorthTopologySelectedGraphObligationEnforcementReceipt, WorthTopologyLegalityCatalogError>
{
    let Some(row_projection) = rows_by_registration.get(registration_digest) else {
        return Err(selected_graph_denial(
            WorthTopologySelectedGraphObligationEnforcementDenialKind::MissingQueryExecutionRow,
            registration_digest,
            "selected validator or invariant family has no Query execution row",
        ));
    };
    let outcome = outcome_from_query_row(
        selected_obligation_row_digest,
        worth_family_identity_digest,
        row_projection,
    );
    let diagnostic_witness_digest = diagnostic_witness_digest(&outcome);
    Ok(
        WorthTopologySelectedGraphObligationEnforcementReceipt::from_query_projection(
            selected_plan_digest,
            selected_obligation_row_digest,
            worth_family_identity_digest,
            registration_digest,
            query_execution_envelope_digest,
            row_projection,
            outcome,
            diagnostic_witness_digest,
        ),
    )
}

fn outcome_from_query_row(
    selected_obligation_row_digest: &str,
    worth_family_identity_digest: &str,
    row: &WorthTopologyGraphObligationExecutionRowProjection,
) -> WorthTopologySelectedGraphObligationEnforcementOutcome {
    let witness = || {
        WorthTopologySelectedGraphObligationDiagnosticWitness::from_query_row(
            selected_obligation_row_digest,
            worth_family_identity_digest,
            row.query_execution_row_digest(),
            row.query_status().as_str(),
            row.query_verdict(),
            row.query_verdict_context(),
        )
    };
    match row.query_status() {
        ForgeQueryGraphObligationExecutionStatus::Executed => match row.query_verdict() {
            Some("allow") => WorthTopologySelectedGraphObligationEnforcementOutcome::Passed,
            Some("advise") => {
                WorthTopologySelectedGraphObligationEnforcementOutcome::Advisory(witness())
            }
            Some("block") => {
                WorthTopologySelectedGraphObligationEnforcementOutcome::Violation(witness())
            }
            _ => WorthTopologySelectedGraphObligationEnforcementOutcome::Violation(witness()),
        },
        ForgeQueryGraphObligationExecutionStatus::DiagnosticOnly
        | ForgeQueryGraphObligationExecutionStatus::DeferredToBackstop
        | ForgeQueryGraphObligationExecutionStatus::NotApplicableAfterStateLoad => {
            WorthTopologySelectedGraphObligationEnforcementOutcome::Advisory(witness())
        }
        ForgeQueryGraphObligationExecutionStatus::BudgetExceeded
        | ForgeQueryGraphObligationExecutionStatus::SuppressedByPolicy
        | ForgeQueryGraphObligationExecutionStatus::BlockedByPrerequisite
        | ForgeQueryGraphObligationExecutionStatus::Selected
        | ForgeQueryGraphObligationExecutionStatus::NotSelected => {
            let denial = WorthTopologySelectedGraphObligationEnforcementDenial::new(
                WorthTopologySelectedGraphObligationEnforcementDenialKind::UnsupportedQueryStatus,
                row.query_execution_row_digest(),
                format!(
                    "Query returned `{}` before executable Worth enforcement",
                    row.query_status().as_str()
                ),
            );
            WorthTopologySelectedGraphObligationEnforcementOutcome::DeniedBeforeExecution(denial)
        }
        ForgeQueryGraphObligationExecutionStatus::Unsupported
        | ForgeQueryGraphObligationExecutionStatus::ExecutorError => {
            WorthTopologySelectedGraphObligationEnforcementOutcome::Violation(witness())
        }
    }
}

fn diagnostic_witness_digest(
    outcome: &WorthTopologySelectedGraphObligationEnforcementOutcome,
) -> Option<String> {
    match outcome {
        WorthTopologySelectedGraphObligationEnforcementOutcome::Advisory(witness)
        | WorthTopologySelectedGraphObligationEnforcementOutcome::Violation(witness) => {
            Some(witness.witness_digest().to_string())
        }
        WorthTopologySelectedGraphObligationEnforcementOutcome::Passed
        | WorthTopologySelectedGraphObligationEnforcementOutcome::DeniedBeforeExecution(_) => None,
    }
}

fn query_rows_by_registration(
    rows: &[WorthTopologyGraphObligationExecutionRowProjection],
) -> BTreeMap<&str, &WorthTopologyGraphObligationExecutionRowProjection> {
    rows.iter()
        .map(|row| (row.registration_digest(), row))
        .collect()
}

fn selected_graph_closeout_digest(
    phase_six_seed_digest: &str,
    selected_plan_digest: &str,
    query_execution_envelope_digest: &str,
    proof_projection_digest: &str,
    source_firewall_digest: &str,
    counters_digest: &str,
    phase_seven_seed_digest: &str,
    receipts: &[WorthTopologySelectedGraphObligationEnforcementReceipt],
) -> String {
    let mut parts = vec![
        "worth-topo-selected-graph-obligation-enforcement-closeout-v1".to_string(),
        format!("phase-six-seed:{phase_six_seed_digest}"),
        format!("selected-plan:{selected_plan_digest}"),
        format!("query-envelope:{query_execution_envelope_digest}"),
        format!("query-proof:{proof_projection_digest}"),
        format!("source-firewall:{source_firewall_digest}"),
        format!("counters:{counters_digest}"),
        format!("phase-seven-seed:{phase_seven_seed_digest}"),
    ];
    parts.extend(
        receipts
            .iter()
            .map(|receipt| format!("receipt:{}", receipt.enforcement_receipt_digest())),
    );
    parts.join("|")
}

fn selected_graph_denial(
    kind: WorthTopologySelectedGraphObligationEnforcementDenialKind,
    authority_digest: impl Into<String>,
    message: impl Into<String>,
) -> WorthTopologyLegalityCatalogError {
    WorthTopologyLegalityCatalogError::PhaseSixGraphObligationEnforcement(
        WorthTopologySelectedGraphObligationEnforcementDenial::new(kind, authority_digest, message),
    )
}
