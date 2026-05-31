use std::collections::BTreeSet;

use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use serde_json::Value;

use super::super::hostile_categories::milestone_three_expected_primitive_family_labels;
use super::super::report::MilestoneThreeHostileSuiteReport;
use super::super::scenario_programs::successor_relocation_declaration;
use super::super::shared::{
    derived_validation_report_from_materialized, first_source_identity_for_relation_kind,
};
use super::primitive_family_closure_types::MilestoneThreePrimitiveFamilyClosureRow;
use super::primitive_family_wire_closure::execute_wire_split_collapse_primitive_closure;
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::TopologyMutationDigest;

#[derive(Debug, Clone)]
struct PrimitiveClosureCase {
    primitive: MilestoneOnePrimitiveCase,
    mutation_program: PrimitiveClosureMutationProgram,
}

#[derive(Debug, Clone)]
enum PrimitiveClosureMutationProgram {
    LoopSuccessorRewire {
        cycle_depth: usize,
        candidate_offset: usize,
    },
    WireSplitCollapse {
        split_offset: usize,
    },
}

pub(in crate::certification::topology_operator_closeout) fn certify_milestone_three_primitive_family_closure_impl<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<Vec<MilestoneThreePrimitiveFamilyClosureRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    primitive_closure_cases()
        .iter()
        .map(|case| certify_primitive_family_closure_row(runtime_factory, stem, case.clone()))
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_primitive_family_closure_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for row in &report.primitive_family_closure_rows {
        if !row.replay_verified
            || row.topology_mutation_digest.mutation_record_count == 0
            || row.derived_validation_row_count == 0
            || row.final_materialized_topology_digest
                != row.replay_final_materialized_topology_digest
        {
            return Err(primitive_family_closure_error(&format!(
                "primitive closure row is not proof-bearing for {}",
                row.primitive_family
            )));
        }
    }

    let covered = covered_primitive_families(report);
    for family in milestone_three_expected_primitive_family_labels() {
        if !covered.contains(*family) {
            return Err(primitive_family_closure_error(&format!(
                "missing primitive family closure for {family}"
            )));
        }
    }
    for family in direct_primitive_closure_family_labels() {
        if !has_direct_proof_bearing_primitive_closure_row(report, family) {
            return Err(primitive_family_closure_error(&format!(
                "missing direct primitive family mutation closure row for {family}"
            )));
        }
    }
    Ok(())
}

pub(in crate::certification::topology_operator_closeout) fn primitive_family_closure_labels(
    report: &MilestoneThreeHostileSuiteReport,
) -> (usize, Vec<String>, Vec<String>) {
    let covered = covered_primitive_families(report);
    let mut evidence_labels = covered
        .iter()
        .map(|family| format!("covered_family={family}"))
        .collect::<Vec<_>>();
    evidence_labels.extend(
        report
            .primitive_family_closure_rows
            .iter()
            .filter(|row| row.replay_verified)
            .map(|row| format!("primitive_family_mutation_closure={}", row.primitive_family)),
    );
    evidence_labels.sort();
    evidence_labels.dedup();

    let gap_labels = milestone_three_expected_primitive_family_labels()
        .iter()
        .filter(|family| !covered.contains(**family))
        .map(|family| format!("missing_family={family}"))
        .collect::<Vec<_>>();
    (covered.len(), evidence_labels, gap_labels)
}

fn certify_primitive_family_closure_row<F>(
    runtime_factory: &mut F,
    stem: &str,
    case: PrimitiveClosureCase,
) -> Result<MilestoneThreePrimitiveFamilyClosureRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_family_label = primitive_family_name(&case.primitive).to_string();
    let left = execute_primitive_family_closure(runtime_factory, stem, case.clone())?;
    let replay = execute_primitive_family_closure(runtime_factory, stem, case.clone())?;
    let replay_verified = left.final_materialized_topology_digest
        == replay.final_materialized_topology_digest
        && left.topology_mutation_digest == replay.topology_mutation_digest
        && left.mutation_families == replay.mutation_families
        && left.derived_validation_row_count == replay.derived_validation_row_count;
    let mutation_record_count = left.topology_mutation_digest.mutation_record_count;
    Ok(MilestoneThreePrimitiveFamilyClosureRow {
        primitive_family: left.primitive_family,
        primitive: case.primitive,
        mutation_families: left.mutation_families,
        topology_mutation_digest: left.topology_mutation_digest,
        replay_verified,
        final_materialized_topology_digest: left.final_materialized_topology_digest,
        replay_final_materialized_topology_digest: replay.final_materialized_topology_digest,
        derived_validation_row_count: left.derived_validation_row_count,
        row_digest: format!(
            "primitive_family={};replay_verified={replay_verified};mutation_records={};derived_validation_rows={}",
            primitive_family_label,
            mutation_record_count,
            left.derived_validation_row_count
        ),
    })
}

pub(in crate::certification::topology_operator_closeout) struct PrimitiveClosureExecution {
    pub(super) primitive_family: String,
    pub(super) mutation_families: Vec<crate::topology_operators::TopologyMutationFamily>,
    pub(super) topology_mutation_digest: TopologyMutationDigest,
    pub(super) final_materialized_topology_digest: crate::certification::DeterministicDigest,
    pub(super) derived_validation_row_count: usize,
}

fn execute_primitive_family_closure<F>(
    runtime_factory: &mut F,
    stem: &str,
    case: PrimitiveClosureCase,
) -> Result<PrimitiveClosureExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    match case.mutation_program {
        PrimitiveClosureMutationProgram::LoopSuccessorRewire {
            cycle_depth,
            candidate_offset,
        } => execute_loop_successor_primitive_closure(
            runtime_factory,
            stem,
            case.primitive,
            cycle_depth,
            candidate_offset,
        ),
        PrimitiveClosureMutationProgram::WireSplitCollapse { split_offset } => {
            execute_wire_split_collapse_primitive_closure(
                runtime_factory,
                stem,
                case.primitive,
                split_offset,
            )
        }
    }
}

fn execute_loop_successor_primitive_closure<F>(
    runtime_factory: &mut F,
    stem: &str,
    primitive: MilestoneOnePrimitiveCase,
    cycle_depth: usize,
    candidate_offset: usize,
) -> Result<PrimitiveClosureExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.primitive_family_closure.{primitive_family}"),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        format!("{stem}.primitive_family_closure.{primitive_family}.runtime"),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &workspace.read::<Value>(surfaces.relations()),
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let domain_query = TopologyReadProofHarness::new();
    let neighborhood = domain_query
        .local_rewire_neighborhood(&mut workspace, &moved_half_edge_identity, cycle_depth)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let chosen_successor_identity = neighborhood
        .cycle_identities()
        .get(candidate_offset)
        .cloned()
        .ok_or_else(|| {
            primitive_family_closure_error("primitive closure candidate should exist in cycle")
        })?;
    let declaration = successor_relocation_declaration(&neighborhood, &chosen_successor_identity)?;
    let topology_mutation_digest = declaration.topology_mutation_digest();
    let mutation_families = declaration.semantic_families();
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let validation = derived_validation_report_from_materialized(&execution.materialized)?;
    Ok(PrimitiveClosureExecution {
        primitive_family,
        mutation_families,
        topology_mutation_digest,
        final_materialized_topology_digest: digest_materialized_topology_view(
            &execution.materialized,
        ),
        derived_validation_row_count: validation.rows.len(),
    })
}

fn covered_primitive_families(report: &MilestoneThreeHostileSuiteReport) -> BTreeSet<&str> {
    let mut families = report
        .scenario_reports
        .iter()
        .map(|scenario| scenario.primitive_family.as_str())
        .collect::<BTreeSet<_>>();
    families.extend(
        report
            .primitive_family_closure_rows
            .iter()
            .filter(|row| row.replay_verified)
            .map(|row| row.primitive_family.as_str()),
    );
    families
}

fn has_direct_proof_bearing_primitive_closure_row(
    report: &MilestoneThreeHostileSuiteReport,
    family: &str,
) -> bool {
    report.primitive_family_closure_rows.iter().any(|row| {
        row.primitive_family == family
            && row.replay_verified
            && row.topology_mutation_digest.mutation_record_count > 0
            && row.derived_validation_row_count > 0
    })
}

fn direct_primitive_closure_family_labels() -> &'static [&'static str] {
    &[
        "WireOpen(n)",
        "WireClosed(n)",
        "SheetDisk(n)",
        "SheetPatch(f)",
        "SolidShell(f)",
    ]
}

fn primitive_closure_cases() -> &'static [PrimitiveClosureCase] {
    &[
        PrimitiveClosureCase {
            primitive: MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
            mutation_program: PrimitiveClosureMutationProgram::WireSplitCollapse {
                split_offset: 2,
            },
        },
        PrimitiveClosureCase {
            primitive: MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 5 },
            mutation_program: PrimitiveClosureMutationProgram::LoopSuccessorRewire {
                cycle_depth: 5,
                candidate_offset: 3,
            },
        },
        PrimitiveClosureCase {
            primitive: MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
            mutation_program: PrimitiveClosureMutationProgram::LoopSuccessorRewire {
                cycle_depth: 5,
                candidate_offset: 3,
            },
        },
        PrimitiveClosureCase {
            primitive: MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
            mutation_program: PrimitiveClosureMutationProgram::LoopSuccessorRewire {
                cycle_depth: 3,
                candidate_offset: 2,
            },
        },
        PrimitiveClosureCase {
            primitive: MilestoneOnePrimitiveCase::SolidShell { face_count: 4 },
            mutation_program: PrimitiveClosureMutationProgram::LoopSuccessorRewire {
                cycle_depth: 3,
                candidate_offset: 2,
            },
        },
    ]
}

pub(in crate::certification::topology_operator_closeout) fn primitive_family_closure_error(
    reason: &str,
) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three primitive family closure failed: {reason}"
    ))
}
