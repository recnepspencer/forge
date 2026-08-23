use std::path::Path;

use worth_store_offline_verifier::RecoveryObserverReport;
use worth_store_recovery_runtime::{RecoveryReportEnvelope, RecoveryReportOutcome};

use super::super::super::super::{comparison, history};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RawPublicationPosture {
    ProvenNoEffect,
    PhysicalEffectObserved,
}

pub(super) fn adjudicate(
    root: &Path,
    report: &RecoveryReportEnvelope,
    before: &history::ParentPhysicalHistory,
    after: &history::ParentPhysicalHistory,
    observer: &RecoveryObserverReport,
    label: &str,
) -> RawPublicationPosture {
    validate(root, report, before, after, observer, label).unwrap_or_else(|error| panic!("{error}"))
}

pub(super) fn validate(
    root: &Path,
    report: &RecoveryReportEnvelope,
    before: &history::ParentPhysicalHistory,
    after: &history::ParentPhysicalHistory,
    observer: &RecoveryObserverReport,
    label: &str,
) -> Result<RawPublicationPosture, String> {
    let posture = if after.publication_changed_from(before) {
        RawPublicationPosture::PhysicalEffectObserved
    } else {
        RawPublicationPosture::ProvenNoEffect
    };
    match posture {
        RawPublicationPosture::ProvenNoEffect => {
            if report.outcome() != RecoveryReportOutcome::Blocked {
                return Err(format!(
                    "{label} raw publication state did not change, so the report must be Blocked"
                ));
            }
        }
        RawPublicationPosture::PhysicalEffectObserved => {
            if report.outcome() != RecoveryReportOutcome::PublicationIndeterminate {
                return Err(format!(
                    "MUTANT_PREDICATE:c8-publication-effect-adjudication {label} raw publication state changed, so the report must be PublicationIndeterminate; paths={:?}",
                    after.changed_paths_from(before)
                ));
            }
            if report.counters().recovery_effects() == 0 {
                return Err(format!(
                    "{label} changed parent history requires a performed recovery effect"
                ));
            }
        }
    }
    comparison::compare_runtime_and_observer(report, observer, after).map_err(|error| {
        format!(
            "{label} publication adjudication disagreed with runtime, observer, or raw parent history at {}: {error:?}",
            root.display()
        )
    })?;
    Ok(posture)
}
