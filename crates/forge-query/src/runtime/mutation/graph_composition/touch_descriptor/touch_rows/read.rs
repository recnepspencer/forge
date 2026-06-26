use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryMutationFamily, ForgeQueryMutationTargetCollectionIdentity,
};

use super::super::read_verb::ForgeQueryGraphTouchReadVerb;
use super::{ForgeQueryGraphTouchDescriptorRow, ForgeQueryGraphTouchDescriptorRowInput};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeQueryGraphReadTouchShape {
    aspect_touches: Vec<ForgeQueryAspectTouch>,
}

impl ForgeQueryGraphReadTouchShape {
    pub fn new(aspect_touches: impl IntoIterator<Item = ForgeQueryAspectTouch>) -> Self {
        Self {
            aspect_touches: aspect_touches
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect(),
        }
    }

    pub fn aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.aspect_touches
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
        declared_collection: Some(ForgeQueryMutationTargetCollectionIdentity::new(
            "graph-read-touch-row",
            collection,
        )),
        relation_kind_id: None,
        declared_symbol: None,
        declared_aspect_operations: Vec::new(),
        touched_aspects: shape.aspect_touches().to_vec(),
        has_symbolic_target_reference: false,
        has_existing_truth_binding: false,
        symbolic_aspect_reference_count: 0,
    })
}
