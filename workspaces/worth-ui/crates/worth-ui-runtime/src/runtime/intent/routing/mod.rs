mod outcome;
mod resolution;
mod stop;

pub use outcome::{
    UiIntentRouteResolution, UiResolvedConfirmationIntentRoute, UiResolvedProductIntentRoute,
};
pub(crate) use resolution::resolve_intent_route;
pub use stop::UiIntentRouteResolutionStop;
