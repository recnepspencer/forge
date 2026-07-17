use std::cmp::Ordering;

use crate::capability::ViewBindingId;
use worth_ui_query_binding::{WorthUiQueryViewIdentity, WorthUiQueryViewShape};

/// UI binding identity paired with the binding-owned admitted Query definition.
/// No reporting string or locally reconstructed digest can stand in for it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingIdentity {
    view_binding_id: ViewBindingId,
    query_view_identity: WorthUiQueryViewIdentity,
    result_shape: WorthUiQueryViewShape,
}

impl WorthUiQueryBindingIdentity {
    pub(crate) fn new(
        view_binding_id: &ViewBindingId,
        definition: &worth_ui_query_binding::WorthUiQueryViewDefinition,
    ) -> Self {
        Self {
            view_binding_id: view_binding_id.clone(),
            query_view_identity: definition.identity().clone(),
            result_shape: definition.shape(),
        }
    }

    pub fn view_binding_id(&self) -> &str {
        self.view_binding_id.as_str()
    }

    pub fn query_view_identity(&self) -> &WorthUiQueryViewIdentity {
        &self.query_view_identity
    }

    pub fn result_shape(&self) -> WorthUiQueryViewShape {
        self.result_shape
    }

    pub fn canonical_identity(&self) -> u64 {
        let digest = self
            .view_binding_id
            .as_str()
            .as_bytes()
            .iter()
            .fold(0xcbf2_9ce4_8422_2325, |digest, byte| {
                (digest ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
            });
        let digest = self
            .query_view_identity
            .as_str()
            .as_bytes()
            .iter()
            .fold(digest, |digest, byte| {
                (digest ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
            });
        digest.rotate_left(17) ^ result_shape_tag(self.result_shape)
    }
}

impl Ord for WorthUiQueryBindingIdentity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.view_binding_id
            .cmp(&other.view_binding_id)
            .then_with(|| self.query_view_identity.cmp(&other.query_view_identity))
            .then_with(|| {
                result_shape_tag(self.result_shape).cmp(&result_shape_tag(other.result_shape))
            })
    }
}

fn result_shape_tag(shape: WorthUiQueryViewShape) -> u64 {
    match shape {
        WorthUiQueryViewShape::Collection => 1,
        WorthUiQueryViewShape::Detail => 2,
    }
}

impl PartialOrd for WorthUiQueryBindingIdentity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::WorthUiQueryBindingIdentity;
    use crate::capability::ViewBindingId;

    #[test]
    fn lifecycle_is_runtime_posture_not_binding_identity() {
        let binding_id = ViewBindingId::new("workspace.view_binding.selection").unwrap();
        let snapshot = worth_ui_query_binding::WorthUiQueryViewDefinition::measurement_snapshot(
            "workspace.query.selection",
        )
        .unwrap();
        let live = worth_ui_query_binding::WorthUiQueryViewDefinition::measurement_live(
            "workspace.query.selection",
        )
        .unwrap();

        let snapshot_identity = WorthUiQueryBindingIdentity::new(&binding_id, &snapshot);
        let live_identity = WorthUiQueryBindingIdentity::new(&binding_id, &live);

        assert_eq!(snapshot_identity, live_identity);
        assert_eq!(
            snapshot_identity.canonical_identity(),
            live_identity.canonical_identity()
        );
    }

    #[test]
    fn ui_binding_and_query_view_identity_are_both_identity_bearing() {
        let definition = worth_ui_query_binding::WorthUiQueryViewDefinition::measurement_snapshot(
            "workspace.query.selection",
        )
        .unwrap();
        let other_definition =
            worth_ui_query_binding::WorthUiQueryViewDefinition::measurement_snapshot(
                "workspace.query.detail",
            )
            .unwrap();
        let selection = WorthUiQueryBindingIdentity::new(
            &ViewBindingId::new("workspace.view_binding.selection").unwrap(),
            &definition,
        );
        let different_binding = WorthUiQueryBindingIdentity::new(
            &ViewBindingId::new("workspace.view_binding.detail").unwrap(),
            &definition,
        );
        let different_query_view = WorthUiQueryBindingIdentity::new(
            &ViewBindingId::new("workspace.view_binding.selection").unwrap(),
            &other_definition,
        );

        assert_ne!(selection, different_binding);
        assert_ne!(selection, different_query_view);
        assert_ne!(
            selection.canonical_identity(),
            different_binding.canonical_identity()
        );
        assert_ne!(
            selection.canonical_identity(),
            different_query_view.canonical_identity()
        );
    }
}
