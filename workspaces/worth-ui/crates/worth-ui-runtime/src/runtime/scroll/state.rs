use std::collections::BTreeMap;

mod anchor_access;
#[cfg(any(test, feature = "certification-support"))]
mod certification;
mod ownership_catalog;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UiScrollOwnerRecord {
    incarnation: super::UiScrollOwnerIncarnation,
    axes: super::UiScrollAxes,
    bounds: super::UiScrollBounds,
    offset: super::UiScrollOffset,
    anchor: Option<super::UiScrollAnchor>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UiScrollOwnershipCatalogRecord {
    incarnation: super::UiScrollOwnerIncarnation,
    resolution:
        Result<super::UiResolvedScrollOwnershipChain, super::UiScrollOwnershipResolutionDenial>,
}

/// Sole owner of semantic scroll offsets for one active application session.
/// Query- or host-provided extents establish bounds; only this state changes offsets.
#[derive(Clone)]
pub(crate) struct UiScrollRuntimeState {
    persistence: crate::runtime::UiServiceStatePersistencePosture,
    owners: BTreeMap<super::UiScrollOwnerIdentity, UiScrollOwnerRecord>,
    ownership_catalog:
        BTreeMap<worth_ui_host_contract::UiMountedInstanceIdentity, UiScrollOwnershipCatalogRecord>,
    ownership_references: BTreeMap<super::UiScrollOwnerIdentity, u64>,
    counters: super::UiScrollCounters,
    ownership_resolutions: u64,
    ownership_graph_nodes_visited: u64,
    ownership_plan_nodes_visited: u64,
    revision: u64,
}

impl UiScrollRuntimeState {
    pub(crate) const fn new_session_restore_candidate() -> Self {
        Self::new(crate::runtime::UiServiceStatePersistencePosture::SessionRestoreCandidate)
    }

    pub(in crate::runtime) const fn new(
        persistence: crate::runtime::UiServiceStatePersistencePosture,
    ) -> Self {
        Self {
            persistence,
            owners: BTreeMap::new(),
            ownership_catalog: BTreeMap::new(),
            ownership_references: BTreeMap::new(),
            counters: super::UiScrollCounters::new(),
            ownership_resolutions: 0,
            ownership_graph_nodes_visited: 0,
            ownership_plan_nodes_visited: 0,
            revision: 0,
        }
    }

    pub(in crate::runtime) const fn persistence(
        &self,
    ) -> crate::runtime::UiServiceStatePersistencePosture {
        self.persistence
    }

    pub(in crate::runtime) fn register(
        &mut self,
        registration: super::UiScrollOwnerRegistration,
    ) -> Result<(), super::UiScrollRouteDenial> {
        if !registration
            .bounds()
            .contains(registration.initial_offset())
        {
            return Err(super::UiScrollRouteDenial::InitialOffsetOutOfBounds);
        }
        self.owners.insert(
            registration.identity(),
            UiScrollOwnerRecord {
                incarnation: registration.incarnation(),
                axes: registration.axes(),
                bounds: registration.bounds(),
                offset: registration.initial_offset(),
                anchor: None,
            },
        );
        Ok(())
    }

    pub(crate) fn synchronize(
        &mut self,
        registration: super::UiScrollOwnerRegistration,
    ) -> Result<super::UiScrollOffset, super::UiScrollRouteDenial> {
        if !registration
            .bounds()
            .contains(registration.initial_offset())
        {
            return Err(super::UiScrollRouteDenial::InitialOffsetOutOfBounds);
        }
        if let Some(record) = self.owners.get_mut(&registration.identity()) {
            if record.incarnation == registration.incarnation() {
                record.axes = registration.axes();
                record.bounds = registration.bounds();
                record.offset = record.bounds.clamp(record.offset);
                return Ok(record.offset);
            }
        }
        self.owners.insert(
            registration.identity(),
            UiScrollOwnerRecord {
                incarnation: registration.incarnation(),
                axes: registration.axes(),
                bounds: registration.bounds(),
                offset: registration.initial_offset(),
                anchor: None,
            },
        );
        Ok(registration.initial_offset())
    }

    pub(in crate::runtime) fn reconcile_bounds(
        &mut self,
        owner: super::UiScrollOwnerIdentity,
        incarnation: super::UiScrollOwnerIncarnation,
        bounds: super::UiScrollBounds,
    ) -> Result<super::UiScrollOffset, super::UiScrollRouteDenial> {
        let record = self.exact_owner_mut(owner, incarnation)?;
        record.bounds = bounds;
        record.offset = bounds.clamp(record.offset);
        Ok(record.offset)
    }

    pub(crate) fn reconcile_rebind(
        &mut self,
        request: super::UiScrollRebindRequest,
    ) -> Result<super::UiScrollAnchorReconciliationReceipt, super::UiScrollRouteDenial> {
        let registration = request.registration();
        if !registration
            .bounds()
            .contains(registration.initial_offset())
        {
            return Err(super::UiScrollRouteDenial::InitialOffsetOutOfBounds);
        }
        let previous = self.owners.get(&registration.identity()).copied();
        let successor_anchor = request.successor_anchor();
        let (outcome, offset, anchor) =
            reconcile_owner_record(previous, registration, successor_anchor, request.policy());
        self.owners.insert(
            registration.identity(),
            UiScrollOwnerRecord {
                incarnation: registration.incarnation(),
                axes: registration.axes(),
                bounds: registration.bounds(),
                offset,
                anchor,
            },
        );
        Ok(super::UiScrollAnchorReconciliationReceipt::new(
            outcome, offset,
        ))
    }

    pub(in crate::runtime) fn offset(
        &self,
        owner: super::UiScrollOwnerIdentity,
        incarnation: super::UiScrollOwnerIncarnation,
    ) -> Result<super::UiScrollOffset, super::UiScrollRouteDenial> {
        Ok(self.exact_owner(owner, incarnation)?.offset)
    }

    pub(super) fn owner_geometry(
        &self,
        owner: super::UiScrollOwnerIdentity,
        incarnation: super::UiScrollOwnerIncarnation,
    ) -> Result<
        (
            super::UiScrollOffset,
            super::UiScrollBounds,
            super::UiScrollAxes,
        ),
        super::UiScrollRouteDenial,
    > {
        let record = self.exact_owner(owner, incarnation)?;
        Ok((record.offset, record.bounds, record.axes))
    }

    pub(crate) fn route(
        &mut self,
        request: super::UiScrollDeltaRequest,
    ) -> Result<super::UiScrollRouteReceipt, super::UiScrollRouteDenial> {
        let result = self.route_prevalidated(request);
        if result.is_err() {
            self.counters.reject();
        }
        result
    }

    fn route_prevalidated(
        &mut self,
        request: super::UiScrollDeltaRequest,
    ) -> Result<super::UiScrollRouteReceipt, super::UiScrollRouteDenial> {
        let surface = request.chain()[0].owner().semantic_surface();
        for entry in request.chain() {
            if entry.owner().semantic_surface() != surface {
                return Err(super::UiScrollRouteDenial::CrossSurfaceChain);
            }
            self.exact_owner(entry.owner(), entry.incarnation())?;
        }
        let next_revision = self
            .revision
            .checked_add(1)
            .ok_or(super::UiScrollRouteDenial::RevisionExhausted)?;
        let mut remainder = request.delta();
        let mut transitions = Vec::with_capacity(request.chain().len());
        for entry in request.chain() {
            let record = self.exact_owner(entry.owner(), entry.incarnation())?;
            let previous = record.offset;
            let (current, consumed) =
                super::routing::consume_delta(previous, record.bounds, record.axes, remainder);
            remainder = remainder.subtract(consumed);
            transitions.push(super::UiScrollChainTransition::new(
                entry.owner(),
                previous,
                current,
                consumed,
            ));
            if remainder.is_zero() {
                break;
            }
        }
        let owners_visited = u16::try_from(transitions.len())
            .expect("scroll chain depth limit is representable by u16");
        let changed = transitions
            .iter()
            .filter(|transition| transition.previous() != transition.current())
            .count();
        let next_counters = self.counters.after_admission(owners_visited, changed)?;
        for (entry, transition) in request.chain().iter().zip(&transitions) {
            self.exact_owner_mut(entry.owner(), entry.incarnation())?
                .offset = transition.current();
        }
        self.counters = next_counters;
        self.revision = next_revision;
        Ok(super::UiScrollRouteReceipt::new(
            request.cause(),
            transitions,
            remainder,
            owners_visited,
            self.revision,
        ))
    }

    pub(in crate::runtime) const fn counters(&self) -> super::UiScrollCounters {
        self.counters
    }

    pub(crate) fn shutdown(&mut self) -> usize {
        let released = self.owners.len();
        self.owners.clear();
        self.ownership_catalog.clear();
        self.ownership_references.clear();
        released
    }

    fn exact_owner(
        &self,
        owner: super::UiScrollOwnerIdentity,
        incarnation: super::UiScrollOwnerIncarnation,
    ) -> Result<&UiScrollOwnerRecord, super::UiScrollRouteDenial> {
        let record = self
            .owners
            .get(&owner)
            .ok_or(super::UiScrollRouteDenial::UnknownOwner)?;
        if record.incarnation != incarnation {
            return Err(super::UiScrollRouteDenial::StaleOwnerIncarnation);
        }
        Ok(record)
    }

    fn exact_owner_mut(
        &mut self,
        owner: super::UiScrollOwnerIdentity,
        incarnation: super::UiScrollOwnerIncarnation,
    ) -> Result<&mut UiScrollOwnerRecord, super::UiScrollRouteDenial> {
        let record = self
            .owners
            .get_mut(&owner)
            .ok_or(super::UiScrollRouteDenial::UnknownOwner)?;
        if record.incarnation != incarnation {
            return Err(super::UiScrollRouteDenial::StaleOwnerIncarnation);
        }
        Ok(record)
    }
}

fn reconcile_owner_record(
    previous: Option<UiScrollOwnerRecord>,
    registration: super::UiScrollOwnerRegistration,
    successor_anchor: Option<super::UiScrollAnchor>,
    policy: super::UiScrollAnchorPolicy,
) -> (
    super::UiScrollAnchorReconciliationOutcome,
    super::UiScrollOffset,
    Option<super::UiScrollAnchor>,
) {
    let Some(previous) = previous else {
        return (
            super::UiScrollAnchorReconciliationOutcome::Replaced,
            registration.initial_offset(),
            successor_anchor,
        );
    };
    match policy {
        super::UiScrollAnchorPolicy::Preserve
            if previous.incarnation == registration.incarnation()
                && previous
                    .anchor
                    .zip(successor_anchor)
                    .is_some_and(|(old, new)| old.exact_basis(new)) =>
        {
            let offset = registration.bounds().clamp(previous.offset);
            let outcome = if offset == previous.offset {
                super::UiScrollAnchorReconciliationOutcome::Preserved
            } else {
                super::UiScrollAnchorReconciliationOutcome::Clamped
            };
            (outcome, offset, successor_anchor)
        }
        super::UiScrollAnchorPolicy::Rebase => match previous.anchor.zip(successor_anchor) {
            Some((old, new)) if old.same_identity(new) => {
                let offset = registration
                    .bounds()
                    .clamp(rebased_offset(previous.offset, old, new));
                (
                    super::UiScrollAnchorReconciliationOutcome::Rebased,
                    offset,
                    successor_anchor,
                )
            }
            _ => dropped(registration),
        },
        super::UiScrollAnchorPolicy::Clamp => (
            super::UiScrollAnchorReconciliationOutcome::Clamped,
            registration.bounds().clamp(previous.offset),
            successor_anchor.or(previous.anchor),
        ),
        super::UiScrollAnchorPolicy::Replace => (
            super::UiScrollAnchorReconciliationOutcome::Replaced,
            registration.initial_offset(),
            successor_anchor,
        ),
        super::UiScrollAnchorPolicy::Preserve | super::UiScrollAnchorPolicy::Drop => {
            dropped(registration)
        }
    }
}

fn dropped(
    registration: super::UiScrollOwnerRegistration,
) -> (
    super::UiScrollAnchorReconciliationOutcome,
    super::UiScrollOffset,
    Option<super::UiScrollAnchor>,
) {
    (
        super::UiScrollAnchorReconciliationOutcome::Dropped,
        registration.initial_offset(),
        None,
    )
}

fn rebased_offset(
    offset: super::UiScrollOffset,
    old: super::UiScrollAnchor,
    new: super::UiScrollAnchor,
) -> super::UiScrollOffset {
    let inline = i128::from(offset.inline_subpixels()) + i128::from(new.inline_subpixels())
        - i128::from(old.inline_subpixels());
    let block = i128::from(offset.block_subpixels()) + i128::from(new.block_subpixels())
        - i128::from(old.block_subpixels());
    super::UiScrollOffset::new(clamp_nonnegative(inline), clamp_nonnegative(block)).unwrap()
}

fn clamp_nonnegative(value: i128) -> i64 {
    value.clamp(0, i128::from(i64::MAX)) as i64
}
