use std::cmp::Ordering;
use std::rc::Rc;

use super::mutation_work::UiPersistentIndexMutationWork;
use super::ordered_map::{height, node_len, Link, Node};

pub(super) fn insert<K: Ord + Clone, V>(
    root: Link<K, V>,
    key: K,
    value: Rc<V>,
    work: &mut UiPersistentIndexMutationWork,
) -> Rc<Node<K, V>> {
    let Some(node) = root else {
        return make_node(key, value, None, None, work);
    };
    work.record_key_probe();
    match key.cmp(&node.key) {
        Ordering::Less => {
            let left = insert(node.left.clone(), key, value, work);
            balance(
                make_node(
                    node.key.clone(),
                    Rc::clone(&node.value),
                    Some(left),
                    node.right.clone(),
                    work,
                ),
                work,
            )
        }
        Ordering::Greater => {
            let right = insert(node.right.clone(), key, value, work);
            balance(
                make_node(
                    node.key.clone(),
                    Rc::clone(&node.value),
                    node.left.clone(),
                    Some(right),
                    work,
                ),
                work,
            )
        }
        Ordering::Equal => make_node(key, value, node.left.clone(), node.right.clone(), work),
    }
}

pub(super) fn remove<K: Ord + Clone, V>(
    root: Link<K, V>,
    key: &K,
    work: &mut UiPersistentIndexMutationWork,
) -> (Link<K, V>, bool) {
    let Some(node) = root else {
        return (None, false);
    };
    work.record_key_probe();
    match key.cmp(&node.key) {
        Ordering::Less => {
            let (left, removed) = remove(node.left.clone(), key, work);
            let root = make_node(
                node.key.clone(),
                Rc::clone(&node.value),
                left,
                node.right.clone(),
                work,
            );
            (Some(balance(root, work)), removed)
        }
        Ordering::Greater => {
            let (right, removed) = remove(node.right.clone(), key, work);
            let root = make_node(
                node.key.clone(),
                Rc::clone(&node.value),
                node.left.clone(),
                right,
                work,
            );
            (Some(balance(root, work)), removed)
        }
        Ordering::Equal => match (&node.left, &node.right) {
            (None, _) => (node.right.clone(), true),
            (_, None) => (node.left.clone(), true),
            (Some(_), Some(right)) => {
                let (successor_key, successor_value, next_right) = take_min(Rc::clone(right), work);
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

fn take_min<K: Ord + Clone, V>(
    node: Rc<Node<K, V>>,
    work: &mut UiPersistentIndexMutationWork,
) -> (K, Rc<V>, Link<K, V>) {
    let Some(left) = &node.left else {
        return (node.key.clone(), Rc::clone(&node.value), node.right.clone());
    };
    let (key, value, next_left) = take_min(Rc::clone(left), work);
    let successor = make_node(
        node.key.clone(),
        Rc::clone(&node.value),
        next_left,
        node.right.clone(),
        work,
    );
    (key, value, Some(balance(successor, work)))
}

fn balance<K: Ord + Clone, V>(
    node: Rc<Node<K, V>>,
    work: &mut UiPersistentIndexMutationWork,
) -> Rc<Node<K, V>> {
    let skew = height(&node.left) as i32 - height(&node.right) as i32;
    if skew > 1 {
        let left = Rc::clone(
            node.left
                .as_ref()
                .expect("left-heavy node has a left child"),
        );
        return if height(&left.left) >= height(&left.right) {
            rotate_right(node, work)
        } else {
            let rotated = rotate_left(left, work);
            let root = with_left(node, rotated, work);
            rotate_right(root, work)
        };
    }
    if skew < -1 {
        let right = Rc::clone(
            node.right
                .as_ref()
                .expect("right-heavy node has a right child"),
        );
        return if height(&right.right) >= height(&right.left) {
            rotate_left(node, work)
        } else {
            let rotated = rotate_right(right, work);
            let root = with_right(node, rotated, work);
            rotate_left(root, work)
        };
    }
    node
}

fn rotate_left<K: Ord + Clone, V>(
    root: Rc<Node<K, V>>,
    work: &mut UiPersistentIndexMutationWork,
) -> Rc<Node<K, V>> {
    let pivot = root
        .right
        .as_ref()
        .expect("left rotation requires right child");
    let left = make_node(
        root.key.clone(),
        Rc::clone(&root.value),
        root.left.clone(),
        pivot.left.clone(),
        work,
    );
    make_node(
        pivot.key.clone(),
        Rc::clone(&pivot.value),
        Some(left),
        pivot.right.clone(),
        work,
    )
}

fn rotate_right<K: Ord + Clone, V>(
    root: Rc<Node<K, V>>,
    work: &mut UiPersistentIndexMutationWork,
) -> Rc<Node<K, V>> {
    let pivot = root
        .left
        .as_ref()
        .expect("right rotation requires left child");
    let right = make_node(
        root.key.clone(),
        Rc::clone(&root.value),
        pivot.right.clone(),
        root.right.clone(),
        work,
    );
    make_node(
        pivot.key.clone(),
        Rc::clone(&pivot.value),
        pivot.left.clone(),
        Some(right),
        work,
    )
}

fn with_left<K: Ord + Clone, V>(
    root: Rc<Node<K, V>>,
    left: Rc<Node<K, V>>,
    work: &mut UiPersistentIndexMutationWork,
) -> Rc<Node<K, V>> {
    make_node(
        root.key.clone(),
        Rc::clone(&root.value),
        Some(left),
        root.right.clone(),
        work,
    )
}

fn with_right<K: Ord + Clone, V>(
    root: Rc<Node<K, V>>,
    right: Rc<Node<K, V>>,
    work: &mut UiPersistentIndexMutationWork,
) -> Rc<Node<K, V>> {
    make_node(
        root.key.clone(),
        Rc::clone(&root.value),
        root.left.clone(),
        Some(right),
        work,
    )
}

fn make_node<K, V>(
    key: K,
    value: Rc<V>,
    left: Link<K, V>,
    right: Link<K, V>,
    work: &mut UiPersistentIndexMutationWork,
) -> Rc<Node<K, V>> {
    work.record_node_copy();
    Rc::new(Node {
        key,
        value,
        height: 1 + height(&left).max(height(&right)),
        len: 1 + node_len(&left) + node_len(&right),
        left,
        right,
    })
}
