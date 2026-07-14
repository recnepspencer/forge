//! Legacy-reference rename ratchet: committed snapshot, governed inventory, shrink-only law.
//!
//! Authority separation (typed facts, not final diagnostics, until classification):
//! - [`occurrence`] owns the neutral occurrence identity
//! - [`snapshot`] owns candidate baseline parsing and structural validation issues
//! - [`inventory`] owns governed-tree evidence and boundary observations
//! - [`classify`] owns empty-inception comparison and sole diagnostic projection
//!
//! This module only orchestrates those named steps.

mod classify;
mod inventory;
mod occurrence;
mod snapshot;

use crate::config::LegacyReferenceRatchetConfig;
use crate::diagnostics::Diagnostic;
use std::path::Path;

/// Validate the legacy-reference ratchet through named authority steps.
///
/// Authority model (Phase 1 empty-inception freeze, non-self-authorizing):
/// - The admitted shrink ceiling is the empty set.
/// - Observed inventory is complete scan evidence of governed surfaces.
/// - Candidate snapshot must equal observed inventory and both must be empty.
pub(crate) fn validate_legacy_references(
    root: &Path,
    config: &LegacyReferenceRatchetConfig,
) -> Result<Vec<Diagnostic>, String> {
    let snapshot_path = root.join(&config.snapshot);
    let candidate = match snapshot::load_and_validate_candidate(&snapshot_path, &config.snapshot)? {
        snapshot::CandidateValidation::Issues(issues) => {
            return Ok(classify::project_snapshot_issues(&issues));
        }
        snapshot::CandidateValidation::Valid(candidate) => candidate,
    };

    let observed = inventory::collect_governed_inventory(root, config)?;
    Ok(classify::classify_empty_inception_ratchet(
        config, &candidate, &observed,
    ))
}
