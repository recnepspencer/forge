use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};
use serde_json::Value;

use super::super::report::{
    MilestoneThreeBrokenRadialWitness, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
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
use crate::projection::TopologyHalfEdgeSharedVertexNeighborhoodView;
use crate::topology_operators::{
    application::TopologyDeclarationContractPayload, TopologyEditDigest, TopologyEditFamily,
    TopologyEditRejectionClass, TopologySpliceRadialAdjacencyDeclaration,
};

struct MilestoneThreeBrokenRadialRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    topology_edit_digest: TopologyEditDigest,
    naming_edit_continuity_matrix: crate::topology_operators::NamingEditContinuityMatrix,
    step_rows: Vec<super::super::report::MilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: Option<crate::certification::DeterministicDigest>,
    outcome_class: MilestoneThreeHostileOutcomeClass,
    rejection_class: Option<TopologyEditRejectionClass>,
    rejected_edit_scope_report: Option<crate::topology_operators::RejectedEditScopeReport>,
    derived_validation_report: Option<crate::validation::DerivedTopologyValidationReport>,
    derived_materialization_fallback_class:
        Option<crate::derived_topology::materialized_graph::MaterializationFallbackClass>,
    witness: MilestoneThreeBrokenRadialWitness,
    detail: String,
}

pub(crate) fn certify_milestone_three_broken_radial_localization_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_broken_radial_localization(&mut runtime_factory, stem)?;
    let replay = execute_broken_radial_localization(&mut runtime_factory, stem)?;
    let replay_report =
        match (
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
            _ => return Err(TopologyCertificationError::Query(
                "broken radial localization replay should preserve an honest final digest basis"
                    .to_string(),
            )),
        };

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::BrokenRadialLocalization,
        primitive_family: left.primitive_family,
        primitive: left.primitive,
        edit_families: vec![TopologyEditFamily::SpliceRadialAdjacency],
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: None,
        broken_radial_witness: Some(left.witness),
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

fn execute_broken_radial_localization<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeBrokenRadialRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.broken_radial_localization"),
        &primitive,
    )?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        &format!("{stem}.broken_radial_localization.runtime"),
    )
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
    let radial = domain_query
        .radial_half_edge_neighborhood(&mut workspace, &source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let shared_vertex = domain_query
        .shared_vertex_half_edge_neighborhood(&mut workspace, &source_identity)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let current_target_identity = radial.current_target_half_edge_identity().to_string();
    let source_radial_next_relation_identity =
        radial.source_radial_next_relation_identity().to_string();
    let witness = build_broken_radial_witness(radial, shared_vertex)?;
    let illegal_target_half_edge_id =
        entity_id_from_query_identity(&witness.illegal_target_half_edge_identity)?;
    let declaration = TopologySpliceRadialAdjacencyDeclaration::new(
        relation_id_from_query_identity(&source_radial_next_relation_identity)?,
        source_half_edge_id,
        illegal_target_half_edge_id,
    );

    match execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration.clone())
    {
        Ok(execution) => {
            let detail = format!(
                "radial splice from `{source_identity}` to illegal target `{}` unexpectedly admitted from current target `{current_target_identity}`",
                witness.illegal_target_half_edge_identity
            );
            let derived_validation_report =
                derived_validation_report_from_materialized(&execution.materialized)?;
            Ok(MilestoneThreeBrokenRadialRun {
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
                final_materialized_topology_digest: Some(digest_materialized_topology_view(
                    &execution.materialized,
                )),
                outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
                rejection_class: None,
                rejected_edit_scope_report: None,
                derived_validation_report: Some(derived_validation_report),
                derived_materialization_fallback_class: execution
                    .materialized
                    .report()
                    .fallback_class,
                witness,
                detail,
            })
        }
        Err(error) => Ok(MilestoneThreeBrokenRadialRun {
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
            witness,
            detail: error.to_string(),
        }),
    }
}

fn build_broken_radial_witness(
    radial: crate::projection::TopologyHalfEdgeRadialNeighborhoodView,
    shared_vertex: TopologyHalfEdgeSharedVertexNeighborhoodView,
) -> Result<MilestoneThreeBrokenRadialWitness, TopologyCertificationError> {
    let illegal_target = shared_vertex
        .vertex_adjacent_different_edge_half_edges()
        .iter()
        .find(|candidate| {
            candidate.half_edge_identity() != radial.current_target_half_edge_identity()
        })
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "seeded edge fan should expose an illegal radial target on a different edge"
                    .to_string(),
            )
        })?;
    let illegal_target_half_edge_identity = illegal_target.half_edge_identity().to_string();
    let illegal_target_edge_identity = illegal_target.edge_identity().to_string();
    Ok(MilestoneThreeBrokenRadialWitness {
        source_half_edge_identity: radial.source_half_edge_identity().to_string(),
        current_target_half_edge_identity: radial.current_target_half_edge_identity,
        illegal_target_half_edge_identity,
        source_edge_identity: radial.source_edge_identity,
        current_target_edge_identity: radial.current_target_edge_identity,
        illegal_target_edge_identity,
    })
}
