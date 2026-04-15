mod entity_catalog;
mod entity_labels;
mod errors;
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

use forge_relational::facade::runtime::RelationalReadView;
use crate::materialization::entity_catalog::collect_entity_kinds;
use crate::materialization::relation_wiring::{apply_relation, finalize_topology_membership};
use crate::materialization::view_builder::{has_topology_content, push_entity_record};

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthTopologyMaterializer;

impl WorthTopologyMaterializer {
    pub fn materialize_from_truth(
        read_view: &RelationalReadView,
    ) -> Result<MaterializedTopologyView, WorthTopologyMaterializationError> {
        let mut view = crate::data::topology_view::WorthTopologyView::default();
        let entity_kind_map = collect_entity_kinds(read_view);

        for record in read_view.entities() {
            push_entity_record(&mut view, record);
        }

        for relation in read_view.relations() {
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
        let topology_relation_count = read_view
            .relations()
            .iter()
            .filter(|relation| {
                worth_schema::facade::WorthRelationKind::from_kind_id(relation.kind.kind_id)
                    .is_some_and(|kind| {
                        matches!(kind, worth_schema::facade::WorthRelationKind::Topology(_))
                    })
            })
            .count();

        Ok(MaterializedTopologyView::new(
            view,
            MaterializationReport {
                breadth: MaterializationBreadthReport {
                    entity_count: read_view.entities().len(),
                    relation_count: read_view.relations().len(),
                    topology_entity_count,
                    topology_relation_count,
                },
                whole_view_materialization: true,
                fallback_class: Some(MaterializationFallbackClass::WholeViewRebuild),
            },
        ))
    }
}
