use std::collections::{BTreeMap, BTreeSet};

use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryViewIdentity,
    WorthUiSettledSnapshotAdmissionDenial, WorthUiSettledSnapshotAdmissionStop,
    WorthUiSettledSnapshotFact, WorthUiSettledSnapshotProjection,
};

#[derive(Default)]
pub(super) struct WorthUiSettledSnapshotRetention {
    slots: Vec<Option<WorthUiSettledSnapshotProjection>>,
    vacant_slots: Vec<usize>,
    index: BTreeMap<WorthUiQueryViewIdentity, usize>,
    next_order: u64,
}

impl std::fmt::Debug for WorthUiSettledSnapshotRetention {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiSettledSnapshotRetention")
            .field("projection_count", &self.index.len())
            .finish()
    }
}

impl WorthUiSettledSnapshotRetention {
    pub(super) fn admit(
        &mut self,
        mut projection: WorthUiSettledSnapshotProjection,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<std::sync::Arc<WorthUiSettledSnapshotFact>, WorthUiSettledSnapshotAdmissionStop>
    {
        if projection.installed_reference() != reference {
            return Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::ForeignInstalledReference,
                projection,
            ));
        }
        let identity = reference.definition().identity().clone();
        if self.index.contains_key(&identity) {
            return Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::DuplicateSettlement,
                projection,
            ));
        }
        let Some(order) = self.next_order.checked_add(1) else {
            return Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::SourceOrderExhausted,
                projection,
            ));
        };
        projection.attach_source_coordinates(
            crate::WorthUiSettledSnapshotSourceGeneration::new(1),
            crate::WorthUiSettledSnapshotSourceOrder::new(order),
        );
        let fact = projection.shared_fact();
        let slot = self.insert_into_vacant_or_append(projection);
        self.index.insert(identity, slot);
        self.next_order = order;
        Ok(fact)
    }

    pub(super) fn refresh(
        &mut self,
        mut projection: WorthUiSettledSnapshotProjection,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<std::sync::Arc<WorthUiSettledSnapshotFact>, WorthUiSettledSnapshotAdmissionStop>
    {
        if projection.installed_reference() != reference {
            return Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::ForeignInstalledReference,
                projection,
            ));
        }
        let identity = reference.definition().identity();
        let Some(slot) = self.index.get(identity).copied() else {
            return Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::MissingPredecessorSettlement,
                projection,
            ));
        };
        let predecessor = self.slots[slot]
            .as_ref()
            .expect("the derived settlement index points only to occupied slots");
        let predecessor_generation = predecessor
            .fact()
            .source_generation()
            .expect("retained settlements carry source coordinates")
            .as_u64();
        let Some(generation) = predecessor_generation.checked_add(1) else {
            return Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::SourceGenerationExhausted,
                projection,
            ));
        };
        let Some(order) = self.next_order.checked_add(1) else {
            return Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::SourceOrderExhausted,
                projection,
            ));
        };
        projection.attach_source_coordinates(
            crate::WorthUiSettledSnapshotSourceGeneration::new(generation),
            crate::WorthUiSettledSnapshotSourceOrder::new(order),
        );
        let fact = projection.shared_fact();
        self.slots[slot] = Some(projection);
        self.next_order = order;
        Ok(fact)
    }

    pub(super) fn fact_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<&WorthUiSettledSnapshotFact> {
        self.projection_for(reference)
            .map(WorthUiSettledSnapshotProjection::fact)
    }

    pub(super) fn shared_fact_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<std::sync::Arc<WorthUiSettledSnapshotFact>> {
        self.projection_for(reference)
            .map(WorthUiSettledSnapshotProjection::shared_fact)
    }

    pub(super) fn exact_evidence_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<crate::WorthUiExactSettledSnapshotEvidence> {
        self.projection_for(reference)
            .map(WorthUiSettledSnapshotProjection::exact_evidence)
    }

    pub(super) fn take(
        &mut self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<WorthUiSettledSnapshotProjection> {
        let slot = self.index.remove(reference.definition().identity())?;
        let projection = self.slots.get_mut(slot)?.take()?;
        self.vacant_slots.push(slot);
        Some(projection)
    }

    pub(super) fn replace(&mut self, projection: WorthUiSettledSnapshotProjection) {
        self.next_order = self.next_order.max(
            projection
                .fact()
                .source_order()
                .expect("retained settlements carry source coordinates")
                .as_u64(),
        );
        let identity = projection
            .installed_reference()
            .definition()
            .identity()
            .clone();
        if let Some(slot) = self.index.get(&identity).copied() {
            self.slots[slot] = Some(projection);
        } else {
            let slot = self.insert_into_vacant_or_append(projection);
            self.index.insert(identity, slot);
        }
    }

    pub(super) fn retain_only(&mut self, references: &[WorthUiInstalledQueryBindingReference]) {
        let retained = references
            .iter()
            .map(|reference| reference.definition().identity())
            .collect::<BTreeSet<_>>();
        for (identity, slot) in std::mem::take(&mut self.index) {
            if retained.contains(&identity) {
                self.index.insert(identity, slot);
            } else {
                self.slots[slot] = None;
                self.vacant_slots.push(slot);
            }
        }
    }

    pub(super) fn observation_counts(
        &self,
        reference_is_valid: impl Fn(&WorthUiInstalledQueryBindingReference) -> bool,
    ) -> (usize, usize) {
        let mut retained = 0;
        let mut orphaned = 0;
        for projection in self.slots.iter().flatten() {
            retained += 1;
            orphaned += usize::from(!reference_is_valid(projection.installed_reference()));
        }
        (retained, orphaned)
    }

    pub(super) fn swap_with(&mut self, other: &mut Self) {
        std::mem::swap(self, other);
    }

    fn projection_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<&WorthUiSettledSnapshotProjection> {
        let slot = *self.index.get(reference.definition().identity())?;
        self.slots.get(slot)?.as_ref()
    }

    fn insert_into_vacant_or_append(
        &mut self,
        projection: WorthUiSettledSnapshotProjection,
    ) -> usize {
        if let Some(slot) = self.vacant_slots.pop() {
            debug_assert!(self.slots[slot].is_none());
            self.slots[slot] = Some(projection);
            slot
        } else {
            let slot = self.slots.len();
            self.slots.push(Some(projection));
            slot
        }
    }
}

#[cfg(test)]
mod exhaustion_tests {
    use super::WorthUiSettledSnapshotRetention;
    use crate::{
        WorthUiQueryBindingPlan, WorthUiQueryViewShape, WorthUiQueryWorkspaceExt,
        WorthUiSettledSnapshotAdmissionDenial, WorthUiSettledSnapshotSourceGeneration,
        WorthUiSettledSnapshotSourceOrder,
    };

    #[test]
    fn source_order_exhaustion_preserves_the_exact_predecessor_projection() {
        let mut workspace = crate::snapshot_refresh_isolation_tests::installed_workspace();
        let (_plan, reference) = binding(&workspace);
        let mut retention = WorthUiSettledSnapshotRetention::default();
        let predecessor = retention
            .admit(
                crate::snapshot_refresh_isolation_tests::settle(&reference, &mut workspace),
                &reference,
            )
            .unwrap();
        retention.next_order = u64::MAX;

        let stop = retention
            .refresh(
                crate::snapshot_refresh_isolation_tests::settle(&reference, &mut workspace),
                &reference,
            )
            .expect_err("an exhausted owner order cannot publish a refresh");

        assert_eq!(
            stop.denial(),
            WorthUiSettledSnapshotAdmissionDenial::SourceOrderExhausted
        );
        assert_eq!(retention.fact_for(&reference), Some(predecessor.as_ref()));
    }

    #[test]
    fn source_generation_exhaustion_preserves_the_exact_predecessor_projection() {
        let mut workspace = crate::snapshot_refresh_isolation_tests::installed_workspace();
        let (_plan, reference) = binding(&workspace);
        let mut predecessor =
            crate::snapshot_refresh_isolation_tests::settle(&reference, &mut workspace);
        predecessor.attach_source_coordinates(
            WorthUiSettledSnapshotSourceGeneration::new(u64::MAX),
            WorthUiSettledSnapshotSourceOrder::new(1),
        );
        let predecessor_fact = predecessor.fact().clone();
        let mut retention = WorthUiSettledSnapshotRetention::default();
        retention.replace(predecessor);

        let stop = retention
            .refresh(
                crate::snapshot_refresh_isolation_tests::settle(&reference, &mut workspace),
                &reference,
            )
            .expect_err("an exhausted source generation cannot publish a refresh");

        assert_eq!(
            stop.denial(),
            WorthUiSettledSnapshotAdmissionDenial::SourceGenerationExhausted
        );
        assert_eq!(retention.fact_for(&reference), Some(&predecessor_fact));
    }

    fn binding(
        workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    ) -> (
        WorthUiQueryBindingPlan,
        crate::WorthUiInstalledQueryBindingReference,
    ) {
        let view = workspace
            .worth_ui()
            .unwrap()
            .measurement_view("dashboard.exhaustion")
            .unwrap();
        let identity = view.definition().identity().clone();
        let plan = WorthUiQueryBindingPlan::default()
            .register_view(view)
            .unwrap();
        let reference = plan
            .resolve_definition(&identity, WorthUiQueryViewShape::Collection)
            .unwrap();
        (plan, reference)
    }
}
