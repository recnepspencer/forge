//! Boolean feature contracts.
//!
//! DOMAIN: Feature pipeline contract declarations for the two boolean
//! features — ParametricBooleanFeature (split-classify-stitch) and
//! EmberBooleanFeature (BSP merge).

use crate::engine::contract::{AuditLevel, EntityOriginKind, InvariantKind};
use forge_core::PolicyKind;

/// Parametric boolean: split → classify → select → assemble → postprocess.
///
/// Handles all geometry types (planar, NURBS, analytic). This is the
/// general-purpose boolean pipeline that will scale to curved geometry.
pub struct ParametricBooleanContract;

impl ParametricBooleanContract {
    /// Feature kind identifier.
    pub fn feature_kind() -> &'static str {
        "parametric_boolean"
    }

    /// Policies this feature requires to be configured.
    pub fn required_policies() -> &'static [PolicyKind] {
        &[
            PolicyKind::CoincidentGeometry,
            PolicyKind::NearTangency,
            PolicyKind::SliverFace,
        ]
    }

    /// Entity origin kinds produced by this feature.
    pub fn entity_origins() -> &'static [EntityOriginKind] {
        &[
            EntityOriginKind::SplitOperator,
            EntityOriginKind::CopyOperator,
            EntityOriginKind::MergeOperator,
        ]
    }

    /// Post-execution invariants to validate.
    pub fn post_invariants() -> &'static [InvariantKind] {
        &[InvariantKind::ManifoldEdges, InvariantKind::NoSliverFaces]
    }

    /// Audit level for tracing/decision logging.
    pub fn audit_level() -> AuditLevel {
        AuditLevel::Full
    }
}

/// EMBER boolean: BSP convert → merge → extract → mesh.
///
/// Fast exact pipeline for planar polyhedra only. Falls back to
/// parametric for curved geometry.
pub struct EmberBooleanContract;

impl EmberBooleanContract {
    /// Feature kind identifier.
    pub fn feature_kind() -> &'static str {
        "boolean::ember"
    }

    /// Policies this feature requires to be configured.
    pub fn required_policies() -> &'static [PolicyKind] {
        &[PolicyKind::CoincidentGeometry]
    }

    /// Entity origin kinds produced by this feature.
    pub fn entity_origins() -> &'static [EntityOriginKind] {
        &[EntityOriginKind::TopoOperator]
    }

    /// Post-execution invariants to validate.
    pub fn post_invariants() -> &'static [InvariantKind] {
        &[InvariantKind::ManifoldEdges]
    }

    /// Audit level for tracing/decision logging.
    pub fn audit_level() -> AuditLevel {
        AuditLevel::Summary
    }
}
