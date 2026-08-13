//! Declared correction-authority axis for application aftermath.

/// Who may produce the corrected state after a committed mutation.
///
/// This axis is declared. The published law-14 posture is never declared; it is
/// derived at installation from this value paired with a correction mechanism.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DeclaredCorrectionAuthority {
    /// The runtime alone can produce the correction.
    RuntimeAlone,
    /// Correction requires the runtime together with a distinct actor or
    /// external truth owner.
    RuntimeWithExternalOwner,
    /// No correction is possible.
    NotCorrectable,
}
