use super::{
    UiMountedPresentationDelta, UiMountedPresentationInitial, UiMountedPresentationReconstruction,
    UiMountedPresentationUnchanged,
};

#[derive(Clone, Copy)]
pub enum UiMountedPresentationWorkView<'work> {
    Initial(&'work UiMountedPresentationInitial),
    Delta(&'work UiMountedPresentationDelta),
    Reconstruction(&'work UiMountedPresentationReconstruction),
    Unchanged(&'work UiMountedPresentationUnchanged),
}

impl UiMountedPresentationWorkView<'_> {
    pub const fn affinity(self) -> super::UiMountedPresentationAffinity {
        match self {
            Self::Initial(initial) => initial.affinity(),
            Self::Delta(delta) => delta.affinity(),
            Self::Reconstruction(reconstruction) => reconstruction.affinity(),
            Self::Unchanged(unchanged) => unchanged.affinity(),
        }
    }
}
