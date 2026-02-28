//! Region and cellular topology (volume) operators.
//!
//! DOMAIN: Region lifecycle, region boundary management,
//! internal wall operations, and outside region handling.
//!
//! OPERATORS (from operators-list.md §F):
//! - F1: CreateRegion, DestroyRegion, SplitRegion, MergeRegions, ExtractRegion, InsertRegion, RehomeRegion
//! - F2: BindShellToRegion, UnbindShellFromRegion, AddRegionBoundaryFace, RemoveRegionBoundaryFace, SwapRegionAdjacency, FlipRegionSense
//! - F3: InsertInternalWall, RemoveInternalWall, SplitRegionWithWall, MergeRegionsAcrossWall, ExtractWallAsSheet, ConvertWallToBoundary
//! - F4: CreateOutsideRegion, BindOutsideRegion, UnbindOutsideRegion, ReclassifyOutsideAdjacency
//!
//! DEPENDENCIES: `euler`, `arena`, `handles`
