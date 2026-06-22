use crate::runtime::ForgeQueryMutationFamily;

use super::super::read_verb::ForgeQueryGraphTouchReadVerb;
use super::{ForgeQueryGraphTouchDescriptorRow, ForgeQueryGraphTouchDescriptorRowInput};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeQueryGraphReadTouchShape {
    aspect_paths: Vec<String>,
}

impl ForgeQueryGraphReadTouchShape {
    pub fn new(aspect_paths: impl IntoIterator<Item = String>) -> Self {
        let mut aspect_paths = aspect_paths
            .into_iter()
            .map(|path| path.trim().to_string())
            .filter(|path| !path.is_empty())
            .collect::<Vec<_>>();
        aspect_paths.sort();
        aspect_paths.dedup();
        Self { aspect_paths }
    }

    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
    }
}

pub(in super::super) fn derive_read_touch_rows(
    collection: &str,
    verbs: impl IntoIterator<Item = ForgeQueryGraphTouchReadVerb>,
    shape: &ForgeQueryGraphReadTouchShape,
) -> Vec<ForgeQueryGraphTouchDescriptorRow> {
    verbs
        .into_iter()
        .enumerate()
        .map(|(component_index, verb)| read_touch_row(collection, component_index, verb, shape))
        .collect()
}

fn read_touch_row(
    collection: &str,
    component_index: usize,
    verb: ForgeQueryGraphTouchReadVerb,
    shape: &ForgeQueryGraphReadTouchShape,
) -> ForgeQueryGraphTouchDescriptorRow {
    ForgeQueryGraphTouchDescriptorRow::new(ForgeQueryGraphTouchDescriptorRowInput {
        component_index,
        mutation_family: ForgeQueryMutationFamily::Assertion,
        read_verb: Some(verb),
        program_step_kind: None,
        lifecycle_family: None,
        declared_collection: Some(collection.to_string()),
        relation_kind_id: None,
        declared_symbol: None,
        declared_aspect_operations: Vec::new(),
        touched_aspect_paths: shape.aspect_paths().to_vec(),
        has_symbolic_target_reference: false,
        has_existing_truth_binding: false,
        symbolic_aspect_reference_count: 0,
    })
}
