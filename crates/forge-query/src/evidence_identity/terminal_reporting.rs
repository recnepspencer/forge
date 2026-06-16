//! Crate-internal terminal reporting labels.
//!
//! Authority-bearing production paths must compose, compare, admit, route, and
//! check coherence through typed identities — not through these projections.

use super::ForgeQueryEvidenceIdentity;

pub(crate) fn evidence_identity_reporting_label(
    identity: &ForgeQueryEvidenceIdentity,
) -> &str {
    identity.reporting_projection()
}
