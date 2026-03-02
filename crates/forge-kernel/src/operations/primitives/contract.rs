//! Primitive feature contract and typed inputs.
//!
//! DOMAIN: Declares what the primitive feature requires (policies, Euler ops,
//! surface types) and validates its inputs.

use forge_core::KernelError;

use crate::engine::facade::{
    AuditLevel, EntityOriginKind, EulerOpKind, FeatureInputs, InvariantKind, SurfaceKind,
};

use super::MakePrimitiveFeature;

// ── Feature Contract ─────────────────────────────────────────────────────

crate::declare_feature!(MakePrimitiveFeature,
    kind: "make_primitive",
    policies: [],
    origins: [EntityOriginKind::DirectInsertion],
    euler_ops: [EulerOpKind::DirectInsertion],
    surfaces: [SurfaceKind::Planar],
    invariants: [InvariantKind::ManifoldEdges],
    audit: AuditLevel::Summary,
    persistent: true,
);

// ── Typed Inputs ─────────────────────────────────────────────────────────

/// Typed inputs for primitive creation — empty (root feature, no dependencies).
pub struct PrimitiveInputs;

impl FeatureInputs for PrimitiveInputs {
    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}
