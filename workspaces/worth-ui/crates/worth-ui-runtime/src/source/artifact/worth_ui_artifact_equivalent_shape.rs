use crate::source::{
    WorthUiArtifact, WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceComparator,
};

pub(crate) struct WorthUiArtifactEquivalentShape;

impl WorthUiArtifactEquivalentShape {
    pub(crate) fn artifacts_are_equivalent(
        left: &WorthUiArtifact,
        right: &WorthUiArtifact,
    ) -> bool {
        WorthUiArtifactEquivalenceComparator::compare(
            left,
            right,
            WorthUiArtifactEquivalenceBasis::semantic(),
        )
        .is_equivalent()
    }
}
