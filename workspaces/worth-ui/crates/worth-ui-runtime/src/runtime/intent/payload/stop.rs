#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiIntentPayloadStop {
    ApplicationGenerationChanged,
    PublicationTransitionInFlight,
    NoCurrentPublication,
    Targeting(crate::runtime::interaction::UiInteractionTargetingDenial),
    ProjectionUnavailable {
        field: &'static str,
        projection: worth_ui_query_binding::WorthUiQueryViewIdentity,
    },
    ProjectionNotCurrent {
        field: &'static str,
        posture: worth_ui_query_binding::UiProjectionInputPosture,
    },
    ProjectionShapeMismatch {
        field: &'static str,
    },
    ProjectionValueMissing {
        field: &'static str,
    },
    TextByteBudgetExceeded {
        field: &'static str,
        observed: usize,
        maximum: usize,
    },
    DraftInteractionRequired {
        field: &'static str,
    },
    DraftFieldMismatch {
        field: &'static str,
    },
    SelectionInteractionRequired {
        field: &'static str,
    },
    SelectionProjectionMismatch {
        field: &'static str,
    },
    SelectionRevisionChanged {
        field: &'static str,
    },
    ApplicationFactUnavailable {
        field: &'static str,
        fact: Box<str>,
    },
    ApplicationFactGenerationChanged {
        field: &'static str,
        fact: Box<str>,
    },
    ApplicationFactKindMismatch {
        field: &'static str,
        fact: Box<str>,
        observed: crate::capability::UiIntentPayloadFieldKind,
    },
    PayloadProjection(crate::capability::UiIntentPayloadProjectionViolation),
}
