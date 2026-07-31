use std::cmp::Ordering;
use std::sync::Arc;

use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

use super::{UiProjectionInputCollectionRow, UiProjectionInputTransitionWork};

#[path = "collection_catalog/mutation.rs"]
mod mutation;

pub(super) type Link = Option<Arc<Node>>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct UiProjectionInputCollectionCatalog {
    root: Link,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Node {
    pub(super) key: UiProjectionOptionKey,
    pub(super) value: Arc<UiProjectionInputCollectionRow>,
    pub(super) left: Link,
    pub(super) right: Link,
    pub(super) height: u16,
    pub(super) len: usize,
}

#[derive(Clone, Debug)]
pub(super) struct UiProjectionOptionKey {
    identity: Arc<WorthQueryEvidenceIdentity>,
}

impl UiProjectionInputCollectionCatalog {
    pub(super) fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.len)
    }

    pub(super) fn replace(
        rows: impl IntoIterator<Item = UiProjectionInputCollectionRow>,
    ) -> Result<(Self, UiProjectionInputTransitionWork), ()> {
        let mut catalog = Self::default();
        let mut work = UiProjectionInputTransitionWork::default();
        for row in rows {
            let (next, previous, mutation) = insert(catalog.root, row);
            if previous.is_some() {
                return Err(());
            }
            catalog.root = Some(next);
            work.record_replaced_row(mutation)?;
        }
        Ok((catalog, work))
    }

    pub(super) fn row(
        &self,
        identity: &WorthQueryEvidenceIdentity,
    ) -> (Option<&UiProjectionInputCollectionRow>, usize) {
        let key = UiProjectionOptionKey::new(identity.clone());
        let mut cursor = self.root.as_deref();
        let mut probes = 0;
        while let Some(node) = cursor {
            probes += 1;
            match key.cmp(&node.key) {
                Ordering::Less => cursor = node.left.as_deref(),
                Ordering::Greater => cursor = node.right.as_deref(),
                Ordering::Equal => return (Some(node.value.as_ref()), probes),
            }
        }
        (None, probes)
    }

    pub(super) fn insert(
        &mut self,
        row: UiProjectionInputCollectionRow,
    ) -> Result<UiProjectionInputTransitionWork, ()> {
        let (root, previous, mutation) = insert(self.root.take(), row);
        if previous.is_some() {
            return Err(());
        }
        self.root = Some(root);
        Ok(mutation)
    }

    pub(super) fn update(
        &mut self,
        row: UiProjectionInputCollectionRow,
    ) -> Result<UiProjectionInputTransitionWork, ()> {
        let (root, previous, mutation) = insert(self.root.take(), row);
        if previous.is_none() {
            return Err(());
        }
        self.root = Some(root);
        Ok(mutation)
    }

    pub(super) fn remove(
        &mut self,
        identity: &WorthQueryEvidenceIdentity,
    ) -> Result<UiProjectionInputTransitionWork, ()> {
        let key = UiProjectionOptionKey::new(identity.clone());
        let mut work = UiProjectionInputTransitionWork::default();
        let (root, removed) = mutation::remove(self.root.take(), &key, &mut work);
        if !removed {
            return Err(());
        }
        self.root = root;
        Ok(work)
    }

    pub(super) fn require(
        &self,
        identity: &WorthQueryEvidenceIdentity,
    ) -> Result<UiProjectionInputTransitionWork, ()> {
        let (row, probes) = self.row(identity);
        if row.is_none() {
            return Err(());
        }
        Ok(UiProjectionInputTransitionWork::with_key_probes(probes))
    }
}

impl UiProjectionOptionKey {
    pub(super) fn new(identity: WorthQueryEvidenceIdentity) -> Self {
        Self {
            identity: Arc::new(identity),
        }
    }
}

impl PartialEq for UiProjectionOptionKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for UiProjectionOptionKey {}

impl PartialOrd for UiProjectionOptionKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UiProjectionOptionKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.identity
            .scope()
            .cmp(&other.identity.scope())
            .then_with(|| self.identity.scheme().cmp(&other.identity.scheme()))
            .then_with(|| {
                self.identity
                    .compare_same_scheme(&other.identity)
                    .expect("scheme was ordered before same-scheme identity comparison")
            })
    }
}

fn insert(
    root: Link,
    row: UiProjectionInputCollectionRow,
) -> (
    Arc<Node>,
    Option<Arc<UiProjectionInputCollectionRow>>,
    UiProjectionInputTransitionWork,
) {
    let key = UiProjectionOptionKey::new(row.row().query_row_identity().clone());
    let mut work = UiProjectionInputTransitionWork::default();
    let (root, previous) = mutation::insert(root, key, Arc::new(row), &mut work);
    (root, previous, work)
}
