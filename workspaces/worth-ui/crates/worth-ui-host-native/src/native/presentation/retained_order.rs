use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::ops::Bound::{Excluded, Unbounded};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct OrderLabel(Box<[u16]>);

pub(super) struct UiNativeRetainedOrder<Identity> {
    labels: HashMap<Identity, OrderLabel>,
    identities: BTreeMap<OrderLabel, Identity>,
}

pub(super) struct UiNativeRetainedOrderSnapshot<Identity> {
    entries: Vec<OriginalOrderEntry<Identity>>,
}

struct OriginalOrderEntry<Identity> {
    identity: Identity,
    predecessor: Option<Identity>,
    existed: bool,
    label: Option<OrderLabel>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiNativeRetainedOrderDenial {
    DuplicateIdentity,
    MissingIdentity,
    MissingPredecessor,
    SelfPredecessor,
}

impl<Identity> UiNativeRetainedOrder<Identity>
where
    Identity: Copy + Eq + Hash,
{
    pub(super) fn initial(
        identities: impl IntoIterator<Item = Identity>,
    ) -> Result<Self, UiNativeRetainedOrderDenial> {
        let mut order = Self {
            labels: HashMap::new(),
            identities: BTreeMap::new(),
        };
        let mut previous = None;
        for identity in identities {
            order.insert_after(identity, previous)?;
            previous = Some(identity);
        }
        Ok(order)
    }

    pub(super) fn remove(&mut self, identity: Identity) -> Result<(), UiNativeRetainedOrderDenial> {
        let label = self
            .labels
            .remove(&identity)
            .ok_or(UiNativeRetainedOrderDenial::MissingIdentity)?;
        self.identities
            .remove(&label)
            .ok_or(UiNativeRetainedOrderDenial::MissingIdentity)?;
        Ok(())
    }

    pub(super) fn place_after(
        &mut self,
        identity: Identity,
        predecessor: Option<Identity>,
    ) -> Result<(), UiNativeRetainedOrderDenial> {
        if predecessor == Some(identity) {
            return Err(UiNativeRetainedOrderDenial::SelfPredecessor);
        }
        if self.labels.contains_key(&identity) {
            self.remove(identity)?;
        }
        self.insert_after(identity, predecessor)
    }

    pub(super) fn ordered(&self) -> impl ExactSizeIterator<Item = Identity> + '_ {
        self.identities.values().copied()
    }

    pub(super) fn contains(&self, identity: Identity) -> bool {
        self.labels.contains_key(&identity)
    }

    pub(super) fn neighbors(
        &self,
        identity: Identity,
    ) -> Result<(Option<Identity>, Option<Identity>), UiNativeRetainedOrderDenial> {
        let label = self
            .labels
            .get(&identity)
            .ok_or(UiNativeRetainedOrderDenial::MissingIdentity)?;
        let predecessor = self
            .identities
            .range((Unbounded, Excluded(label)))
            .next_back()
            .map(|(_, identity)| *identity);
        let successor = self
            .identities
            .range((Excluded(label), Unbounded))
            .next()
            .map(|(_, identity)| *identity);
        Ok((predecessor, successor))
    }

    pub(super) fn successor_after(
        &self,
        predecessor: Option<Identity>,
    ) -> Result<Option<Identity>, UiNativeRetainedOrderDenial> {
        match predecessor {
            Some(predecessor) => {
                let label = self
                    .labels
                    .get(&predecessor)
                    .ok_or(UiNativeRetainedOrderDenial::MissingPredecessor)?;
                Ok(self
                    .identities
                    .range((Excluded(label), Unbounded))
                    .next()
                    .map(|(_, identity)| *identity))
            }
            None => Ok(self
                .identities
                .first_key_value()
                .map(|(_, identity)| *identity)),
        }
    }

    pub(super) fn ordered_subset(
        &self,
        identities: impl IntoIterator<Item = Identity>,
    ) -> Result<Vec<Identity>, UiNativeRetainedOrderDenial> {
        let mut labeled = identities
            .into_iter()
            .map(|identity| {
                self.labels
                    .get(&identity)
                    .cloned()
                    .map(|label| (label, identity))
                    .ok_or(UiNativeRetainedOrderDenial::MissingIdentity)
            })
            .collect::<Result<Vec<_>, _>>()?;
        labeled.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        Ok(labeled.into_iter().map(|(_, identity)| identity).collect())
    }

    pub(super) fn snapshot(
        &self,
        identities: impl IntoIterator<Item = Identity>,
    ) -> UiNativeRetainedOrderSnapshot<Identity> {
        let mut seen = HashMap::new();
        let mut entries = Vec::new();
        for identity in identities {
            if seen.insert(identity, ()).is_some() {
                continue;
            }
            let label = self.labels.get(&identity).cloned();
            let predecessor = label.as_ref().and_then(|label| {
                self.identities
                    .range((Unbounded, Excluded(label)))
                    .next_back()
                    .map(|(_, identity)| *identity)
            });
            entries.push(OriginalOrderEntry {
                identity,
                predecessor,
                existed: label.is_some(),
                label,
            });
        }
        entries.sort_unstable_by(|left, right| left.label.cmp(&right.label));
        UiNativeRetainedOrderSnapshot { entries }
    }

    pub(super) fn restore(
        &mut self,
        snapshot: UiNativeRetainedOrderSnapshot<Identity>,
    ) -> Result<(), UiNativeRetainedOrderDenial> {
        for entry in &snapshot.entries {
            if self.contains(entry.identity) {
                self.remove(entry.identity)?;
            }
        }
        for entry in snapshot.entries.into_iter().filter(|entry| entry.existed) {
            self.insert_after(entry.identity, entry.predecessor)?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn label_for(&self, identity: Identity) -> Option<&[u16]> {
        self.labels.get(&identity).map(|label| label.0.as_ref())
    }

    fn insert_after(
        &mut self,
        identity: Identity,
        predecessor: Option<Identity>,
    ) -> Result<(), UiNativeRetainedOrderDenial> {
        if self.labels.contains_key(&identity) {
            return Err(UiNativeRetainedOrderDenial::DuplicateIdentity);
        }
        let left = predecessor
            .map(|predecessor| {
                self.labels
                    .get(&predecessor)
                    .ok_or(UiNativeRetainedOrderDenial::MissingPredecessor)
            })
            .transpose()?;
        let right = match left {
            Some(left) => self
                .identities
                .range((Excluded(left), Unbounded))
                .next()
                .map(|(label, _)| label),
            None => self.identities.first_key_value().map(|(label, _)| label),
        };
        let label = label_between(left, right);
        self.labels.insert(identity, label.clone());
        if self.identities.insert(label, identity).is_some() {
            return Err(UiNativeRetainedOrderDenial::DuplicateIdentity);
        }
        Ok(())
    }
}

fn label_between(left: Option<&OrderLabel>, right: Option<&OrderLabel>) -> OrderLabel {
    let mut label = Vec::new();
    let mut index = 0;
    loop {
        let lower = left
            .and_then(|label| label.0.get(index))
            .copied()
            .unwrap_or(0);
        let upper = right
            .and_then(|label| label.0.get(index))
            .copied()
            .unwrap_or(u16::MAX);
        if upper.saturating_sub(lower) > 1 {
            label.push(lower + (upper - lower) / 2);
            return OrderLabel(label.into_boxed_slice());
        }
        label.push(lower);
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{UiNativeRetainedOrder, UiNativeRetainedOrderDenial};

    #[test]
    fn repeated_front_middle_and_tail_edits_preserve_authored_order() {
        let mut order = UiNativeRetainedOrder::initial([1_u64, 2, 3]).unwrap();
        order.place_after(4, None).unwrap();
        order.place_after(3, Some(1)).unwrap();
        order.place_after(5, Some(2)).unwrap();
        order.remove(1).unwrap();
        assert_eq!(order.ordered().collect::<Vec<_>>(), vec![4, 3, 2, 5]);
        assert!(order.label_for(4) < order.label_for(3));
        assert!(order.label_for(3) < order.label_for(2));
    }

    #[test]
    fn invalid_edits_deny_without_identity_ordering() {
        let mut order = UiNativeRetainedOrder::initial([1_u64, 2]).unwrap();
        assert_eq!(
            order.place_after(3, Some(9)),
            Err(UiNativeRetainedOrderDenial::MissingPredecessor)
        );
        assert_eq!(
            order.place_after(1, Some(1)),
            Err(UiNativeRetainedOrderDenial::SelfPredecessor)
        );
        assert_eq!(
            order.remove(9),
            Err(UiNativeRetainedOrderDenial::MissingIdentity)
        );
    }

    #[test]
    fn repeated_insertions_into_one_gap_never_require_global_relabeling() {
        let mut order = UiNativeRetainedOrder::initial([1_u64, 2]).unwrap();
        for identity in 3..2_000 {
            order.place_after(identity, Some(1)).unwrap();
        }
        assert_eq!(order.ordered().count(), 1_999);
        assert!(order.label_for(1) < order.label_for(1_999));
        assert!(order.label_for(1_999) < order.label_for(1_998));
        assert!(order.label_for(3) < order.label_for(2));
    }
}
