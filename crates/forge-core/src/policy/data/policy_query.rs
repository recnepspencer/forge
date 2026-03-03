//! Structured policy query for Doctrine D2.

use super::policy_kind::PolicyKind;

/// A query for a policy decision (Doctrine D2).
///
/// When the kernel encounters an ambiguous situation and it is mapped
/// from a geometry-layer `AmbiguousResult`, it enters this structured
/// policy request.
#[derive(Debug, Clone)]
pub struct PolicyQuery {
    /// What kind of decision is needed
    pub kind: PolicyKind,
    /// 3D location where the ambiguity occurred
    pub location: [f64; 3],
    /// How marginal this case is (lower = closer to the boundary)
    pub margin: f64,
    /// Whether the caller can override this with a policy setting
    pub overridable: bool,
}
