use worth_ui_host_contract::{UiHostSurfacePresentationDenial, UiMountedEffectFamily};

use super::super::consumption_view::UiMountedHostPresentationAuthority;
use super::super::work_producer::{
    UiMountedPresentationCandidates, UiMountedPresentationState,
    UiMountedPresentationWorkProductionDenial,
};

pub(super) struct UiPreparedSurfacePresentation {
    pub(super) work: super::super::UiMountedPresentationWork,
    pub(super) expected_effects: Box<[UiMountedEffectFamily]>,
}

pub(super) struct UiPreparedFramePresentation {
    pub(super) surfaces: Vec<UiPreparedSurfacePresentation>,
    pub(super) candidates: UiMountedPresentationCandidates,
}

pub(super) fn prepare(
    frame: &crate::mounting::UiPreparedMountedFrame,
    retained: &UiMountedPresentationCandidates,
    reconstruction_bindings: &std::collections::BTreeSet<
        worth_ui_host_contract::UiSurfaceBindingGeneration,
    >,
    authority: &UiMountedHostPresentationAuthority<'_>,
) -> Result<UiPreparedFramePresentation, UiHostSurfacePresentationDenial> {
    let source = frame.presentation_delta_source();
    let mut surfaces = Vec::with_capacity(frame.surfaces().len());
    let mut candidates = UiMountedPresentationCandidates::new();
    for surface in frame.surfaces() {
        let predecessor = retained.get(&surface.requirement().binding());
        let reconstruction_required =
            reconstruction_bindings.contains(&surface.requirement().binding());
        let (candidate, mut work) = match (source.predecessor(), predecessor) {
            (Some(source_frame), Some(predecessor))
                if reconstruction_required && source_frame == predecessor.frame() =>
            {
                let complete_projection = worth_ui_host_contract::UiMountedPresentationAuxiliaryState::from_runtime_mounting(
                    surface.projection(),
                )
                .reconstruct_authored()
                .map_err(|_| UiHostSurfacePresentationDenial::MalformedProjection)?;
                let candidate = UiMountedPresentationState::from_projection(
                    &complete_projection,
                    surface.requirement(),
                    Some(source_frame),
                );
                let work = candidate.issue_reconstruction(
                    authority.presentation(),
                    &complete_projection,
                    source_frame,
                );
                Ok((candidate, work))
            }
            (Some(source_frame), Some(predecessor)) if source_frame == predecessor.frame() => {
                let candidate = UiMountedPresentationState::successor_from_source(
                    predecessor,
                    source,
                    None,
                    surface.requirement(),
                );
                predecessor
                    .issue_successor(
                        &candidate,
                        source.changed_instances(),
                        source.frame().presentation_command_changes(),
                        source.surface_changed(surface.requirement().semantic_surface()),
                        source.predecessor(),
                        authority.presentation(),
                    )
                    .map(|work| (candidate, work))
                    .map_err(UiWorkPreparationError::from)
            }
            (None, None) => {
                let candidate = UiMountedPresentationState::from_projection(
                    surface.projection(),
                    surface.requirement(),
                    None,
                );
                let work = candidate.issue_initial(authority.presentation(), surface.projection());
                Ok((candidate, work))
            }
            (Some(source_frame), None) => {
                let complete_projection = worth_ui_host_contract::UiMountedPresentationAuxiliaryState::from_runtime_mounting(
                    surface.projection(),
                )
                .reconstruct_authored()
                .map_err(|_| UiHostSurfacePresentationDenial::MalformedProjection)?;
                let candidate = UiMountedPresentationState::from_projection(
                    &complete_projection,
                    surface.requirement(),
                    Some(source_frame),
                );
                let work = candidate.issue_reconstruction(
                    authority.presentation(),
                    &complete_projection,
                    source_frame,
                );
                Ok((candidate, work))
            }
            (None, Some(_)) | (Some(_), Some(_)) => Err(UiWorkPreparationError::Source(
                UiHostSurfacePresentationDenial::StalePredecessor,
            )),
        }
        .map_err(classify_work_error)?;
        work.bind_layout_owner(surface.projection_owner());
        let expected_effects = candidate
            .expected_completion_effects(
                predecessor,
                &work,
                surface.requirement().presentation_mode(),
            )
            .into_boxed_slice();
        candidates.insert(surface.requirement().binding(), candidate);
        surfaces.push(UiPreparedSurfacePresentation {
            work,
            expected_effects,
        });
    }
    Ok(UiPreparedFramePresentation {
        surfaces,
        candidates,
    })
}

enum UiWorkPreparationError {
    Production(UiMountedPresentationWorkProductionDenial),
    Source(UiHostSurfacePresentationDenial),
}

impl From<UiMountedPresentationWorkProductionDenial> for UiWorkPreparationError {
    fn from(denial: UiMountedPresentationWorkProductionDenial) -> Self {
        Self::Production(denial)
    }
}

impl From<UiHostSurfacePresentationDenial> for UiWorkPreparationError {
    fn from(denial: UiHostSurfacePresentationDenial) -> Self {
        Self::Source(denial)
    }
}

fn classify_work_error(error: UiWorkPreparationError) -> UiHostSurfacePresentationDenial {
    match error {
        UiWorkPreparationError::Source(denial) => denial,
        UiWorkPreparationError::Production(
            UiMountedPresentationWorkProductionDenial::StalePredecessor,
        ) => UiHostSurfacePresentationDenial::StalePredecessor,
        UiWorkPreparationError::Production(
            UiMountedPresentationWorkProductionDenial::SurfaceChanged
            | UiMountedPresentationWorkProductionDenial::BindingChanged
            | UiMountedPresentationWorkProductionDenial::BaselineChanged,
        ) => UiHostSurfacePresentationDenial::SurfaceBindingChanged,
    }
}
