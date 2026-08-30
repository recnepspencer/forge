#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct UiPresentationMotionDamage {
    predecessor: Option<[f32; 4]>,
    successor: Option<[f32; 4]>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UiPresentationSampledClipGeometry([f32; 4]);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UiPresentationMotionDamageRegion([f32; 4]);

impl UiPresentationMotionDamage {
    pub(super) const fn between(
        predecessor: Option<[f32; 4]>,
        successor: Option<[f32; 4]>,
    ) -> Self {
        Self {
            predecessor,
            successor,
        }
    }

    pub(crate) fn clipped_to(
        self,
        clip: UiPresentationSampledClipGeometry,
    ) -> [Option<UiPresentationMotionDamageRegion>; 2] {
        [self.predecessor, self.successor]
            .map(|region| region.and_then(|bounds| intersect(bounds, clip.0)))
    }
}

impl UiPresentationSampledClipGeometry {
    pub(crate) fn from_presented_components(
        components: [f32; 4],
    ) -> Result<Self, super::UiPresentationGeometrySamplingDenial> {
        if components.iter().any(|value| !value.is_finite()) {
            return Err(super::UiPresentationGeometrySamplingDenial::NonFinite);
        }
        if components[2] < 0.0 || components[3] < 0.0 {
            return Err(super::UiPresentationGeometrySamplingDenial::NegativeExtent);
        }
        Ok(Self(components))
    }
}

impl UiPresentationMotionDamageRegion {
    pub(crate) const fn components(self) -> [f32; 4] {
        self.0
    }
}

fn intersect(bounds: [f32; 4], clip: [f32; 4]) -> Option<UiPresentationMotionDamageRegion> {
    let x = bounds[0].max(clip[0]);
    let y = bounds[1].max(clip[1]);
    let right = (bounds[0] + bounds[2]).min(clip[0] + clip[2]);
    let bottom = (bounds[1] + bounds[3]).min(clip[1] + clip[3]);
    (right > x && bottom > y).then_some(UiPresentationMotionDamageRegion([
        x,
        y,
        right - x,
        bottom - y,
    ]))
}
