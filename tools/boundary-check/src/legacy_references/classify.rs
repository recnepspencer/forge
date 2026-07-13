//! Ratchet classification and sole diagnostic-projection responsibility.
//!
//! Translates typed snapshot issues, boundary observations, and empty-inception
//! comparison cases into `Diagnostic` values. Does not parse snapshots or
//! traverse the filesystem.

use crate::config::LegacyReferenceRatchetConfig;
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::legacy_references::inventory::{BoundaryObservation, ObservedInventory};
use crate::legacy_references::snapshot::{SnapshotIssue, ValidatedCandidateSnapshot};

/// Project typed snapshot structural issues into baseline diagnostics.
pub(super) fn project_snapshot_issues(issues: &[SnapshotIssue]) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<_> = issues.iter().map(project_snapshot_issue).collect();
    sort_diagnostics(&mut diagnostics);
    diagnostics
}

/// Classify candidate vs observed inventory under the empty-inception freeze,
/// and project all resulting cases (including boundary observations).
pub(super) fn classify_empty_inception_ratchet(
    config: &LegacyReferenceRatchetConfig,
    candidate: &ValidatedCandidateSnapshot,
    observed: &ObservedInventory,
) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<_> = observed
        .boundary_observations
        .iter()
        .map(project_boundary_observation)
        .collect();
    diagnostics.extend(deny_candidate_growth(config, candidate));
    diagnostics.extend(deny_unbaseline_occurrences(config, candidate, observed));
    diagnostics.extend(deny_stale_candidate_rows(candidate, observed));
    sort_diagnostics(&mut diagnostics);
    diagnostics
}

fn project_snapshot_issue(issue: &SnapshotIssue) -> Diagnostic {
    match issue {
        SnapshotIssue::WrongSchemaVersion {
            snapshot_relative,
            found,
            expected,
        } => Diagnostic::new(
            DiagnosticCode::Bc6002LegacyReferenceBaseline,
            snapshot_relative.clone(),
            format!("legacy-reference snapshot schema_version must be {expected}, found {found}"),
        ),
        SnapshotIssue::DuplicateRow {
            snapshot_relative,
            path,
            location,
            fragment,
        } => Diagnostic::new(
            DiagnosticCode::Bc6002LegacyReferenceBaseline,
            snapshot_relative.clone(),
            format!("duplicate baseline row path={path} location={location} fragment={fragment}"),
        ),
    }
}

fn project_boundary_observation(observation: &BoundaryObservation) -> Diagnostic {
    match observation {
        BoundaryObservation::GovernedSymlinkOrJunction { relative_path } => Diagnostic::new(
            DiagnosticCode::Bc6002LegacyReferenceBaseline,
            relative_path.clone(),
            "governed path is a symlink or junction; legacy-reference ratchet rejects symlinks under governed roots (fail closed, no follow)".to_owned(),
        ),
    }
}

/// Phase 1 empty-inception freeze: any non-empty candidate row is growth.
///
/// Do not consult Git HEAD — in CI the candidate commit *is* HEAD and would
/// self-authorize matching source + snapshot rows.
fn deny_candidate_growth(
    config: &LegacyReferenceRatchetConfig,
    candidate: &ValidatedCandidateSnapshot,
) -> Vec<Diagnostic> {
    candidate
        .occurrences
        .iter()
        .map(|occurrence| {
            Diagnostic::new(
                DiagnosticCode::Bc6001LegacyReferenceGrowth,
                occurrence.subject(),
                format!(
                    "legacy-reference baseline growth is denied for fragment `{}`; snapshot may only shrink from the empty Phase 1 inception baseline at {}; {}",
                    occurrence.fragment, config.snapshot, config.replacement_guidance
                ),
            )
        })
        .collect()
}

fn deny_unbaseline_occurrences(
    config: &LegacyReferenceRatchetConfig,
    candidate: &ValidatedCandidateSnapshot,
    observed: &ObservedInventory,
) -> Vec<Diagnostic> {
    observed
        .occurrences
        .iter()
        .filter(|occurrence| !candidate.occurrences.contains(occurrence))
        .map(|occurrence| {
            Diagnostic::new(
                DiagnosticCode::Bc6001LegacyReferenceGrowth,
                occurrence.subject(),
                format!(
                    "retired fragment `{}` is not in the committed baseline; {}; baseline: {}",
                    occurrence.fragment, config.replacement_guidance, config.snapshot
                ),
            )
        })
        .collect()
}

fn deny_stale_candidate_rows(
    candidate: &ValidatedCandidateSnapshot,
    observed: &ObservedInventory,
) -> Vec<Diagnostic> {
    candidate
        .occurrences
        .difference(&observed.occurrences)
        .map(|stale| {
            Diagnostic::new(
                DiagnosticCode::Bc6002LegacyReferenceBaseline,
                stale.subject(),
                format!(
                    "baseline row for fragment `{}` is stale or unexpected; shrink tools/boundary-check/snapshots/legacy-references.toml to match the tree",
                    stale.fragment
                ),
            )
        })
        .collect()
}

fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|left, right| left.compare_subject_message(right));
}
