use forge_query::facade::{
    ForgeQueryAuthoritativeMutationObligationDispatchProjectionRow, ForgeQueryBatchWriteReceipt,
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationSupportLane,
};

use super::super::surface_vocab::TopologyConstructionQueryMutationSurface;
use super::coverage::TopologyPrimitiveConstructionBirthMaterializationCoverage;
use super::program::TopologyPrimitiveConstructionBirthComposeProgram;
use crate::construction::query_native_boundary::digest_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionBirthComposeEvidence {
    mutation_surface: TopologyConstructionQueryMutationSurface,
    source_admitted_handoff_digest: String,
    compose_program_digest: String,
    materialization_coverage: TopologyPrimitiveConstructionBirthMaterializationCoverage,
    batch_receipt_digest: String,
    graph_obligation_envelope_digest: String,
    graph_obligation_selected_count: usize,
    selected_obligation_rows: Vec<TopologyPrimitiveConstructionBirthSelectedObligationRow>,
    evidence_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionBirthSelectedObligationRow {
    rule_identity_digest: String,
    rule_namespace: String,
    rule_name: String,
    rule_semantic_version: String,
    obligation_kind: ForgeQueryGraphObligationKind,
    verdict: String,
    verdict_context: Option<String>,
    dispatch_plan_digest: String,
    execution_input_digest: String,
    executor_contract_digest: String,
    execution_budget_digest: String,
    support_lane: ForgeQueryGraphObligationSupportLane,
    execution_status: Option<ForgeQueryGraphObligationExecutionStatus>,
    row_digest: String,
}

impl TopologyPrimitiveConstructionBirthSelectedObligationRow {
    fn from_projection_row(
        row: &ForgeQueryAuthoritativeMutationObligationDispatchProjectionRow,
    ) -> Self {
        Self::new(
            row.rule_identity_digest(),
            row.rule_namespace(),
            row.rule_name(),
            row.rule_semantic_version(),
            row.obligation_kind(),
            row.verdict(),
            row.verdict_context(),
            row.dispatch_plan_digest(),
            row.execution_input_digest(),
            row.executor_contract_digest(),
            row.execution_budget_digest(),
            row.support_lane(),
            row.execution_status(),
        )
    }

    fn new(
        rule_identity_digest: impl Into<String>,
        rule_namespace: impl Into<String>,
        rule_name: impl Into<String>,
        rule_semantic_version: impl Into<String>,
        obligation_kind: ForgeQueryGraphObligationKind,
        verdict: impl Into<String>,
        verdict_context: Option<&str>,
        dispatch_plan_digest: impl Into<String>,
        execution_input_digest: impl Into<String>,
        executor_contract_digest: impl Into<String>,
        execution_budget_digest: impl Into<String>,
        support_lane: ForgeQueryGraphObligationSupportLane,
        execution_status: Option<ForgeQueryGraphObligationExecutionStatus>,
    ) -> Self {
        let rule_identity_digest = rule_identity_digest.into();
        let rule_namespace = rule_namespace.into();
        let rule_name = rule_name.into();
        let rule_semantic_version = rule_semantic_version.into();
        let verdict = verdict.into();
        let verdict_context = verdict_context.map(str::to_string);
        let dispatch_plan_digest = dispatch_plan_digest.into();
        let execution_input_digest = execution_input_digest.into();
        let executor_contract_digest = executor_contract_digest.into();
        let execution_budget_digest = execution_budget_digest.into();
        let row_digest = digest_parts(&[
            "primitive-construction-birth-selected-obligation-row".to_string(),
            rule_identity_digest.clone(),
            rule_namespace.clone(),
            rule_name.clone(),
            rule_semantic_version.clone(),
            obligation_kind.as_str().to_string(),
            verdict.clone(),
            verdict_context
                .as_deref()
                .unwrap_or("no-verdict-context")
                .to_string(),
            dispatch_plan_digest.clone(),
            execution_input_digest.clone(),
            executor_contract_digest.clone(),
            execution_budget_digest.clone(),
            support_lane.as_str().to_string(),
            execution_status
                .map(|status| status.as_str().to_string())
                .unwrap_or_else(|| "no-execution-status".to_string()),
        ]);
        Self {
            rule_identity_digest,
            rule_namespace,
            rule_name,
            rule_semantic_version,
            obligation_kind,
            verdict,
            verdict_context,
            dispatch_plan_digest,
            execution_input_digest,
            executor_contract_digest,
            execution_budget_digest,
            support_lane,
            execution_status,
            row_digest,
        }
    }

    pub fn rule_identity_digest(&self) -> &str {
        &self.rule_identity_digest
    }

    pub fn rule_namespace(&self) -> &str {
        &self.rule_namespace
    }

    pub fn rule_name(&self) -> &str {
        &self.rule_name
    }

    pub fn rule_semantic_version(&self) -> &str {
        &self.rule_semantic_version
    }

    pub fn obligation_kind(&self) -> ForgeQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn verdict(&self) -> &str {
        &self.verdict
    }

    pub fn verdict_context(&self) -> Option<&str> {
        self.verdict_context.as_deref()
    }

    pub fn dispatch_plan_digest(&self) -> &str {
        &self.dispatch_plan_digest
    }

    pub fn execution_input_digest(&self) -> &str {
        &self.execution_input_digest
    }

    pub fn executor_contract_digest(&self) -> &str {
        &self.executor_contract_digest
    }

    pub fn execution_budget_digest(&self) -> &str {
        &self.execution_budget_digest
    }

    pub fn support_lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn execution_status(&self) -> Option<ForgeQueryGraphObligationExecutionStatus> {
        self.execution_status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl TopologyPrimitiveConstructionBirthComposeEvidence {
    pub(crate) fn from_receipt(
        program: &TopologyPrimitiveConstructionBirthComposeProgram,
        receipt: &ForgeQueryBatchWriteReceipt,
        graph_obligation_envelope_digest: String,
    ) -> Self {
        let batch_receipt_digest = receipt.batch_digest().to_string();
        let graph_obligation_selected_count = receipt
            .graph_obligation_evidence()
            .map(|evidence| evidence.selected_obligation_count())
            .unwrap_or(0);
        let selected_obligation_rows = receipt
            .obligation_dispatch()
            .map(|dispatch| {
                dispatch
                    .evidence_projection()
                    .rows()
                    .iter()
                    .map(TopologyPrimitiveConstructionBirthSelectedObligationRow::from_projection_row)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let evidence_digest = digest_parts(&[
            "primitive-construction-birth-compose-evidence".to_string(),
            program.mutation_surface().as_str().to_string(),
            program.source_admitted_handoff_digest().to_string(),
            program.program_digest().to_string(),
            program
                .materialization_coverage()
                .coverage_digest()
                .to_string(),
            batch_receipt_digest.clone(),
            graph_obligation_envelope_digest.clone(),
            graph_obligation_selected_count.to_string(),
            selected_obligation_rows
                .iter()
                .map(|row| row.row_digest())
                .collect::<Vec<_>>()
                .join("|"),
        ]);
        Self {
            mutation_surface: program.mutation_surface(),
            source_admitted_handoff_digest: program.source_admitted_handoff_digest().to_string(),
            compose_program_digest: program.program_digest().to_string(),
            materialization_coverage: program.materialization_coverage().clone(),
            batch_receipt_digest,
            graph_obligation_envelope_digest,
            graph_obligation_selected_count,
            selected_obligation_rows,
            evidence_digest,
        }
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.mutation_surface
    }

    pub fn source_admitted_handoff_digest(&self) -> &str {
        &self.source_admitted_handoff_digest
    }

    pub fn compose_program_digest(&self) -> &str {
        &self.compose_program_digest
    }

    pub fn materialization_coverage(
        &self,
    ) -> &TopologyPrimitiveConstructionBirthMaterializationCoverage {
        &self.materialization_coverage
    }

    pub fn batch_receipt_digest(&self) -> &str {
        &self.batch_receipt_digest
    }

    pub fn graph_obligation_envelope_digest(&self) -> &str {
        &self.graph_obligation_envelope_digest
    }

    pub fn graph_obligation_selected_count(&self) -> usize {
        self.graph_obligation_selected_count
    }

    pub fn selected_obligation_rows(
        &self,
    ) -> &[TopologyPrimitiveConstructionBirthSelectedObligationRow] {
        &self.selected_obligation_rows
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}
