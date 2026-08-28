#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum UiCommandRoutingContextKey {
    Application,
    Surface,
    ActiveRegion,
    FocusedControl(crate::capability::UiCommandRouteScopeIdentity),
    ActivePortal(crate::capability::UiCommandRouteScopeIdentity),
}

/// Coherent context snapshot supplied to the command-routing owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiCommandRoutingContext {
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    focused_participant: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
    focused_target: Option<worth_ui_host_contract::UiHostFocusPlacementTarget>,
    focused_scope: Option<crate::capability::UiCommandRouteScopeIdentity>,
    focus_revision: u64,
    selection_owner: Option<crate::graph::UiGraphNodeIdentity>,
    selected_count: usize,
    selection_available: bool,
    selection_revision: u64,
    active_portal_scopes: Box<[crate::capability::UiCommandRouteScopeIdentity]>,
    portal_revision: u64,
    declaration_ready: bool,
    ime_composing: bool,
    text_entry_active: bool,
    presentation: Option<worth_ui_host_contract::UiHostObservationPresentationBasis>,
    sequence: Option<worth_ui_host_contract::UiHostObservationSequence>,
    time_basis: Option<worth_ui_host_contract::UiHostObservationTimeBasis>,
}

impl UiCommandRoutingContext {
    pub(crate) fn new(surface: worth_ui_host_contract::UiSemanticSurfaceIdentity) -> Self {
        Self {
            surface,
            focused_participant: None,
            focused_target: None,
            focused_scope: None,
            focus_revision: 0,
            selection_owner: None,
            selected_count: 0,
            selection_available: false,
            selection_revision: 0,
            active_portal_scopes: Box::new([]),
            portal_revision: 0,
            declaration_ready: true,
            ime_composing: false,
            text_entry_active: false,
            presentation: None,
            sequence: None,
            time_basis: None,
        }
    }

    pub(crate) fn with_host_observation(
        mut self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        sequence: worth_ui_host_contract::UiHostObservationSequence,
        time_basis: worth_ui_host_contract::UiHostObservationTimeBasis,
    ) -> Self {
        self.presentation = Some(presentation);
        self.sequence = Some(sequence);
        self.time_basis = Some(time_basis);
        self
    }

    pub(crate) fn with_focus(
        mut self,
        participant: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
        target: Option<worth_ui_host_contract::UiHostFocusPlacementTarget>,
        scope: Option<crate::capability::UiCommandRouteScopeIdentity>,
        revision: u64,
    ) -> Self {
        self.focused_participant = participant;
        self.focused_target = target;
        self.focused_scope = scope;
        self.focus_revision = revision;
        self
    }

    pub(crate) fn with_selection(
        mut self,
        owner: crate::graph::UiGraphNodeIdentity,
        selected_count: usize,
        revision: u64,
    ) -> Self {
        self.selection_owner = Some(owner);
        self.selected_count = selected_count;
        self.selection_available = true;
        self.selection_revision = revision;
        self
    }

    pub(crate) fn with_portals(
        mut self,
        active_scopes: Box<[crate::capability::UiCommandRouteScopeIdentity]>,
        revision: u64,
    ) -> Self {
        self.active_portal_scopes = active_scopes;
        self.portal_revision = revision;
        self
    }

    pub(crate) fn with_declaration_readiness(mut self, ready: bool) -> Self {
        self.declaration_ready = ready;
        self
    }

    pub(crate) fn with_text_input(mut self, ime_composing: bool, text_entry_active: bool) -> Self {
        self.ime_composing = ime_composing;
        self.text_entry_active = text_entry_active;
        self
    }

    pub(crate) const fn surface(&self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub(super) fn scope_is_active(
        &self,
        route: crate::capability::UiCommandRouteDeclaration,
    ) -> bool {
        match route.scope() {
            crate::capability::UiCommandRouteScope::Application
            | crate::capability::UiCommandRouteScope::Surface => true,
            crate::capability::UiCommandRouteScope::ActiveRegion => false,
            crate::capability::UiCommandRouteScope::FocusedControl => route
                .scope_identity()
                .is_some_and(|identity| self.focused_scope == Some(identity)),
            crate::capability::UiCommandRouteScope::ActivePortal => route
                .scope_identity()
                .is_some_and(|identity| self.active_portal_scopes.contains(&identity)),
        }
    }

    pub(super) fn active_keys(&self) -> Vec<UiCommandRoutingContextKey> {
        let mut keys = vec![
            UiCommandRoutingContextKey::Application,
            UiCommandRoutingContextKey::Surface,
        ];
        if let Some(identity) = self.focused_scope {
            keys.push(UiCommandRoutingContextKey::FocusedControl(identity));
        }
        keys.extend(
            self.active_portal_scopes
                .iter()
                .copied()
                .map(UiCommandRoutingContextKey::ActivePortal),
        );
        keys
    }

    pub(super) fn supports_consumption(
        &self,
        consumption: crate::capability::UiCommandContextConsumption,
    ) -> bool {
        (!consumption.consumes_focus() || self.focused_participant.is_some())
            && (!consumption.consumes_selection() || self.selection_available)
    }

    pub(super) fn same_prefix_affinity(&self, other: &Self) -> bool {
        self.surface == other.surface
            && self.focused_participant == other.focused_participant
            && self.focused_scope == other.focused_scope
            && self.focus_revision == other.focus_revision
            && self.selection_owner == other.selection_owner
            && self.selected_count == other.selected_count
            && self.selection_available == other.selection_available
            && self.selection_revision == other.selection_revision
            && self.active_portal_scopes == other.active_portal_scopes
            && self.portal_revision == other.portal_revision
            && self.declaration_ready == other.declaration_ready
            && self.ime_composing == other.ime_composing
            && self.text_entry_active == other.text_entry_active
    }

    pub(super) const fn focused_participant(
        &self,
    ) -> Option<worth_ui_host_contract::UiMountedInstanceIdentity> {
        self.focused_participant
    }

    pub(super) const fn focused_target(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostFocusPlacementTarget> {
        self.focused_target
    }

    pub(super) const fn focused_scope(
        &self,
    ) -> Option<crate::capability::UiCommandRouteScopeIdentity> {
        self.focused_scope
    }

    pub(super) const fn presentation(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostObservationPresentationBasis> {
        self.presentation
    }

    pub(super) const fn sequence(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostObservationSequence> {
        self.sequence
    }

    pub(super) const fn time_basis(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostObservationTimeBasis> {
        self.time_basis
    }

    pub(super) const fn focus_revision(&self) -> u64 {
        self.focus_revision
    }

    pub(super) const fn selection_owner(&self) -> Option<crate::graph::UiGraphNodeIdentity> {
        self.selection_owner
    }

    pub(super) const fn selected_count(&self) -> usize {
        self.selected_count
    }

    pub(super) const fn selection_revision(&self) -> u64 {
        self.selection_revision
    }

    pub(super) fn active_portal_scopes(&self) -> &[crate::capability::UiCommandRouteScopeIdentity] {
        &self.active_portal_scopes
    }

    pub(super) const fn portal_revision(&self) -> u64 {
        self.portal_revision
    }

    pub(super) const fn declaration_ready(&self) -> bool {
        self.declaration_ready
    }

    pub(super) const fn ime_composing(&self) -> bool {
        self.ime_composing
    }

    pub(super) const fn text_entry_active(&self) -> bool {
        self.text_entry_active
    }

    #[cfg(test)]
    pub(super) fn with_time_basis_for_test(mut self, millis: u64) -> Self {
        self.time_basis =
            Some(worth_ui_host_contract::UiHostObservationTimeBasis::HostMonotonicMillis(millis));
        self
    }
}

impl UiCommandRoutingContextKey {
    pub(super) fn for_route(route: crate::capability::UiCommandRouteDeclaration) -> Option<Self> {
        Some(match route.scope() {
            crate::capability::UiCommandRouteScope::Application => Self::Application,
            crate::capability::UiCommandRouteScope::Surface => Self::Surface,
            crate::capability::UiCommandRouteScope::ActiveRegion => Self::ActiveRegion,
            crate::capability::UiCommandRouteScope::FocusedControl => {
                Self::FocusedControl(route.scope_identity()?)
            }
            crate::capability::UiCommandRouteScope::ActivePortal => {
                Self::ActivePortal(route.scope_identity()?)
            }
        })
    }
}
