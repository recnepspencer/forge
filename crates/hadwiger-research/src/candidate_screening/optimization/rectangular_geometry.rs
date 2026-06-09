use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

use super::ScreeningRational;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScreeningRectangularRegion {
    region_id: String,
    x_min: ScreeningRational,
    x_max: ScreeningRational,
    y_min: ScreeningRational,
    y_max: ScreeningRational,
}

impl ScreeningRectangularRegion {
    pub fn new(
        region_id: impl Into<String>,
        x_min: ScreeningRational,
        x_max: ScreeningRational,
        y_min: ScreeningRational,
        y_max: ScreeningRational,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if x_max.sub(&x_min).is_positive() && y_max.sub(&y_min).is_positive() {
            Ok(Self {
                region_id: require_non_empty(region_id, "region_id")?,
                x_min,
                x_max,
                y_min,
                y_max,
            })
        } else {
            Err(HadwigerArtifactShapeError::EmptyField {
                field: "rectangular_region_extent",
            })
        }
    }

    pub fn region_id(&self) -> &str {
        &self.region_id
    }

    pub(crate) fn translated(&self, dx: &ScreeningRational, dy: &ScreeningRational) -> Self {
        Self {
            region_id: self.region_id.clone(),
            x_min: self.x_min.add(dx),
            x_max: self.x_max.add(dx),
            y_min: self.y_min.add(dy),
            y_max: self.y_max.add(dy),
        }
    }

    pub(crate) fn unit_circle_intersects_difference(&self, other: &Self) -> bool {
        self.difference_min_squared_distance(other)
            .cmp_integer(1)
            .is_le()
            && self
                .difference_max_squared_distance(other)
                .cmp_integer(1)
                .is_ge()
    }

    pub(crate) fn difference_min_squared_distance(&self, other: &Self) -> ScreeningRational {
        let x_min = self.x_min.sub(&other.x_max);
        let x_max = self.x_max.sub(&other.x_min);
        let y_min = self.y_min.sub(&other.y_max);
        let y_max = self.y_max.sub(&other.y_min);
        rectangle_min_squared_distance_to_origin(&x_min, &x_max, &y_min, &y_max)
    }

    pub(crate) fn difference_max_squared_distance(&self, other: &Self) -> ScreeningRational {
        let x_min = self.x_min.sub(&other.x_max);
        let x_max = self.x_max.sub(&other.x_min);
        let y_min = self.y_min.sub(&other.y_max);
        let y_max = self.y_max.sub(&other.y_min);
        rectangle_max_squared_distance_to_origin(&x_min, &x_max, &y_min, &y_max)
    }

    pub(crate) fn diameter_squared(&self) -> ScreeningRational {
        self.x_max
            .sub(&self.x_min)
            .mul(&self.x_max.sub(&self.x_min))
            .add(
                &self
                    .y_max
                    .sub(&self.y_min)
                    .mul(&self.y_max.sub(&self.y_min)),
            )
    }

    pub(crate) fn forbidden_displacement_contains(
        &self,
        dx: &ScreeningRational,
        dy: &ScreeningRational,
    ) -> bool {
        self.unit_circle_intersects_difference(&self.translated(dx, dy))
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.region_id,
            self.x_min.stable_token(),
            self.x_max.stable_token(),
            self.y_min.stable_token(),
            self.y_max.stable_token()
        )
    }
}

fn rectangle_min_squared_distance_to_origin(
    x_min: &ScreeningRational,
    x_max: &ScreeningRational,
    y_min: &ScreeningRational,
    y_max: &ScreeningRational,
) -> ScreeningRational {
    interval_min_abs_square(x_min, x_max).add(&interval_min_abs_square(y_min, y_max))
}

fn rectangle_max_squared_distance_to_origin(
    x_min: &ScreeningRational,
    x_max: &ScreeningRational,
    y_min: &ScreeningRational,
    y_max: &ScreeningRational,
) -> ScreeningRational {
    interval_max_abs_square(x_min, x_max).add(&interval_max_abs_square(y_min, y_max))
}

fn interval_min_abs_square(min: &ScreeningRational, max: &ScreeningRational) -> ScreeningRational {
    if min.cmp_integer(0).is_le() && max.cmp_integer(0).is_ge() {
        ScreeningRational::integer(0)
    } else {
        min.mul(min).min(max.mul(max))
    }
}

fn interval_max_abs_square(min: &ScreeningRational, max: &ScreeningRational) -> ScreeningRational {
    min.mul(min).max(max.mul(max))
}
