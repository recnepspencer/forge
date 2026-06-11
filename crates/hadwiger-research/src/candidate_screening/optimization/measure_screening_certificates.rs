use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

use super::{PeriodicMeasureWindow, ScreeningRational, ScreeningSolverTranscript};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutocorrelationOverlapCertificate {
    color_id: String,
    dx: ScreeningRational,
    dy: ScreeningRational,
    claimed_overlap_area: ScreeningRational,
    solver_transcript: ScreeningSolverTranscript,
}

impl AutocorrelationOverlapCertificate {
    pub fn new(
        color_id: impl Into<String>,
        dx: ScreeningRational,
        dy: ScreeningRational,
        claimed_overlap_area: ScreeningRational,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            color_id: require_non_empty(color_id, "color_id")?,
            dx,
            dy,
            claimed_overlap_area,
            solver_transcript,
        })
    }

    pub(crate) fn color_id(&self) -> &str {
        &self.color_id
    }

    pub(crate) fn dx(&self) -> &ScreeningRational {
        &self.dx
    }

    pub(crate) fn dy(&self) -> &ScreeningRational {
        &self.dy
    }

    pub(crate) fn claimed_overlap_area(&self) -> &ScreeningRational {
        &self.claimed_overlap_area
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.color_id,
            self.dx.stable_token(),
            self.dy.stable_token(),
            self.claimed_overlap_area.stable_token(),
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DensityCapCertificate {
    color_id: String,
    density_cap: ScreeningRational,
    theorem_source: String,
    solver_transcript: ScreeningSolverTranscript,
}

impl DensityCapCertificate {
    pub fn new(
        color_id: impl Into<String>,
        density_cap: ScreeningRational,
        theorem_source: impl Into<String>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            color_id: require_non_empty(color_id, "color_id")?,
            density_cap,
            theorem_source: require_non_empty(theorem_source, "density_theorem_source")?,
            solver_transcript,
        })
    }

    pub(crate) fn color_id(&self) -> &str {
        &self.color_id
    }

    pub(crate) fn density_cap(&self) -> &ScreeningRational {
        &self.density_cap
    }

    pub(crate) fn retained_cap_reference(&self) -> &str {
        &self.theorem_source
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.color_id,
            self.density_cap.stable_token(),
            self.theorem_source,
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalDensityWindowCertificate {
    color_id: String,
    window: PeriodicMeasureWindow,
    density_cap: ScreeningRational,
    bound_source: String,
    solver_transcript: ScreeningSolverTranscript,
}

impl LocalDensityWindowCertificate {
    pub fn new(
        color_id: impl Into<String>,
        window: PeriodicMeasureWindow,
        density_cap: ScreeningRational,
        bound_source: impl Into<String>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            color_id: require_non_empty(color_id, "color_id")?,
            window,
            density_cap,
            bound_source: require_non_empty(bound_source, "window_bound_source")?,
            solver_transcript,
        })
    }

    pub(crate) fn color_id(&self) -> &str {
        &self.color_id
    }

    pub(crate) fn window(&self) -> &PeriodicMeasureWindow {
        &self.window
    }

    pub(crate) fn density_cap(&self) -> &ScreeningRational {
        &self.density_cap
    }

    pub(crate) fn retained_bound_reference(&self) -> &str {
        &self.bound_source
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.color_id,
            self.window.stable_token(),
            self.density_cap.stable_token(),
            self.bound_source,
            self.solver_transcript.stable_token()
        )
    }
}
