use forge_relational::facade::runtime::RelationalRuntime;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::WorthTopologyRelationKind;

use super::report::{
    WorthMilestoneThreeBrokenRadialWitness, WorthMilestoneThreeHostileOutcomeClass,
    WorthMilestoneThreeHostileScenario, WorthMilestoneThreeHostileScenarioReport,
};
use super::shared::{
    accepted_step_row, aggregate_naming_edit_continuity_matrix, aggregate_topology_edit_digest,
    rejected_step_row, replay_checked, replay_checked_rejected,
};
use crate::certification::error::WorthTopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::edit::{
    WorthTopologyEditApplicationMode, WorthTopologyEditBatch, WorthTopologyEditContract,
    WorthTopologyEditDigest, WorthTopologyEditFamily, WorthTopologyEditRejectionClass,
};
use crate::parity::digest_materialized_topology_view;
use crate::query::{
    worth_topology_runtime, WorthTopologyDomainQuery, WorthTopologyQueryAssembly,
    WorthTopologyRuntimeAdapters,
};

struct WorthMilestoneThreeBrokenRadialRun {
    primitive_family: String,
    primitive: WorthMilestoneOnePrimitiveCase,
    topology_edit_digest: WorthTopologyEditDigest,
    naming_edit_continuity_matrix: crate::edit::WorthNamingEditContinuityMatrix,
    step_rows: Vec<super::report::WorthMilestoneThreeEditReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::WorthDeterministicDigest,
    final_materialized_topology_digest: Option<crate::certification::WorthDeterministicDigest>,
    outcome_class: WorthMilestoneThreeHostileOutcomeClass,
    rejection_class: Option<WorthTopologyEditRejectionClass>,
    rejected_edit_scope_report: Option<crate::edit::WorthRejectedEditScopeReport>,
    witness: WorthMilestoneThreeBrokenRadialWitness,
    detail: String,
}

pub(crate) fn certify_milestone_three_broken_radial_localization_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneThreeHostileScenarioReport, WorthTopologyCertificationError>
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
                WorthMilestoneThreeHostileOutcomeClass::Accepted,
                Some(final_materialized_topology_digest),
                Some(replay_final_materialized_topology_digest),
            ) => replay_checked(
                left.step_rows.clone(),
                replay.step_rows.clone(),
                left.baseline_materialized_topology_digest.clone(),
                final_materialized_topology_digest,
                replay_final_materialized_topology_digest,
            ),
            (WorthMilestoneThreeHostileOutcomeClass::Rejected, _, _) => replay_checked_rejected(
                left.step_rows.clone(),
                replay.step_rows.clone(),
                left.baseline_materialized_topology_digest.clone(),
            ),
            _ => return Err(WorthTopologyCertificationError::Query(
                "broken radial localization replay should preserve an honest final digest basis"
                    .to_string(),
            )),
        };

    Ok(WorthMilestoneThreeHostileScenarioReport {
        scenario: WorthMilestoneThreeHostileScenario::BrokenRadialLocalization,
        primitive_family: left.primitive_family,
        primitive: left.primitive,
        edit_families: vec![WorthTopologyEditFamily::SpliceRadialAdjacency],
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        broken_radial_witness: Some(left.witness),
        topology_edit_digest: left.topology_edit_digest,
        naming_edit_continuity_matrix: left.naming_edit_continuity_matrix.clone(),
        continuity_outcome_class: left.naming_edit_continuity_matrix.outcome_class(),
        continuity_rejection_class: left.naming_edit_continuity_matrix.rejection_class(),
        outcome_class: left.outcome_class,
        rejection_class: left.rejection_class,
        rejected_edit_scope_report: left.rejected_edit_scope_report,
        edit_replay_parity_report: replay_report,
        detail: left.detail,
    })
}

fn execute_broken_radial_localization<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<WorthMilestoneThreeBrokenRadialRun, WorthTopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        &format!("{stem}.broken_radial_localization"),
        &primitive,
    )?;
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        &format!("{stem}.broken_radial_localization.runtime"),
    )
    .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let baseline_snapshot = assembly
        .snapshot_for_read_basis(&mut workspace, &verified.read_basis)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_snapshot.materialized);
    let domain_query = WorthTopologyDomainQuery::load(&workspace, &assembly)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let source_identity = domain_query
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeRadialNext)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let current_target_identity = domain_query
        .outgoing_target_identity(
            &source_identity,
            WorthTopologyRelationKind::HalfEdgeRadialNext,
        )
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let source_half_edge_id = domain_query
        .find_entity_id_by_identity(&source_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let witness = build_broken_radial_witness(&domain_query, &source_identity)?;
    let illegal_target_half_edge_id = domain_query
        .find_entity_id_by_identity(&witness.illegal_target_half_edge_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::splice_radial_adjacency(
            domain_query
                .relation_id_for_source_kind(
                    &source_identity,
                    WorthTopologyRelationKind::HalfEdgeRadialNext,
                )
                .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?,
            source_half_edge_id,
            illegal_target_half_edge_id,
        )])
        .expect("broken radial localization batch should be non-empty");
    let batches = vec![batch.clone()];

    match assembly.apply_edit(
        &mut workspace,
        batch.clone(),
        WorthTopologyEditApplicationMode::Mainline,
    ) {
        Ok(execution) => {
            let detail = format!(
                "radial splice from `{source_identity}` to illegal target `{}` unexpectedly admitted from current target `{current_target_identity}`",
                witness.illegal_target_half_edge_identity
            );
            Ok(WorthMilestoneThreeBrokenRadialRun {
                primitive_family,
                primitive,
                topology_edit_digest: aggregate_topology_edit_digest(&batches),
                naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
                step_rows: vec![accepted_step_row(0, &batch, &execution)],
                baseline_materialized_topology_digest,
                final_materialized_topology_digest: Some(digest_materialized_topology_view(
                    &execution.materialized,
                )),
                outcome_class: WorthMilestoneThreeHostileOutcomeClass::Accepted,
                rejection_class: None,
                rejected_edit_scope_report: None,
                witness,
                detail,
            })
        }
        Err(error) => Ok(WorthMilestoneThreeBrokenRadialRun {
            primitive_family,
            primitive,
            topology_edit_digest: aggregate_topology_edit_digest(&batches),
            naming_edit_continuity_matrix: aggregate_naming_edit_continuity_matrix(&batches),
            step_rows: vec![rejected_step_row(0, &batch, &error)],
            baseline_materialized_topology_digest,
            final_materialized_topology_digest: None,
            outcome_class: WorthMilestoneThreeHostileOutcomeClass::Rejected,
            rejection_class: error.rejection_class(),
            rejected_edit_scope_report: error.rejected_edit_scope_report(&batch),
            witness,
            detail: error.to_string(),
        }),
    }
}

fn build_broken_radial_witness(
    domain_query: &WorthTopologyDomainQuery,
    source_identity: &str,
) -> Result<WorthMilestoneThreeBrokenRadialWitness, WorthTopologyCertificationError> {
    let radial = domain_query
        .radial_half_edge_neighborhood(source_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;
    let illegal_target_half_edge_identity = radial
        .different_edge_half_edge_identities
        .into_iter()
        .find(|identity| identity != &radial.current_target_half_edge_identity)
        .ok_or_else(|| {
            WorthTopologyCertificationError::Query(
                "seeded edge fan should expose an illegal radial target on a different edge"
                    .to_string(),
            )
        })?;
    let illegal_target_edge_identity = domain_query
        .edge_identity_of_half_edge(&illegal_target_half_edge_identity)
        .map_err(|error| WorthTopologyCertificationError::Query(error.to_string()))?;

    Ok(WorthMilestoneThreeBrokenRadialWitness {
        source_half_edge_identity: source_identity.to_string(),
        current_target_half_edge_identity: radial.current_target_half_edge_identity,
        illegal_target_half_edge_identity,
        source_edge_identity: radial.source_edge_identity,
        current_target_edge_identity: radial.current_target_edge_identity,
        illegal_target_edge_identity,
    })
}
