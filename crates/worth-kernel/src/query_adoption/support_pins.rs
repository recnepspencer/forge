use forge_query::facade::consumer_kit::{
    project_support_snapshot, support_pinning_contract, ForgeQueryPinnedSupportStatus,
    ForgeQueryPinnedTeachingPosture, ForgeQueryRuntimeFacadeFamily, ForgeQuerySupportPinContract,
    ForgeQuerySupportPinReport, ForgeQuerySupportPinningError, ForgeQuerySupportSnapshot,
};
use forge_query::facade::runtime::{
    ForgeQueryRuntimePublicApiContract, ForgeQueryRuntimePublicSupportMatrix,
    ForgeQueryRuntimeSupportProfile,
};

const REQUIRED_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 3] = [
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Inspect,
    ForgeQueryRuntimeFacadeFamily::SharedRead,
];

pub(super) fn current_kernel_support_snapshot() -> ForgeQuerySupportSnapshot {
    snapshot_from_profile(ForgeQueryRuntimeSupportProfile::scaffold_backend_profile())
}

pub(super) fn current_kernel_support_pin_contract(
    snapshot: &ForgeQuerySupportSnapshot,
) -> Result<ForgeQuerySupportPinContract, ForgeQuerySupportPinningError> {
    let mut builder = support_pinning_contract("worth-kernel.query-adoption.phase-three")
        .against_snapshot(snapshot)?;

    for family in REQUIRED_FAMILIES {
        builder = builder.require_family(family, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })?;
    }

    builder
        .observe_family(ForgeQueryRuntimeFacadeFamily::Replay)?
        .seal()
}

pub(super) fn evaluate_current_kernel_support_pins(
) -> Result<ForgeQuerySupportPinReport, ForgeQuerySupportPinningError> {
    let snapshot = current_kernel_support_snapshot();
    current_kernel_support_pin_contract(&snapshot)?.evaluate_snapshot(&snapshot)
}

#[cfg(test)]
fn snapshot_from_profile(profile: ForgeQueryRuntimeSupportProfile) -> ForgeQuerySupportSnapshot {
    let contract = ForgeQueryRuntimePublicApiContract::from_support_profile(&profile);
    let matrix = ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    project_support_snapshot(&matrix)
}

#[cfg(not(test))]
fn snapshot_from_profile(profile: ForgeQueryRuntimeSupportProfile) -> ForgeQuerySupportSnapshot {
    let contract = ForgeQueryRuntimePublicApiContract::from_support_profile(&profile);
    let matrix = ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    project_support_snapshot(&matrix)
}

#[cfg(test)]
mod tests {
    use forge_query::facade::consumer_kit::ForgeQuerySupportPinFindingKind;
    use forge_query::facade::runtime::ForgeQueryRuntimeFamilySupport;

    use super::*;

    #[test]
    fn kernel_support_pins_evaluate_against_real_query_snapshot() {
        let report = evaluate_current_kernel_support_pins()
            .expect("kernel support pins should evaluate through Query");

        assert!(report.satisfied());
        assert_eq!(
            report.consumer_name(),
            "worth-kernel.query-adoption.phase-three"
        );
        assert_eq!(report.requirement_count(), REQUIRED_FAMILIES.len());
        assert_eq!(report.matched_required_count(), REQUIRED_FAMILIES.len());
        assert_eq!(report.observed_count(), 1);
        assert_eq!(report.blocking_finding_count(), 0);
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn kernel_support_pin_drift_is_localized_to_write_requirement() {
        let basis = current_kernel_support_snapshot();
        let drifted = snapshot_from_profile(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
                ForgeQueryRuntimeFamilySupport::deferred(
                    ForgeQueryRuntimeFacadeFamily::Write,
                    "worth-kernel phase three drift fixture",
                ),
            ),
        );
        let contract = current_kernel_support_pin_contract(&basis)
            .expect("kernel support pin contract should seal");

        let report = contract
            .evaluate_snapshot(&drifted)
            .expect("Query support pin evaluation should report drift");

        assert!(!report.satisfied());
        assert!(report.findings().iter().any(|finding| {
            finding.family() == Some(ForgeQueryRuntimeFacadeFamily::Write)
                && finding.kind() == ForgeQuerySupportPinFindingKind::StatusMismatch
                && finding.blocking()
                && finding.expected() == Some("supported")
                && finding.found() == Some("deferred-debt")
        }));
        assert!(report.findings().iter().any(|finding| {
            finding.family() == Some(ForgeQueryRuntimeFacadeFamily::Write)
                && finding.kind() == ForgeQuerySupportPinFindingKind::LiveRowDigestMismatch
                && finding.blocking()
        }));
    }
}
