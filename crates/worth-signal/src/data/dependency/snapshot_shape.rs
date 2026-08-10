use std::collections::HashMap;
use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use super::DependencySortKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct DependencySnapshotShape {
    keys: std::sync::Arc<Vec<DependencySortKey>>,
}

impl DependencySnapshotShape {
    pub fn from_ordered_unique(keys: impl IntoIterator<Item = DependencySortKey>) -> Self {
        let keys = keys.into_iter().collect::<Vec<_>>();
        debug_assert!(is_strict_snapshot_shape_order(keys.as_slice()));
        Self {
            keys: std::sync::Arc::new(keys),
        }
    }

    pub fn as_slice(&self) -> &[DependencySortKey] {
        self.keys.as_slice()
    }

    pub(crate) fn intern(&self, store: &mut DependencySnapshotShapeStore) -> SnapshotShapeHandle {
        store.intern(self.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct SnapshotShapeHandle(Option<NonZeroU32>);

impl SnapshotShapeHandle {
    pub const EMPTY: Self = Self(None);

    fn from_index(index: usize) -> Self {
        debug_assert!(index > 0);
        Self(NonZeroU32::new(index as u32))
    }

    fn index(self) -> Option<usize> {
        self.0.map(|index| index.get() as usize)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DependencySnapshotShapeStore {
    shapes: Vec<DependencySnapshotShape>,
    #[serde(skip, default)]
    interner: HashMap<DependencySnapshotShape, SnapshotShapeHandle>,
}

impl DependencySnapshotShapeStore {
    fn rebuild_interner_if_needed(&mut self) {
        if !self.interner.is_empty() || self.shapes.is_empty() {
            return;
        }
        self.interner.reserve(self.shapes.len());
        for (index, shape) in self.shapes.iter().cloned().enumerate() {
            self.interner
                .insert(shape, SnapshotShapeHandle::from_index(index + 1));
        }
    }

    pub fn intern(&mut self, shape: DependencySnapshotShape) -> SnapshotShapeHandle {
        if shape.as_slice().is_empty() {
            return SnapshotShapeHandle::EMPTY;
        }
        self.rebuild_interner_if_needed();
        if let Some(handle) = self.interner.get(&shape).copied() {
            return handle;
        }
        self.shapes.push(shape);
        let handle = SnapshotShapeHandle::from_index(self.shapes.len());
        let shape = self.shapes[handle.index().expect("shape handle should index") - 1].clone();
        self.interner.insert(shape, handle);
        handle
    }
}

fn is_strict_snapshot_shape_order(keys: &[DependencySortKey]) -> bool {
    keys.windows(2).all(|pair| pair[0] < pair[1])
}
