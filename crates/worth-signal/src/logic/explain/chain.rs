use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;

pub fn dependency_chain_to(
    graph: &SignalGraph,
    root: NodeId,
    target: NodeId,
) -> Result<Option<Vec<NodeId>>, SignalError> {
    graph.get_entry(root)?;
    graph.get_entry(target)?;

    if root == target {
        return Ok(Some(vec![root]));
    }

    let mut queue = VecDeque::from([root]);
    let mut visited = BTreeSet::from([root]);
    let mut previous = BTreeMap::<NodeId, NodeId>::new();

    while let Some(current) = queue.pop_front() {
        let mut subscribers = graph.subscribers_of(current)?.to_vec();
        subscribers.sort();
        for subscriber in subscribers {
            if !visited.insert(subscriber) {
                continue;
            }
            previous.insert(subscriber, current);
            if subscriber == target {
                let mut path = vec![target];
                let mut cursor = target;
                while let Some(parent) = previous.get(&cursor).copied() {
                    path.push(parent);
                    if parent == root {
                        path.reverse();
                        return Ok(Some(path));
                    }
                    cursor = parent;
                }
            }
            queue.push_back(subscriber);
        }
    }

    Ok(None)
}
