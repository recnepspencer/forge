mod entity_catalog;
mod entity_labels;
mod errors;
mod input_rows;
mod relation_wiring;
mod traits;
mod types;
mod view_builder;

#[cfg(test)]
mod tests;

pub use errors::WorthTopologyMaterializationError;
pub use types::{
    MaterializationBreadthReport, MaterializationFallbackClass, MaterializationReport,
    MaterializedTopologyView,
};

use crate::materialization::entity_catalog::collect_entity_kinds;
use crate::materialization::input_rows::{MaterializationEntityRow, MaterializationRelationRow};
use crate::materialization::relation_wiring::{apply_relation, finalize_topology_membership};
use crate::materialization::view_builder::{has_topology_content, push_entity_row};
use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::runtime::RelationalReadView;

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthTopologyMaterializer;

impl WorthTopologyMaterializer {
    pub fn materialize_from_truth(
        read_view: &RelationalReadView,
    ) -> Result<MaterializedTopologyView, WorthTopologyMaterializationError> {
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

    pub(crate) fn materialize_from_query_rows(
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<MaterializedTopologyView, WorthTopologyMaterializationError> {
        let entities = entity_rows
            .iter()
            .map(MaterializationEntityRow::from_query_row)
            .collect::<Result<Vec<_>, _>>()?;
        let relations = relation_rows
            .iter()
            .map(MaterializationRelationRow::from_query_row)
            .collect::<Result<Vec<_>, _>>()?;
        Self::materialize_from_rows(
            &entities,
            &relations,
            entity_rows.len(),
            relation_rows.len(),
        )
    }

    fn materialize_from_rows(
        entities: &[MaterializationEntityRow],
        relations: &[MaterializationRelationRow],
        entity_count: usize,
        relation_count: usize,
    ) -> Result<MaterializedTopologyView, WorthTopologyMaterializationError> {
        let mut view = crate::data::topology_view::WorthTopologyView::default();
        let entity_kind_map = collect_entity_kinds(entities);

        for record in entities {
            push_entity_row(&mut view, record);
        }

        for relation in relations {
            apply_relation(&mut view, &entity_kind_map, relation)?;
        }

        finalize_topology_membership(&mut view)?;

        if !has_topology_content(&view) {
            return Err(WorthTopologyMaterializationError::new(
                "worth topology materialization requires at least one topological entity kind",
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
                    worth_schema::facade::WorthRelationKind::Topology(_)
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
