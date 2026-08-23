use crate::mounting::{UiMountedCollectionTextChange, UiMountedCollectionTextRow};
use crate::runtime::persistent_index::{UiPersistentOrdMap, UiPersistentRankedSequence};

use super::super::super::super::UiMountedProjectionDenial;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::mounting::projection) struct UiMountedCollectionTextKey([u8; 32]);

#[derive(Clone, Default)]
pub(in crate::mounting::projection) struct UiMountedCollectionTextSource {
    rows: UiPersistentOrdMap<UiMountedCollectionTextKey, UiMountedCollectionTextRow>,
    order: UiPersistentRankedSequence<UiMountedCollectionTextKey>,
    selected_value_count: usize,
    selected_byte_count: usize,
}

#[derive(Clone, Copy)]
enum RankMutation {
    Insert(usize),
    Remove(usize),
    Move { from: usize, to: usize },
}

impl UiMountedCollectionTextSource {
    pub(super) fn replace(
        rows: &[UiMountedCollectionTextRow],
    ) -> Result<Self, UiMountedProjectionDenial> {
        let mut source = Self::default();
        for row in rows {
            let key = UiMountedCollectionTextKey::for_row(row);
            if source.rows.get(&key).is_some() {
                return Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch);
            }
            source.selected_value_count = source
                .selected_value_count
                .checked_add(row.selected_values().len())
                .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
            source.selected_byte_count = source
                .selected_byte_count
                .checked_add(selected_bytes(row))
                .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
            source
                .order
                .insert(source.order.len(), key)
                .map_err(|()| UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
            source.rows.insert(key, row.clone());
        }
        Ok(source)
    }

    pub(super) fn apply(
        &self,
        changes: &[UiMountedCollectionTextChange],
    ) -> Result<Self, UiMountedProjectionDenial> {
        let mut successor = self.clone();
        let mut rank_mutations = Vec::with_capacity(changes.len());
        for change in changes {
            successor.apply_one(self, change, &mut rank_mutations)?;
        }
        Ok(successor)
    }

    fn apply_one(
        &mut self,
        predecessor: &Self,
        change: &UiMountedCollectionTextChange,
        ranks: &mut Vec<RankMutation>,
    ) -> Result<(), UiMountedProjectionDenial> {
        use UiMountedCollectionTextChange as Change;
        match change {
            Change::Insert { row, at } => self.insert(row, *at, ranks),
            Change::Remove { identity, from } => {
                let key = UiMountedCollectionTextKey::for_identity(identity);
                self.require_predecessor(predecessor, *from, key)?;
                self.remove(key, translated_rank(*from, ranks), ranks)
            }
            Change::Move { identity, from, to } => {
                let key = UiMountedCollectionTextKey::for_identity(identity);
                self.require_predecessor(predecessor, *from, key)?;
                self.move_row(key, translated_rank(*from, ranks), *to, ranks)
            }
            Change::Regroup { identity } => {
                let key = UiMountedCollectionTextKey::for_identity(identity);
                self.rows
                    .get(&key)
                    .map(|_| ())
                    .ok_or(UiMountedProjectionDenial::InvalidSemanticCollectionPatch)
            }
            Change::Update(row) => self.update(row),
            Change::WindowShift => Ok(()),
        }
    }

    fn insert(
        &mut self,
        row: &UiMountedCollectionTextRow,
        at: usize,
        ranks: &mut Vec<RankMutation>,
    ) -> Result<(), UiMountedProjectionDenial> {
        let key = UiMountedCollectionTextKey::for_row(row);
        if self.rows.get(&key).is_some() || at > self.order.len() {
            return Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch);
        }
        self.adjust_value_count(0, row.selected_values().len())?;
        self.adjust_byte_count(0, selected_bytes(row))?;
        self.order
            .insert(at, key)
            .map_err(|()| UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
        self.rows.insert(key, row.clone());
        ranks.push(RankMutation::Insert(at));
        Ok(())
    }

    fn remove(
        &mut self,
        key: UiMountedCollectionTextKey,
        at: usize,
        ranks: &mut Vec<RankMutation>,
    ) -> Result<(), UiMountedProjectionDenial> {
        let row = self
            .rows
            .get(&key)
            .ok_or(UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
        if self.order.get(at) != Some(&key) {
            return Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch);
        }
        let removed_values = row.selected_values().len();
        let removed_bytes = selected_bytes(row);
        self.adjust_value_count(removed_values, 0)?;
        self.adjust_byte_count(removed_bytes, 0)?;
        self.order
            .remove(at)
            .map_err(|()| UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
        self.rows.remove(&key);
        ranks.push(RankMutation::Remove(at));
        Ok(())
    }

    fn move_row(
        &mut self,
        key: UiMountedCollectionTextKey,
        from: usize,
        to: usize,
        ranks: &mut Vec<RankMutation>,
    ) -> Result<(), UiMountedProjectionDenial> {
        if self.order.get(from) != Some(&key) || to >= self.order.len() {
            return Err(UiMountedProjectionDenial::InvalidSemanticCollectionPatch);
        }
        self.order
            .move_rank(from, to)
            .map_err(|()| UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
        ranks.push(RankMutation::Move { from, to });
        Ok(())
    }

    fn update(
        &mut self,
        row: &UiMountedCollectionTextRow,
    ) -> Result<(), UiMountedProjectionDenial> {
        let key = UiMountedCollectionTextKey::for_row(row);
        let predecessor = self
            .rows
            .get(&key)
            .ok_or(UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
        let removed_values = predecessor.selected_values().len();
        let removed_bytes = selected_bytes(predecessor);
        self.adjust_value_count(removed_values, row.selected_values().len())?;
        self.adjust_byte_count(removed_bytes, selected_bytes(row))?;
        self.rows.insert(key, row.clone());
        Ok(())
    }

    fn require_predecessor(
        &self,
        predecessor: &Self,
        at: usize,
        key: UiMountedCollectionTextKey,
    ) -> Result<(), UiMountedProjectionDenial> {
        (predecessor.order.get(at) == Some(&key))
            .then_some(())
            .ok_or(UiMountedProjectionDenial::InvalidSemanticCollectionPatch)
    }

    fn adjust_value_count(
        &mut self,
        removed: usize,
        inserted: usize,
    ) -> Result<(), UiMountedProjectionDenial> {
        self.selected_value_count = self
            .selected_value_count
            .checked_sub(removed)
            .and_then(|count| count.checked_add(inserted))
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
        Ok(())
    }

    fn adjust_byte_count(
        &mut self,
        removed: usize,
        inserted: usize,
    ) -> Result<(), UiMountedProjectionDenial> {
        self.selected_byte_count = self
            .selected_byte_count
            .checked_sub(removed)
            .and_then(|count| count.checked_add(inserted))
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
        Ok(())
    }

    pub(in crate::mounting::projection) fn selected_value_count(&self) -> usize {
        self.selected_value_count
    }

    pub(in crate::mounting::projection) fn selected_byte_count(&self) -> usize {
        self.selected_byte_count
    }

    pub(in crate::mounting::projection) fn row(
        &self,
        key: UiMountedCollectionTextKey,
    ) -> Option<&UiMountedCollectionTextRow> {
        self.rows.get(&key)
    }

    pub(in crate::mounting::projection) fn rows(
        &self,
    ) -> impl ExactSizeIterator<Item = &UiMountedCollectionTextRow> {
        self.order.iter().map(|key| {
            self.rows
                .get(key)
                .expect("collection order names an indexed row")
        })
    }
}

fn selected_bytes(row: &UiMountedCollectionTextRow) -> usize {
    row.selected_values().iter().map(|value| value.len()).sum()
}

impl UiMountedCollectionTextKey {
    pub(in crate::mounting::projection) fn for_row(row: &UiMountedCollectionTextRow) -> Self {
        Self::for_identity(row.identity())
    }

    pub(in crate::mounting::projection) fn for_identity(
        identity: &crate::mounting::UiMountedCollectionRowIdentity,
    ) -> Self {
        Self(
            identity
                .query_reference()
                .query_identity()
                .operational_key()
                .correlation_digest(),
        )
    }

    pub(in crate::mounting::projection) const fn correlation_digest(self) -> [u8; 32] {
        self.0
    }
}

fn translated_rank(mut rank: usize, mutations: &[RankMutation]) -> usize {
    for mutation in mutations {
        match *mutation {
            RankMutation::Insert(at) if at <= rank => rank += 1,
            RankMutation::Remove(at) if at < rank => rank -= 1,
            RankMutation::Move { from, to } if rank == from => rank = to,
            RankMutation::Move { from, to } if from < rank && to >= rank => rank -= 1,
            RankMutation::Move { from, to } if from > rank && to <= rank => rank += 1,
            RankMutation::Insert(_) | RankMutation::Remove(_) | RankMutation::Move { .. } => {}
        }
    }
    rank
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn patch_updates_and_reorders_without_copying_unrelated_rows() {
        let predecessor = UiMountedCollectionTextSource::replace(&[
            row(1, "Alpha"),
            row(2, "Bravo"),
            row(3, "Charlie"),
        ])
        .unwrap();
        let successor = predecessor
            .apply(&[
                UiMountedCollectionTextChange::Update(row(2, "Bravo updated")),
                UiMountedCollectionTextChange::Move {
                    identity: identity(3),
                    from: 2,
                    to: 0,
                },
                UiMountedCollectionTextChange::Remove {
                    identity: identity(1),
                    from: 0,
                },
            ])
            .unwrap();
        assert_eq!(
            successor
                .rows()
                .map(|row| row.selected_values()[0].as_ref())
                .collect::<Vec<_>>(),
            ["Charlie", "Bravo updated"]
        );
        assert_eq!(predecessor.rows().count(), 3);
    }

    fn row(local: u64, value: &str) -> UiMountedCollectionTextRow {
        UiMountedCollectionTextRow::new(identity(local), [Arc::from(value)])
    }

    fn identity(local: u64) -> crate::mounting::UiMountedCollectionRowIdentity {
        crate::mounting::UiMountedCollectionRowIdentity::from_query(
            &worth_ui_query_binding::certification::query_row_reference_fixture(local),
        )
    }
}
