use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

use super::{ScreeningRational, ScreeningSolverTranscript};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicMeasureCell {
    color_id: String,
    x_min: ScreeningRational,
    x_max: ScreeningRational,
    y_min: ScreeningRational,
    y_max: ScreeningRational,
}

impl PeriodicMeasureCell {
    pub fn rectangle(
        color_id: impl Into<String>,
        x_min: ScreeningRational,
        x_max: ScreeningRational,
        y_min: ScreeningRational,
        y_max: ScreeningRational,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if x_max.sub(&x_min).is_negative() || x_max.sub(&x_min).is_zero() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "cell_x_extent",
            });
        }
        if y_max.sub(&y_min).is_negative() || y_max.sub(&y_min).is_zero() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "cell_y_extent",
            });
        }
        Ok(Self {
            color_id: require_non_empty(color_id, "color_id")?,
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }

    pub(crate) fn color_id(&self) -> &str {
        &self.color_id
    }

    pub(crate) fn area(&self) -> ScreeningRational {
        self.x_max
            .sub(&self.x_min)
            .mul(&self.y_max.sub(&self.y_min))
    }

    pub(crate) fn overlap_area_after_translation(
        &self,
        other: &Self,
        dx: &ScreeningRational,
        dy: &ScreeningRational,
    ) -> ScreeningRational {
        interval_overlap(
            &self.x_min,
            &self.x_max,
            &other.x_min.add(dx),
            &other.x_max.add(dx),
        )
        .mul(&interval_overlap(
            &self.y_min,
            &self.y_max,
            &other.y_min.add(dy),
            &other.y_max.add(dy),
        ))
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.color_id,
            self.x_min.stable_token(),
            self.x_max.stable_token(),
            self.y_min.stable_token(),
            self.y_max.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicColorClassMeasureModel {
    model_id: String,
    period_width: ScreeningRational,
    period_height: ScreeningRational,
    cells: Vec<PeriodicMeasureCell>,
}

impl PeriodicColorClassMeasureModel {
    pub fn new(
        model_id: impl Into<String>,
        period_width: ScreeningRational,
        period_height: ScreeningRational,
        cells: Vec<PeriodicMeasureCell>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if !period_width.is_positive() || !period_height.is_positive() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "period_extent",
            });
        }
        if cells.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "measure_cells",
            });
        }
        for cell in &cells {
            if cell.x_min.is_negative()
                || cell.y_min.is_negative()
                || cell.x_max > period_width
                || cell.y_max > period_height
            {
                return Err(HadwigerArtifactShapeError::EmptyField {
                    field: "cell_outside_period",
                });
            }
        }
        Ok(Self {
            model_id: require_non_empty(model_id, "model_id")?,
            period_width,
            period_height,
            cells,
        })
    }

    pub(crate) fn cells(&self) -> &[PeriodicMeasureCell] {
        &self.cells
    }

    pub(crate) fn period_area(&self) -> ScreeningRational {
        self.period_width.mul(&self.period_height)
    }

    pub(crate) fn color_area(&self, color_id: &str) -> ScreeningRational {
        self.cells
            .iter()
            .filter(|cell| cell.color_id() == color_id)
            .fold(ScreeningRational::integer(0), |sum, cell| {
                sum.add(&cell.area())
            })
    }

    pub fn stable_token(&self) -> String {
        let mut token = format!(
            "{}:{}:{}",
            self.model_id,
            self.period_width.stable_token(),
            self.period_height.stable_token()
        );
        for cell in &self.cells {
            token.push_str(&format!(":{}", cell.stable_token()));
        }
        token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicMeasureWindow {
    window_id: String,
    x_min: ScreeningRational,
    x_max: ScreeningRational,
    y_min: ScreeningRational,
    y_max: ScreeningRational,
}

impl PeriodicMeasureWindow {
    pub fn rectangle(
        window_id: impl Into<String>,
        x_min: ScreeningRational,
        x_max: ScreeningRational,
        y_min: ScreeningRational,
        y_max: ScreeningRational,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let cell = PeriodicMeasureCell::rectangle("window", x_min, x_max, y_min, y_max)?;
        Ok(Self {
            window_id: require_non_empty(window_id, "window_id")?,
            x_min: cell.x_min,
            x_max: cell.x_max,
            y_min: cell.y_min,
            y_max: cell.y_max,
        })
    }

    pub(crate) fn area(&self) -> ScreeningRational {
        self.x_max
            .sub(&self.x_min)
            .mul(&self.y_max.sub(&self.y_min))
    }

    pub(crate) fn overlap_area(&self, cell: &PeriodicMeasureCell) -> ScreeningRational {
        interval_overlap(&self.x_min, &self.x_max, &cell.x_min, &cell.x_max).mul(&interval_overlap(
            &self.y_min,
            &self.y_max,
            &cell.y_min,
            &cell.y_max,
        ))
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.window_id,
            self.x_min.stable_token(),
            self.x_max.stable_token(),
            self.y_min.stable_token(),
            self.y_max.stable_token()
        )
    }
}

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

fn interval_overlap(
    left_min: &ScreeningRational,
    left_max: &ScreeningRational,
    right_min: &ScreeningRational,
    right_max: &ScreeningRational,
) -> ScreeningRational {
    let min_max = if left_max <= right_max {
        left_max
    } else {
        right_max
    };
    let max_min = if left_min >= right_min {
        left_min
    } else {
        right_min
    };
    let width = min_max.sub(max_min);
    if width.is_positive() {
        width
    } else {
        ScreeningRational::integer(0)
    }
}
