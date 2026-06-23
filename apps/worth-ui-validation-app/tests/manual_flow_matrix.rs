use worth_ui::facade::{WorthUiHeaderFrameRebindStatus, WorthUiPageHostRebindStatus};
use worth_ui_validation_app::reload::{
    ValidationHeaderRebindEvidence, ValidationPageHostRebindEvidence, ValidationReloadEvidenceEntry,
};
use worth_ui_validation_app::{
    validation_manual_flow_catalog, ValidationAppProofSnapshot, ValidationManualFlowId,
    ValidationManualFlowVisibleRow, ValidationMixedReloadStormProof,
};

mod validation_app_reload_fixture;

use validation_app_reload_fixture::ValidationAppReloadFixture;

#[test]
fn every_manual_flow_matrix_row_has_an_automated_counterpart() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    let catalog = validation_manual_flow_catalog();

    for definition in catalog.definitions() {
        app.run_manual_flow(definition.id());
        let proof = app.proof_snapshot();
        let snapshot = app.manual_flow_matrix_snapshot();
        let row = snapshot
            .rows()
            .iter()
            .find(|row| row.flow_id() == definition.id())
            .expect("every catalog row should project into the app matrix");

        assert_manual_flow_row(row, &proof);
    }
}

#[test]
fn mixed_product_storm_row_replays_deterministically() {
    let fixture = ValidationAppReloadFixture::new();
    let mut first_app = fixture.build_app();
    let mut second_app = fixture.build_app();

    first_app.run_manual_flow(ValidationManualFlowId::MixedProductStorm);
    second_app.run_manual_flow(ValidationManualFlowId::MixedProductStorm);

    let first = first_app
        .proof_snapshot()
        .mixed_reload_storm()
        .expect("mixed storm should project a storm proof")
        .clone();
    let second = second_app
        .proof_snapshot()
        .mixed_reload_storm()
        .expect("rerun should project the same storm proof")
        .clone();

    let replay = ValidationMixedReloadStormProof::certify_replay(&first, &second)
        .expect("manual flow storm should certify replay deterministically");

    assert_eq!(first.replay_artifact(), second.replay_artifact());
    assert_eq!(
        replay.scenario_digest(),
        first.replay_artifact().scenario_digest()
    );
    assert_eq!(
        replay.projection_frame_digest(),
        first.replay_artifact().projection_frame_digest()
    );
    assert_eq!(replay.step_count(), first.steps().len());
}

fn assert_manual_flow_row(
    row: &ValidationManualFlowVisibleRow,
    proof: &ValidationAppProofSnapshot,
) {
    assert_eq!(
        row.observed_status(),
        row.expected_status(),
        "status mismatch for {}",
        row.title()
    );
    assert_eq!(
        row.observed_visible_result(),
        row.expected_visible_result(),
        "visible result mismatch for {}",
        row.title()
    );
    assert_eq!(
        row.observed_counter_posture(),
        row.expected_counter_posture(),
        "counter posture mismatch for {}",
        row.title()
    );
    assert_eq!(
        row.observed_replay_posture(),
        row.expected_replay_posture(),
        "replay posture mismatch for {}",
        row.title()
    );
    assert_changed_facts(row);
    assert_contains_all(
        row.expected_rebuilt_projections(),
        row.observed_rebuilt_projections(),
        row.title(),
        "rebuilt projections",
    );
    assert_contains_all(
        row.expected_preserved_projections(),
        row.observed_preserved_projections(),
        row.title(),
        "preserved projections",
    );
    assert_eq!(
        row.observed_projection_digest(),
        expected_projection_digest(row.flow_id(), proof),
        "projection digest mismatch for {}",
        row.title()
    );
    assert_counter_evidence(row.flow_id(), proof, row.title());
    assert_independent_runtime_evidence(row.flow_id(), proof, row.title());
}

fn assert_changed_facts(row: &ValidationManualFlowVisibleRow) {
    use std::collections::BTreeSet;

    let expected = row
        .expected_changed_facts()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let observed = row
        .observed_changed_facts()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if row.flow_id() == ValidationManualFlowId::MixedProductStorm {
        assert!(
            expected.iter().all(|fact| observed.contains(fact)),
            "mixed storm changed facts missing expected subset for {}",
            row.title()
        );
    } else {
        assert_eq!(
            observed,
            expected,
            "changed facts mismatch for {}",
            row.title()
        );
    }
}

fn assert_contains_all(expected: &[String], observed: &[String], title: &str, lane: &str) {
    use std::collections::BTreeSet;

    let expected = expected.iter().cloned().collect::<BTreeSet<_>>();
    let observed = observed.iter().cloned().collect::<BTreeSet<_>>();
    assert!(
        expected.iter().all(|entry| observed.contains(entry)),
        "{} mismatch for {}",
        lane,
        title
    );
}

fn expected_projection_digest(
    flow_id: ValidationManualFlowId,
    proof: &ValidationAppProofSnapshot,
) -> String {
    if flow_id == ValidationManualFlowId::MixedProductStorm {
        let storm = proof
            .mixed_reload_storm()
            .expect("mixed storm rows should expose storm proof");
        return format!(
            "storm projection digest {}",
            storm.projection_frame_digest()
        );
    }

    let entry = proof
        .latest_evidence()
        .expect("manual flow row should have latest evidence");
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
    format!("header {expected_header}; page-host {expected_page_host}")
}

fn assert_counter_evidence(
    flow_id: ValidationManualFlowId,
    proof: &ValidationAppProofSnapshot,
    title: &str,
) {
    if flow_id == ValidationManualFlowId::MixedProductStorm {
        let storm = proof
            .mixed_reload_storm()
            .expect("mixed storm rows should expose storm proof");
        let counters = storm.projection_counters();
        let posture = storm.posture();
        assert!(
            posture.is_mixed(),
            "mixed storm posture mismatch for {}",
            title
        );
        assert_eq!(
            counters.rebuild_attempt_count(),
            counters.dependency_intersection_count(),
            "mixed storm counter mismatch for {}",
            title
        );
        assert_eq!(
            counters.rebuilt_frame_count(),
            counters.rebuild_attempt_count(),
            "mixed storm rebuilt-frame mismatch for {}",
            title
        );
        return;
    }

    let entry = proof
        .latest_evidence()
        .expect("manual flow row should have latest evidence");
    match flow_id {
        ValidationManualFlowId::HeaderText
        | ValidationManualFlowId::HeaderColor
        | ValidationManualFlowId::HeaderFontSize
        | ValidationManualFlowId::DropdownRowPadding
        | ValidationManualFlowId::DropdownContainerPadding
        | ValidationManualFlowId::DropdownShadow
        | ValidationManualFlowId::SingleToMultiMode
        | ValidationManualFlowId::MultiToSingleReconciliation => {
            let header = header_rebind(entry).expect("header receipt required");
            let page_host = page_host_rebind(entry).expect("page-host receipt required");
            assert!(
                header.rebuild_attempt_count() > 0,
                "header rebuild evidence missing for {}",
                title
            );
            assert!(
                header.dependency_intersection_count() >= header.rebuild_attempt_count(),
                "header counter mismatch for {}",
                title
            );
            assert!(matches!(
                page_host.status(),
                WorthUiPageHostRebindStatus::EquivalentAfterActivation
                    | WorthUiPageHostRebindStatus::ReboundAfterActivation
            ));
        }
        ValidationManualFlowId::ComponentDescriptor => {
            let header = header_rebind(entry).expect("header receipt required");
            let page_host = page_host_rebind(entry).expect("page-host receipt required");
            assert!(
                header.dependency_intersection_count() >= header.rebuild_attempt_count(),
                "component counter mismatch for {}",
                title
            );
            assert!(matches!(
                page_host.status(),
                WorthUiPageHostRebindStatus::EquivalentAfterActivation
                    | WorthUiPageHostRebindStatus::ReboundAfterActivation
            ));
        }
        ValidationManualFlowId::PageSlotReassignment => {
            let page_host = page_host_rebind(entry).expect("page-host receipt required");
            assert!(matches!(
                page_host.status(),
                WorthUiPageHostRebindStatus::EquivalentAfterActivation
                    | WorthUiPageHostRebindStatus::ReboundAfterActivation
            ));
        }
        ValidationManualFlowId::LayoutGap | ValidationManualFlowId::ThreadInset => {
            let page_host = page_host_rebind(entry).expect("page-host receipt required");
            assert!(
                page_host.rebuild_attempt_count() > 0,
                "page-host rebuild evidence missing for {}",
                title
            );
            assert!(matches!(
                page_host.status(),
                WorthUiPageHostRebindStatus::EquivalentAfterActivation
                    | WorthUiPageHostRebindStatus::ReboundAfterActivation
            ));
        }
        ValidationManualFlowId::InvalidAppearanceDenial => {
            let header = header_rebind(entry).expect("header receipt required");
            let page_host = page_host_rebind(entry).expect("page-host receipt required");
            assert_eq!(
                header.status(),
                WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
            );
            assert_eq!(
                page_host.status(),
                WorthUiPageHostRebindStatus::PreservedDeniedReload
            );
        }
        ValidationManualFlowId::EquivalentCanonicalAppearance => {
            let header = header_rebind(entry).expect("header receipt required");
            assert_eq!(
                header.status(),
                WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload
            );
            assert_eq!(header.rebuild_attempt_count(), 0);
        }
        ValidationManualFlowId::MixedProductStorm => unreachable!(),
    }
}

fn assert_independent_runtime_evidence(
    flow_id: ValidationManualFlowId,
    proof: &ValidationAppProofSnapshot,
    title: &str,
) {
    if flow_id != ValidationManualFlowId::PageSlotReassignment {
        return;
    }
    assert!(
        proof.latest_evidence().is_some_and(|entry| match entry {
            ValidationReloadEvidenceEntry::RuntimeReload { changed_facts, .. } =>
                changed_facts.iter().any(|fact| {
                    fact.family()
                        == worth_ui::facade::WorthUiRuntimeFactFamily::PrimitiveInteraction
                        && fact.identity() == "worth.surface.preview.primitive.proof"
                }),
            _ => false,
        }),
        "primitive interaction evidence missing for {}",
        title
    );
}

fn header_rebind(entry: &ValidationReloadEvidenceEntry) -> Option<&ValidationHeaderRebindEvidence> {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::ThemeReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::CommandReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::ComponentReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::CommandProjectionReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::AppearanceReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::DensityReload { header_rebind, .. } => {
            header_rebind.as_ref()
        }
        _ => None,
    }
}

fn page_host_rebind(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<&ValidationPageHostRebindEvidence> {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::ThemeReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::CommandReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::ComponentReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::CommandProjectionReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::AppearanceReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::DensityReload {
            page_host_rebind, ..
        } => page_host_rebind.as_ref(),
        _ => None,
    }
}
