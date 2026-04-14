mod entity_catalog;
mod entity_labels;
mod errors;
mod relation_wiring;
mod traits;
mod view_builder;

#[cfg(test)]
mod tests;

pub use errors::WorthTopologyMaterializationError;

use forge_relational::facade::runtime::RelationalReadView;

use crate::data::topology_view::WorthTopologyView;
use crate::materialization::entity_catalog::collect_entity_kinds;
use crate::materialization::relation_wiring::{apply_relation, finalize_topology_membership};
use crate::materialization::view_builder::{has_topology_content, push_entity_record};

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthTopologyMaterializer;

impl WorthTopologyMaterializer {
    pub fn materialize_from_truth(
        read_view: &RelationalReadView,
    ) -> Result<WorthTopologyView, WorthTopologyMaterializationError> {
        let mut view = WorthTopologyView::default();
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

        Ok(view)
    }
}
