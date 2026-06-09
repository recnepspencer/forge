use crate::candidate_screening::{ScreeningRational, ScreeningRectangularRegion};
use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;
use crate::mathematical_verification::ExactRational;

use super::boundary_ownership::{BoundaryOwnershipKind, BoundaryOwnershipPolicy};
use super::tiling_geometry_errors::TilingGeometryError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TilingTileId {
    value: String,
}

impl TilingTileId {
    pub fn new(value: impl Into<String>) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            value: require_non_empty(value, "tile_id")?,
        })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TilingColorId {
    value: String,
}

impl TilingColorId {
    pub fn new(value: impl Into<String>) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            value: require_non_empty(value, "color_id")?,
        })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RectangularTileRegion {
    tile_id: TilingTileId,
    color_id: TilingColorId,
    x_min: ExactRational,
    x_max: ExactRational,
    y_min: ExactRational,
    y_max: ExactRational,
    boundary_ownership: Option<BoundaryOwnershipPolicy>,
}

impl RectangularTileRegion {
    pub fn new(
        tile_id: impl Into<String>,
        color_id: TilingColorId,
        x_min: ExactRational,
        x_max: ExactRational,
        y_min: ExactRational,
        y_max: ExactRational,
    ) -> Result<Self, TilingGeometryError> {
        if x_min >= x_max || y_min >= y_max {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "rectangular_tile_extent",
            }
            .into());
        }
        Ok(Self {
            tile_id: TilingTileId::new(tile_id)?,
            color_id,
            x_min,
            x_max,
            y_min,
            y_max,
            boundary_ownership: None,
        })
    }

    pub fn with_boundary_ownership(mut self, policy: BoundaryOwnershipPolicy) -> Self {
        self.boundary_ownership = Some(policy);
        self
    }

    pub fn tile_id(&self) -> &TilingTileId {
        &self.tile_id
    }

    pub fn color_id(&self) -> &TilingColorId {
        &self.color_id
    }

    pub fn boundary_ownership(&self) -> Option<&BoundaryOwnershipPolicy> {
        self.boundary_ownership.as_ref()
    }

    pub(crate) fn overlaps_interior(&self, other: &Self) -> bool {
        self.x_min < other.x_max
            && other.x_min < self.x_max
            && self.y_min < other.y_max
            && other.y_min < self.y_max
    }

    pub(crate) fn has_closed_boundary_intersection(&self, other: &Self) -> bool {
        let both_closed = self
            .boundary_ownership()
            .is_some_and(|policy| matches!(policy.kind(), BoundaryOwnershipKind::OwnedClosed))
            && other
                .boundary_ownership()
                .is_some_and(|policy| matches!(policy.kind(), BoundaryOwnershipKind::OwnedClosed));
        both_closed
            && self.x_min <= other.x_max
            && other.x_min <= self.x_max
            && self.y_min <= other.y_max
            && other.y_min <= self.y_max
    }

    pub(crate) fn to_screening_region(
        &self,
    ) -> Result<ScreeningRectangularRegion, TilingGeometryError> {
        ScreeningRectangularRegion::new(
            self.tile_id.as_str(),
            exact_to_screening_rational(&self.x_min)?,
            exact_to_screening_rational(&self.x_max)?,
            exact_to_screening_rational(&self.y_min)?,
            exact_to_screening_rational(&self.y_max)?,
        )
        .map_err(Into::into)
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}",
            self.tile_id.as_str(),
            self.color_id.as_str(),
            self.x_min.stable_token(),
            self.x_max.stable_token(),
            self.y_min.stable_token(),
            self.y_max.stable_token(),
            self.boundary_ownership
                .as_ref()
                .map(BoundaryOwnershipPolicy::stable_token)
                .unwrap_or_else(|| "missing_boundary".to_string())
        )
    }
}

fn exact_to_screening_rational(
    value: &ExactRational,
) -> Result<ScreeningRational, TilingGeometryError> {
    let token = value.stable_token();
    let (numerator, denominator) = token
        .split_once('/')
        .ok_or(TilingGeometryError::RationalConversion)?;
    ScreeningRational::fraction(
        numerator
            .parse()
            .map_err(|_| TilingGeometryError::RationalConversion)?,
        denominator
            .parse()
            .map_err(|_| TilingGeometryError::RationalConversion)?,
    )
    .map_err(Into::into)
}
