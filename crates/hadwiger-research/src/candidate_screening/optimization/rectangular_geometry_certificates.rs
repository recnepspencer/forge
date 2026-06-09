use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

use super::{ScreeningRational, ScreeningRectangularRegion, ScreeningSolverTranscript};

macro_rules! rectangular_pair_certificate {
    ($type:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $type {
            left: ScreeningRectangularRegion,
            right: ScreeningRectangularRegion,
            solver_transcript: ScreeningSolverTranscript,
        }

        impl $type {
            pub fn new(
                left: ScreeningRectangularRegion,
                right: ScreeningRectangularRegion,
                solver_transcript: ScreeningSolverTranscript,
            ) -> Result<Self, HadwigerArtifactShapeError> {
                Ok(Self {
                    left,
                    right,
                    solver_transcript,
                })
            }

            pub(crate) fn left(&self) -> &ScreeningRectangularRegion {
                &self.left
            }

            pub(crate) fn right(&self) -> &ScreeningRectangularRegion {
                &self.right
            }

            pub fn stable_token(&self) -> String {
                format!(
                    "{}:{}:{}",
                    self.left.stable_token(),
                    self.right.stable_token(),
                    self.solver_transcript.stable_token()
                )
            }
        }
    };
}

rectangular_pair_certificate!(ExactUnitDistanceConflictCertificate);
rectangular_pair_certificate!(SameColorSeparationCertificate);
rectangular_pair_certificate!(NumericalMarginCertificate);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinkowskiUnitIntersectionCertificate {
    left: ScreeningRectangularRegion,
    right: ScreeningRectangularRegion,
    solver_transcript: ScreeningSolverTranscript,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileDiameterCertificate {
    tile: ScreeningRectangularRegion,
    solver_transcript: ScreeningSolverTranscript,
}

impl TileDiameterCertificate {
    pub fn new(
        tile: ScreeningRectangularRegion,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            tile,
            solver_transcript,
        })
    }

    pub(crate) fn tile(&self) -> &ScreeningRectangularRegion {
        &self.tile
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}",
            self.tile.stable_token(),
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactConflictGraphEdgeCertificate {
    left_tile_id: String,
    right_tile_id: String,
    left: ScreeningRectangularRegion,
    right: ScreeningRectangularRegion,
    solver_transcript: ScreeningSolverTranscript,
}

impl ExactConflictGraphEdgeCertificate {
    pub fn new(
        left_tile_id: impl Into<String>,
        right_tile_id: impl Into<String>,
        left: ScreeningRectangularRegion,
        right: ScreeningRectangularRegion,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            left_tile_id: require_non_empty(left_tile_id, "left_tile_id")?,
            right_tile_id: require_non_empty(right_tile_id, "right_tile_id")?,
            left,
            right,
            solver_transcript,
        })
    }

    pub(crate) fn left(&self) -> &ScreeningRectangularRegion {
        &self.left
    }

    pub(crate) fn right(&self) -> &ScreeningRectangularRegion {
        &self.right
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.left_tile_id,
            self.right_tile_id,
            self.left.stable_token(),
            self.right.stable_token(),
            self.solver_transcript.stable_token()
        )
    }
}

impl MinkowskiUnitIntersectionCertificate {
    pub fn new(
        left: ScreeningRectangularRegion,
        right: ScreeningRectangularRegion,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            left,
            right,
            solver_transcript,
        })
    }

    pub(crate) fn left(&self) -> &ScreeningRectangularRegion {
        &self.left
    }

    pub(crate) fn right(&self) -> &ScreeningRectangularRegion {
        &self.right
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.left.stable_token(),
            self.right.stable_token(),
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForbiddenDisplacementCertificate {
    tile: ScreeningRectangularRegion,
    dx: ScreeningRational,
    dy: ScreeningRational,
    solver_transcript: ScreeningSolverTranscript,
}

impl ForbiddenDisplacementCertificate {
    pub fn new(
        tile: ScreeningRectangularRegion,
        dx: ScreeningRational,
        dy: ScreeningRational,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            tile,
            dx,
            dy,
            solver_transcript,
        })
    }

    pub(crate) fn tile(&self) -> &ScreeningRectangularRegion {
        &self.tile
    }

    pub(crate) fn dx(&self) -> &ScreeningRational {
        &self.dx
    }

    pub(crate) fn dy(&self) -> &ScreeningRational {
        &self.dy
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.tile.stable_token(),
            self.dx.stable_token(),
            self.dy.stable_token(),
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicQuotientTile {
    tile_id: String,
    color_id: String,
    region: ScreeningRectangularRegion,
}

impl PeriodicQuotientTile {
    pub fn new(
        tile_id: impl Into<String>,
        color_id: impl Into<String>,
        region: ScreeningRectangularRegion,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            tile_id: require_non_empty(tile_id, "tile_id")?,
            color_id: require_non_empty(color_id, "color_id")?,
            region,
        })
    }

    pub(crate) fn tile_id(&self) -> &str {
        &self.tile_id
    }

    pub(crate) fn color_id(&self) -> &str {
        &self.color_id
    }

    pub(crate) fn region(&self) -> &ScreeningRectangularRegion {
        &self.region
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.tile_id,
            self.color_id,
            self.region.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicQuotientRectangleModel {
    model_id: String,
    tiles: Vec<PeriodicQuotientTile>,
}

impl PeriodicQuotientRectangleModel {
    pub fn new(
        model_id: impl Into<String>,
        mut tiles: Vec<PeriodicQuotientTile>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if tiles.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "periodic_quotient_tiles",
            });
        }
        tiles.sort_by_key(|tile| tile.stable_token());
        Ok(Self {
            model_id: require_non_empty(model_id, "model_id")?,
            tiles,
        })
    }

    pub(crate) fn tile(&self, tile_id: &str) -> Option<&PeriodicQuotientTile> {
        self.tiles.iter().find(|tile| tile.tile_id() == tile_id)
    }

    pub fn stable_token(&self) -> String {
        let mut token = self.model_id.clone();
        for tile in &self.tiles {
            token.push_str(&format!(":{}", tile.stable_token()));
        }
        token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicQuotientConflictCertificate {
    left_tile_id: String,
    right_tile_id: String,
    translation_dx: ScreeningRational,
    translation_dy: ScreeningRational,
    solver_transcript: ScreeningSolverTranscript,
}

impl PeriodicQuotientConflictCertificate {
    pub fn new(
        left_tile_id: impl Into<String>,
        right_tile_id: impl Into<String>,
        translation_dx: ScreeningRational,
        translation_dy: ScreeningRational,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            left_tile_id: require_non_empty(left_tile_id, "left_tile_id")?,
            right_tile_id: require_non_empty(right_tile_id, "right_tile_id")?,
            translation_dx,
            translation_dy,
            solver_transcript,
        })
    }

    pub(crate) fn left_tile_id(&self) -> &str {
        &self.left_tile_id
    }

    pub(crate) fn right_tile_id(&self) -> &str {
        &self.right_tile_id
    }

    pub(crate) fn translation_dx(&self) -> &ScreeningRational {
        &self.translation_dx
    }

    pub(crate) fn translation_dy(&self) -> &ScreeningRational {
        &self.translation_dy
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.left_tile_id,
            self.right_tile_id,
            self.translation_dx.stable_token(),
            self.translation_dy.stable_token(),
            self.solver_transcript.stable_token()
        )
    }
}
