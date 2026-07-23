use std::rc::Rc;

use super::record::WorthUiPlanRegionRecord;
use super::{WorthUiPlanRegionIdentity, WorthUiPlanRegionStorageCounters};

const RADIX_WIDTH: usize = 16;
const RADIX_LEVELS: usize = 16;

#[derive(Clone, Debug)]
pub(super) enum WorthUiPlanRegionIdentityTrieNode {
    Branch([Option<Rc<Self>>; RADIX_WIDTH]),
    Bucket(Vec<Rc<WorthUiPlanRegionRecord>>),
}

pub(super) fn lookup<'store>(
    root: &'store Option<Rc<WorthUiPlanRegionIdentityTrieNode>>,
    identity: &WorthUiPlanRegionIdentity,
) -> Option<&'store Rc<WorthUiPlanRegionRecord>> {
    let mut node = root.as_ref()?;
    for level in 0..RADIX_LEVELS {
        let WorthUiPlanRegionIdentityTrieNode::Branch(children) = node.as_ref() else {
            return None;
        };
        node = children[nibble(identity.routing_fingerprint(), level)].as_ref()?;
    }
    let WorthUiPlanRegionIdentityTrieNode::Bucket(records) = node.as_ref() else {
        return None;
    };
    records
        .binary_search_by(|record| record.schema.identity().cmp(identity))
        .ok()
        .map(|index| &records[index])
}

pub(super) fn insert(
    root: &Option<Rc<WorthUiPlanRegionIdentityTrieNode>>,
    record: Rc<WorthUiPlanRegionRecord>,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> Option<Rc<WorthUiPlanRegionIdentityTrieNode>> {
    Some(insert_at(root.as_ref(), record, 0, counters))
}

pub(super) fn remove(
    root: &Option<Rc<WorthUiPlanRegionIdentityTrieNode>>,
    identity: &WorthUiPlanRegionIdentity,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> Option<Rc<WorthUiPlanRegionIdentityTrieNode>> {
    remove_at(root.as_ref()?, identity, 0, counters)
}

pub(super) fn collect_records(
    root: &Option<Rc<WorthUiPlanRegionIdentityTrieNode>>,
    output: &mut Vec<Rc<WorthUiPlanRegionRecord>>,
) {
    let Some(root) = root else {
        return;
    };
    match root.as_ref() {
        WorthUiPlanRegionIdentityTrieNode::Branch(children) => {
            for child in children {
                collect_records(child, output);
            }
        }
        WorthUiPlanRegionIdentityTrieNode::Bucket(records) => {
            output.extend(records.iter().cloned())
        }
    }
}

#[cfg(test)]
pub(super) fn reachable_node_count(root: &Option<Rc<WorthUiPlanRegionIdentityTrieNode>>) -> usize {
    let Some(root) = root else {
        return 0;
    };
    1 + match root.as_ref() {
        WorthUiPlanRegionIdentityTrieNode::Branch(children) => {
            children.iter().map(reachable_node_count).sum::<usize>()
        }
        WorthUiPlanRegionIdentityTrieNode::Bucket(_) => 0,
    }
}

fn insert_at(
    node: Option<&Rc<WorthUiPlanRegionIdentityTrieNode>>,
    record: Rc<WorthUiPlanRegionRecord>,
    level: usize,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> Rc<WorthUiPlanRegionIdentityTrieNode> {
    counters.record_trie_node_copy();
    if level == RADIX_LEVELS {
        let mut records = match node.map(Rc::as_ref) {
            Some(WorthUiPlanRegionIdentityTrieNode::Bucket(records)) => records.clone(),
            _ => Vec::new(),
        };
        counters.record_storage_pointer_copies(records.len());
        match records
            .binary_search_by(|existing| existing.schema.identity().cmp(record.schema.identity()))
        {
            Ok(index) => records[index] = record,
            Err(index) => records.insert(index, record),
        }
        return Rc::new(WorthUiPlanRegionIdentityTrieNode::Bucket(records));
    }

    let mut children = match node.map(Rc::as_ref) {
        Some(WorthUiPlanRegionIdentityTrieNode::Branch(children)) => children.clone(),
        _ => std::array::from_fn(|_| None),
    };
    counters.record_storage_pointer_copies(RADIX_WIDTH);
    let child_index = nibble(record.schema.identity().routing_fingerprint(), level);
    children[child_index] = Some(insert_at(
        children[child_index].as_ref(),
        record,
        level + 1,
        counters,
    ));
    Rc::new(WorthUiPlanRegionIdentityTrieNode::Branch(children))
}

fn remove_at(
    node: &Rc<WorthUiPlanRegionIdentityTrieNode>,
    identity: &WorthUiPlanRegionIdentity,
    level: usize,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> Option<Rc<WorthUiPlanRegionIdentityTrieNode>> {
    counters.record_trie_node_copy();
    if level == RADIX_LEVELS {
        let WorthUiPlanRegionIdentityTrieNode::Bucket(existing) = node.as_ref() else {
            return Some(Rc::clone(node));
        };
        let mut records = existing.clone();
        counters.record_storage_pointer_copies(records.len());
        let index = records
            .binary_search_by(|record| record.schema.identity().cmp(identity))
            .ok()?;
        records.remove(index);
        return (!records.is_empty())
            .then(|| Rc::new(WorthUiPlanRegionIdentityTrieNode::Bucket(records)));
    }

    let WorthUiPlanRegionIdentityTrieNode::Branch(existing) = node.as_ref() else {
        return Some(Rc::clone(node));
    };
    let child_index = nibble(identity.routing_fingerprint(), level);
    let child = existing[child_index].as_ref()?;
    let mut children = existing.clone();
    counters.record_storage_pointer_copies(RADIX_WIDTH);
    children[child_index] = remove_at(child, identity, level + 1, counters);
    children
        .iter()
        .any(Option::is_some)
        .then(|| Rc::new(WorthUiPlanRegionIdentityTrieNode::Branch(children)))
}

fn nibble(route: u64, level: usize) -> usize {
    ((route >> ((RADIX_LEVELS - level - 1) * 4)) & 0x0f) as usize
}
