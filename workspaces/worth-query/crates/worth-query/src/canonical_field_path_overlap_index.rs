use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{CanonicalFieldPath, FieldKey};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthQueryCanonicalPathIndexWork {
    pub(crate) node_probes: usize,
    pub(crate) targeted_values_visited: usize,
    pub(crate) overlap_deduplications: usize,
}

#[derive(Clone)]
pub(crate) struct WorthQueryCanonicalPathOverlapIndex<T: Clone + Ord> {
    root: WorthQueryCanonicalPathNode<T>,
}

impl<T: Clone + Ord> Default for WorthQueryCanonicalPathOverlapIndex<T> {
    fn default() -> Self {
        Self {
            root: WorthQueryCanonicalPathNode::default(),
        }
    }
}

#[derive(Clone)]
struct WorthQueryCanonicalPathNode<T: Clone + Ord> {
    exact: BTreeSet<T>,
    subtree: BTreeSet<T>,
    children: BTreeMap<FieldKey, WorthQueryCanonicalPathNode<T>>,
}

impl<T: Clone + Ord> Default for WorthQueryCanonicalPathNode<T> {
    fn default() -> Self {
        Self {
            exact: BTreeSet::new(),
            subtree: BTreeSet::new(),
            children: BTreeMap::new(),
        }
    }
}

impl<T: Clone + Ord> WorthQueryCanonicalPathOverlapIndex<T> {
    pub(crate) fn is_empty(&self) -> bool {
        self.root.subtree.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.root.subtree.len()
    }

    pub(crate) fn insert(&mut self, path: &CanonicalFieldPath, value: T) {
        let mut node = &mut self.root;
        node.subtree.insert(value.clone());
        for field in path.fields() {
            node = node.children.entry(field.clone()).or_default();
            node.subtree.insert(value.clone());
        }
        node.exact.insert(value);
    }

    pub(crate) fn remove(&mut self, path: &CanonicalFieldPath, value: &T) {
        remove_from_node(&mut self.root, path.fields(), value);
    }

    pub(crate) fn overlapping(
        &self,
        path: &CanonicalFieldPath,
    ) -> (BTreeSet<T>, WorthQueryCanonicalPathIndexWork) {
        let mut selected = BTreeSet::new();
        let mut work = WorthQueryCanonicalPathIndexWork::default();
        let mut node = &self.root;
        let fields = path.fields();
        for (ordinal, field) in fields.iter().enumerate() {
            work.node_probes += 1;
            let Some(child) = node.children.get(field) else {
                return (selected, work);
            };
            node = child;
            if ordinal + 1 < fields.len() {
                insert_values(&node.exact, &mut selected, &mut work);
            }
        }
        insert_values(&node.subtree, &mut selected, &mut work);
        (selected, work)
    }
}

fn remove_from_node<T: Clone + Ord>(
    node: &mut WorthQueryCanonicalPathNode<T>,
    fields: &[FieldKey],
    value: &T,
) -> bool {
    node.subtree.remove(value);
    if let Some((field, tail)) = fields.split_first() {
        let remove_child = node
            .children
            .get_mut(field)
            .is_some_and(|child| remove_from_node(child, tail, value));
        if remove_child {
            node.children.remove(field);
        }
    } else {
        node.exact.remove(value);
    }
    node.subtree.is_empty() && node.exact.is_empty() && node.children.is_empty()
}

fn insert_values<T: Clone + Ord>(
    source: &BTreeSet<T>,
    selected: &mut BTreeSet<T>,
    work: &mut WorthQueryCanonicalPathIndexWork,
) {
    for value in source {
        work.targeted_values_visited += 1;
        if !selected.insert(value.clone()) {
            work.overlap_deduplications += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_and_descendant_paths_overlap_without_sibling_selection() {
        let mut index = WorthQueryCanonicalPathOverlapIndex::default();
        index.insert(&path(&["address"]), "parent");
        index.insert(&path(&["address", "city"]), "city");
        index.insert(&path(&["address", "postal"]), "postal");

        assert_eq!(
            index.overlapping(&path(&["address", "city", "name"])).0,
            BTreeSet::from(["city", "parent"])
        );
        assert_eq!(
            index.overlapping(&path(&["address"])).0,
            BTreeSet::from(["city", "parent", "postal"])
        );
    }

    fn path(fields: &[&str]) -> CanonicalFieldPath {
        CanonicalFieldPath::new(fields.iter().map(|field| FieldKey::new(*field).unwrap())).unwrap()
    }
}
