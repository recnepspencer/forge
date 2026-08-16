use std::any::Any;
use std::collections::BTreeMap;

use super::error::SubscriberContextError;

/// Shared typed staging/committed store for subscriber coordination.
///
/// Subscribers write staged values during `on_checkpoint`.
/// At successful flush end, staged values are promoted to committed values.
/// Values move with their exclusively owned runtime rather than becoming
/// concurrently shared state.
#[derive(Default)]
pub struct SubscriberContext<D: Copy + Ord + std::fmt::Debug + 'static> {
    staged: BTreeMap<D, Box<dyn Any + Send>>,
    committed: BTreeMap<D, Box<dyn Any + Send>>,
}

impl<D: Copy + Ord + std::fmt::Debug + 'static> SubscriberContext<D> {
    /// Create a new empty context.
    pub fn new() -> Self {
        Self {
            staged: BTreeMap::new(),
            committed: BTreeMap::new(),
        }
    }

    /// Stage one typed value for this checkpoint.
    ///
    /// Returns an error if the same `data_id` is staged more than once
    /// in one checkpoint cycle.
    pub fn stage<T: Any + Send>(
        &mut self,
        id: D,
        value: T,
    ) -> Result<(), SubscriberContextError<D>> {
        if self.staged.contains_key(&id) {
            return Err(SubscriberContextError::DuplicateStagedDataId { data_id: id });
        }
        self.staged.insert(id, Box::new(value));
        Ok(())
    }

    /// Read a staged typed value.
    pub fn staged<T: Any>(&self, id: D) -> Option<&T> {
        self.staged.get(&id)?.downcast_ref::<T>()
    }

    /// Read a committed typed value.
    pub fn committed<T: Any>(&self, id: D) -> Option<&T> {
        self.committed.get(&id)?.downcast_ref::<T>()
    }

    /// Return the currently staged data ids in deterministic order.
    pub fn staged_ids(&self) -> Vec<D> {
        self.staged.keys().copied().collect()
    }

    /// Return the currently committed data ids in deterministic order.
    pub fn committed_ids(&self) -> Vec<D> {
        self.committed.keys().copied().collect()
    }

    /// Promote staged values to committed values.
    ///
    /// Existing committed entries for the same IDs are replaced.
    pub fn finalize(&mut self) {
        for (id, value) in std::mem::take(&mut self.staged) {
            self.committed.insert(id, value);
        }
    }

    /// Clear any staged values without committing.
    pub fn clear_staged(&mut self) {
        self.staged.clear();
    }

    /// Clear both staged and committed values.
    pub fn clear_all(&mut self) {
        self.staged.clear();
        self.committed.clear();
    }
}
