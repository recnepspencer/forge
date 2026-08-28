#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCommandInvocationOrigin {
    KeyboardShortcut,
    CommandProjection,
    NativeMenu,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCommandRouteLossReason {
    LowerScopePrecedence,
    LowerDeclaredPriority,
    LowerSpecificity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCommandRouteLoss {
    command: crate::capability::CommandId,
    reason: UiCommandRouteLossReason,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiCommandRouteReceipt {
    command: crate::capability::CommandId,
    destination: crate::capability::UiCommandRouteDestination,
    application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    origin: UiCommandInvocationOrigin,
    scope: crate::capability::UiCommandRouteScope,
    scope_identity: Option<crate::capability::UiCommandRouteScopeIdentity>,
    presentation: Option<worth_ui_host_contract::UiHostObservationPresentationBasis>,
    sequence: Option<worth_ui_host_contract::UiHostObservationSequence>,
    time_basis: Option<worth_ui_host_contract::UiHostObservationTimeBasis>,
    focused_target: Option<worth_ui_host_contract::UiHostFocusPlacementTarget>,
    focused_participant: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
    focused_scope: Option<crate::capability::UiCommandRouteScopeIdentity>,
    invocation_target: Option<worth_ui_host_contract::UiHostFocusPlacementTarget>,
    focus_revision: Option<u64>,
    selection_revision: Option<u64>,
    selection_owner: Option<crate::graph::UiGraphNodeIdentity>,
    selected_count: Option<usize>,
    portal_revision: Option<u64>,
    portal_scopes: Box<[crate::capability::UiCommandRouteScopeIdentity]>,
    losers: Box<[UiCommandRouteLoss]>,
}

#[derive(Clone)]
pub(crate) struct UiCommandRouteEvidence {
    destination: crate::capability::UiCommandRouteDestination,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCommandPrefixReceipt {
    first: crate::capability::UiCommandShortcutStroke,
    candidates: usize,
    occupancy_revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCommandAmbiguity {
    commands: Box<[crate::capability::CommandId]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCommandRoutingSuppression {
    DeclarationNotReady,
    RepeatSuppressed,
    ImeComposition,
    TextEntry,
    PrefixConflict,
    PrefixContextChanged,
    PrefixExpired,
    PrefixBasisUnavailable,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UiCommandRoutingOutcome {
    Routed(UiCommandRouteReceipt),
    AwaitingPrefix(UiCommandPrefixReceipt),
    Ambiguous(UiCommandAmbiguity),
    Suppressed(UiCommandRoutingSuppression),
    Unmatched,
}

impl UiCommandRouteLoss {
    pub(super) fn new(
        command: crate::capability::CommandId,
        reason: UiCommandRouteLossReason,
    ) -> Self {
        Self { command, reason }
    }

    pub fn command(&self) -> &crate::capability::CommandId {
        &self.command
    }

    pub const fn reason(&self) -> UiCommandRouteLossReason {
        self.reason
    }
}

impl UiCommandRouteReceipt {
    pub(super) fn new(
        candidate: &super::candidate::UiCommandRouteCandidate,
        application: &crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
        context: &super::UiCommandRoutingContext,
        origin: UiCommandInvocationOrigin,
        losers: Box<[UiCommandRouteLoss]>,
    ) -> Self {
        let consumption = candidate.route().context();
        let focused_scope_route =
            candidate.route().scope() == crate::capability::UiCommandRouteScope::FocusedControl;
        Self {
            command: candidate.command().clone(),
            destination: candidate.route().destination(),
            application: application.clone(),
            surface: context.surface(),
            origin,
            scope: candidate.route().scope(),
            scope_identity: candidate.route().scope_identity(),
            presentation: context.presentation(),
            sequence: context.sequence(),
            time_basis: context.time_basis(),
            focused_target: (consumption.consumes_focus() || focused_scope_route)
                .then(|| context.focused_target())
                .flatten(),
            focused_participant: (consumption.consumes_focus() || focused_scope_route)
                .then(|| context.focused_participant())
                .flatten(),
            focused_scope: (consumption.consumes_focus() || focused_scope_route)
                .then(|| context.focused_scope())
                .flatten(),
            invocation_target: context.focused_target(),
            focus_revision: (consumption.consumes_focus() || focused_scope_route)
                .then(|| context.focus_revision()),
            selection_revision: consumption
                .consumes_selection()
                .then(|| context.selection_revision()),
            selection_owner: consumption
                .consumes_selection()
                .then(|| context.selection_owner())
                .flatten(),
            selected_count: consumption
                .consumes_selection()
                .then(|| context.selected_count()),
            portal_revision: consumption
                .consumes_portal_chain()
                .then(|| context.portal_revision())
                .or_else(|| {
                    (candidate.route().scope()
                        == crate::capability::UiCommandRouteScope::ActivePortal)
                        .then(|| context.portal_revision())
                }),
            portal_scopes: if consumption.consumes_portal_chain()
                || candidate.route().scope() == crate::capability::UiCommandRouteScope::ActivePortal
            {
                context.active_portal_scopes().into()
            } else {
                Box::new([])
            },
            losers,
        }
    }

    pub fn command(&self) -> &crate::capability::CommandId {
        &self.command
    }

    pub const fn destination(&self) -> crate::capability::UiCommandRouteDestination {
        self.destination
    }

    pub const fn application(
        &self,
    ) -> &crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity {
        &self.application
    }

    pub const fn origin(&self) -> UiCommandInvocationOrigin {
        self.origin
    }

    pub fn losers(&self) -> &[UiCommandRouteLoss] {
        &self.losers
    }

    pub const fn focus_revision(&self) -> Option<u64> {
        self.focus_revision
    }

    pub const fn selection_revision(&self) -> Option<u64> {
        self.selection_revision
    }

    pub const fn portal_revision(&self) -> Option<u64> {
        self.portal_revision
    }

    pub(crate) const fn surface(&self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub(crate) fn evidence(&self) -> UiCommandRouteEvidence {
        UiCommandRouteEvidence {
            destination: self.destination,
        }
    }

    pub(crate) fn consumed_context_is_current(
        &self,
        current: &super::UiCommandRoutingContext,
    ) -> bool {
        if self.surface != current.surface() {
            return false;
        }
        let focus_current = self.focus_revision.is_none()
            || (self.focus_revision == Some(current.focus_revision())
                && self.focused_participant == current.focused_participant()
                && self.focused_scope == current.focused_scope());
        let selection_current = self.selection_revision.is_none()
            || (self.selection_revision == Some(current.selection_revision())
                && self.selection_owner == current.selection_owner()
                && self.selected_count == Some(current.selected_count()));
        let portal_current = self.portal_revision.is_none()
            || (self.portal_revision == Some(current.portal_revision())
                && self.portal_scopes.as_ref() == current.active_portal_scopes());
        let scope_current = match self.scope {
            crate::capability::UiCommandRouteScope::Application
            | crate::capability::UiCommandRouteScope::Surface => true,
            crate::capability::UiCommandRouteScope::ActiveRegion => false,
            crate::capability::UiCommandRouteScope::FocusedControl => {
                self.scope_identity == current.focused_scope()
            }
            crate::capability::UiCommandRouteScope::ActivePortal => self
                .scope_identity
                .is_some_and(|identity| current.active_portal_scopes().contains(&identity)),
        };
        focus_current && selection_current && portal_current && scope_current
    }

    pub(crate) const fn presentation(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostObservationPresentationBasis> {
        self.presentation
    }

    pub(crate) const fn sequence(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostObservationSequence> {
        self.sequence
    }

    pub(crate) const fn time_basis(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostObservationTimeBasis> {
        self.time_basis
    }

    pub(crate) const fn focused_target(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostFocusPlacementTarget> {
        self.focused_target
    }

    pub(crate) const fn invocation_target(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostFocusPlacementTarget> {
        self.invocation_target
    }
}

impl UiCommandRouteEvidence {
    pub(crate) const fn destination(&self) -> crate::capability::UiCommandRouteDestination {
        self.destination
    }
}

impl UiCommandPrefixReceipt {
    pub(super) const fn new(
        first: crate::capability::UiCommandShortcutStroke,
        candidates: usize,
        occupancy_revision: u64,
    ) -> Self {
        Self {
            first,
            candidates,
            occupancy_revision,
        }
    }

    pub const fn first_stroke(&self) -> crate::capability::UiCommandShortcutStroke {
        self.first
    }

    pub const fn candidate_count(&self) -> usize {
        self.candidates
    }

    pub const fn occupancy_revision(&self) -> u64 {
        self.occupancy_revision
    }
}

impl UiCommandAmbiguity {
    pub(super) fn new(commands: Vec<crate::capability::CommandId>) -> Self {
        Self {
            commands: commands.into_boxed_slice(),
        }
    }

    pub fn commands(&self) -> &[crate::capability::CommandId] {
        &self.commands
    }
}
