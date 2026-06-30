mod entity_catalog;
mod entity_labels;
mod errors;
mod input_rows;
pub(crate) mod query_input_decode;
mod relation_wiring;
mod relation_wiring_support;
mod traits;
mod types;
mod view_builder;

#[cfg(test)]
mod tests;

pub use errors::TopologyMaterializationError;
pub(crate) use types::MaterializationBreadthReport;
pub use types::{MaterializationFallbackClass, MaterializationReport, MaterializedTopologyView};

use crate::derived_topology::materialized_graph::entity_catalog::collect_entity_kinds;
use crate::derived_topology::materialized_graph::input_rows::{
    MaterializationEntityRow, MaterializationRelationRow,
};
use crate::derived_topology::materialized_graph::relation_wiring::{
    apply_relation, finalize_topology_membership,
};
use crate::derived_topology::materialized_graph::view_builder::{
    has_topology_content, push_entity_row,
};
use forge_relational::facade::runtime::RelationalReadView;

#[derive(Debug, Default, Clone, Copy)]
pub struct TopologyMaterializer;

impl TopologyMaterializer {
    pub fn materialize_from_truth(
        read_view: &RelationalReadView,
    ) -> Result<MaterializedTopologyView, TopologyMaterializationError> {
        let entities = read_view
            .entities()
            .iter()
            .filter_map(MaterializationEntityRow::from_truth_record)
            .collect::<Vec<_>>();
        let relations = read_view
            .relations()
            .iter()
            .filter_map(MaterializationRelationRow::from_truth_record)
            .collect::<Vec<_>>();
        Self::materialize_from_rows(
            &entities,
            &relations,
            read_view.entities().len(),
            read_view.relations().len(),
        )
    }

    fn materialize_from_rows(
        entities: &[MaterializationEntityRow],
        relations: &[MaterializationRelationRow],
        entity_count: usize,
        relation_count: usize,
    ) -> Result<MaterializedTopologyView, TopologyMaterializationError> {
        let mut view = crate::brep::topology_graph::TopologyView::default();
        let entity_kind_map = collect_entity_kinds(entities);

        for record in entities {
            push_entity_row(&mut view, record);
        }

        for relation in relations {
            apply_relation(&mut view, &entity_kind_map, relation)?;
        }

        finalize_topology_membership(&mut view)?;

        if !has_topology_content(&view) {
            return Err(TopologyMaterializationError::new(
                " topology materialization requires at least one topological entity kind",
            ));
        }

        let topology_entity_count = view.models.len()
            + view.bodies.len()
            + view.lumps.len()
            + view.regions.len()
            + view.shells.len()
            + view.faces.len()
            + view.loops.len()
            + view.wires.len()
            + view.half_edges.len()
            + view.edges.len()
            + view.vertices.len();
        let topology_relation_count = relations
            .iter()
            .filter(|relation| {
                matches!(
                    relation.kind,
                    schema::facade::platform::relations::RelationKind::Topology(_)
                )
            })
            .count();

        Ok(MaterializedTopologyView::new(
            view,
            MaterializationReport {
                breadth: MaterializationBreadthReport {
                    entity_count,
                    relation_count,
                    topology_entity_count,
                    topology_relation_count,
                },
                whole_view_materialization: true,
                fallback_class: Some(MaterializationFallbackClass::WholeViewRebuild),
            },
        ))
    }
}
