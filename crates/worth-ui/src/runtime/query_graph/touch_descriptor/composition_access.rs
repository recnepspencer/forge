use forge_query::facade::runtime::{ForgeQueryGraphTouchDescriptor, ForgeQueryMutationFamily};

use crate::runtime::{WorthUiQueryGraphTouchDescriptor, WorthUiRuntimeFactId};

use super::super::operation_declaration::composition_graph_access_touch_operations;
use super::fact_paths;

impl WorthUiQueryGraphTouchDescriptor {
    pub fn composition_graph_access(
        root_id: impl Into<String>,
        access_kind: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let root_id = root_id.into();
        let access_kind = access_kind.into();
        let mut touched_paths = fact_paths(dependency_facts);
        touched_paths.push(format!("composition_root.{root_id}"));
        touched_paths.push(format!("composition_access.{access_kind}"));
        touched_paths.sort();
        touched_paths.dedup();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::COMPOSITION_GRAPH_ACCESS_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            composition_graph_access_touch_operations(),
            touched_paths,
        )
        .expect("Worth composition access descriptors use validated non-empty constants");
        Self {
            interaction_id: format!("worth.ui.composition-access.{root_id}.{access_kind}"),
            surface_id: root_id,
            descriptor,
        }
    }
}
