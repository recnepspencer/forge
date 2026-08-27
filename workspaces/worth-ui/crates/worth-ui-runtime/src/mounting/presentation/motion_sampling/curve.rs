pub(super) fn interpolate_geometry(
    channels: crate::runtime::motion::UiMotionPropertyChannels,
    start: Option<[f32; 4]>,
    end: Option<[f32; 4]>,
    progress: f32,
) -> Option<[f32; 4]> {
    let (Some(start), Some(end)) = (start, end) else {
        return end.or(start);
    };
    let mut result = end;
    if channels.contains(crate::runtime::motion::UiMotionPropertyChannel::Geometry) {
        for index in 0..4 {
            result[index] = interpolate(start[index], end[index], progress);
        }
    }
    if channels.contains(crate::runtime::motion::UiMotionPropertyChannel::TranslationX) {
        result[0] = interpolate(start[0], end[0], progress);
    }
    if channels.contains(crate::runtime::motion::UiMotionPropertyChannel::TranslationY) {
        result[1] = interpolate(start[1], end[1], progress);
    }
    Some(result)
}

pub(super) fn ease(easing: crate::runtime::motion::UiMotionEasing, progress: f32) -> f32 {
    match easing {
        crate::runtime::motion::UiMotionEasing::Linear => progress,
        crate::runtime::motion::UiMotionEasing::EaseOutCubic => 1.0 - (1.0 - progress).powi(3),
    }
}

pub(super) fn interpolate(start: f32, end: f32, progress: f32) -> f32 {
    start + (end - start) * progress
}
