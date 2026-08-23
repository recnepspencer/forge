use std::collections::{BTreeMap, HashMap};

use crate::history::data::BranchId;

use super::{RelationalBranchCellCheckpoint, RelationalBranchReferenceCell};

/// Mutable owner of runtime-affine branch-reference cells.
///
/// This registry is the only map that may insert or rebind live cells. The
/// immutable commit catalog is a sibling owner and cannot mint a cell.
#[derive(Debug, Clone, Default)]
pub(crate) struct RelationalBranchReferenceRegistry {
    cells: HashMap<BranchId, RelationalBranchReferenceCell>,
}

impl RelationalBranchReferenceRegistry {
    pub(crate) fn from_main(main: RelationalBranchReferenceCell) -> Self {
        let branch_id = main.identity().branch_id().clone();
        Self {
            cells: HashMap::from([(branch_id, main)]),
        }
    }

    pub(crate) fn get(&self, branch_id: &BranchId) -> Option<&RelationalBranchReferenceCell> {
        self.cells.get(branch_id)
    }

    pub(crate) fn get_mut(
        &mut self,
        branch_id: &BranchId,
    ) -> Option<&mut RelationalBranchReferenceCell> {
        self.cells.get_mut(branch_id)
    }

    pub(crate) fn insert(&mut self, cell: RelationalBranchReferenceCell) {
        self.cells.insert(cell.identity().branch_id().clone(), cell);
    }

    pub(crate) fn contains(&self, branch_id: &BranchId) -> bool {
        self.cells.contains_key(branch_id)
    }

    pub(crate) fn len(&self) -> usize {
        self.cells.len()
    }

    pub(crate) fn keys(&self) -> impl Iterator<Item = &BranchId> {
        self.cells.keys()
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &RelationalBranchReferenceCell> {
        self.cells.values()
    }

    pub(crate) fn take_all(&mut self) -> BTreeMap<BranchId, RelationalBranchReferenceCell> {
        std::mem::take(&mut self.cells).into_iter().collect()
    }

    pub(crate) fn restore_all(&mut self, cells: BTreeMap<BranchId, RelationalBranchReferenceCell>) {
        self.cells = cells.into_iter().collect();
    }

    pub(crate) fn checkpoints(&self) -> Vec<RelationalBranchCellCheckpoint> {
        let mut checkpoints = self
            .cells
            .values()
            .map(RelationalBranchReferenceCell::checkpoint)
            .collect::<Vec<_>>();
        checkpoints.sort_by(|left, right| left.branch_id.cmp(&right.branch_id));
        checkpoints
    }
}
