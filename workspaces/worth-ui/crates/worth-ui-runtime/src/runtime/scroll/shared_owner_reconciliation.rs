use std::collections::BTreeMap;

/// Collects every mounted contribution before reconciling one surface- or
/// viewport-owned scroll record. Shared owners must never inherit whichever
/// mounted sibling happened to be visited last.
pub(crate) struct UiSharedScrollOwnerReconciliation {
    registration: super::UiScrollOwnerRegistration,
    anchors: BTreeMap<worth_ui_host_contract::UiMountedInstanceIdentity, super::UiScrollAnchor>,
}

impl UiSharedScrollOwnerReconciliation {
    pub(crate) fn new(
        registration: super::UiScrollOwnerRegistration,
        mounted: worth_ui_host_contract::UiMountedInstanceIdentity,
        anchor: Option<super::UiScrollAnchor>,
    ) -> Self {
        let mut reconciliation = Self {
            registration,
            anchors: BTreeMap::new(),
        };
        reconciliation.absorb(registration, mounted, anchor);
        reconciliation
    }

    pub(crate) fn absorb(
        &mut self,
        registration: super::UiScrollOwnerRegistration,
        mounted: worth_ui_host_contract::UiMountedInstanceIdentity,
        anchor: Option<super::UiScrollAnchor>,
    ) {
        assert_eq!(self.registration.identity(), registration.identity());
        assert_eq!(
            self.registration.incarnation(),
            registration.incarnation(),
            "one shared Scroll owner has one surface-local incarnation"
        );
        let prior = self.registration.bounds();
        let next = registration.bounds();
        let bounds = super::UiScrollBounds::new(
            prior
                .max_inline_subpixels()
                .max(next.max_inline_subpixels()),
            prior.max_block_subpixels().max(next.max_block_subpixels()),
        )
        .expect("admitted Scroll bounds remain nonnegative");
        self.registration = super::UiScrollOwnerRegistration::new(
            registration.identity(),
            registration.incarnation(),
            axes_for(bounds),
            bounds,
            super::UiScrollOffset::origin(),
        );
        if let Some(anchor) = anchor {
            self.anchors.insert(mounted, anchor);
        }
    }

    pub(crate) fn reconcile(
        self,
        state: &mut super::UiScrollRuntimeState,
    ) -> Result<super::UiScrollAnchorReconciliationReceipt, super::UiScrollRouteDenial> {
        let previous_anchor = state
            .owner_anchor(
                self.registration.identity(),
                self.registration.incarnation(),
            )
            .ok()
            .flatten();
        let retained_anchor = previous_anchor
            .and_then(super::UiScrollAnchor::mounted_identity)
            .and_then(|identity| self.anchors.get(&identity).copied());
        let successor_anchor = retained_anchor.or_else(|| self.anchors.values().next().copied());
        let policy = if previous_anchor
            .zip(successor_anchor)
            .is_some_and(|(prior, next)| prior.same_identity(next))
        {
            super::UiScrollAnchorPolicy::Rebase
        } else {
            super::UiScrollAnchorPolicy::Clamp
        };
        state.reconcile_rebind(super::UiScrollRebindRequest::new(
            self.registration,
            successor_anchor,
            policy,
        ))
    }
}

fn axes_for(bounds: super::UiScrollBounds) -> super::UiScrollAxes {
    match (
        bounds.max_inline_subpixels() > 0,
        bounds.max_block_subpixels() > 0,
    ) {
        (true, false) => super::UiScrollAxes::Inline,
        (false, true) => super::UiScrollAxes::Block,
        (true, true) | (false, false) => super::UiScrollAxes::Both,
    }
}
