use worth_ui_host_contract::{
    UiHostSurfacePresentationMode, UiMountedEffectFamily, UiMountedPresentationWorkView,
};

use super::{super::UiMountedPresentationWork, UiMountedPresentationState};

impl UiMountedPresentationState {
    pub(in crate::mounting::presentation) fn expected_completion_effects(
        &self,
        predecessor: Option<&Self>,
        work: &UiMountedPresentationWork,
        mode: UiHostSurfacePresentationMode,
    ) -> Vec<UiMountedEffectFamily> {
        if mode == UiHostSurfacePresentationMode::RecordOnly {
            return vec![UiMountedEffectFamily::RecordedProjection];
        }
        match work.view() {
            UiMountedPresentationWorkView::Initial(_)
            | UiMountedPresentationWorkView::Reconstruction(_) => {
                let mut effects = self.effects.to_vec();
                effects.push(UiMountedEffectFamily::NativePaint);
                effects.sort();
                effects.dedup();
                effects
            }
            UiMountedPresentationWorkView::Delta(delta) => {
                let mut effects = Vec::new();
                if !delta.changes().is_empty()
                    || !delta.order().is_empty()
                    || !delta.damage().is_empty()
                {
                    effects.push(UiMountedEffectFamily::NativePaint);
                }
                if overlay_changed(
                    self,
                    predecessor,
                    delta.auxiliary().is_some() || !delta.nodes().is_empty(),
                ) {
                    effects.push(UiMountedEffectFamily::IdentityOverlay);
                }
                effects
            }
            UiMountedPresentationWorkView::Unchanged(_) => Vec::new(),
        }
    }
}

fn overlay_changed(
    successor: &UiMountedPresentationState,
    predecessor: Option<&UiMountedPresentationState>,
    presentation_changed: bool,
) -> bool {
    presentation_changed
        && predecessor.is_some_and(|prior| {
            prior
                .effects
                .contains(&UiMountedEffectFamily::IdentityOverlay)
                || successor
                    .effects
                    .contains(&UiMountedEffectFamily::IdentityOverlay)
        })
}
