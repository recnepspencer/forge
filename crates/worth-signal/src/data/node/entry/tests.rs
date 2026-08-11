use super::NodeEntry;
use crate::data::aspect::{Aspect, AspectVersion};
use crate::data::output::{ChangedRegion, PartitionSubscription};
use crate::data::trace::{CausalityMetadata, RuntimeArtifactState};

#[test]
fn checkpoint_image_round_trips_node_entry() {
    let mut entry = NodeEntry::new();
    entry.transition_dirty(
        Aspect::new(0),
        &[PartitionSubscription::partition_and_detail(
            "wing", "rib-12",
        )],
    );
    entry.apply_aspect_version(
        AspectVersion::from_updates([(Aspect::new(0), 7)]),
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    );
    entry.set_tombstoned(true);
    let mut runtime = RuntimeArtifactState::default();
    runtime.hot_mut().dependency_count = 3;
    entry.set_runtime_artifact_state(Some(runtime));
    entry.set_causality(Some(CausalityMetadata {
        kind: "checkpoint-test".to_string(),
        fields: Default::default(),
    }));

    let image = entry.to_checkpoint_image();
    let restored = NodeEntry::from_checkpoint_image(image);

    assert_eq!(restored.get_state(), entry.get_state());
    assert_eq!(restored.get_dirty_aspects(), entry.get_dirty_aspects());
    assert_eq!(
        restored.get_dirty_partition_scopes(),
        entry.get_dirty_partition_scopes()
    );
    assert_eq!(restored.get_aspect_version(), entry.get_aspect_version());
    assert_eq!(restored.is_tombstoned(), entry.is_tombstoned());
    assert_eq!(
        restored.get_runtime_artifact_state(),
        entry.get_runtime_artifact_state()
    );
    assert_eq!(restored.get_causality(), entry.get_causality());
}
