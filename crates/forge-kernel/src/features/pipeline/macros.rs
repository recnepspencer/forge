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
///     origins: [EntityOriginKind::EulerOperator],
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
        invariants: [$($inv:expr),* $(,)?],
        audit: $audit:expr $(,)?
    ) => {
        impl $crate::features::pipeline::contract::FeatureContract for $feature_ty {
            fn feature_kind(&self) -> &str {
                $kind
            }

            fn required_policies(&self) -> &[forge_core::PolicyKind] {
                &[$($policy),*]
            }

            fn entity_origins(&self) -> &[$crate::features::pipeline::contract::EntityOriginKind] {
                &[$($origin),*]
            }

            fn post_invariants(&self) -> &[$crate::features::pipeline::contract::InvariantKind] {
                &[$($inv),*]
            }

            fn audit_level(&self) -> $crate::features::pipeline::contract::AuditLevel {
                $audit
            }
        }
    };
}
