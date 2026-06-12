use std::collections::BTreeMap;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::artifact_core;

use super::canonical_geometry_digest::cell_payload;
use super::rectangular_regions::RectangularTileRegion;
use super::tiling_geometry_errors::TilingGeometryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingCell {
    core: HadwigerArtifactCore,
    cell_id: String,
    tiles: Vec<RectangularTileRegion>,
    index: TilingCellIndex,
}

impl TilingCell {
    pub fn builder(cell_id: impl Into<String>) -> TilingCellBuilder {
        TilingCellBuilder {
            cell_id: cell_id.into(),
            tiles: Vec::new(),
        }
    }

    pub(crate) fn checked(
        cell_id: String,
        tiles: Vec<RectangularTileRegion>,
    ) -> Result<Self, TilingGeometryError> {
        let index = TilingCellIndex::new(&tiles)?;
        let core = artifact_core(
            HadwigerArtifactKind::TilingCell,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "tiling_cell_geometry".to_string(),
            },
            Vec::new(),
            cell_payload(&cell_id, &tiles),
        )?;
        Ok(Self {
            core,
            cell_id,
            tiles,
            index,
        })
    }

    pub fn cell_id(&self) -> &str {
        &self.cell_id
    }

    pub fn tiles(&self) -> &[RectangularTileRegion] {
        &self.tiles
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    pub(crate) fn require_tile(
        &self,
        tile_id: &str,
    ) -> Result<&RectangularTileRegion, TilingGeometryError> {
        self.index
            .tile_positions
            .get(tile_id)
            .map(|index| &self.tiles[*index])
            .ok_or_else(|| TilingGeometryError::MissingTile {
                tile_id: tile_id.to_string(),
            })
    }
}

impl_hadwiger_artifact!(TilingCell, core);

#[derive(Clone, Debug)]
pub struct TilingCellBuilder {
    cell_id: String,
    tiles: Vec<RectangularTileRegion>,
}

impl TilingCellBuilder {
    pub fn with_rectangular_tile(
        mut self,
        tile: RectangularTileRegion,
    ) -> Result<Self, TilingGeometryError> {
        if self
            .tiles
            .iter()
            .any(|existing| existing.tile_id().as_str() == tile.tile_id().as_str())
        {
            return Err(TilingGeometryError::DuplicateTile {
                tile_id: tile.tile_id().as_str().to_string(),
            });
        }
        self.tiles.push(tile);
        self.tiles.sort_by_key(RectangularTileRegion::stable_token);
        Ok(self)
    }

    pub fn finish(self) -> Result<TilingCell, TilingGeometryError> {
        let cell_id = require_non_empty(self.cell_id, "cell_id")?;
        if self.tiles.is_empty() {
            return Err(
                crate::domain_artifacts::HadwigerArtifactShapeError::EmptyField {
                    field: "tiling_cell_tiles",
                }
                .into(),
            );
        }
        for tile in &self.tiles {
            if tile.boundary_ownership().is_none() {
                return Err(TilingGeometryError::MissingBoundaryOwnership {
                    tile_id: tile.tile_id().as_str().to_string(),
                });
            }
        }
        reject_ambiguous_rectangular_geometry(&self.tiles)?;
        TilingCell::checked(cell_id, self.tiles)
    }
}

fn reject_ambiguous_rectangular_geometry(
    tiles: &[RectangularTileRegion],
) -> Result<(), TilingGeometryError> {
    for left_index in 0..tiles.len() {
        for right_index in (left_index + 1)..tiles.len() {
            let left = &tiles[left_index];
            let right = &tiles[right_index];
            if left.overlaps_interior(right) || left.has_closed_boundary_intersection(right) {
                return Err(TilingGeometryError::AmbiguousBoundaryOwnership {
                    tile_id: format!("{}|{}", left.tile_id().as_str(), right.tile_id().as_str()),
                });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TilingCellIndex {
    tile_positions: BTreeMap<String, usize>,
}

impl TilingCellIndex {
    fn new(tiles: &[RectangularTileRegion]) -> Result<Self, TilingGeometryError> {
        let mut tile_positions = BTreeMap::new();
        for (index, tile) in tiles.iter().enumerate() {
            if tile_positions
                .insert(tile.tile_id().as_str().to_string(), index)
                .is_some()
            {
                return Err(TilingGeometryError::DuplicateTile {
                    tile_id: tile.tile_id().as_str().to_string(),
                });
            }
        }
        Ok(Self { tile_positions })
    }
}
