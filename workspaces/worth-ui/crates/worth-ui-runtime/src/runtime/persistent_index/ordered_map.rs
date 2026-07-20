use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

type Link<K, V> = Option<Rc<Node<K, V>>>;

/// Immutable AVL index used by replacement truth that must fork without
/// copying unaffected rows. Updates allocate only the search path.
pub(crate) struct UiPersistentOrdMap<K, V> {
    root: Link<K, V>,
}

struct Node<K, V> {
    key: K,
    value: Rc<V>,
    left: Link<K, V>,
    right: Link<K, V>,
    height: u16,
    len: usize,
}

impl<K, V> Clone for UiPersistentOrdMap<K, V> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
        }
    }
}

impl<K, V> Default for UiPersistentOrdMap<K, V> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<K: Ord + Clone, V> UiPersistentOrdMap<K, V> {
    pub(crate) fn len(&self) -> usize {
        node_len(&self.root)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        let mut cursor = self.root.as_deref();
        while let Some(node) = cursor {
            match key.cmp(&node.key) {
                Ordering::Less => cursor = node.left.as_deref(),
                Ordering::Greater => cursor = node.right.as_deref(),
                Ordering::Equal => return Some(node.value.as_ref()),
            }
        }
        None
    }

    pub(crate) fn insert(&mut self, key: K, value: V) {
        self.root = Some(insert(self.root.take(), key, Rc::new(value)));
    }

    pub(crate) fn remove(&mut self, key: &K) -> bool {
        let (root, removed) = remove(self.root.take(), key);
        self.root = root;
        removed
    }

    pub(crate) fn iter(&self) -> UiPersistentOrdMapIter<'_, K, V> {
        UiPersistentOrdMapIter::new(self.root.as_deref())
    }

    #[cfg(test)]
    pub(crate) fn root_is_shared_with(&self, other: &Self) -> bool {
        match (&self.root, &other.root) {
            (Some(left), Some(right)) => Rc::ptr_eq(left, right),
            (None, None) => true,
            _ => false,
        }
    }

    #[cfg(test)]
    pub(crate) fn shared_node_count_with(&self, other: &Self) -> usize {
        let mut predecessor_nodes = std::collections::HashSet::new();
        collect_node_addresses(other.root.as_deref(), &mut predecessor_nodes);
        count_shared_nodes(self.root.as_deref(), &predecessor_nodes)
    }
}

#[cfg(test)]
fn collect_node_addresses<K, V>(
    node: Option<&Node<K, V>>,
    addresses: &mut std::collections::HashSet<*const Node<K, V>>,
) {
    let Some(node) = node else {
        return;
    };
    addresses.insert(std::ptr::from_ref(node));
    collect_node_addresses(node.left.as_deref(), addresses);
    collect_node_addresses(node.right.as_deref(), addresses);
}

#[cfg(test)]
fn count_shared_nodes<K, V>(
    node: Option<&Node<K, V>>,
    addresses: &std::collections::HashSet<*const Node<K, V>>,
) -> usize {
    let Some(node) = node else {
        return 0;
    };
    usize::from(addresses.contains(&std::ptr::from_ref(node)))
        + count_shared_nodes(node.left.as_deref(), addresses)
        + count_shared_nodes(node.right.as_deref(), addresses)
}

fn insert<K: Ord + Clone, V>(root: Link<K, V>, key: K, value: Rc<V>) -> Rc<Node<K, V>> {
    let Some(node) = root else {
        return make_node(key, value, None, None);
    };
    match key.cmp(&node.key) {
        Ordering::Less => balance(make_node(
            node.key.clone(),
            Rc::clone(&node.value),
            Some(insert(node.left.clone(), key, value)),
            node.right.clone(),
        )),
        Ordering::Greater => balance(make_node(
            node.key.clone(),
            Rc::clone(&node.value),
            node.left.clone(),
            Some(insert(node.right.clone(), key, value)),
        )),
        Ordering::Equal => make_node(key, value, node.left.clone(), node.right.clone()),
    }
}

fn remove<K: Ord + Clone, V>(root: Link<K, V>, key: &K) -> (Link<K, V>, bool) {
    let Some(node) = root else {
        return (None, false);
    };
    match key.cmp(&node.key) {
        Ordering::Less => {
            let (left, removed) = remove(node.left.clone(), key);
            let root = make_node(
                node.key.clone(),
                Rc::clone(&node.value),
                left,
                node.right.clone(),
            );
            (Some(balance(root)), removed)
        }
        Ordering::Greater => {
            let (right, removed) = remove(node.right.clone(), key);
            let root = make_node(
                node.key.clone(),
                Rc::clone(&node.value),
                node.left.clone(),
                right,
            );
            (Some(balance(root)), removed)
        }
        Ordering::Equal => match (&node.left, &node.right) {
            (None, _) => (node.right.clone(), true),
            (_, None) => (node.left.clone(), true),
            (Some(_), Some(right)) => {
                let (successor_key, successor_value, next_right) = take_min(Rc::clone(right));
                (
                    Some(balance(make_node(
                        successor_key,
                        successor_value,
                        node.left.clone(),
                        next_right,
                    ))),
                    true,
                )
            }
        },
    }
}

fn take_min<K: Ord + Clone, V>(node: Rc<Node<K, V>>) -> (K, Rc<V>, Link<K, V>) {
    let Some(left) = &node.left else {
        return (node.key.clone(), Rc::clone(&node.value), node.right.clone());
    };
    let (key, value, next_left) = take_min(Rc::clone(left));
    let successor = balance(make_node(
        node.key.clone(),
        Rc::clone(&node.value),
        next_left,
        node.right.clone(),
    ));
    (key, value, Some(successor))
}

fn balance<K: Ord + Clone, V>(node: Rc<Node<K, V>>) -> Rc<Node<K, V>> {
    let skew = height(&node.left) as i32 - height(&node.right) as i32;
    if skew > 1 {
        let left = Rc::clone(
            node.left
                .as_ref()
                .expect("left-heavy node has a left child"),
        );
        return if height(&left.left) >= height(&left.right) {
            rotate_right(node)
        } else {
            rotate_right(with_left(node, rotate_left(left)))
        };
    }
    if skew < -1 {
        let right = Rc::clone(
            node.right
                .as_ref()
                .expect("right-heavy node has a right child"),
        );
        return if height(&right.right) >= height(&right.left) {
            rotate_left(node)
        } else {
            rotate_left(with_right(node, rotate_right(right)))
        };
    }
    node
}

fn rotate_left<K: Ord + Clone, V>(root: Rc<Node<K, V>>) -> Rc<Node<K, V>> {
    let pivot = root
        .right
        .as_ref()
        .expect("left rotation requires right child");
    let left = make_node(
        root.key.clone(),
        Rc::clone(&root.value),
        root.left.clone(),
        pivot.left.clone(),
    );
    make_node(
        pivot.key.clone(),
        Rc::clone(&pivot.value),
        Some(left),
        pivot.right.clone(),
    )
}

fn rotate_right<K: Ord + Clone, V>(root: Rc<Node<K, V>>) -> Rc<Node<K, V>> {
    let pivot = root
        .left
        .as_ref()
        .expect("right rotation requires left child");
    let right = make_node(
        root.key.clone(),
        Rc::clone(&root.value),
        pivot.right.clone(),
        root.right.clone(),
    );
    make_node(
        pivot.key.clone(),
        Rc::clone(&pivot.value),
        pivot.left.clone(),
        Some(right),
    )
}

fn with_left<K: Ord + Clone, V>(root: Rc<Node<K, V>>, left: Rc<Node<K, V>>) -> Rc<Node<K, V>> {
    make_node(
        root.key.clone(),
        Rc::clone(&root.value),
        Some(left),
        root.right.clone(),
    )
}

fn with_right<K: Ord + Clone, V>(root: Rc<Node<K, V>>, right: Rc<Node<K, V>>) -> Rc<Node<K, V>> {
    make_node(
        root.key.clone(),
        Rc::clone(&root.value),
        root.left.clone(),
        Some(right),
    )
}

fn make_node<K, V>(key: K, value: Rc<V>, left: Link<K, V>, right: Link<K, V>) -> Rc<Node<K, V>> {
    Rc::new(Node {
        key,
        value,
        height: 1 + height(&left).max(height(&right)),
        len: 1 + node_len(&left) + node_len(&right),
        left,
        right,
    })
}

fn height<K, V>(node: &Link<K, V>) -> u16 {
    node.as_ref().map_or(0, |node| node.height)
}

fn node_len<K, V>(node: &Link<K, V>) -> usize {
    node.as_ref().map_or(0, |node| node.len)
}

pub(crate) struct UiPersistentOrdMapIter<'a, K, V> {
    stack: Vec<&'a Node<K, V>>,
}

impl<'a, K, V> UiPersistentOrdMapIter<'a, K, V> {
    fn new(root: Option<&'a Node<K, V>>) -> Self {
        let mut iter = Self { stack: Vec::new() };
        iter.push_left(root);
        iter
    }

    fn push_left(&mut self, mut node: Option<&'a Node<K, V>>) {
        while let Some(current) = node {
            self.stack.push(current);
            node = current.left.as_deref();
        }
    }
}

impl<'a, K, V> Iterator for UiPersistentOrdMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        self.push_left(node.right.as_deref());
        Some((&node.key, node.value.as_ref()))
    }
}

impl<K: fmt::Debug + Ord + Clone, V: fmt::Debug> fmt::Debug for UiPersistentOrdMap<K, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Ord + Clone + PartialEq, V: PartialEq> PartialEq for UiPersistentOrdMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl<K: Ord + Clone + Eq, V: Eq> Eq for UiPersistentOrdMap<K, V> {}

#[cfg(test)]
mod tests {
    use super::UiPersistentOrdMap;

    #[test]
    fn updates_share_unaffected_tree_and_preserve_sorted_truth() {
        let mut map = UiPersistentOrdMap::default();
        for key in 0..1_000 {
            map.insert(key, key * 2);
        }
        let predecessor = map.clone();
        map.insert(500, 9_999);
        map.remove(&501);

        assert_eq!(predecessor.get(&500), Some(&1_000));
        assert_eq!(map.get(&500), Some(&9_999));
        assert_eq!(map.get(&501), None);
        assert_eq!(predecessor.len(), 1_000);
        assert_eq!(map.len(), 999);
        assert!(map.iter().map(|(key, _)| *key).is_sorted());
        assert!(!map.root_is_shared_with(&predecessor));
    }

    #[test]
    fn cloning_an_unchanged_catalog_is_constant_identity_sharing() {
        let mut map = UiPersistentOrdMap::default();
        map.insert(7, "seven");
        let clone = map.clone();
        assert!(map.root_is_shared_with(&clone));
    }

    #[test]
    fn mixed_local_churn_retains_the_unrelated_persistent_catalog() {
        const ROWS: usize = 4_096;
        let mut map = UiPersistentOrdMap::default();
        for key in 0..ROWS {
            map.insert(key, key * 2);
        }
        let predecessor = map.clone();

        for key in 2_040..2_048 {
            assert!(map.remove(&key));
        }
        for key in 2_048..2_056 {
            map.insert(key, key * 3);
        }
        for key in ROWS..ROWS + 8 {
            map.insert(key, key * 2);
        }

        assert_eq!(predecessor.len(), ROWS);
        assert_eq!(map.len(), ROWS);
        assert!(map.iter().map(|(key, _)| *key).is_sorted());
        assert!(
            map.shared_node_count_with(&predecessor) > ROWS - 128,
            "a bounded local delta must retain almost all unrelated predecessor nodes"
        );
    }
}
