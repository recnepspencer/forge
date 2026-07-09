use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath};
use worth_query::facade::{
    WorthQueryAspectTouch, WorthQueryEntityIdentity, WorthQueryExistingTruthTargetBinding,
    WorthQueryLiveArtifactTarget, WorthQueryMutationTargetCollectionIdentity,
};

pub(super) type NativeExternalRow = BTreeMap<CanonicalFieldPath, AspectValue>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct PublicExistingTruthKey {
    binding_digest: String,
    target_collection: String,
    aspect_touch: WorthQueryAspectTouch,
}

impl PublicExistingTruthKey {
    pub(super) fn new(
        binding: &WorthQueryExistingTruthTargetBinding,
        aspect_touch: WorthQueryAspectTouch,
    ) -> Self {
        Self {
            binding_digest: binding.binding_digest(),
            target_collection: binding
                .terminal_target_collection_projection()
                .unwrap_or("none")
                .to_string(),
            aspect_touch,
        }
    }

    pub(super) fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub(super) fn target_collection(&self) -> &str {
        &self.target_collection
    }

    pub(super) fn admitted_aspect_touch_reporting_projection(&self) -> String {
        let field_path = self
            .aspect_touch
            .native_field_path()
            .map(|path| {
                path.fields()
                    .iter()
                    .map(|field| field.as_str())
                    .collect::<Vec<_>>()
                    .join(".")
            })
            .unwrap_or_else(|| "<whole-aspect>".to_string());
        format!(
            "{}:{field_path}",
            self.aspect_touch.native_aspect_key().as_str()
        )
    }
}

#[derive(Default)]
pub(super) struct PublicBridgeRuntimeState {
    pub(super) live_views:
        BTreeMap<WorthQueryLiveArtifactTarget, WorthQueryMutationTargetCollectionIdentity>,
    pub(super) rows_by_collection:
        BTreeMap<String, BTreeMap<WorthQueryEntityIdentity, NativeExternalRow>>,
    pub(super) collection_by_identity: BTreeMap<WorthQueryEntityIdentity, String>,
    pub(super) identity_by_symbol: BTreeMap<String, WorthQueryEntityIdentity>,
    pub(super) existing_truth_values: BTreeMap<PublicExistingTruthKey, AspectValue>,
    pub(super) next_entity_identity: usize,
    pub(super) next_commit_identity: usize,
    pub(super) next_snapshot_token: usize,
}
