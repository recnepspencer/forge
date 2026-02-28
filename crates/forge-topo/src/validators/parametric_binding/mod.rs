//! Parametric (NURBS) binding invariant validators.
//!
//! DOMAIN: Curve-to-edge and PCurve-to-coedge binding existence,
//! trim loop closure in UV space, sense consistency, UV domain
//! checks with period handling, seam crossing accounting,
//! shared-edge dual trim compatibility, and curve-surface inversion
//! residual checking.
//!
//! VALIDATORS (from validators.md §9):
//! - ValidateEveryCoedgeHasPCurveWhenRequired
//! - ValidatePCurveSenseMatchesCoedgeSense
//! - ValidateTrimLoopClosureInUV
//! - ValidateTrimLoopIsSimpleInUV
//! - ValidateUVInDomainWithPeriodHandling
//! - ValidateSeamCrossingAccounting
//! - ValidateConsistentSharedEdgeDualTrims
//! - ValidateCurveSurfaceInversionResiduals
//!
//! DEPENDENCIES: `arena`, `handles`, `forge-geom` (curve/surface)
