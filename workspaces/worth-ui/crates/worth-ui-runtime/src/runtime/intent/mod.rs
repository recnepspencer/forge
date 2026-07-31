mod routing;

pub(crate) use routing::resolve_intent_route;
pub use routing::{
    UiIntentRouteResolution, UiIntentRouteResolutionStop, UiResolvedConfirmationIntentRoute,
    UiResolvedProductIntentRoute,
};
