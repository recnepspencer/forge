use forge_query::facade::ForgeQueryWorkspace;
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::tests::support::blocking_boundary;
use crate::construction::tests::support::blocking_boundary::PrimitiveConstructionBlockingBoundary;
use crate::construction::tests::support::branch_basis_digest::prepare_branch_basis_digest;
use crate::construction::tests::support::corpus_cases::{
    primitive_construction_corpus, PrimitiveConstructionCorpusScenario,
};
use crate::construction::tests::support::corpus_ordering::{
    apply_corpus_authoring_order_lane, PrimitiveConstructionCorpusAuthoringOrderLane,
};
use crate::construction::tests::support::corpus_replay_generation::{
    build_corpus_rows, PrimitiveConstructionCorpusReplaySiegeError,
};
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusReplaySiegeRow;
use crate::construction::tests::support::evidence_reports::sealed_report_identity;
use crate::construction::tests::support::runtime_truth::{
    PrimitiveConstructionAdmittedRuntimeTruth, PrimitiveConstructionCertificationRuntimeTruth,
    PrimitiveConstructionRejectedRuntimeTruth,
};

pub(crate) type PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow = (
    PrimitiveConstructionCorpusAuthoringOrderLane,
    String,
    String,
);

pub(crate) fn row_digest(
    workspace: &mut ForgeQueryWorkspace,
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Result<String, PrimitiveConstructionCorpusReplaySiegeError> {
    let _ = workspace;
    let scenario = scenario_for(row)?;
    let runtime_truth = row.runtime_truth();
    let mut branch_basis_workspace = branch_basis_workspace_for(row.scenario_id());
    let branch_basis_digest =
        prepare_branch_basis_digest(&mut branch_basis_workspace, &scenario.intent)?;
    let replay_digest = runtime_truth.outcome_digest().to_string();

    Ok(sealed_report_identity(
        "worth-kernel.construction.corpus-replay",
        "corpus-replay-row",
        |report| {
            report
                .value_participating("scenario", row.scenario_id().to_string())?
                .shape_participating("family", row.family().as_str())?
                .shape_participating("parameter-role", row.parameter_role().as_str())?
                .shape_participating("runtime-truth-kind", runtime_truth_kind(runtime_truth))?
                .value_participating("outcome", replay_digest.clone())?
                .value_participating("branch-basis", branch_basis_digest)?
                .value_participating(
                    "birth-truth",
                    admitted_runtime_truth(runtime_truth)
                        .map(PrimitiveConstructionAdmittedRuntimeTruth::birth_truth_digest)
                        .unwrap_or_default()
                        .to_string(),
                )?
                .optional_value_participating(
                    "realization-strategy",
                    admitted_runtime_truth(runtime_truth)
                        .map(PrimitiveConstructionAdmittedRuntimeTruth::realization_strategy)
                        .map(PrimitiveRealizationStrategy::as_str),
                )?
                .value_sequence_participating(
                    "attempted-realization-strategies",
                    attempted_realization_strategies(runtime_truth)
                        .iter()
                        .map(|strategy| (*strategy).as_str()),
                )?
                .optional_value_participating(
                    "stability-class",
                    stability_class(runtime_truth).map(PrimitiveStabilityClass::as_str),
                )?
                .optional_value_participating(
                    "feature-conditioning-class",
                    feature_conditioning_class(runtime_truth)
                        .map(PrimitiveFeatureConditioningClass::as_str),
                )?
                .optional_value_participating(
                    "support-normal-class",
                    support_normal_class(runtime_truth).map(PrimitiveSupportNormalClass::as_str),
                )?
                .optional_value_participating(
                    "normalization-disposition",
                    normalization_disposition(runtime_truth)
                        .map(PrimitiveNormalizationDisposition::as_str),
                )?
                .optional_value_participating(
                    "exhaustion-reason",
                    exhaustion_reason(runtime_truth)
                        .map(PrimitiveRealizationExhaustionReason::as_str),
                )?
                .optional_value_participating(
                    "rejection-class",
                    rejection_class(runtime_truth).map(PrimitiveConstructionRejectionClass::as_str),
                )?
                .optional_value_participating(
                    "rejection-locality",
                    rejection_locality(runtime_truth)
                        .map(PrimitiveConstructionRejectionLocality::as_str),
                )?
                .optional_value_participating(
                    "blocking-boundary",
                    rejection_locality(runtime_truth)
                        .map(blocking_boundary::blocking_boundary_for)
                        .map(PrimitiveConstructionBlockingBoundary::as_str),
                )?
                .usize_participating("construction-breadth", construction_breadth(runtime_truth))
        },
    ))
}

pub(crate) fn prepare_authoring_order_lane_digest_rows(
    workspace: &mut ForgeQueryWorkspace,
    canonical_rows: &[PrimitiveConstructionCorpusReplaySiegeRow],
) -> Result<
    Vec<PrimitiveConstructionCorpusAuthoringOrderLaneDigestRow>,
    PrimitiveConstructionCorpusReplaySiegeError,
> {
    let scenarios = primitive_construction_corpus();
    let mut rows = Vec::new();
    for lane in PrimitiveConstructionCorpusAuthoringOrderLane::all() {
        let lane_rows = if lane == PrimitiveConstructionCorpusAuthoringOrderLane::Canonical {
            canonical_rows.to_vec()
        } else {
            let lane_scenarios = apply_corpus_authoring_order_lane(lane, &scenarios);
            build_corpus_rows(workspace, &lane_scenarios)?
        };
        let lane_row_digests = lane_rows
            .iter()
            .map(|row| row_digest(workspace, row))
            .collect::<Result<Vec<_>, _>>()?;
        rows.push((
            lane,
            lane_digest(lane_row_digests.iter().cloned()),
            normalized_matrix_digest(lane_rows.iter().zip(lane_row_digests.into_iter()).map(
                |(row, digest): (&PrimitiveConstructionCorpusReplaySiegeRow, String)| {
                    (row.scenario_id().to_string(), digest)
                },
            )),
        ));
    }
    Ok(rows)
}

fn branch_basis_workspace_for(scenario_id: &str) -> ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        format!("worth-kernel.corpus-branch-basis.{scenario_id}"),
    )
    .expect("branch basis workspace")
}

fn scenario_for(
    row: &PrimitiveConstructionCorpusReplaySiegeRow,
) -> Result<PrimitiveConstructionCorpusScenario, PrimitiveConstructionCorpusReplaySiegeError> {
    primitive_construction_corpus()
        .into_iter()
        .find(|scenario| {
            scenario.scenario_id == row.scenario_id()
                && scenario.family == row.family()
                && scenario.parameter_role == row.parameter_role()
        })
        .ok_or_else(|| {
            PrimitiveConstructionCorpusReplaySiegeError::QueryRuntime(
                forge_query::facade::ForgeQueryRuntimeError::MissingDerivedView(format!(
                    "missing replay scenario for {}:{}:{}",
                    row.scenario_id(),
                    row.family().as_str(),
                    row.parameter_role().as_str()
                )),
            )
        })
}

fn admitted_runtime_truth(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<&PrimitiveConstructionAdmittedRuntimeTruth> {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => Some(outcome),
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_) => None,
    }
}

fn rejected_runtime_truth(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<&PrimitiveConstructionRejectedRuntimeTruth> {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(_) => None,
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => Some(rejected),
    }
}

fn runtime_truth_kind(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> &'static str {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(_) => "admitted",
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_) => "rejected",
    }
}

fn attempted_realization_strategies(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> &[PrimitiveRealizationStrategy] {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            outcome.attempted_realization_strategies()
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.attempted_realization_strategies()
        }
    }
}

fn stability_class(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<PrimitiveStabilityClass> {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.stability_class())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.stability_class()
        }
    }
}

fn feature_conditioning_class(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<PrimitiveFeatureConditioningClass> {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.feature_conditioning_class())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.feature_conditioning_class()
        }
    }
}

fn support_normal_class(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<PrimitiveSupportNormalClass> {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.support_normal_class())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.support_normal_class()
        }
    }
}

fn normalization_disposition(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<PrimitiveNormalizationDisposition> {
    match runtime_truth {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
            Some(outcome.normalization_disposition())
        }
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
            rejected.normalization_disposition()
        }
    }
}

fn exhaustion_reason(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<PrimitiveRealizationExhaustionReason> {
    rejected_runtime_truth(runtime_truth)
        .and_then(PrimitiveConstructionRejectedRuntimeTruth::exhaustion_reason)
}

fn rejection_class(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<PrimitiveConstructionRejectionClass> {
    rejected_runtime_truth(runtime_truth)
        .map(PrimitiveConstructionRejectedRuntimeTruth::rejection_class)
}

fn rejection_locality(
    runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth,
) -> Option<PrimitiveConstructionRejectionLocality> {
    rejected_runtime_truth(runtime_truth)
        .map(PrimitiveConstructionRejectedRuntimeTruth::rejection_locality)
}

fn construction_breadth(runtime_truth: &PrimitiveConstructionCertificationRuntimeTruth) -> usize {
    admitted_runtime_truth(runtime_truth)
        .map(PrimitiveConstructionAdmittedRuntimeTruth::topology_fact_breadth)
        .unwrap_or(0)
}

fn lane_digest(row_digests: impl IntoIterator<Item = String>) -> String {
    sealed_report_identity(
        "worth-kernel.construction.corpus-replay",
        "authoring-order-lane",
        |report| report.value_sequence_participating("row-identities", row_digests),
    )
}

fn normalized_matrix_digest(row_pairs: impl IntoIterator<Item = (String, String)>) -> String {
    let mut parts = row_pairs
        .into_iter()
        .map(|(scenario_id, row_digest)| format!("{scenario_id}:{row_digest}"))
        .collect::<Vec<_>>();
    parts.sort();
    sealed_report_identity(
        "worth-kernel.construction.corpus-replay",
        "normalized-authoring-order-matrix",
        |report| report.value_sequence_participating("scenario-row-identities", parts),
    )
}
