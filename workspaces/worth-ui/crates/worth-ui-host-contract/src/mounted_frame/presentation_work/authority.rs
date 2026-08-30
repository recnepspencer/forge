use super::{
    UiMountedPresentationDelta, UiMountedPresentationInitial, UiMountedPresentationReconstruction,
    UiMountedPresentationSample, UiMountedPresentationUnchanged,
};

#[derive(Clone, Copy)]
pub enum UiMountedPresentationWorkView<'work> {
    Initial(&'work UiMountedPresentationInitial),
    Delta(&'work UiMountedPresentationDelta),
    Reconstruction(&'work UiMountedPresentationReconstruction),
    Sample(&'work UiMountedPresentationSample),
    Unchanged(&'work UiMountedPresentationUnchanged),
}

impl UiMountedPresentationWorkView<'_> {
    pub const fn affinity(self) -> super::UiMountedPresentationAffinity {
        match self {
            Self::Initial(initial) => initial.affinity(),
            Self::Delta(delta) => delta.affinity(),
            Self::Reconstruction(reconstruction) => reconstruction.affinity(),
            Self::Sample(sample) => sample.affinity(),
            Self::Unchanged(unchanged) => unchanged.affinity(),
        }
    }

    pub const fn production_cost(self) -> crate::UiMountedPresentationProductionCost {
        match self {
            Self::Initial(initial) => initial.production_cost(),
            Self::Delta(delta) => delta.production_cost(),
            Self::Reconstruction(reconstruction) => reconstruction.production_cost(),
            Self::Sample(sample) => sample.production_cost(),
            Self::Unchanged(unchanged) => unchanged.production_cost(),
        }
    }

    pub fn contains_semantic_text(self) -> bool {
        match self {
            Self::Initial(initial) => initial.commands().iter().any(is_semantic_text_command),
            Self::Delta(delta) => delta.changes().iter().any(|change| match change {
                super::UiMountedPaintCommandChange::Insert(command)
                | super::UiMountedPaintCommandChange::Replace {
                    successor: command, ..
                } => is_semantic_text_command(command),
                super::UiMountedPaintCommandChange::Remove(identity) => {
                    identity.semantic_text_identity_parts().is_some()
                }
            }),
            Self::Reconstruction(reconstruction) => reconstruction
                .commands()
                .iter()
                .any(is_semantic_text_command),
            Self::Sample(_) => false,
            Self::Unchanged(_) => false,
        }
    }
}

fn is_semantic_text_command(command: &super::UiMountedPaintCommand) -> bool {
    matches!(command, super::UiMountedPaintCommand::SemanticText { .. })
}
