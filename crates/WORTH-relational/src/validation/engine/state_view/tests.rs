use std::collections::BTreeMap;
use std::collections::BTreeSet;

use worth_foundational::{
    admit_authoritative_record_aspect_state, aspects, validate_aspect_value, AspectContract,
    AspectContractRevision, AspectIdentity, AspectKey, AspectValue, AuthoritativeRecordAspectState,
    InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;

use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::storage::logic::state::{EntityArena, PartitionState, RelationArena, SlotInit};
use crate::storage::overlay::{
    EntityWorkingSetLayout, OverlayStateView, PartitionCloneMode, WorkingState,
};
use crate::storage::substrate::{EntityRecordKind, RecordKind, RelationEndpoints, RelationExtra};

use super::InvariantStateView;

#[test]
fn sparse_speculative_overlay_reads_untouched_entity_truth_from_base_partition() {
    let policy = AdjacencyPolicy {
        backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
        small_degree_inline_capacity: 4,
    };
    let partition_id = PartitionId(1);
    let mut base_partition = PartitionState {
        partition_id,
        adjacency_policy: policy.clone(),
        relation_overlay_is_sparse: false,
        entity_arena: EntityArena::with_capacity(2),
        relation_arena: RelationArena::with_capacity(0),
        adjacency: Vec::new(),
        reverse_adjacency: Vec::new(),
    };
    let name_contract = scalar_string_contract(AspectKey::new("name").unwrap(), 1);
    let _ = base_partition.entity_arena.push_slot(SlotInit {
        partition_id,
        kind_id: KindId(1),
        version_id: VersionId(1),
        extra: entity_extra_with_aspect_state(authoritative_string_state(&name_contract, "left")),
    });
    let _ = base_partition.entity_arena.push_slot(SlotInit {
        partition_id,
        kind_id: KindId(1),
        version_id: VersionId(1),
        extra: EntityRecordKind::empty_extra(),
    });

    let mut base = BTreeMap::new();
    base.insert(partition_id, base_partition);
    let sparse_slots = BTreeMap::from([(partition_id, [1usize].into_iter().collect())]);
    let staged = WorkingState::from_touched_partitions_with_layout_and_sparse_slots(
        &base,
        [partition_id],
        policy,
        PartitionCloneMode::EntityOnly,
        EntityWorkingSetLayout::AoSoACandidate { chunk_width: 128 },
        Some(&sparse_slots),
        None,
    );
    let overlay = OverlayStateView::new(&base, &staged);
    let state_view = InvariantStateView::new(&overlay, VersionId(1));

    let untouched_entity = EntityId::new(partition_id, 0, 1);
    let aspect_state = state_view
        .entity_aspect_state(untouched_entity)
        .expect("untouched aspect state should read through to base");
    let metadata = state_view
        .entity_metadata(untouched_entity)
        .expect("untouched metadata should read through to base");

    assert!(aspect_state.get(name_contract.key()).is_some());
    assert_eq!(metadata.kind_id, KindId(1));
    assert_eq!(metadata.entity_id, untouched_entity);
}

#[test]
fn sparse_speculative_overlay_reads_untouched_relation_truth_from_base_partition() {
    let policy = AdjacencyPolicy {
        backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
        small_degree_inline_capacity: 4,
    };
    let partition_id = PartitionId(1);
    let mut base_partition = PartitionState {
        partition_id,
        adjacency_policy: policy.clone(),
        relation_overlay_is_sparse: false,
        entity_arena: EntityArena::with_capacity(2),
        relation_arena: RelationArena::with_capacity(1),
        adjacency: Vec::new(),
        reverse_adjacency: Vec::new(),
    };
    let relation_kind_contract =
        scalar_string_contract(AspectKey::new("relation.kind").unwrap(), 2);
    let (left_slot, left_generation, _) = base_partition.entity_arena.push_slot(SlotInit {
        partition_id,
        kind_id: KindId(1),
        version_id: VersionId(1),
        extra: EntityRecordKind::empty_extra(),
    });
    let (right_slot, right_generation, _) = base_partition.entity_arena.push_slot(SlotInit {
        partition_id,
        kind_id: KindId(1),
        version_id: VersionId(1),
        extra: EntityRecordKind::empty_extra(),
    });
    let left = EntityId::new(partition_id, left_slot as u64, left_generation);
    let right = EntityId::new(partition_id, right_slot as u64, right_generation);
    let (relation_slot, relation_generation, _) =
        base_partition.relation_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(9),
            version_id: VersionId(1),
            extra: RelationExtra {
                endpoints: Some(RelationEndpoints {
                    source: left,
                    target: right,
                }),
                authoritative_aspect_state: Some(authoritative_string_state(
                    &relation_kind_contract,
                    "edge",
                )),
            },
        });
    let relation_id = RelationId::new(partition_id, relation_slot as u64, relation_generation);

    let mut base = BTreeMap::new();
    base.insert(partition_id, base_partition);
    let sparse_slots = BTreeMap::from([(partition_id, [0usize].into_iter().collect())]);
    let sparse_relation_partitions = BTreeSet::from([partition_id]);
    let staged = WorkingState::from_touched_partitions_with_layout_and_sparse_slots(
        &base,
        [partition_id],
        policy,
        PartitionCloneMode::GraphSparseEntities,
        EntityWorkingSetLayout::AoSoACandidate { chunk_width: 128 },
        Some(&sparse_slots),
        Some(&sparse_relation_partitions),
    );
    let overlay = OverlayStateView::new(&base, &staged);
    let state_view = InvariantStateView::new(&overlay, VersionId(1));

    let aspect_state = state_view
        .relation_aspect_state(relation_id)
        .expect("untouched relation aspect state should read through to base");
    let metadata = state_view
        .relation_metadata(relation_id)
        .expect("untouched relation metadata should read through to base");

    assert!(aspect_state.get(relation_kind_contract.key()).is_some());
    assert_eq!(metadata.kind_id, KindId(9));
    assert_eq!(metadata.relation_id, relation_id);
    assert_eq!(metadata.source, left);
    assert_eq!(metadata.target, right);
}

fn scalar_string_contract(aspect_key: AspectKey, identity: u64) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(AspectIdentity(identity))
        .at_revision(AspectContractRevision(1))
        .scalar(ScalarAspectType::String)
}

fn authoritative_string_state(
    contract: &AspectContract,
    value: &str,
) -> AuthoritativeRecordAspectState {
    let TransitionOutcome::Success(validated) = validate_aspect_value(
        contract,
        AspectValue::String(InternedString::Raw(value.to_string())).into(),
    ) else {
        panic!("test value should validate against scalar string contract");
    };
    let TransitionOutcome::Success(state) = admit_authoritative_record_aspect_state([validated])
    else {
        panic!("test state should admit one validated aspect");
    };
    let (state, _proofs, _basis) = state.into_parts().into_parts();
    state
}

fn entity_extra_with_aspect_state(
    authoritative_aspect_state: AuthoritativeRecordAspectState,
) -> crate::storage::substrate::EntityExtra {
    crate::storage::substrate::EntityExtra {
        authoritative_aspect_state: Some(authoritative_aspect_state),
        ..EntityRecordKind::empty_extra()
    }
}
