use std::collections::BTreeSet;

use worth_ui::facade::{WorthUiHeaderFrameRebindStatus, WorthUiPageHostRebindStatus};

use crate::manual_flow::{
    ValidationManualFlowExpectation, ValidationManualFlowId, ValidationManualFlowProof,
};
use crate::ValidationAppProofSnapshot;

use super::evidence::{header_rebind, page_host_rebind};

pub(super) fn flow_expectation_matches(
    flow_id: ValidationManualFlowId,
    expectation: ValidationManualFlowExpectation,
    observed: &ValidationManualFlowProof,
    proof: &ValidationAppProofSnapshot,
    was_last_run: bool,
) -> bool {
    was_last_run
        && observed.status() == expectation.status()
        && observed.visible_result_label() == expectation.visible_result()
        && observed.counter_posture_label() == expectation.counter_posture()
        && observed.replay_posture_label() == expectation.replay_posture()
        && changed_facts_match(
            flow_id,
            expectation.changed_facts(),
            observed.changed_facts(),
        )
        && contains_expected(
            expectation.rebuilt_projections(),
            observed.rebuilt_projections(),
        )
        && contains_expected(
            expectation.preserved_projections(),
            observed.preserved_projections(),
        )
        && projection_digest_matches(flow_id, observed.projection_digest(), proof)
        && counter_posture_matches(flow_id, proof)
}

fn counter_posture_matches(
    flow_id: ValidationManualFlowId,
    proof: &ValidationAppProofSnapshot,
) -> bool {
    if flow_id == ValidationManualFlowId::MixedProductStorm {
        let Some(storm) = proof.mixed_reload_storm() else {
            return false;
        };
        let counters = storm.projection_counters();
        let posture = storm.posture();
        return posture.is_mixed()
            && counters.rebuild_attempt_count() == counters.dependency_intersection_count()
            && counters.rebuilt_frame_count() == counters.rebuild_attempt_count();
    }

    let Some(entry) = proof.latest_evidence() else {
        return false;
    };
    match flow_id {
        ValidationManualFlowId::HeaderText
        | ValidationManualFlowId::HeaderColor
        | ValidationManualFlowId::HeaderFontSize
        | ValidationManualFlowId::DropdownRowPadding
        | ValidationManualFlowId::DropdownContainerPadding
        | ValidationManualFlowId::DropdownShadow
        | ValidationManualFlowId::SingleToMultiMode
        | ValidationManualFlowId::MultiToSingleReconciliation => {
            let (Some(header), Some(page_host)) = (header_rebind(entry), page_host_rebind(entry))
            else {
                return false;
            };
            header.rebuild_attempt_count() > 0
                && header.dependency_intersection_count() >= header.rebuild_attempt_count()
                && matches!(
                    page_host.status(),
                    WorthUiPageHostRebindStatus::EquivalentAfterActivation
                        | WorthUiPageHostRebindStatus::ReboundAfterActivation
                )
        }
        ValidationManualFlowId::ComponentDescriptor => {
            let (Some(header), Some(page_host)) = (header_rebind(entry), page_host_rebind(entry))
            else {
                return false;
            };
            header.dependency_intersection_count() >= header.rebuild_attempt_count()
                && matches!(
                    page_host.status(),
                    WorthUiPageHostRebindStatus::EquivalentAfterActivation
                        | WorthUiPageHostRebindStatus::ReboundAfterActivation
                )
        }
        ValidationManualFlowId::LayoutGap | ValidationManualFlowId::ThreadInset => {
            let Some(page_host) = page_host_rebind(entry) else {
                return false;
            };
            page_host.rebuild_attempt_count() > 0
                && matches!(
                    page_host.status(),
                    WorthUiPageHostRebindStatus::EquivalentAfterActivation
                        | WorthUiPageHostRebindStatus::ReboundAfterActivation
                )
        }
        ValidationManualFlowId::PageSlotReassignment => {
            let Some(page_host) = page_host_rebind(entry) else {
                return false;
            };
            matches!(
                page_host.status(),
                WorthUiPageHostRebindStatus::EquivalentAfterActivation
                    | WorthUiPageHostRebindStatus::ReboundAfterActivation
            )
        }
        ValidationManualFlowId::InvalidAppearanceDenial => {
            let (Some(header), Some(page_host)) = (header_rebind(entry), page_host_rebind(entry))
            else {
                return false;
            };
            header.status() == WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
                && page_host.status() == WorthUiPageHostRebindStatus::PreservedDeniedReload
        }
        ValidationManualFlowId::EquivalentCanonicalAppearance => {
            let Some(header) = header_rebind(entry) else {
                return false;
            };
            header.status() == WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload
                && header.rebuild_attempt_count() == 0
        }
        ValidationManualFlowId::MixedProductStorm => unreachable!("handled above"),
    }
}

fn exact_match(expected: &[&str], observed: &[String]) -> bool {
    let expected = expected
        .iter()
        .map(|entry| (*entry).to_owned())
        .collect::<BTreeSet<_>>();
    let observed = observed.iter().cloned().collect::<BTreeSet<_>>();
    expected == observed
}

fn changed_facts_match(
    flow_id: ValidationManualFlowId,
    expected: &[&str],
    observed: &[String],
) -> bool {
    if flow_id == ValidationManualFlowId::MixedProductStorm {
        contains_expected(expected, observed)
    } else {
        exact_match(expected, observed)
    }
}

fn projection_digest_matches(
    flow_id: ValidationManualFlowId,
    observed_projection_digest: &str,
    proof: &ValidationAppProofSnapshot,
) -> bool {
    match flow_id {
        ValidationManualFlowId::MixedProductStorm => {
            proof.mixed_reload_storm().is_some_and(|storm| {
                observed_projection_digest
                    == format!(
                        "storm projection digest {}",
                        storm.projection_frame_digest()
                    )
            })
        }
        _ => {
            let Some(entry) = proof.latest_evidence() else {
                return false;
            };
            let expected_header = header_rebind(entry)
                .map(|receipt| {
                    format!(
                        "{} -> {}",
                        receipt.previous_frame_digest(),
                        receipt.rebound_frame_digest()
                    )
                })
                .unwrap_or_else(|| proof.header().frame_digest().to_string());
            let expected_page_host = page_host_rebind(entry)
                .map(|receipt| {
                    format!(
                        "{} -> {}",
                        receipt.previous_frame_digest(),
                        receipt.rebound_frame_digest()
                    )
                })
                .unwrap_or_else(|| proof.product_summary().page_host_frame_digest().to_string());
            observed_projection_digest
                == format!("header {expected_header}; page-host {expected_page_host}")
        }
    }
}

fn contains_expected(expected: &[&str], observed: &[String]) -> bool {
    let expected = expected
        .iter()
        .map(|entry| (*entry).to_owned())
        .collect::<BTreeSet<_>>();
    let observed = observed.iter().cloned().collect::<BTreeSet<_>>();
    expected.iter().all(|entry| observed.contains(entry))
}
