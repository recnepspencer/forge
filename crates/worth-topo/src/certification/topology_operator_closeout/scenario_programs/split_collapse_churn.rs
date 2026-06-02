use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{DerivedTopologyReadBasis, MilestoneOnePrimitiveCase};

use super::super::replay_step_rows::accepted_step_row_for_execution;
use super::super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeMutationReplayStepRow,
    MilestoneThreeScenarioMutationSynopsis, MilestoneThreeSplitCollapseChurnWitness,
};
use super::super::shared::{derived_validation_report_from_materialized, replay_checked};
use super::scenario_mutation_report_lowering::{
    accepted_mutation_synopsis_from_step_rows, accepted_semantic_summary_from_step_rows,
};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::{
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration, TopologyWireRehomeHalfEdgeMember,
    TopologyWireSplitHalfEdgeMember,
};

struct MilestoneThreeSplitCollapseRun {
    primitive_family: String,
    primitive: MilestoneOnePrimitiveCase,
    declared_mutation_synopsis: MilestoneThreeScenarioMutationSynopsis,
    accepted_semantic_summary:
        crate::certification::topology_operator_closeout::report::MilestoneThreeScenarioMutationSemanticSummary,
    step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    baseline_materialized_topology_digest: crate::certification::DeterministicDigest,
    final_materialized_topology_digest: crate::certification::DeterministicDigest,
    derived_validation_report: crate::validation::DerivedTopologyValidationReport,
    derived_materialization_fallback_class:
        Option<crate::derived_topology::materialized_graph::MaterializationFallbackClass>,
    witness: MilestoneThreeSplitCollapseChurnWitness,
}

pub(crate) fn certify_milestone_three_split_collapse_churn_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<MilestoneThreeHostileScenarioReport, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let left = execute_split_collapse_churn(&mut runtime_factory, stem)?;
    let replay = execute_split_collapse_churn(&mut runtime_factory, stem)?;
    let replay_report = replay_checked(
        left.step_rows.clone(),
        replay.step_rows.clone(),
        left.baseline_materialized_topology_digest.clone(),
        left.final_materialized_topology_digest.clone(),
        replay.final_materialized_topology_digest.clone(),
    );
    let semantic_summary = left.accepted_semantic_summary;
    let continuity_outcome_class = semantic_summary.continuity_outcome_class;

    Ok(MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::SplitCollapseChurn,
        primitive_family: left.primitive_family,
        primitive: left.primitive,
        declared_mutation_synopsis: left.declared_mutation_synopsis,
        semantic_summary,
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: Some(left.witness),
        broken_radial_witness: None,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_mutation_scope_report: None,
        derived_validation_report: Some(left.derived_validation_report),
        derived_materialization_fallback_class: left.derived_materialization_fallback_class,
        mutation_replay_parity_report: replay_report,
        detail: format!(
            "connected wire split and owner-retiring collapse churn both executed through query-native membership programs with continuity `{continuity_outcome_class:?}`"
        ),
    })
}

fn execute_split_collapse_churn<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<MilestoneThreeSplitCollapseRun, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive = MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 };
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        &format!("{stem}.split_collapse_churn"),
        &primitive,
    )?;
    let (original_wire_id, half_edge_ids) =
        seeded_wire_and_half_edges(&runtime, &verified.read_basis())?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, &format!("{stem}.split_collapse_churn.runtime"))
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized =
        crate::certification::support::current_head_materialized_topology::current_head_materialized_topology(
            &mut workspace,
            &surfaces,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let baseline_materialized_topology_digest =
        digest_materialized_topology_view(&baseline_materialized);
    let original_wire_identity = wire_identity(
        &baseline_materialized,
        original_wire_id,
        "original split-collapse wire",
    )?;
    let moved_half_edge_ids = vec![half_edge_ids[2], half_edge_ids[3]];
    let retained_half_edge_ids = vec![half_edge_ids[0], half_edge_ids[1]];
    let moved_half_edge_identities =
        half_edge_identities(&baseline_materialized, &moved_half_edge_ids)?;
    let retained_half_edge_identities =
        half_edge_identities(&baseline_materialized, &retained_half_edge_ids)?;

    let split_wire_key = CreateKey::new(&format!("{stem}.split_collapse_churn.split_wire"));
    let split_declaration = split_wire_declaration(split_wire_key.as_str(), &moved_half_edge_ids)?;
    let split_execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        split_declaration.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let split_wire = split_execution
        .materialized()
        .topology()
        .wires
        .iter()
        .find(|wire| wire.label == split_wire_key.as_str())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "split-collapse churn should materialize the split wire".to_string(),
            )
        })?;
    let split_wire_id = split_wire.entity_id;
    let split_wire_half_edge_ids = split_wire.half_edge_ids.clone();
    let split_step_wire_count = split_execution.materialized().topology().wires.len();

    let collapse_wire_key = CreateKey::new(&format!("{stem}.split_collapse_churn.collapse_wire"));
    let collapse_declaration = collapse_split_wire_declaration(
        collapse_wire_key.as_str(),
        split_wire_id,
        &split_wire_half_edge_ids,
    )?;
    let collapse_execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        collapse_declaration.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let final_materialized_topology_digest =
        digest_materialized_topology_view(collapse_execution.materialized());
    let derived_validation_report =
        derived_validation_report_from_materialized(collapse_execution.materialized())?;
    let step_rows = vec![
        accepted_step_row_for_execution(0, &split_execution),
        accepted_step_row_for_execution(1, &collapse_execution),
    ];
    let declared_mutation_synopsis = accepted_mutation_synopsis_from_step_rows(&step_rows);
    let accepted_semantic_summary =
        accepted_semantic_summary_from_step_rows(&step_rows, "accepted split-collapse churn")?;

    Ok(MilestoneThreeSplitCollapseRun {
        primitive_family,
        primitive,
        declared_mutation_synopsis,
        accepted_semantic_summary,
        step_rows,
        baseline_materialized_topology_digest,
        final_materialized_topology_digest,
        derived_validation_report,
        derived_materialization_fallback_class: collapse_execution
            .materialized()
            .report()
            .fallback_class,
        witness: MilestoneThreeSplitCollapseChurnWitness {
            original_wire_identity,
            split_wire_identity: split_wire_key.as_str().to_string(),
            collapse_wire_identity: collapse_wire_key.as_str().to_string(),
            moved_half_edge_identities,
            retained_half_edge_identities,
            split_step_wire_count,
            final_wire_count: collapse_execution.materialized().topology().wires.len(),
        },
    })
}

fn split_wire_declaration(
    split_wire_key: &str,
    moved_half_edge_ids: &[EntityId],
) -> Result<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration, TopologyCertificationError> {
    Ok(TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
        split_wire_key.to_string(),
        moved_half_edge_ids
            .iter()
            .enumerate()
            .map(|(index, half_edge_id)| {
                TopologyWireSplitHalfEdgeMember::new(
                    format!("{split_wire_key}.owns_half_edge_{}", index + 1),
                    *half_edge_id,
                )
            })
            .collect(),
    ))
}

fn collapse_split_wire_declaration(
    collapse_wire_key: &str,
    retired_wire_id: EntityId,
    moved_half_edge_ids: &[EntityId],
) -> Result<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyCertificationError> {
    Ok(TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        collapse_wire_key.to_string(),
        retired_wire_id,
        moved_half_edge_ids
            .iter()
            .enumerate()
            .map(|(index, half_edge_id)| {
                TopologyWireRehomeHalfEdgeMember::new(
                    format!("{collapse_wire_key}.owns_half_edge_{}", index + 1),
                    *half_edge_id,
                )
            })
            .collect(),
    ))
}

fn seeded_wire_and_half_edges(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<(EntityId, Vec<EntityId>), TopologyCertificationError> {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "split-collapse seeded snapshot should remain readable".to_string(),
            )
        })?;
    let wire = read_view
        .entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id)
                == Some(EntityKind::Topology(TopologyEntityKind::Wire))
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| {
            TopologyCertificationError::Query(
                "split-collapse primitive should contain one wire".to_string(),
            )
        })?;
    let half_edge_ids = read_view
        .relations()
        .iter()
        .filter(|record| {
            record.source == wire
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(
                        TopologyRelationKind::WireOwnsHalfEdge,
                    ))
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    Ok((wire, half_edge_ids))
}

fn wire_identity(
    materialized: &crate::derived_topology::materialized_graph::MaterializedTopologyView,
    wire_id: EntityId,
    description: &str,
) -> Result<String, TopologyCertificationError> {
    materialized
        .topology()
        .wires
        .iter()
        .find(|wire| wire.entity_id == wire_id)
        .map(|wire| wire.label.clone())
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "materialized topology should expose {description}"
            ))
        })
}

fn half_edge_identities(
    materialized: &crate::derived_topology::materialized_graph::MaterializedTopologyView,
    half_edge_ids: &[EntityId],
) -> Result<Vec<String>, TopologyCertificationError> {
    half_edge_ids
        .iter()
        .map(|half_edge_id| {
            materialized
                .topology()
                .half_edges
                .iter()
                .find(|half_edge| half_edge.entity_id == *half_edge_id)
                .map(|half_edge| half_edge.label.clone())
                .ok_or_else(|| {
                    TopologyCertificationError::Query(format!(
                        "materialized topology should expose half-edge `{:?}`",
                        half_edge_id
                    ))
                })
        })
        .collect()
}
