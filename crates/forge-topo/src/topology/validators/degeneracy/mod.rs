//! Degeneracy classification and "binary line" validators.
//!
//! DOMAIN: Policy consistency for degenerate entities, robust
//! area/volume computation paths, zero-length edge and zero-area
//! face detection, and singularity encoding consistency.
//!
//! VALIDATORS (from validators.md §8):
//! - ValidateDegeneracyPolicyConsistency
//! - ValidateAreaVolumeComputationRobust
//! - ValidateNoZeroLengthEdgesUnlessMarkedDegenerate
//! - ValidateNoZeroAreaFacesUnlessMarkedDegenerate
//! - ValidateSingularityEncodingConsistency
//!
//! DEPENDENCIES: `arena`, `handles`
