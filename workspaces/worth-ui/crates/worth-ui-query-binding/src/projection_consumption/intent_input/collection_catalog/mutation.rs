use std::cmp::Ordering;
use std::sync::Arc;

use super::{Link, Node, UiProjectionOptionKey};
use crate::projection_consumption::UiProjectionInputCollectionRow;

use super::super::UiProjectionInputTransitionWork;

pub(super) fn insert(
    root: Link,
    key: UiProjectionOptionKey,
    value: Arc<UiProjectionInputCollectionRow>,
    work: &mut UiProjectionInputTransitionWork,
) -> (Arc<Node>, Option<Arc<UiProjectionInputCollectionRow>>) {
    let Some(node) = root else {
        return (make_node(key, value, None, None, work), None);
    };
    work.record_key_probe();
    match key.cmp(&node.key) {
        Ordering::Less => {
            let (left, previous) = insert(node.left.clone(), key, value, work);
            (
                balance(
                    make_node(
                        node.key.clone(),
                        Arc::clone(&node.value),
                        Some(left),
                        node.right.clone(),
                        work,
                    ),
                    work,
                ),
                previous,
            )
        }
        Ordering::Greater => {
            let (right, previous) = insert(node.right.clone(), key, value, work);
            (
                balance(
                    make_node(
                        node.key.clone(),
                        Arc::clone(&node.value),
                        node.left.clone(),
                        Some(right),
                        work,
                    ),
                    work,
                ),
                previous,
            )
        }
        Ordering::Equal => (
            make_node(key, value, node.left.clone(), node.right.clone(), work),
            Some(Arc::clone(&node.value)),
        ),
    }
}

pub(super) fn remove(
    root: Link,
    key: &UiProjectionOptionKey,
    work: &mut UiProjectionInputTransitionWork,
) -> (Link, bool) {
    let Some(node) = root else {
        return (None, false);
    };
    work.record_key_probe();
    match key.cmp(&node.key) {
        Ordering::Less => {
            let (left, removed) = remove(node.left.clone(), key, work);
            if !removed {
                return (Some(node), false);
            }
            let root = make_node(
                node.key.clone(),
                Arc::clone(&node.value),
                left,
                node.right.clone(),
                work,
            );
            (Some(balance(root, work)), true)
        }
        Ordering::Greater => {
            let (right, removed) = remove(node.right.clone(), key, work);
            if !removed {
                return (Some(node), false);
            }
            let root = make_node(
                node.key.clone(),
                Arc::clone(&node.value),
                node.left.clone(),
                right,
                work,
            );
            (Some(balance(root, work)), true)
        }
        Ordering::Equal => match (&node.left, &node.right) {
            (None, _) => (node.right.clone(), true),
            (_, None) => (node.left.clone(), true),
            (Some(_), Some(right)) => {
                let (successor_key, successor_value, next_right) =
                    take_min(Arc::clone(right), work);
                let root = make_node(
                    successor_key,
                    successor_value,
                    node.left.clone(),
                    next_right,
                    work,
                );
                (Some(balance(root, work)), true)
            }
        },
    }
}

fn take_min(
    node: Arc<Node>,
    work: &mut UiProjectionInputTransitionWork,
) -> (
    UiProjectionOptionKey,
    Arc<UiProjectionInputCollectionRow>,
    Link,
) {
    let Some(left) = &node.left else {
        return (
            node.key.clone(),
            Arc::clone(&node.value),
            node.right.clone(),
        );
    };
    let (key, value, next_left) = take_min(Arc::clone(left), work);
    let successor = make_node(
        node.key.clone(),
        Arc::clone(&node.value),
        next_left,
        node.right.clone(),
        work,
    );
    (key, value, Some(balance(successor, work)))
}

fn balance(node: Arc<Node>, work: &mut UiProjectionInputTransitionWork) -> Arc<Node> {
    let skew = height(&node.left) as i32 - height(&node.right) as i32;
    if skew > 1 {
        let left = Arc::clone(
            node.left
                .as_ref()
                .expect("left-heavy catalog node has a left child"),
        );
        return if height(&left.left) >= height(&left.right) {
            rotate_right(node, work)
        } else {
            let rotated = rotate_left(left, work);
            rotate_right(with_left(node, rotated, work), work)
        };
    }
    if skew < -1 {
        let right = Arc::clone(
            node.right
                .as_ref()
                .expect("right-heavy catalog node has a right child"),
        );
        return if height(&right.right) >= height(&right.left) {
            rotate_left(node, work)
        } else {
            let rotated = rotate_right(right, work);
            rotate_left(with_right(node, rotated, work), work)
        };
    }
    node
}

fn rotate_left(root: Arc<Node>, work: &mut UiProjectionInputTransitionWork) -> Arc<Node> {
    let pivot = root
        .right
        .as_ref()
        .expect("left rotation requires right child");
    let left = make_node(
        root.key.clone(),
        Arc::clone(&root.value),
        root.left.clone(),
        pivot.left.clone(),
        work,
    );
    make_node(
        pivot.key.clone(),
        Arc::clone(&pivot.value),
        Some(left),
        pivot.right.clone(),
        work,
    )
}

fn rotate_right(root: Arc<Node>, work: &mut UiProjectionInputTransitionWork) -> Arc<Node> {
    let pivot = root
        .left
        .as_ref()
        .expect("right rotation requires left child");
    let right = make_node(
        root.key.clone(),
        Arc::clone(&root.value),
        pivot.right.clone(),
        root.right.clone(),
        work,
    );
    make_node(
        pivot.key.clone(),
        Arc::clone(&pivot.value),
        pivot.left.clone(),
        Some(right),
        work,
    )
}

fn with_left(
    root: Arc<Node>,
    left: Arc<Node>,
    work: &mut UiProjectionInputTransitionWork,
) -> Arc<Node> {
    make_node(
        root.key.clone(),
        Arc::clone(&root.value),
        Some(left),
        root.right.clone(),
        work,
    )
}

fn with_right(
    root: Arc<Node>,
    right: Arc<Node>,
    work: &mut UiProjectionInputTransitionWork,
) -> Arc<Node> {
    make_node(
        root.key.clone(),
        Arc::clone(&root.value),
        root.left.clone(),
        Some(right),
        work,
    )
}

fn make_node(
    key: UiProjectionOptionKey,
    value: Arc<UiProjectionInputCollectionRow>,
    left: Link,
    right: Link,
    work: &mut UiProjectionInputTransitionWork,
) -> Arc<Node> {
    work.record_node_copy();
    Arc::new(Node {
        key,
        value,
        height: 1 + height(&left).max(height(&right)),
        len: 1 + len(&left) + len(&right),
        left,
        right,
    })
}

fn height(link: &Link) -> u16 {
    link.as_ref().map_or(0, |node| node.height)
}

fn len(link: &Link) -> usize {
    link.as_ref().map_or(0, |node| node.len)
}
