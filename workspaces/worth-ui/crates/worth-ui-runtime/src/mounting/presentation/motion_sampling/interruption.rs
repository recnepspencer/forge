#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum UiPresentationMotionInstallation {
    Install {
        geometry: Option<[f32; 4]>,
        opacity: f32,
        duration_ticks: u32,
    },
    SnapToTarget,
}

pub(super) fn resolve(
    track: crate::runtime::motion::UiCommittedMotionTrack,
    current: Option<(Option<[f32; 4]>, f32)>,
    reduced_motion: super::UiPresentationReducedMotionPosture,
) -> UiPresentationMotionInstallation {
    let declaration = track.declaration();
    if reduced_motion == super::UiPresentationReducedMotionPosture::Reduce
        && declaration.reduced_motion()
            == crate::runtime::motion::UiMotionReducedMotionPolicy::SystemRespecting
    {
        if declaration.decorative() {
            return UiPresentationMotionInstallation::SnapToTarget;
        }
        return UiPresentationMotionInstallation::Install {
            geometry: semantic_predecessor(track),
            opacity: predecessor_opacity(track),
            duration_ticks: 1,
        };
    }
    let duration_ticks = declaration.duration_ticks();
    match track.retarget() {
        None => UiPresentationMotionInstallation::Install {
            geometry: semantic_predecessor(track),
            opacity: predecessor_opacity(track),
            duration_ticks,
        },
        Some(crate::runtime::motion::UiMotionRetargetDisposition::Install {
            predecessor:
                crate::runtime::motion::UiMotionRetargetPredecessor::CurrentPresentationSample,
        }) => {
            let (geometry, opacity) = current
                .unwrap_or_else(|| (semantic_predecessor(track), predecessor_opacity(track)));
            UiPresentationMotionInstallation::Install {
                geometry,
                opacity,
                duration_ticks,
            }
        }
    }
}

pub(super) fn semantic_predecessor(
    track: crate::runtime::motion::UiCommittedMotionTrack,
) -> Option<[f32; 4]> {
    track
        .predecessor_geometry()
        .map(crate::runtime::motion::UiMotionSemanticGeometry::components)
}

pub(super) const fn predecessor_opacity(
    track: crate::runtime::motion::UiCommittedMotionTrack,
) -> f32 {
    if track.predecessor_visible() {
        1.0
    } else {
        0.0
    }
}
