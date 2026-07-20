use std::rc::Rc;

use super::record::WorthUiPlanRegionRecord;
use super::WorthUiPlanRegionStorageCounters;

const RADIX_WIDTH: usize = 16;
const RADIX_LEVELS: usize = 16;

#[derive(Clone, Debug)]
pub(super) enum WorthUiPlanRegionSlotTrieNode {
    Branch([Option<Rc<Self>>; RADIX_WIDTH]),
    Slot(Rc<WorthUiPlanRegionRecord>),
}

pub(super) fn lookup(
    root: &Option<Rc<WorthUiPlanRegionSlotTrieNode>>,
    stable_slot: u64,
) -> Option<&Rc<WorthUiPlanRegionRecord>> {
    let mut node = root.as_ref()?;
    for level in 0..RADIX_LEVELS {
        let WorthUiPlanRegionSlotTrieNode::Branch(children) = node.as_ref() else {
            return None;
        };
        node = children[nibble(stable_slot, level)].as_ref()?;
    }
    let WorthUiPlanRegionSlotTrieNode::Slot(record) = node.as_ref() else {
        return None;
    };
    Some(record)
}

pub(super) fn insert(
    root: &Option<Rc<WorthUiPlanRegionSlotTrieNode>>,
    record: Rc<WorthUiPlanRegionRecord>,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> Option<Rc<WorthUiPlanRegionSlotTrieNode>> {
    Some(insert_at(root.as_ref(), record, 0, counters))
}

pub(super) fn remove(
    root: &Option<Rc<WorthUiPlanRegionSlotTrieNode>>,
    stable_slot: u64,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> Option<Rc<WorthUiPlanRegionSlotTrieNode>> {
    remove_at(root.as_ref()?, stable_slot, 0, counters)
}

#[cfg(test)]
pub(super) fn reachable_node_count(root: &Option<Rc<WorthUiPlanRegionSlotTrieNode>>) -> usize {
    let Some(root) = root else {
        return 0;
    };
    1 + match root.as_ref() {
        WorthUiPlanRegionSlotTrieNode::Branch(children) => {
            children.iter().map(reachable_node_count).sum::<usize>()
        }
        WorthUiPlanRegionSlotTrieNode::Slot(_) => 0,
    }
}

#[cfg(any(test, feature = "certification-support"))]
pub(super) fn collect_records(
    root: &Option<Rc<WorthUiPlanRegionSlotTrieNode>>,
    records: &mut Vec<Rc<WorthUiPlanRegionRecord>>,
) {
    let Some(root) = root else {
        return;
    };
    match root.as_ref() {
        WorthUiPlanRegionSlotTrieNode::Branch(children) => {
            for child in children {
                collect_records(child, records);
            }
        }
        WorthUiPlanRegionSlotTrieNode::Slot(record) => records.push(Rc::clone(record)),
    }
}

fn insert_at(
    node: Option<&Rc<WorthUiPlanRegionSlotTrieNode>>,
    record: Rc<WorthUiPlanRegionRecord>,
    level: usize,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> Rc<WorthUiPlanRegionSlotTrieNode> {
    counters.record_trie_node_copy();
    if level == RADIX_LEVELS {
        return Rc::new(WorthUiPlanRegionSlotTrieNode::Slot(record));
    }

    let mut children = match node.map(Rc::as_ref) {
        Some(WorthUiPlanRegionSlotTrieNode::Branch(children)) => children.clone(),
        _ => std::array::from_fn(|_| None),
    };
    counters.record_storage_pointer_copies(RADIX_WIDTH);
    let child_index = nibble(record.handle.stable_slot(), level);
    children[child_index] = Some(insert_at(
        children[child_index].as_ref(),
        record,
        level + 1,
        counters,
    ));
    Rc::new(WorthUiPlanRegionSlotTrieNode::Branch(children))
}

fn remove_at(
    node: &Rc<WorthUiPlanRegionSlotTrieNode>,
    stable_slot: u64,
    level: usize,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> Option<Rc<WorthUiPlanRegionSlotTrieNode>> {
    counters.record_trie_node_copy();
    if level == RADIX_LEVELS {
        return None;
    }

    let WorthUiPlanRegionSlotTrieNode::Branch(existing) = node.as_ref() else {
        return Some(Rc::clone(node));
    };
    let child_index = nibble(stable_slot, level);
    let child = existing[child_index].as_ref()?;
    let mut children = existing.clone();
    counters.record_storage_pointer_copies(RADIX_WIDTH);
    children[child_index] = remove_at(child, stable_slot, level + 1, counters);
    children
        .iter()
        .any(Option::is_some)
        .then(|| Rc::new(WorthUiPlanRegionSlotTrieNode::Branch(children)))
}

fn nibble(stable_slot: u64, level: usize) -> usize {
    ((stable_slot >> ((RADIX_LEVELS - level - 1) * 4)) & 0x0f) as usize
}
