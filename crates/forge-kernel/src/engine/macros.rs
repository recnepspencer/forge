//! Declarative macro for implementing `FeatureContract`.
//!
//! DOMAIN: Eliminates boilerplate when declaring a feature's contract.
//! Each invocation generates a `FeatureContract` impl with compile-time
//! static slices for policies, origins, and invariants.

/// Implement `FeatureContract` for a feature struct with declarative syntax.
///
/// # Example
///
/// ```ignore
/// declare_feature!(MakeCubeFeature,
///     kind: "make_cube",
///     policies: [],
///     origins: [EntityOriginKind::TopoOperator],
///     invariants: [InvariantKind::ManifoldEdges],
///     audit: AuditLevel::Summary,
/// );
/// ```
#[macro_export]
macro_rules! declare_feature {
    ($feature_ty:ty,
        kind: $kind:expr,
        policies: [$($policy:expr),* $(,)?],
        origins: [$($origin:expr),* $(,)?],
        euler_ops: [$($euler:expr),* $(,)?],
        surfaces: [$($surface:expr),* $(,)?],
        invariants: [$($inv:expr),* $(,)?],
        audit: $audit:expr,
        persistent: $persistent:expr,
        conditioning: $conditioning:expr $(,)?
    ) => {
        impl $crate::engine::contract::FeatureContract for $feature_ty {
            fn feature_kind(&self) -> &'static str {
                $kind
            }

            fn required_policies(&self) -> &[forge_core::PolicyKind] {
                &[$($policy),*]
            }

            fn entity_origins(&self) -> &[$crate::engine::contract::EntityOriginKind] {
                &[$($origin),*]
            }

            fn euler_ops(&self) -> &[$crate::engine::contract::EulerOpKind] {
                &[$($euler),*]
            }

            fn surface_types(&self) -> &[$crate::engine::contract::SurfaceKind] {
                &[$($surface),*]
            }

            fn post_invariants(&self) -> &[$crate::engine::contract::InvariantKind] {
                &[$($inv),*]
            }

            fn audit_level(&self) -> $crate::engine::contract::AuditLevel {
                $audit
            }

            fn persistent_output(&self) -> bool {
                $persistent
            }

            fn conditioning_mode(&self) -> $crate::engine::contract::ConditioningMode {
                $conditioning
            }
        }
    };
}
