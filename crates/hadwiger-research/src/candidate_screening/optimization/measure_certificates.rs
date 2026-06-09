use std::collections::BTreeMap;

use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

use super::ScreeningRational;

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
    color_cell_index: BTreeMap<String, Vec<usize>>,
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
        let color_cell_index = color_cell_index(&cells);
        Ok(Self {
            model_id: require_non_empty(model_id, "model_id")?,
            period_width,
            period_height,
            cells,
            color_cell_index,
        })
    }

    pub(crate) fn period_area(&self) -> ScreeningRational {
        self.period_width.mul(&self.period_height)
    }

    pub(crate) fn color_area(&self, color_id: &str) -> ScreeningRational {
        self.cells_for_color(color_id)
            .fold(ScreeningRational::integer(0), |sum, cell| {
                sum.add(&cell.area())
            })
    }

    pub(crate) fn color_area_in_window(
        &self,
        color_id: &str,
        window: &PeriodicMeasureWindow,
    ) -> ScreeningRational {
        self.cells_for_color(color_id)
            .fold(ScreeningRational::integer(0), |sum, cell| {
                sum.add(&window.overlap_area(cell))
            })
    }

    pub(crate) fn same_color_translated_overlap_area(
        &self,
        color_id: &str,
        dx: &ScreeningRational,
        dy: &ScreeningRational,
    ) -> ScreeningRational {
        let same_color = self.cells_for_color(color_id).collect::<Vec<_>>();
        same_color
            .iter()
            .flat_map(|left| same_color.iter().map(move |right| (*left, *right)))
            .fold(ScreeningRational::integer(0), |sum, (left, right)| {
                sum.add(&left.overlap_area_after_translation(right, dx, dy))
            })
    }

    fn cells_for_color<'a>(
        &'a self,
        color_id: &'a str,
    ) -> impl Iterator<Item = &'a PeriodicMeasureCell> + 'a {
        self.color_cell_index
            .get(color_id)
            .into_iter()
            .flatten()
            .map(|index| &self.cells[*index])
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

fn color_cell_index(cells: &[PeriodicMeasureCell]) -> BTreeMap<String, Vec<usize>> {
    let mut index: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (cell_index, cell) in cells.iter().enumerate() {
        index
            .entry(cell.color_id().to_string())
            .or_default()
            .push(cell_index);
    }
    index
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
