use std::collections::BTreeSet;

use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde_json::Value;

use super::super::mutation_sequence_support::aggregate_topology_mutation_digest_for_declarations;
use super::super::report::MilestoneThreeHostileSuiteReport;
use super::super::shared::{
    derived_validation_report_from_materialized, first_source_identity_for_relation_kind,
};
use super::scale_pressure_branch::certify_large_branch_history_row;
use super::scale_pressure_detach::{certify_detach_pressure_row, DetachPressureKind};
use super::scale_pressure_loop::high_cardinality_loop_declarations;
use super::scale_pressure_shell::high_face_count_shell_rehome_declaration;
use super::scale_pressure_types::{
    MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep,
};
use super::sweeps::certify_radial_splice_pressure_row;
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyDeclaredMutationSequence, TopologyMutationDigest,
    TopologyMutationFamily, TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopSuccessorProgramDeclaration,
};

#[derive(Debug, Clone)]
struct ScaleRewireCase {
    sweep: MilestoneThreeScalePressureSweep,
    primitive: MilestoneOnePrimitiveCase,
    cycle_depth: usize,
    candidate_offset: usize,
    workload_size: usize,
}

struct ScaleRewireExecution {
    primitive_family: String,
    topology_mutation_digest: TopologyMutationDigest,
    mutation_families: Vec<TopologyMutationFamily>,
    final_state_digest: String,
    derived_validation_row_count: usize,
}

#[derive(Clone)]
enum ScaleRewireDeclaration {
    LoopSuccessor(TopologyRewireLoopSuccessorProgramDeclaration),
    LoopEndpoint(TopologyRewireLoopEndpointDeclaration),
    ShellRehome(TopologyRehomeAllOwnedFacesToNewShellDeclaration),
}

impl ScaleRewireDeclaration {
    fn semantic_families(&self) -> Vec<TopologyMutationFamily> {
        match self {
            Self::LoopSuccessor(declaration) => declaration.semantic_families(),
            Self::LoopEndpoint(declaration) => declaration.semantic_families(),
            Self::ShellRehome(declaration) => declaration.semantic_families(),
        }
    }
}

impl TopologyDeclarationMutationPayload for ScaleRewireDeclaration {
    const SEMANTIC_FAMILY_KEY: &'static str = "topology.scale_rewire_declaration";

    fn into_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        match self {
            Self::LoopSuccessor(declaration) => declaration.into_mutation_sequence(),
            Self::LoopEndpoint(declaration) => declaration.into_mutation_sequence(),
            Self::ShellRehome(declaration) => declaration.into_mutation_sequence(),
        }
    }
}

pub(in crate::certification::topology_operator_closeout) fn certify_milestone_three_scale_pressure_impl<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<Vec<MilestoneThreeScalePressureRow>, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut rows = scale_rewire_cases()
        .iter()
        .map(|case| certify_scale_rewire_row(runtime_factory, stem, case.clone()))
        .collect::<Result<Vec<_>, _>>()?;
    rows.push(certify_large_branch_history_row(runtime_factory, stem)?);
    rows.push(certify_detach_pressure_row(
        runtime_factory,
        stem,
        MilestoneThreeScalePressureSweep::WireMembershipDetach,
        MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
        TopologyRelationKind::WireOwnsHalfEdge,
        DetachPressureKind::ShellOrWire(ShellOrWireMembershipKind::WireOwnsHalfEdge),
    )?);
    rows.push(certify_radial_splice_pressure_row(runtime_factory, stem)?);
    rows.push(certify_detach_pressure_row(
        runtime_factory,
        stem,
        MilestoneThreeScalePressureSweep::RadialAdjacencyDetach,
        MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
        TopologyRelationKind::HalfEdgeRadialNext,
        DetachPressureKind::Radial,
    )?);
    Ok(rows)
}

pub(in crate::certification::topology_operator_closeout) fn ensure_scale_pressure_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    let covered = covered_scale_pressure_sweeps(report);
    for sweep in required_scale_pressure_sweeps() {
        if !covered.contains(sweep) {
            return Err(scale_pressure_error(&format!(
                "missing scale pressure sweep {}",
                sweep.as_str()
            )));
        }
    }
    for row in &report.scale_pressure_rows {
        if !row.replay_verified
            || row.topology_mutation_digest.mutation_record_count == 0
            || row.workload_size == 0
            || row.mutation_step_count == 0
            || row.final_state_digest != row.replay_final_state_digest
        {
            return Err(scale_pressure_error(&format!(
                "scale pressure row is not proof-bearing for {}",
                row.sweep.as_str()
            )));
        }
    }
    Ok(())
}

pub(in crate::certification::topology_operator_closeout) fn scale_pressure_labels(
    report: &MilestoneThreeHostileSuiteReport,
) -> (usize, Vec<String>, Vec<String>) {
    let covered = covered_scale_pressure_sweeps(report);
    let evidence_labels = report
        .scale_pressure_rows
        .iter()
        .filter(|row| row.replay_verified)
        .map(|row| format!("scale_pressure={}", row.sweep.as_str()))
        .collect::<Vec<_>>();
    let gap_labels = required_scale_pressure_sweeps()
        .iter()
        .filter(|sweep| !covered.contains(sweep))
        .map(|sweep| format!("missing_scale_sweep={}", sweep.as_str()))
        .collect::<Vec<_>>();
    (covered.len(), evidence_labels, gap_labels)
}

fn certify_scale_rewire_row<F>(
    runtime_factory: &mut F,
    stem: &str,
    case: ScaleRewireCase,
) -> Result<MilestoneThreeScalePressureRow, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_scale_rewire(runtime_factory, stem, &case, true)?;
    let replay = execute_scale_rewire(runtime_factory, stem, &case, false)?;
    let replay_verified = left.topology_mutation_digest == replay.topology_mutation_digest
        && left.final_state_digest == replay.final_state_digest;
    Ok(MilestoneThreeScalePressureRow {
        sweep: case.sweep,
        primitive_family: left.primitive_family,
        primitive: case.primitive,
        workload_size: case.workload_size,
        mutation_step_count: left.topology_mutation_digest.mutation_record_count,
        mutation_families: left.mutation_families,
        branch_local: false,
        branch_authoring_boundary: None,
        topology_mutation_digest: left.topology_mutation_digest,
        replay_verified,
        final_state_digest: left.final_state_digest.clone(),
        replay_final_state_digest: replay.final_state_digest,
        derived_validation_row_count: left.derived_validation_row_count,
        row_digest: scale_pressure_row_digest(case.sweep, replay_verified, case.workload_size),
    })
}

fn execute_scale_rewire<F>(
    runtime_factory: &mut F,
    stem: &str,
    case: &ScaleRewireCase,
    derive_validation: bool,
) -> Result<ScaleRewireExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_family = primitive_family_name(&case.primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        &format!("{stem}.scale_pressure.{}", case.sweep.as_str()),
        &case.primitive,
    )?;
    let high_face_shell_declaration =
        if case.sweep == MilestoneThreeScalePressureSweep::HighFaceCountShells {
            Some(high_face_count_shell_rehome_declaration(
                &runtime,
                &verified.read_basis(),
                stem,
                case.workload_size,
            )?)
        } else {
            None
        };
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, format!("{stem}.scale_pressure.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let declarations = scale_rewire_declarations(
        &mut workspace,
        &relation_rows,
        high_face_shell_declaration,
        case,
    )?;
    scale_rewire_execution(
        primitive_family,
        &mut workspace,
        &surfaces,
        declarations,
        derive_validation,
    )
}

fn scale_rewire_declarations(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    high_face_shell_declaration: Option<TopologyRehomeAllOwnedFacesToNewShellDeclaration>,
    case: &ScaleRewireCase,
) -> Result<Vec<ScaleRewireDeclaration>, TopologyCertificationError> {
    let declarations = if let Some(declaration) = high_face_shell_declaration {
        vec![ScaleRewireDeclaration::ShellRehome(declaration)]
    } else if case.sweep == MilestoneThreeScalePressureSweep::HighCardinalityLoops {
        let moved_half_edge_identity = first_source_identity_for_relation_kind(
            &relation_rows,
            TopologyRelationKind::HalfEdgeNext,
        )?;
        let (span, endpoint) = high_cardinality_loop_declarations(
            workspace,
            relation_rows,
            &moved_half_edge_identity,
        )?;
        vec![
            ScaleRewireDeclaration::LoopSuccessor(span),
            ScaleRewireDeclaration::LoopEndpoint(endpoint),
        ]
    } else {
        vec![ScaleRewireDeclaration::LoopSuccessor(
            local_successor_declaration(
                workspace,
                relation_rows,
                case.cycle_depth,
                case.candidate_offset,
            )?,
        )]
    };
    Ok(declarations)
}

fn local_successor_declaration(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    cycle_depth: usize,
    candidate_offset: usize,
) -> Result<TopologyRewireLoopSuccessorProgramDeclaration, TopologyCertificationError> {
    let moved_half_edge_identity =
        first_source_identity_for_relation_kind(relation_rows, TopologyRelationKind::HalfEdgeNext)?;
    let neighborhood = TopologyReadProofHarness::current_head()
        .local_rewire_neighborhood(workspace, &moved_half_edge_identity, cycle_depth)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let chosen_successor_identity =
        super::super::scenario_programs::successor_candidate_with_retained_predecessor(
            &neighborhood,
            candidate_offset,
            "scale rewire",
        )?;
    super::super::scenario_programs::successor_relocation_declaration(
        &neighborhood,
        &chosen_successor_identity,
    )
}

fn scale_rewire_execution(
    primitive_family: String,
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    declarations: Vec<ScaleRewireDeclaration>,
    derive_validation: bool,
) -> Result<ScaleRewireExecution, TopologyCertificationError> {
    let topology_mutation_digest =
        aggregate_topology_mutation_digest_for_declarations(declarations.clone());
    let mutation_families = declarations
        .iter()
        .flat_map(ScaleRewireDeclaration::semantic_families)
        .collect();
    let mut final_state_digest = String::new();
    let mut final_materialized = None;
    for declaration in declarations {
        let execution = execute_scale_rewire_declaration(workspace, surfaces, declaration)?;
        let materialized = execution.into_materialized();
        final_state_digest = digest_materialized_topology_view(&materialized).digest_hex;
        final_materialized = Some(materialized);
    }
    let final_materialized = final_materialized
        .ok_or_else(|| scale_pressure_error("scale pressure executed no batches"))?;
    let derived_validation_row_count = if derive_validation {
        derived_validation_report_from_materialized(&final_materialized)?
            .rows
            .len()
    } else {
        0
    };
    Ok(ScaleRewireExecution {
        primitive_family,
        topology_mutation_digest,
        mutation_families,
        final_state_digest,
        derived_validation_row_count,
    })
}

fn execute_scale_rewire_declaration(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
    declaration: ScaleRewireDeclaration,
) -> Result<
    crate::topology_operators::application::TopologyDeclaredMutationArtifact,
    TopologyCertificationError,
> {
    match declaration {
        ScaleRewireDeclaration::LoopSuccessor(declaration) => {
            execute_current_head_topology_declaration(workspace, surfaces, declaration)
        }
        ScaleRewireDeclaration::LoopEndpoint(declaration) => {
            execute_current_head_topology_declaration(workspace, surfaces, declaration)
        }
        ScaleRewireDeclaration::ShellRehome(declaration) => {
            execute_current_head_topology_declaration(workspace, surfaces, declaration)
        }
    }
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

fn covered_scale_pressure_sweeps(
    report: &MilestoneThreeHostileSuiteReport,
) -> BTreeSet<MilestoneThreeScalePressureSweep> {
    report
        .scale_pressure_rows
        .iter()
        .filter(|row| row.replay_verified)
        .map(|row| row.sweep)
        .collect()
}

fn required_scale_pressure_sweeps() -> &'static [MilestoneThreeScalePressureSweep] {
    &[
        MilestoneThreeScalePressureSweep::HighCardinalityLoops,
        MilestoneThreeScalePressureSweep::HighFaceCountShells,
        MilestoneThreeScalePressureSweep::LargeBranchLocalHistories,
        MilestoneThreeScalePressureSweep::RadialAdjacencySplice,
    ]
}

fn scale_rewire_cases() -> &'static [ScaleRewireCase] {
    &[
        ScaleRewireCase {
            sweep: MilestoneThreeScalePressureSweep::HighCardinalityLoops,
            primitive: MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 8 },
            cycle_depth: 5,
            candidate_offset: 3,
            workload_size: 8,
        },
        ScaleRewireCase {
            sweep: MilestoneThreeScalePressureSweep::HighFaceCountShells,
            primitive: MilestoneOnePrimitiveCase::SolidShell { face_count: 8 },
            cycle_depth: 3,
            candidate_offset: 2,
            workload_size: 8,
        },
    ]
}

fn scale_pressure_row_digest(
    sweep: MilestoneThreeScalePressureSweep,
    replay_verified: bool,
    workload_size: usize,
) -> String {
    format!(
        "scale_pressure={};replay_verified={replay_verified};workload_size={workload_size}",
        sweep.as_str()
    )
}

fn scale_pressure_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!("milestone three scale pressure failed: {reason}"))
}
