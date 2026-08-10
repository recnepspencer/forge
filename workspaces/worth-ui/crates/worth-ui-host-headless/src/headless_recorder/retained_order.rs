use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::{Excluded, Unbounded};

use worth_ui_host_contract::{
    UiMountedPaintOrderEdit, UiMountedPaintOrderIdentity, UiMountedPaintOrderIntegrity,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct OrderLabel(Box<[u16]>);

pub(super) struct UiHeadlessRetainedOrder {
    labels: HashMap<UiMountedPaintOrderIdentity, OrderLabel>,
    identities: BTreeMap<OrderLabel, UiMountedPaintOrderIdentity>,
    integrity: UiMountedPaintOrderIntegrity,
}

pub(super) struct UiHeadlessRetainedOrderSnapshot {
    entries: Vec<OriginalOrderEntry>,
    integrity: UiMountedPaintOrderIntegrity,
}

struct OriginalOrderEntry {
    identity: UiMountedPaintOrderIdentity,
    predecessor: Option<UiMountedPaintOrderIdentity>,
    existed: bool,
    label: Option<OrderLabel>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiHeadlessRetainedOrderDenial {
    DuplicateIdentity,
    MissingIdentity,
    MissingPredecessor,
    SelfPredecessor,
    IntegrityMismatch,
}

impl UiHeadlessRetainedOrder {
    pub(super) fn initial(
        identities: &[UiMountedPaintOrderIdentity],
        integrity: UiMountedPaintOrderIntegrity,
    ) -> Result<Self, UiHeadlessRetainedOrderDenial> {
        if !integrity.admits(identities) {
            return Err(UiHeadlessRetainedOrderDenial::IntegrityMismatch);
        }
        let mut order = Self {
            labels: HashMap::new(),
            identities: BTreeMap::new(),
            integrity,
        };
        let mut predecessor = None;
        for identity in identities {
            order.insert_after(*identity, predecessor)?;
            predecessor = Some(*identity);
        }
        order.integrity = integrity;
        Ok(order)
    }

    pub(super) fn apply(
        &mut self,
        edits: &[UiMountedPaintOrderEdit],
        expected: UiMountedPaintOrderIntegrity,
    ) -> Result<(), UiHeadlessRetainedOrderDenial> {
        for edit in edits {
            if edit.is_removal() {
                self.remove(edit.identity())?;
            } else {
                self.place_after(edit.identity(), edit.predecessor())?;
            }
        }
        if self.integrity != expected {
            return Err(UiHeadlessRetainedOrderDenial::IntegrityMismatch);
        }
        Ok(())
    }

    pub(super) fn ordered(
        &self,
    ) -> impl ExactSizeIterator<Item = UiMountedPaintOrderIdentity> + '_ {
        self.identities.values().copied()
    }

    pub(super) fn contains(&self, identity: UiMountedPaintOrderIdentity) -> bool {
        self.labels.contains_key(&identity)
    }

    pub(super) fn snapshot(
        &self,
        identities: impl IntoIterator<Item = UiMountedPaintOrderIdentity>,
    ) -> UiHeadlessRetainedOrderSnapshot {
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
        UiHeadlessRetainedOrderSnapshot {
            entries,
            integrity: self.integrity,
        }
    }

    pub(super) fn restore(
        &mut self,
        snapshot: UiHeadlessRetainedOrderSnapshot,
    ) -> Result<(), UiHeadlessRetainedOrderDenial> {
        for entry in &snapshot.entries {
            if self.contains(entry.identity) {
                let label = self
                    .labels
                    .remove(&entry.identity)
                    .ok_or(UiHeadlessRetainedOrderDenial::MissingIdentity)?;
                self.identities.remove(&label);
            }
        }
        for entry in snapshot.entries.into_iter().filter(|entry| entry.existed) {
            self.insert_after(entry.identity, entry.predecessor)?;
        }
        self.integrity = snapshot.integrity;
        Ok(())
    }

    fn remove(
        &mut self,
        identity: UiMountedPaintOrderIdentity,
    ) -> Result<(), UiHeadlessRetainedOrderDenial> {
        let (predecessor, successor) = self.neighbors(identity)?;
        self.integrity = self
            .integrity
            .remove_edge(predecessor, identity, successor)
            .ok_or(UiHeadlessRetainedOrderDenial::IntegrityMismatch)?;
        let label = self
            .labels
            .remove(&identity)
            .ok_or(UiHeadlessRetainedOrderDenial::MissingIdentity)?;
        self.identities
            .remove(&label)
            .ok_or(UiHeadlessRetainedOrderDenial::MissingIdentity)?;
        Ok(())
    }

    fn place_after(
        &mut self,
        identity: UiMountedPaintOrderIdentity,
        predecessor: Option<UiMountedPaintOrderIdentity>,
    ) -> Result<(), UiHeadlessRetainedOrderDenial> {
        if predecessor == Some(identity) {
            return Err(UiHeadlessRetainedOrderDenial::SelfPredecessor);
        }
        if self.contains(identity) {
            self.remove(identity)?;
        }
        let successor = self.successor_after(predecessor)?;
        self.integrity = self
            .integrity
            .insert_edge(predecessor, identity, successor)
            .ok_or(UiHeadlessRetainedOrderDenial::IntegrityMismatch)?;
        self.insert_after(identity, predecessor)
    }

    fn neighbors(
        &self,
        identity: UiMountedPaintOrderIdentity,
    ) -> Result<
        (
            Option<UiMountedPaintOrderIdentity>,
            Option<UiMountedPaintOrderIdentity>,
        ),
        UiHeadlessRetainedOrderDenial,
    > {
        let label = self
            .labels
            .get(&identity)
            .ok_or(UiHeadlessRetainedOrderDenial::MissingIdentity)?;
        Ok((
            self.identities
                .range((Unbounded, Excluded(label)))
                .next_back()
                .map(|(_, identity)| *identity),
            self.identities
                .range((Excluded(label), Unbounded))
                .next()
                .map(|(_, identity)| *identity),
        ))
    }

    fn successor_after(
        &self,
        predecessor: Option<UiMountedPaintOrderIdentity>,
    ) -> Result<Option<UiMountedPaintOrderIdentity>, UiHeadlessRetainedOrderDenial> {
        match predecessor {
            Some(predecessor) => {
                let label = self
                    .labels
                    .get(&predecessor)
                    .ok_or(UiHeadlessRetainedOrderDenial::MissingPredecessor)?;
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

    fn insert_after(
        &mut self,
        identity: UiMountedPaintOrderIdentity,
        predecessor: Option<UiMountedPaintOrderIdentity>,
    ) -> Result<(), UiHeadlessRetainedOrderDenial> {
        if self.contains(identity) {
            return Err(UiHeadlessRetainedOrderDenial::DuplicateIdentity);
        }
        let left = predecessor
            .map(|predecessor| {
                self.labels
                    .get(&predecessor)
                    .ok_or(UiHeadlessRetainedOrderDenial::MissingPredecessor)
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
            return Err(UiHeadlessRetainedOrderDenial::DuplicateIdentity);
        }
        Ok(())
    }
}

fn label_between(left: Option<&OrderLabel>, right: Option<&OrderLabel>) -> OrderLabel {
    let mut label = Vec::new();
    let mut index = 0;
    loop {
        let lower = left
            .and_then(|value| value.0.get(index))
            .copied()
            .unwrap_or(0);
        let upper = right
            .and_then(|value| value.0.get(index))
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
