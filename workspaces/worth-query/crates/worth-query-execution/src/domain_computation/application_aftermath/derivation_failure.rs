//! Typed internal derivation failures for Phase 8 aftermath identity work.
//!
//! Consumer-visible denials remain typed kinds (`WorthQueryUndoDenialKind`,
//! `WorthQueryRedoDenialKind`, …). These variants replace `Result<_, &'static str>`
//! on digest-preparation paths so the residue cannot be rediscovered as a
//! third stringly error surface (Gate 8.5 finding / Gate 8.6 residue sweep).

/// Internal digest-preparation failure for aftermath identity derivation.
///
/// Consumer-visible denials remain the typed undo/redo/recovery kinds. This
/// enum exists so preparation failures are not `&'static str` (Gate 8.6 residue).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAftermathDerivationFailure {
    /// The Relational owner could not retain the exact current branch basis.
    RetentionCapacityExhausted,
    /// The Relational owner permanently exhausted retention identity space.
    RetentionIdentityExhausted,
    /// The Relational owner permanently exhausted snapshot identity space.
    SnapshotIdentityExhausted,
    /// Canonical basis sequence rejected the prepared entries.
    BasisRejected,
    /// Digest preparation rejected the ready sequence under budget.
    DigestRejected,
    /// Correlation basis carried an empty required text field.
    EmptyCorrelationBasis,
    /// A declared external effect did not carry its installed typed emission.
    MissingExternalPayload,
    /// The installed runtime clock could not supply the classification sample.
    RuntimeTimeUnavailable,
}
