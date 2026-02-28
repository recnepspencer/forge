//! Import sanity and "soup recovery" validators.
//!
//! DOMAIN: Importer wiring checks (twins, loops, senses),
//! duplicate coincident entity detection per tolerance policy,
//! missing trim inference completeness, seam rebuild consistency,
//! and cleanup-level determinism.
//!
//! VALIDATORS (from validators.md §15):
//! - ValidateImporterWiring
//! - ValidateNoDuplicateCoincidentEntities
//! - ValidateMissingTrimInferenceCompleteness
//! - ValidateSeamRebuildConsistency
//! - ValidateCleanupLevelDeterminism
//!
//! DEPENDENCIES: `arena`, `handles`, `forge-geom` (curve/surface)
