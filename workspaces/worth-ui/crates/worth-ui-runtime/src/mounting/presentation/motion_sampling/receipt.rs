#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPresentationReducedMotionPosture {
    NoPreference,
    Reduce,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPresentationMotionSamplePosture {
    Delayed,
    Active,
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiPresentationMotionTerminalRequest {
    track: crate::runtime::motion::UiMotionTrackIdentity,
    cause: crate::runtime::motion::UiMotionTerminalCause,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiPresentationMotionSamplingCost {
    tracks_considered: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UiPresentationMotionSampleReceipt {
    track: crate::runtime::motion::UiMotionTrackIdentity,
    target: crate::runtime::motion::UiMotionTargetIdentity,
    tick: u64,
    base_geometry: Option<crate::runtime::motion::UiMotionSemanticGeometry>,
    geometry: Option<super::UiPresentationSampledGeometry>,
    opacity: f32,
    hit_test_visible: bool,
    damage: super::UiPresentationMotionDamage,
    posture: UiPresentationMotionSamplePosture,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct UiPresentationMotionSamplingReceipt {
    pub(super) samples: Box<[UiPresentationMotionSampleReceipt]>,
    terminals: Box<[UiPresentationMotionTerminalRequest]>,
    cost: UiPresentationMotionSamplingCost,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UiPresentationMotionInstallationReceipt {
    sample: Option<UiPresentationMotionSampleReceipt>,
    terminal: Option<UiPresentationMotionTerminalRequest>,
}

impl UiPresentationMotionTerminalRequest {
    pub(super) const fn new(
        track: crate::runtime::motion::UiMotionTrackIdentity,
        cause: crate::runtime::motion::UiMotionTerminalCause,
    ) -> Self {
        Self { track, cause }
    }

    pub(crate) const fn track(self) -> crate::runtime::motion::UiMotionTrackIdentity {
        self.track
    }

    pub(crate) const fn cause(self) -> crate::runtime::motion::UiMotionTerminalCause {
        self.cause
    }
}

impl UiPresentationMotionSamplingCost {
    pub(super) const fn new(tracks: usize) -> Self {
        Self {
            tracks_considered: tracks as u64,
        }
    }

    pub(crate) const fn tracks_considered(self) -> u64 {
        self.tracks_considered
    }
}

impl UiPresentationMotionSampleReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(super) const fn new(
        track: crate::runtime::motion::UiMotionTrackIdentity,
        target: crate::runtime::motion::UiMotionTargetIdentity,
        tick: u64,
        base_geometry: Option<crate::runtime::motion::UiMotionSemanticGeometry>,
        geometry: Option<super::UiPresentationSampledGeometry>,
        opacity: f32,
        hit_test_visible: bool,
        damage: super::UiPresentationMotionDamage,
        posture: UiPresentationMotionSamplePosture,
    ) -> Self {
        Self {
            track,
            target,
            tick,
            base_geometry,
            geometry,
            opacity,
            hit_test_visible,
            damage,
            posture,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn from_track_sample(
        track: crate::runtime::motion::UiCommittedMotionTrack,
        tick: u64,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        components: Option<[f32; 4]>,
        opacity: f32,
        posture: UiPresentationMotionSamplePosture,
        damage: super::UiPresentationMotionDamage,
    ) -> Result<Self, super::UiPresentationGeometrySamplingDenial> {
        let semantic_basis = track.successor_geometry();
        let geometry = components
            .zip(semantic_basis)
            .map(|(components, semantic_basis)| {
                super::UiPresentationSampledGeometry::from_motion_sample(
                    track.target(),
                    track.successor_revision(),
                    track.successor_presentation(),
                    semantic_basis,
                    components,
                    presentation,
                )
            })
            .transpose()?;
        if components.is_some() != semantic_basis.is_some() {
            return Err(super::UiPresentationGeometrySamplingDenial::MissingSemanticBasis);
        }
        let visual_visible = opacity > 0.0 && geometry.is_some();
        Ok(Self::new(
            track.identity(),
            track.target(),
            tick,
            semantic_basis,
            geometry,
            opacity,
            track.successor_visible() && visual_visible,
            damage,
            posture,
        ))
    }

    pub(crate) const fn track(self) -> crate::runtime::motion::UiMotionTrackIdentity {
        self.track
    }
    pub(crate) const fn target(self) -> crate::runtime::motion::UiMotionTargetIdentity {
        self.target
    }
    pub(crate) const fn tick(self) -> u64 {
        self.tick
    }
    pub(crate) const fn base_geometry(
        self,
    ) -> Option<crate::runtime::motion::UiMotionSemanticGeometry> {
        self.base_geometry
    }
    pub(crate) const fn geometry(self) -> Option<super::UiPresentationSampledGeometry> {
        self.geometry
    }
    pub(crate) const fn opacity(self) -> f32 {
        self.opacity
    }
    pub(crate) const fn hit_test_visible(self) -> bool {
        self.hit_test_visible
    }
    pub(crate) const fn damage(self) -> super::UiPresentationMotionDamage {
        self.damage
    }
    pub(crate) const fn posture(self) -> UiPresentationMotionSamplePosture {
        self.posture
    }

    pub(super) fn with_presentation_basis(
        mut self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Result<Self, super::UiPresentationGeometrySamplingDenial> {
        self.geometry = self
            .geometry
            .map(|geometry| geometry.with_presentation_basis(presentation))
            .transpose()?;
        Ok(self)
    }
}

impl UiPresentationMotionSamplingReceipt {
    pub(super) fn new(
        samples: Vec<UiPresentationMotionSampleReceipt>,
        terminals: Vec<UiPresentationMotionTerminalRequest>,
        tracks_considered: usize,
    ) -> Self {
        let cost = UiPresentationMotionSamplingCost::new(tracks_considered);
        Self {
            samples: samples.into_boxed_slice(),
            terminals: terminals.into_boxed_slice(),
            cost,
        }
    }

    pub(crate) fn samples(&self) -> &[UiPresentationMotionSampleReceipt] {
        &self.samples
    }
    pub(crate) fn terminals(&self) -> &[UiPresentationMotionTerminalRequest] {
        &self.terminals
    }
    pub(crate) const fn cost(&self) -> UiPresentationMotionSamplingCost {
        self.cost
    }
}

impl UiPresentationMotionInstallationReceipt {
    pub(super) const fn new(
        sample: Option<UiPresentationMotionSampleReceipt>,
        terminal: Option<UiPresentationMotionTerminalRequest>,
    ) -> Self {
        Self { sample, terminal }
    }

    #[cfg(test)]
    pub(crate) const fn sample(self) -> Option<UiPresentationMotionSampleReceipt> {
        self.sample
    }
    pub(crate) const fn terminal(self) -> Option<UiPresentationMotionTerminalRequest> {
        self.terminal
    }
}
