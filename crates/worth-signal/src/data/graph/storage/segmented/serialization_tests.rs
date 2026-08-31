use std::cell::Cell;

use serde::{Deserialize, Serialize};

use super::{SegmentedStorage, SegmentedStore};
use crate::data::graph::storage::handles::DependencySetId;

thread_local! {
    static ITEM_CLONES: Cell<usize> = const { Cell::new(0) };
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct Counted(u64);

#[derive(Serialize)]
struct ExpectedWire<'a> {
    items: &'a [Counted],
    segments: Vec<ExpectedSegment>,
}

#[derive(Serialize)]
struct ExpectedSegment {
    start: u32,
    len: u32,
}

impl Clone for Counted {
    fn clone(&self) -> Self {
        ITEM_CLONES.set(ITEM_CLONES.get() + 1);
        Self(self.0)
    }
}

#[test]
fn exclusive_serialization_borrows_flat_truth_without_cloning_items() {
    for segment_count in [64_u64, 4_096, 65_536] {
        let mut store = SegmentedStore::<Counted, DependencySetId>::default();
        for value in 0..segment_count {
            store.insert_from_slice(&[Counted(value)]);
        }
        let SegmentedStorage::Exclusive(flat) = &store.storage else {
            panic!("ordinary store must remain flat");
        };
        assert_eq!(flat.items.len(), segment_count as usize);
        let expected_items = (0..segment_count).map(Counted).collect::<Vec<_>>();
        ITEM_CLONES.set(0);
        let expected = serde_json::to_vec(&ExpectedWire {
            items: &expected_items,
            segments: (0..segment_count)
                .map(|start| ExpectedSegment {
                    start: start as u32,
                    len: 1,
                })
                .collect(),
        })
        .expect("borrowed native wire serializes");
        let actual = serde_json::to_vec(&store).expect("ordinary segmented store serializes");

        assert_eq!(actual, expected, "scale {segment_count} wire changed");
        assert_eq!(
            ITEM_CLONES.get(),
            0,
            "scale {segment_count} serialization cloned stored items"
        );
    }
}

#[test]
fn fork_shared_serialization_flattens_current_destination_truth() {
    let mut source = SegmentedStore::<Counted, DependencySetId>::default();
    let inherited = source.insert_from_slice(&[Counted(1), Counted(2)]);
    let mut destination = source.fork_persistent();
    assert!(source.shares_storage_with(&destination));
    let appended = destination.insert_from_slice(&[Counted(3)]);

    let wire = serde_json::to_string(&destination).expect("shared destination serializes");
    let restored: SegmentedStore<Counted, DependencySetId> =
        serde_json::from_str(&wire).expect("shared destination wire restores");
    assert_eq!(restored.get(inherited), &[Counted(1), Counted(2)]);
    assert_eq!(restored.get(appended), &[Counted(3)]);
    assert_eq!(source.get(inherited), &[Counted(1), Counted(2)]);
    assert_eq!(source.live_segment_count(), 1);
}
