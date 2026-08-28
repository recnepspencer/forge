mod candidate;
mod context;
mod host_input;
mod input_stroke;
mod outcome;
mod plan;
mod prefix;
mod resolution;
mod state;

pub(crate) use context::UiCommandRoutingContext;
pub(crate) use host_input::keyboard_stroke;
pub(crate) use outcome::UiCommandRouteEvidence;
pub use outcome::{
    UiCommandAmbiguity, UiCommandInvocationOrigin, UiCommandPrefixReceipt, UiCommandRouteLoss,
    UiCommandRouteLossReason, UiCommandRouteReceipt, UiCommandRoutingOutcome,
    UiCommandRoutingSuppression,
};
pub(crate) use state::UiCommandRoutingRuntimeState;

#[cfg(test)]
mod currentness_tests;
#[cfg(test)]
mod tests;
