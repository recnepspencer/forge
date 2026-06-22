use forge_query::facade::{
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationSupportLane,
};
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::outcome::{
    prepare_primitive_construction_executed_outcome, PrimitiveConstructionExecutedPreparedOutcome,
};
use crate::construction::request::{PrimitiveConstructionFamily, PRIMITIVE_CONSTRUCTION_FAMILIES};
use crate::construction::result::{
    prepare_primitive_construction_executed_result,
    ExecutedPrimitiveConstructionGraphAuthorityResult,
};
use crate::construction::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionGraphObligationExecutionMatrixRow {
    family: PrimitiveConstructionFamily,
    result_digest: String,
    outcome_digest: String,
    evidence_digest: String,
    envelope_digest: String,
    selected_count: usize,
    selected_row_digest: String,
    rule_identity_digest: String,
    obligation_kind: ForgeQueryGraphObligationKind,
    support_lane: ForgeQueryGraphObligationSupportLane,
    execution_status: Option<ForgeQueryGraphObligationExecutionStatus>,
    verdict: String,
    verdict_context: Option<String>,
    dispatch_plan_digest: String,
    execution_input_digest: String,
    executor_contract_digest: String,
    execution_budget_digest: String,
}

impl PrimitiveConstructionGraphObligationExecutionMatrixRow {
    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub(crate) fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub(crate) fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub(crate) fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub(crate) fn selected_count(&self) -> usize {
        self.selected_count
    }

    pub(crate) fn selected_row_digest(&self) -> &str {
        &self.selected_row_digest
    }

    pub(crate) fn rule_identity_digest(&self) -> &str {
        &self.rule_identity_digest
    }

    pub(crate) fn obligation_kind(&self) -> ForgeQueryGraphObligationKind {
        self.obligation_kind
    }

    pub(crate) fn support_lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub(crate) fn execution_status(&self) -> Option<ForgeQueryGraphObligationExecutionStatus> {
        self.execution_status
    }

    pub(crate) fn verdict(&self) -> &str {
        &self.verdict
    }

    pub(crate) fn verdict_context(&self) -> Option<&str> {
        self.verdict_context.as_deref()
    }

    pub(crate) fn has_authoritative_dispatch_identity(&self) -> bool {
        !self.dispatch_plan_digest.is_empty()
            && !self.execution_input_digest.is_empty()
            && !self.executor_contract_digest.is_empty()
            && !self.execution_budget_digest.is_empty()
    }
}

pub(crate) fn primitive_construction_graph_obligation_execution_matrix(
) -> Vec<PrimitiveConstructionGraphObligationExecutionMatrixRow> {
    PRIMITIVE_CONSTRUCTION_FAMILIES
        .into_iter()
        .map(|family| execute_primitive_family_graph_obligation_case(family, "matrix"))
        .collect()
}

pub(crate) fn primitive_construction_graph_obligation_replay_pair(
    family: PrimitiveConstructionFamily,
) -> (
    PrimitiveConstructionGraphObligationExecutionMatrixRow,
    PrimitiveConstructionGraphObligationExecutionMatrixRow,
) {
    (
        execute_primitive_family_graph_obligation_case(family, "replay-a"),
        execute_primitive_family_graph_obligation_case(family, "replay-b"),
    )
}

pub(crate) fn primitive_construction_graph_obligation_execution_closeout_passes() -> bool {
    let rows = primitive_construction_graph_obligation_execution_matrix();
    rows.len() == PRIMITIVE_CONSTRUCTION_FAMILIES.len()
        && rows.iter().all(|row| {
            row.selected_count() == 1
                && !row.result_digest().is_empty()
                && !row.outcome_digest().is_empty()
                && !row.evidence_digest().is_empty()
                && !row.envelope_digest().is_empty()
                && row.execution_status()
                    == Some(ForgeQueryGraphObligationExecutionStatus::Executed)
                && row.has_authoritative_dispatch_identity()
        })
}

fn execute_primitive_family_graph_obligation_case(
    family: PrimitiveConstructionFamily,
    label: &str,
) -> PrimitiveConstructionGraphObligationExecutionMatrixRow {
    let intent = representative_intent(family);
    let result = execute_primitive_family_result_with_compose_evidence(&intent, label);
    let evidence = result.topology_compose_evidence();
    let outcome_digest = execute_primitive_family_outcome_evidence_digest(intent, label);

    primitive_family_execution_matrix_row_from_evidence(&result, evidence, &outcome_digest)
}

fn execute_primitive_family_result_with_compose_evidence(
    intent: &PrimitiveConstructionIntent,
    label: &str,
) -> ExecutedPrimitiveConstructionGraphAuthorityResult {
    let family = intent.family();
    let mut result_workspace =
        phase_eighteen_workspace(&format!("{label}.result.{}", family.as_str()));
    prepare_primitive_construction_executed_result(&mut result_workspace, intent.clone())
        .expect("executed primitive construction result")
}

fn execute_primitive_family_outcome_evidence_digest(
    intent: PrimitiveConstructionIntent,
    label: &str,
) -> String {
    let family = intent.family();
    let mut outcome_workspace =
        phase_eighteen_workspace(&format!("{label}.outcome.{}", family.as_str()));
    let outcome = prepare_primitive_construction_executed_outcome(&mut outcome_workspace, intent);
    let PrimitiveConstructionExecutedPreparedOutcome::Accepted(accepted) = outcome else {
        panic!("executed primitive construction outcome should be accepted");
    };
    assert_eq!(accepted.graph_obligation_selected_count(), 1);
    assert!(!accepted.topology_compose_evidence_digest().is_empty());
    assert!(!accepted.graph_obligation_envelope_digest().is_empty());
    accepted.outcome_digest().to_string()
}

fn primitive_family_execution_matrix_row_from_evidence(
    result: &ExecutedPrimitiveConstructionGraphAuthorityResult,
    evidence: &topology::facade::TopologyPrimitiveConstructionBirthComposeEvidence,
    outcome_digest: &str,
) -> PrimitiveConstructionGraphObligationExecutionMatrixRow {
    let selected_row = evidence
        .selected_obligation_rows()
        .first()
        .expect("executed primitive construction result must select an obligation");

    PrimitiveConstructionGraphObligationExecutionMatrixRow {
        family: result.family(),
        result_digest: result.result_digest().to_string(),
        outcome_digest: outcome_digest.to_string(),
        evidence_digest: evidence.evidence_digest().to_string(),
        envelope_digest: evidence.graph_obligation_envelope_digest().to_string(),
        selected_count: evidence.graph_obligation_selected_count(),
        selected_row_digest: selected_row.row_digest().to_string(),
        rule_identity_digest: selected_row.rule_identity_digest().to_string(),
        obligation_kind: selected_row.obligation_kind(),
        support_lane: selected_row.support_lane(),
        execution_status: selected_row.execution_status(),
        verdict: selected_row.verdict().to_string(),
        verdict_context: selected_row.verdict_context().map(str::to_string),
        dispatch_plan_digest: selected_row.dispatch_plan_digest().to_string(),
        execution_input_digest: selected_row.execution_input_digest().to_string(),
        executor_contract_digest: selected_row.executor_contract_digest().to_string(),
        execution_budget_digest: selected_row.execution_budget_digest().to_string(),
    }
}

fn representative_intent(family: PrimitiveConstructionFamily) -> PrimitiveConstructionIntent {
    match family {
        PrimitiveConstructionFamily::SimplexSolid => {
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(1.25))
        }
        PrimitiveConstructionFamily::Orthotope => {
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 1.5, 2.0],
            })
        }
        PrimitiveConstructionFamily::RegularPrism => {
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 5,
                radius: 1.0,
                height: 2.0,
            })
        }
        PrimitiveConstructionFamily::RegularPyramid => {
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 5,
                radius: 1.0,
                height: 1.75,
            })
        }
        PrimitiveConstructionFamily::WireBody => {
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 4 })
        }
        PrimitiveConstructionFamily::ShellWithHole => {
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![3, 4],
            })
        }
    }
}

fn phase_eighteen_workspace(label: &str) -> forge_query::facade::ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        format!("worth-kernel.phase-eighteen.{label}"),
    )
    .expect("workspace")
}
