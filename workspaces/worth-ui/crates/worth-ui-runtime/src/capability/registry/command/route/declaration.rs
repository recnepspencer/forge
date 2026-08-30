use super::{
    UiCommandContextConsumption, UiCommandRegistrationOwner, UiCommandRepeatPolicy,
    UiCommandRouteDestination, UiCommandRoutePriority, UiCommandRouteScope,
    UiCommandRouteScopeIdentity, UiCommandTextInputPolicy,
};

/// Stable route meaning declared by one command capability.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandRouteDeclaration {
    destination: UiCommandRouteDestination,
    scope: UiCommandRouteScope,
    scope_identity: Option<UiCommandRouteScopeIdentity>,
    context: UiCommandContextConsumption,
    priority: UiCommandRoutePriority,
    repeat: UiCommandRepeatPolicy,
    text_input: UiCommandTextInputPolicy,
    registration_owner: Option<UiCommandRegistrationOwner>,
}

impl UiCommandRouteDeclaration {
    pub const fn new(destination: UiCommandRouteDestination) -> Self {
        Self {
            destination,
            scope: UiCommandRouteScope::Application,
            scope_identity: None,
            context: UiCommandContextConsumption::none(),
            priority: UiCommandRoutePriority::normal(),
            repeat: UiCommandRepeatPolicy::Suppress,
            text_input: UiCommandTextInputPolicy::SuppressDuringCompositionAndTextInput,
            registration_owner: None,
        }
    }

    pub const fn with_scope(mut self, scope: UiCommandRouteScope) -> Self {
        self.scope = scope;
        self.scope_identity = None;
        self
    }

    pub const fn for_focused_control(mut self, identity: UiCommandRouteScopeIdentity) -> Self {
        self.scope = UiCommandRouteScope::FocusedControl;
        self.scope_identity = Some(identity);
        self
    }

    pub const fn for_active_portal(mut self, identity: UiCommandRouteScopeIdentity) -> Self {
        self.scope = UiCommandRouteScope::ActivePortal;
        self.scope_identity = Some(identity);
        self
    }

    pub const fn consuming(mut self, context: UiCommandContextConsumption) -> Self {
        self.context = context;
        self
    }

    pub const fn with_priority(mut self, priority: UiCommandRoutePriority) -> Self {
        self.priority = priority;
        self
    }

    pub const fn with_repeat_policy(mut self, repeat: UiCommandRepeatPolicy) -> Self {
        self.repeat = repeat;
        self
    }

    pub const fn with_text_input_policy(mut self, text_input: UiCommandTextInputPolicy) -> Self {
        self.text_input = text_input;
        self
    }

    pub const fn destination(self) -> UiCommandRouteDestination {
        self.destination
    }

    pub const fn scope(self) -> UiCommandRouteScope {
        self.scope
    }

    pub const fn scope_identity(self) -> Option<UiCommandRouteScopeIdentity> {
        self.scope_identity
    }

    pub const fn context(self) -> UiCommandContextConsumption {
        self.context
    }

    pub const fn priority(self) -> UiCommandRoutePriority {
        self.priority
    }

    pub const fn repeat_policy(self) -> UiCommandRepeatPolicy {
        self.repeat
    }

    pub const fn text_input_policy(self) -> UiCommandTextInputPolicy {
        self.text_input
    }

    pub const fn registration_owner(self) -> Option<UiCommandRegistrationOwner> {
        self.registration_owner
    }

    pub(crate) fn digest_basis(self) -> String {
        let repeat = match self.repeat {
            UiCommandRepeatPolicy::Suppress => "suppress",
            UiCommandRepeatPolicy::Allow => "allow",
        };
        let text_input = match self.text_input {
            UiCommandTextInputPolicy::SuppressDuringCompositionAndTextInput => {
                "suppress_composition_and_text"
            }
            UiCommandTextInputPolicy::SuppressDuringComposition => "suppress_composition",
            UiCommandTextInputPolicy::Allow => "allow",
        };
        let owner = self.registration_owner.map_or_else(
            || "application".to_owned(),
            |owner| {
                format!(
                    "extension:{}:{}",
                    owner.identity().value(),
                    owner.generation().value()
                )
            },
        );
        format!(
            "{}:{}:{}:{}:{}:{repeat}:{text_input}:{owner}",
            self.destination.intent().as_str(),
            self.scope.precedence(),
            match self.scope_identity {
                Some(identity) => identity.digest(),
                None => 0,
            },
            self.context.bits(),
            self.priority.value(),
        )
    }
}
