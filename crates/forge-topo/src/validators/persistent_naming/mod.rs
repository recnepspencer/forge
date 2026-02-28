//! Persistent naming and selector validators.
//!
//! DOMAIN: Persistent name uniqueness, name survival through
//! split/merge operations, selector resolution determinism,
//! and dangling name reference detection.
//!
//! VALIDATORS (from validators.md §14):
//! - ValidatePersistentNameUniqueness
//! - ValidateNameSurvivalThroughSplitMerge
//! - ValidateSelectorResolutionDeterminism
//! - ValidateNoDanglingNameReferences
//!
//! DEPENDENCIES: `arena`, `handles`, `naming`
