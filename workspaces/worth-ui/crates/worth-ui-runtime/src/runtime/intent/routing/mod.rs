mod outcome;
mod resolution;
mod stop;

pub(crate) use outcome::UiIntentProductInputSource;
pub use outcome::{
    UiIntentRouteResolution, UiResolvedConfirmationIntentRoute, UiResolvedProductIntentRoute,
};
pub(crate) use outcome::{
    UiResolvedConfirmationIntentRouteInput, UiResolvedProductIntentRouteInput,
};
pub(crate) use resolution::resolve_intent_route;
pub use stop::UiIntentRouteResolutionStop;
