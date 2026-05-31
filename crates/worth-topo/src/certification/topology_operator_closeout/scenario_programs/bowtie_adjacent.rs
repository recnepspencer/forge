use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use serde_json::Value;

use super::super::report::{
    MilestoneThreeBowtieAdjacentWitness, MilestoneThreeEditReplayStepRow,
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport,
};
use super::super::shared::{
    accepted_step_row_for_declaration, derived_validation_report_from_materialized,
    entity_id_from_query_identity, first_source_identity_for_relation_kind,
    rejected_step_row_for_declaration, relation_id_from_query_identity, replay_checked,
    replay_checked_rejected,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    application::TopologyDeclarationContractPayload, NamingEditContinuityMatrix,
    RejectedEditScopeReport, TopologyEditDigest, TopologyEditFamily, TopologyEditRejectionClass,
    TopologySpliceRadialAdjacencyDeclaration,
};

struct MilestoneThreeBowtieAdjacentRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    topology_edit_digest: TopologyEditDigest,
    naming_edit_continuity_matrix: NamingEditContinuityMatrix,
    step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: Option<crate::certification::DeterministicDigest>,
    outcome_class: MilestoneThreeHostileOutcomeClass,
    rejection_class: Option<TopologyEditRejectionClass>,
    rejected_edit_scope_report: Option<RejectedEditScopeReport>,
    derived_validation_report: Option<crate::validation::DerivedTopologyValidationReport>,
    derived_materialization_fallback_class:
        Option<crate::derived_topology::materialized_graph::MaterializationFallbackClass>,
    witness: MilestoneThreeBowtieAdjacentWitness,
    detail: String,
}

pub(crate) fn certify_milestone_three_bowtie_adjacent_rewire_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_bowtie_adjacent_rewire(&mut runtime_factory, stem)?;
    let replay = execute_bowtie_adjacent_rewire(&mut runtime_factory, stem)?;
    let replay_report = match (
        left.outcome_class,
        left.final_materialized_topology_digest.clone(),
        replay.final_materialized_topology_digest.clone(),
    ) {
        (
            MilestoneThreeHostileOutcomeClass::Accepted,
            Some(final_materialized_topology_digest),
            Some(replay_final_materialized_topology_digest),
        ) => replay_checked(
            left.step_rows.clone(),
            replay.step_rows.clone(),
            left.baseline_materialized_topology_digest.clone(),
            final_materialized_topology_digest,
            replay_final_materialized_topology_digest,
        ),
        (MilestoneThreeHostileOutcomeClass::Rejected, _, _) => replay_checked_rejected(
            left.step_rows.clone(),
            replay.step_rows.clone(),
            left.baseline_materialized_topology_digest.clone(),
        ),
        _ => {
            return Err(TopologyCertificationError::Query(
                "bowtie adjacent replay should preserve an honest final digest basis".to_string(),
            ))
        }
    };

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::BowtieAdjacentRewire,
        primitive_family: left.primitive_family,
        primitive: left.primitive,
        edit_families: vec![TopologyEditFamily::SpliceRadialAdjacency],
        bowtie_adjacent_witness: Some(left.witness),
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: None,
        broken_radial_witness: None,
        topology_edit_digest: left.topology_edit_digest,
        naming_edit_continuity_matrix: left.naming_edit_continuity_matrix.clone(),
        continuity_outcome_class: left.naming_edit_continuity_matrix.outcome_class(),
        continuity_rejection_class: left.naming_edit_continuity_matrix.rejection_class(),
        outcome_class: left.outcome_class,
        rejection_class: left.rejection_class,
        rejected_edit_scope_report: left.rejected_edit_scope_report,
        derived_validation_report: left.derived_validation_report,
        derived_materialization_fallback_class: left.derived_materialization_fallback_class,
        edit_replay_parity_report: replay_report,
        detail: left.detail,
    })
}

fn execute_bowtie_adjacent_rewire<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeBowtieAdjacentRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified =
        seed_milestone_one_primitive(&mut runtime, &format!("{stem}.bowtie_adjacent"), &primitive)?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{stem}.bowtie_adjacent.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = surfaces
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis())
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);
    let domain_query = TopologyReadProofHarness::new();
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let source_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeRadialNext,
    )?;
    let source_half_edge_id = entity_id_from_query_identity(&source_identity)?;
    let bowtie_adjacent_witness =
        build_bowtie_adjacent_witness(&domain_query, &mut workspace, &source_identity)?;
    let radial = domain_query
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let declaration = TopologySpliceRadialAdjacencyDeclaration::new(
        relation_id_from_query_identity(radial.source_radial_next_relation_identity())?,
        source_half_edge_id,
        entity_id_from_query_identity(bowtie_adjacent_witness.target_half_edge_identity.as_str())?,
    );

    match execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration.clone())
    {
        Ok(execution) => {
            let final_materialized_topology_digest =
                digest_materialized_topology_view(&execution.materialized);
            let derived_validation_report =
                derived_validation_report_from_materialized(&execution.materialized)?;
            Ok(MilestoneThreeBowtieAdjacentRun {
                primitive_family,
                primitive,
                topology_edit_digest: declaration.topology_edit_digest(),
                naming_edit_continuity_matrix: declaration.naming_continuity_matrix(),
                step_rows: vec![accepted_step_row_for_declaration(
                    0,
                    &declaration,
                    &execution,
                )],
                baseline_materialized_topology_digest,
                final_materialized_topology_digest: Some(final_materialized_topology_digest),
                outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
                rejection_class: None,
                rejected_edit_scope_report: None,
                derived_validation_report: Some(derived_validation_report),
                derived_materialization_fallback_class: execution
                    .materialized
                    .report()
                    .fallback_class,
                witness: bowtie_adjacent_witness,
                detail:
                    "bowtie-adjacent rewire committed successfully on the admitted runtime lane"
                        .to_string(),
            })
        }
        Err(error) => Ok(MilestoneThreeBowtieAdjacentRun {
            primitive_family,
            primitive,
            topology_edit_digest: declaration.topology_edit_digest(),
            naming_edit_continuity_matrix: declaration.naming_continuity_matrix(),
            step_rows: vec![rejected_step_row_for_declaration(0, &declaration, &error)],
            baseline_materialized_topology_digest,
            final_materialized_topology_digest: None,
            outcome_class: MilestoneThreeHostileOutcomeClass::Rejected,
            rejection_class: error.rejection_class(),
            rejected_edit_scope_report: error.rejected_declaration_scope_report(&declaration),
            derived_validation_report: None,
            derived_materialization_fallback_class: None,
            witness: bowtie_adjacent_witness,
            detail: error.to_string(),
        }),
    }
}

fn build_bowtie_adjacent_witness(
    domain_query: &TopologyReadProofHarness,
    workspace: &mut ForgeQueryWorkspace,
    source_identity: &str,
) -> Result<MilestoneThreeBowtieAdjacentWitness, TopologyCertificationError> {
    let neighborhood = domain_query
        .shared_vertex_half_edge_neighborhood(workspace, source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let target = neighborhood
        .vertex_adjacent_different_edge_half_edges()
        .iter()
        .find(|candidate| !candidate.shared_vertex_identities().is_empty())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded edge fan should provide a vertex-adjacent halfedge on a different edge"
                    .to_string(),
            )
        })?;
    let shared_vertex_identity = target
        .shared_vertex_identities()
        .first()
        .cloned()
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded edge fan should expose a shared vertex for bowtie-adjacent witness"
                    .to_string(),
            )
        })?;
    let target_half_edge_identity = target.half_edge_identity().to_string();
    let target_edge_identity = target.edge_identity().to_string();
    Ok(MilestoneThreeBowtieAdjacentWitness {
        source_half_edge_identity: source_identity.to_string(),
        target_half_edge_identity,
        source_edge_identity: neighborhood.source_edge_identity,
        target_edge_identity,
        shared_vertex_identity,
    })
}
