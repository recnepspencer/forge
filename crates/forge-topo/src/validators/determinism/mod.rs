//! Determinism validators.
//!
//! DOMAIN: Canonical ordering stability, hash stability across runs,
//! journal replay exactness, tie-breaker coverage, and stable
//! floating-point normalization rules.
//!
//! VALIDATORS (from validators.md §12):
//! - ValidateCanonicalOrderingStable
//! - ValidateHashStabilityAcrossRuns
//! - ValidateJournalReplayExactness
//! - ValidateTieBreakerCoverage
//! - ValidateStableFloatingNormalization
//!
//! DEPENDENCIES: `arena`, `handles`, `history`
