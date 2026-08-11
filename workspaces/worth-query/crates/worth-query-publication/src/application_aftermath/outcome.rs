use super::WorthQueryPublishedExternalEffectPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublishedAftermathPosture {
    Reversible,
    Compensatable,
    Reconcilable,
    Irreversible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Closed descriptive output; private fields prevent caller-authored posture.
///
/// ```compile_fail
/// use worth_query_publication::facade::application_aftermath::{
///     WorthQueryPublishedApplicationAftermath,
///     WorthQueryPublishedExternalEffectPosture,
/// };
///
/// let forged = WorthQueryPublishedApplicationAftermath {
///     posture: None,
///     external_effect: WorthQueryPublishedExternalEffectPosture::Completed,
/// };
/// ```
pub struct WorthQueryPublishedApplicationAftermath {
    posture: Option<WorthQueryPublishedAftermathPosture>,
    external_effect: WorthQueryPublishedExternalEffectPosture,
}

impl WorthQueryPublishedApplicationAftermath {
    pub(super) const fn new(
        posture: Option<WorthQueryPublishedAftermathPosture>,
        external_effect: WorthQueryPublishedExternalEffectPosture,
    ) -> Self {
        Self {
            posture,
            external_effect,
        }
    }

    pub const fn posture(&self) -> Option<WorthQueryPublishedAftermathPosture> {
        self.posture
    }

    pub const fn external_effect(&self) -> WorthQueryPublishedExternalEffectPosture {
        self.external_effect
    }
}

pub(super) const fn publish_posture(
    posture: worth_query_installation::facade::PublishedAftermathPosture,
) -> WorthQueryPublishedAftermathPosture {
    use worth_query_installation::facade::PublishedAftermathPosture as Installed;

    match posture {
        Installed::Reversible => WorthQueryPublishedAftermathPosture::Reversible,
        Installed::Compensatable => WorthQueryPublishedAftermathPosture::Compensatable,
        Installed::Reconcilable => WorthQueryPublishedAftermathPosture::Reconcilable,
        Installed::Irreversible => WorthQueryPublishedAftermathPosture::Irreversible,
    }
}
