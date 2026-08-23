use super::{UiPersistentIndexMutationWork, UiPersistentOrdMap};

/// Structurally shared authored order for identities whose ordinary insertion is append-only.
/// Explicit arbitrary reorder remains proportional to the declared reordered row count.
#[derive(Clone)]
pub(crate) struct UiPersistentOrder<Identity> {
    by_identity: UiPersistentOrdMap<Identity, u64>,
    by_position: UiPersistentOrdMap<u64, Identity>,
    next_position: u64,
}

impl<Identity> Default for UiPersistentOrder<Identity> {
    fn default() -> Self {
        Self {
            by_identity: UiPersistentOrdMap::default(),
            by_position: UiPersistentOrdMap::default(),
            next_position: 0,
        }
    }
}

impl<Identity> UiPersistentOrder<Identity>
where
    Identity: Copy + Ord,
{
    pub(crate) fn len(&self) -> usize {
        self.by_identity.len()
    }

    pub(crate) fn append(
        &mut self,
        identity: Identity,
    ) -> Result<UiPersistentIndexMutationWork, ()> {
        if self.by_identity.get(&identity).is_some() {
            return Err(());
        }
        let position = self.next_position;
        self.next_position = position.checked_add(1).ok_or(())?;
        let mut work = self.by_identity.insert_with_work(identity, position);
        work.merge(self.by_position.insert_with_work(position, identity))?;
        Ok(work)
    }

    pub(crate) fn remove(
        &mut self,
        identity: Identity,
    ) -> Result<UiPersistentIndexMutationWork, ()> {
        let position = *self.by_identity.get(&identity).ok_or(())?;
        let (removed_identity, mut work) = self.by_identity.remove_with_work(&identity);
        let (removed_position, position_work) = self.by_position.remove_with_work(&position);
        if !removed_identity || !removed_position {
            return Err(());
        }
        work.merge(position_work)?;
        Ok(work)
    }

    pub(crate) fn replace_all(
        &mut self,
        identities: &[Identity],
    ) -> Result<UiPersistentIndexMutationWork, ()> {
        let mut replacement = Self::default();
        let mut work = UiPersistentIndexMutationWork::default();
        for identity in identities {
            work.merge(replacement.append(*identity)?)?;
        }
        *self = replacement;
        Ok(work)
    }

    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = &Identity> {
        self.by_position.iter().map(|(_, identity)| identity)
    }

    pub(crate) fn position(&self, identity: Identity) -> Option<u64> {
        self.by_identity.get(&identity).copied()
    }

    pub(crate) fn retained_structural_bytes(&self) -> Option<usize> {
        std::mem::size_of::<Self>()
            .checked_add(self.by_identity.retained_structural_bytes()?)?
            .checked_add(self.by_position.retained_structural_bytes()?)
    }
}

#[cfg(test)]
mod tests {
    use super::UiPersistentOrder;

    #[test]
    fn append_remove_and_snapshot_share_authored_order_without_relabeling() {
        let mut order = UiPersistentOrder::default();
        for identity in 0..4_096_u64 {
            order.append(identity).expect("bounded append");
        }
        let predecessor = order.clone();
        order.remove(2_048).expect("existing identity");
        order.append(4_096).expect("successor append");

        assert_eq!(predecessor.len(), 4_096);
        assert_eq!(order.len(), 4_096);
        assert_eq!(order.iter().nth(4_095), Some(&4_096));
        assert_eq!(predecessor.iter().nth(2_048), Some(&2_048));
        assert!(!order.iter().any(|identity| *identity == 2_048));
    }

    #[test]
    fn explicit_reorder_rebuilds_only_when_the_owner_declares_all_rows() {
        let mut order = UiPersistentOrder::default();
        order.replace_all(&[3_u64, 1, 2]).expect("exact order");
        assert_eq!(order.iter().copied().collect::<Vec<_>>(), [3, 1, 2]);
    }
}
