use std::collections::{BTreeMap, BTreeSet};

use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryViewIdentity,
    WorthUiSettledSnapshotAdmissionDenial, WorthUiSettledSnapshotAdmissionStop,
    WorthUiSettledSnapshotFact, WorthUiSettledSnapshotProjection,
};

#[derive(Default)]
pub(super) struct WorthUiSettledSnapshotRetention {
    slots: Vec<Option<WorthUiSettledSnapshotProjection>>,
    index: BTreeMap<WorthUiQueryViewIdentity, usize>,
    next_order: u64,
}

impl std::fmt::Debug for WorthUiSettledSnapshotRetention {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiSettledSnapshotRetention")
            .field("projection_count", &self.slots.iter().flatten().count())
            .finish()
    }
}

impl WorthUiSettledSnapshotRetention {
    pub(super) fn for_identities(
        identities: impl IntoIterator<Item = WorthUiQueryViewIdentity>,
    ) -> Self {
        let identities = identities.into_iter();
        let (minimum, _) = identities.size_hint();
        let mut slots = Vec::with_capacity(minimum);
        let mut index = BTreeMap::new();
        for identity in identities {
            let slot = slots.len();
            slots.push(None);
            index.insert(identity, slot);
        }
        Self {
            slots,
            index,
            next_order: 0,
        }
    }

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
        let identity = reference.definition().identity();
        let Some(slot) = self.index.get(identity).copied() else {
            return Err(WorthUiSettledSnapshotAdmissionStop::new(
                WorthUiSettledSnapshotAdmissionDenial::ForeignInstalledReference,
                projection,
            ));
        };
        if self.slots[slot].is_some() {
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
        self.slots[slot] = Some(projection);
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
        let slot = *self.index.get(reference.definition().identity())?;
        self.slots.get_mut(slot)?.take()
    }

    pub(super) fn replace(&mut self, projection: WorthUiSettledSnapshotProjection) {
        self.next_order = self.next_order.max(
            projection
                .fact()
                .source_order()
                .expect("retained settlements carry source coordinates")
                .as_u64(),
        );
        let identity = projection.installed_reference().definition().identity();
        let slot = *self
            .index
            .get(identity)
            .expect("installed settlement references have reserved slots");
        self.slots[slot] = Some(projection);
    }

    pub(super) fn retain_only(&mut self, retained: &BTreeSet<WorthUiQueryViewIdentity>) {
        for (identity, slot) in &self.index {
            if !retained.contains(identity) {
                self.slots[*slot] = None;
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

    fn projection_for(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Option<&WorthUiSettledSnapshotProjection> {
        let slot = *self.index.get(reference.definition().identity())?;
        self.slots.get(slot)?.as_ref()
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
        let mut retention = WorthUiSettledSnapshotRetention::for_identities([reference
            .definition()
            .identity()
            .clone()]);
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
        let mut retention = WorthUiSettledSnapshotRetention::for_identities([reference
            .definition()
            .identity()
            .clone()]);
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
