use crate::source::{WorthUiArtifactIdentitySeed, WorthUiIdentityReplacementClass};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiIdentityReplacementClassifier;

impl WorthUiIdentityReplacementClassifier {
    pub(crate) fn classify(
        previous: &WorthUiArtifactIdentitySeed,
        next: &WorthUiArtifactIdentitySeed,
    ) -> WorthUiIdentityReplacementClass {
        if previous == next {
            WorthUiIdentityReplacementClass::CarryForward
        } else {
            WorthUiIdentityReplacementClass::Replacement
        }
    }
}
