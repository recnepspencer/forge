use worth_ui::facade::WorthUiCapabilityReloadStatus;

use crate::manual_flow::{
    ValidationManualFlowCounterPosture, ValidationManualFlowId, ValidationManualFlowProof,
    ValidationManualFlowReplayPosture, ValidationManualFlowVisibleResult,
};
use crate::reload::{ValidationPageHostRebindEvidence, ValidationReloadEvidenceEntry};
use crate::ValidationAppProofSnapshot;

use super::evidence::{
    command_label, file_menu, header_rebind, observed_changed_facts, observed_projection_sets,
    page_host_rebind, runtime_fact_label,
};

pub(super) fn observed_for_flow(
    flow_id: ValidationManualFlowId,
    proof: &ValidationAppProofSnapshot,
) -> ValidationManualFlowProof {
    if flow_id == ValidationManualFlowId::MixedProductStorm {
        return mixed_storm_observed(proof);
    }

    let Some(latest) = proof.latest_evidence() else {
        return ValidationManualFlowProof::not_run_yet();
    };
    let (rebuilt_projections, preserved_projections) = observed_projection_sets(latest);
    ValidationManualFlowProof::new(
        observed_status(latest),
        observed_visible_result(flow_id, proof, latest),
        observed_counter_posture(flow_id, latest),
        observed_counter_details(latest),
        ValidationManualFlowReplayPosture::NotApplicable,
        observed_projection_digest(proof, latest),
        observed_changed_facts(latest),
        rebuilt_projections,
        preserved_projections,
    )
}

fn mixed_storm_observed(proof: &ValidationAppProofSnapshot) -> ValidationManualFlowProof {
    let Some(storm) = proof.mixed_reload_storm() else {
        if let Some(denial) = proof.mixed_reload_storm_denial() {
            return ValidationManualFlowProof::new(
                "Denied(StormQualification)",
                ValidationManualFlowVisibleResult::Unavailable(format!(
                    "Mixed storm unavailable: {}",
                    denial.reason()
                )),
                ValidationManualFlowCounterPosture::MixedStormUnavailable,
                "qualification failed before proof materialization",
                ValidationManualFlowReplayPosture::NotAvailable,
                "storm projection digest unavailable",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
        }
        return ValidationManualFlowProof::not_run_yet();
    };
    let posture = storm.posture();
    ValidationManualFlowProof::new(
        "MixedStorm",
        ValidationManualFlowVisibleResult::MixedStormPosture {
            activated: posture.activated_step_count(),
            equivalent: posture.equivalent_step_count(),
            denied: posture.denied_step_count(),
        },
        ValidationManualFlowCounterPosture::MixedStormReplayStable,
        format!(
            "inspected {} intersections {} rebuilds {} preserved {} denied {} rebuilt {}",
            storm.projection_counters().inspected_projection_count(),
            storm.projection_counters().dependency_intersection_count(),
            storm.projection_counters().rebuild_attempt_count(),
            storm.projection_counters().preserved_frame_count(),
            storm.projection_counters().denied_frame_count(),
            storm.projection_counters().rebuilt_frame_count(),
        ),
        ValidationManualFlowReplayPosture::ReplayAvailable,
        format!(
            "storm projection digest {}",
            storm.projection_frame_digest()
        ),
        storm
            .steps()
            .iter()
            .flat_map(|step| step.changed_facts().iter())
            .map(runtime_fact_label)
            .collect(),
        storm
            .projection_roster()
            .rebuilt_projection_ids()
            .into_iter()
            .collect(),
        storm
            .projection_roster()
            .preserved_projection_ids()
            .into_iter()
            .collect(),
    )
}

fn observed_status(entry: &ValidationReloadEvidenceEntry) -> String {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload { status, .. } => format!("{status:?}"),
        ValidationReloadEvidenceEntry::AuthoredBatchReload { runtime_change, .. } => {
            format!("{:?}", runtime_change.posture())
        }
        ValidationReloadEvidenceEntry::ThemeReload { status, .. }
        | ValidationReloadEvidenceEntry::CommandReload { status, .. }
        | ValidationReloadEvidenceEntry::ComponentReload { status, .. }
        | ValidationReloadEvidenceEntry::CommandProjectionReload { status, .. }
        | ValidationReloadEvidenceEntry::AppearanceReload { status, .. }
        | ValidationReloadEvidenceEntry::DensityReload { status, .. } => {
            family_status_label(status)
        }
        ValidationReloadEvidenceEntry::ThemeDenied(denial) => format!("{:?}", denial.reason()),
        ValidationReloadEvidenceEntry::SourceActivationDenied(stage) => format!("{stage:?}"),
        ValidationReloadEvidenceEntry::ThemeActivationDenied(stage)
        | ValidationReloadEvidenceEntry::ComponentActivationDenied(stage)
        | ValidationReloadEvidenceEntry::CommandActivationDenied(stage)
        | ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(stage)
        | ValidationReloadEvidenceEntry::AppearanceActivationDenied(stage)
        | ValidationReloadEvidenceEntry::DensityActivationDenied(stage) => format!("{stage:?}"),
        ValidationReloadEvidenceEntry::InputUnreadable(denial) => {
            format!("Unreadable({})", denial.reason())
        }
    }
}

fn family_status_label(status: &WorthUiCapabilityReloadStatus) -> String {
    match status {
        WorthUiCapabilityReloadStatus::Activated => "Activated".to_owned(),
        WorthUiCapabilityReloadStatus::EquivalentNoOp => "EquivalentNoOp".to_owned(),
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary => "ReadyForFrameBoundary".to_owned(),
        WorthUiCapabilityReloadStatus::Denied(stage) => format!("Denied({stage:?})"),
    }
}

fn observed_visible_result(
    flow_id: ValidationManualFlowId,
    proof: &ValidationAppProofSnapshot,
    latest: &ValidationReloadEvidenceEntry,
) -> ValidationManualFlowVisibleResult {
    match flow_id {
        ValidationManualFlowId::HeaderText => ValidationManualFlowVisibleResult::SaveLabel(
            command_label(proof, "validation.command.file.save"),
        ),
        ValidationManualFlowId::HeaderColor => {
            ValidationManualFlowVisibleResult::HeaderPanelFill({
                let color = proof.header().applied_style().panel_fill();
                format!(
                    "#{:02x}{:02x}{:02x}{:02x}",
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a()
                )
            })
        }
        ValidationManualFlowId::HeaderFontSize => {
            ValidationManualFlowVisibleResult::HeaderFontSizePx(
                proof.header().applied_style().font_size_points().round() as u32,
            )
        }
        ValidationManualFlowId::DropdownRowPadding => {
            ValidationManualFlowVisibleResult::HeaderRowPaddingPx {
                vertical: proof
                    .header()
                    .applied_style()
                    .row_padding_vertical_points()
                    .round() as u32,
                horizontal: proof
                    .header()
                    .applied_style()
                    .row_padding_horizontal_points()
                    .round() as u32,
            }
        }
        ValidationManualFlowId::DropdownContainerPadding => {
            let margin = proof.header().applied_style().container_margin();
            ValidationManualFlowVisibleResult::HeaderContainerPaddingPx {
                top: margin.top as i8,
                right: margin.right as i8,
                bottom: margin.bottom as i8,
                left: margin.left as i8,
            }
        }
        ValidationManualFlowId::DropdownShadow => ValidationManualFlowVisibleResult::HeaderShadow(
            proof.page_slot_interaction().shadow_summary().to_owned(),
        ),
        ValidationManualFlowId::SingleToMultiMode => {
            ValidationManualFlowVisibleResult::FileMenuSelectionMode(format!(
                "{:?}",
                file_menu(proof)
                    .map(|menu| menu.selection_mode())
                    .unwrap_or(worth_ui::facade::CommandProjectionSelectionMode::SingleSelect)
            ))
        }
        ValidationManualFlowId::MultiToSingleReconciliation => {
            ValidationManualFlowVisibleResult::FileMenuReconciliation(
                file_menu(proof)
                    .map(|menu| menu.selection_reconciliation_status())
                    .unwrap_or(worth_ui::facade::WorthUiDropdownSelectionStateStatus::Empty),
            )
        }
        ValidationManualFlowId::ComponentDescriptor => {
            let component = observed_changed_facts(latest)
                .into_iter()
                .find(|fact| fact.contains("Component("))
                .unwrap_or_else(|| "Component change not visible".to_owned());
            ValidationManualFlowVisibleResult::ComponentFactChanged(
                component
                    .trim_start_matches("Component(")
                    .trim_end_matches(')')
                    .to_owned(),
            )
        }
        ValidationManualFlowId::PageSlotReassignment => {
            changed_fact_result(latest, "PrimitiveInteraction(")
        }
        ValidationManualFlowId::LayoutGap => changed_fact_result(latest, "LayoutGap("),
        ValidationManualFlowId::ThreadInset => changed_fact_result(latest, "LayoutPadding("),
        ValidationManualFlowId::InvalidAppearanceDenial => {
            ValidationManualFlowVisibleResult::PreservedHeaderFontSizePx(
                proof.header().applied_style().font_size_points().round() as u32,
            )
        }
        ValidationManualFlowId::EquivalentCanonicalAppearance => {
            ValidationManualFlowVisibleResult::HeaderMenuMinWidthPx(
                proof
                    .header()
                    .applied_style()
                    .menu_min_width_points()
                    .round() as u32,
            )
        }
        ValidationManualFlowId::MixedProductStorm => unreachable!("handled separately"),
    }
}

fn observed_counter_posture(
    flow_id: ValidationManualFlowId,
    entry: &ValidationReloadEvidenceEntry,
) -> ValidationManualFlowCounterPosture {
    let page_host = page_host_rebind(entry);
    if flow_id == ValidationManualFlowId::PageSlotReassignment {
        let Some(header) = header_rebind(entry) else {
            return ValidationManualFlowCounterPosture::NoVisibleRebindReceipts;
        };
        let Some(page_host) = page_host else {
            return ValidationManualFlowCounterPosture::NoVisibleRebindReceipts;
        };
        return match (
            header.rebuild_attempt_count() > 0,
            page_host.rebuild_attempt_count() > 0,
        ) {
            (false, false) => ValidationManualFlowCounterPosture::HeaderPreservedPageHostPreserved,
            (false, true) => ValidationManualFlowCounterPosture::HeaderPreservedPageHostRebuilt,
            (true, false) => ValidationManualFlowCounterPosture::HeaderRebuiltPageHostPreserved,
            (true, true) => ValidationManualFlowCounterPosture::HeaderRebuiltPageHostRebuilt,
        };
    }
    let Some(header) = header_rebind(entry) else {
        return if page_host.is_some_and(|receipt| receipt.rebuild_attempt_count() > 0) {
            ValidationManualFlowCounterPosture::HeaderPreservedPageHostRebuilt
        } else {
            ValidationManualFlowCounterPosture::NoVisibleRebindReceipts
        };
    };
    match (
        header.rebuild_attempt_count() > 0,
        page_host.map(|receipt| receipt.rebuild_attempt_count() > 0),
    ) {
        (true, Some(false)) => ValidationManualFlowCounterPosture::HeaderRebuiltPageHostPreserved,
        (false, Some(true)) => ValidationManualFlowCounterPosture::HeaderPreservedPageHostRebuilt,
        (false, Some(false))
            if header.status()
                == worth_ui::facade::WorthUiHeaderFrameRebindStatus::PreservedDeniedReload =>
        {
            ValidationManualFlowCounterPosture::HeaderPreservedDeniedPageHostPreservedDenied
        }
        (false, _)
            if header.status()
                == worth_ui::facade::WorthUiHeaderFrameRebindStatus::PreservedEquivalentReload =>
        {
            ValidationManualFlowCounterPosture::HeaderPreservedEquivalentNoRebuild
        }
        (true, Some(true)) => ValidationManualFlowCounterPosture::HeaderRebuiltPageHostRebuilt,
        _ => ValidationManualFlowCounterPosture::VisibleRebindPostureMixed,
    }
}

fn observed_counter_details(entry: &ValidationReloadEvidenceEntry) -> String {
    let Some(header) = header_rebind(entry) else {
        return "header/page-host rebind receipts unavailable".to_owned();
    };
    let page_host = page_host_rebind(entry);
    format!(
        "header {:?} rebuilds {} intersections {} preserved {} denied {} rebuilt {}; page-host {}",
        header.status(),
        header.rebuild_attempt_count(),
        header.dependency_intersection_count(),
        header.preserved_frame_count(),
        header.denied_frame_count(),
        header.rebuilt_frame_count(),
        page_host
            .map(page_host_counter_details)
            .unwrap_or_else(|| "none".to_owned())
    )
}

fn page_host_counter_details(receipt: &ValidationPageHostRebindEvidence) -> String {
    format!(
        "{:?} rebuilds {} intersections {} preserved {} denied {} rebuilt {}",
        receipt.status(),
        receipt.rebuild_attempt_count(),
        receipt.dependency_intersection_count(),
        receipt.preserved_frame_count(),
        receipt.denied_frame_count(),
        receipt.rebuilt_frame_count()
    )
}

fn changed_fact_result(
    entry: &ValidationReloadEvidenceEntry,
    prefix: &str,
) -> ValidationManualFlowVisibleResult {
    let fact = observed_changed_facts(entry)
        .into_iter()
        .find(|fact| fact.starts_with(prefix))
        .unwrap_or_else(|| format!("{prefix}not-visible)"));
    ValidationManualFlowVisibleResult::ChangedFact(fact)
}

fn observed_projection_digest(
    proof: &ValidationAppProofSnapshot,
    entry: &ValidationReloadEvidenceEntry,
) -> String {
    let header_digest = header_rebind(entry)
        .map(|receipt| {
            format!(
                "{} -> {}",
                receipt.previous_frame_digest(),
                receipt.rebound_frame_digest()
            )
        })
        .unwrap_or_else(|| proof.header().frame_digest().to_string());
    let page_host_digest = page_host_rebind(entry)
        .map(|receipt| {
            format!(
                "{} -> {}",
                receipt.previous_frame_digest(),
                receipt.rebound_frame_digest()
            )
        })
        .unwrap_or_else(|| proof.product_summary().page_host_frame_digest().to_string());
    format!("header {header_digest}; page-host {page_host_digest}")
}
