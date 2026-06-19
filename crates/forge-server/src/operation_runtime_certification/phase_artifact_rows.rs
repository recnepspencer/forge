use crate::{
    operation_runtime_certification::ForgeServerProductEditorReadinessCertification,
    ForgeServerOperationFamily, ForgeServerRouteInventory,
};

use super::{
    ForgeServerProductOperationRuntimeRequirementRow,
    ForgeServerProductOperationRuntimeRequirementStatus,
};

pub(super) fn authority_footprint_requirement_row(
    route_inventory: &ForgeServerRouteInventory,
) -> ForgeServerProductOperationRuntimeRequirementRow {
    let product_routes = route_inventory
        .rows()
        .iter()
        .filter(|row| row.operation_name().is_some())
        .collect::<Vec<_>>();
    let ready = product_routes.iter().all(|row| {
        matches!(
            row.operation_family(),
            Some(
                ForgeServerOperationFamily::ProductApplicationRead
                    | ForgeServerOperationFamily::ProductApplicationMutation
                    | ForgeServerOperationFamily::ProductSessionCoordination
            )
        )
    });
    let digest = product_routes
        .iter()
        .map(|row| {
            format!(
                "{}:{:?}:{:?}",
                row.operation_name().unwrap_or("missing"),
                row.operation_family(),
                row.surface_family()
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    requirement_row(
        "authority-footprint",
        ready,
        digest,
        "semantic routes must close over generic product authority families only",
    )
}

pub(super) fn authorization_posture_requirement_row(
    route_inventory: &ForgeServerRouteInventory,
) -> ForgeServerProductOperationRuntimeRequirementRow {
    let product_routes = route_inventory
        .rows()
        .iter()
        .filter(|row| row.operation_name().is_some())
        .collect::<Vec<_>>();
    let ready = product_routes.iter().all(|row| {
        row.diagnostics_policy() == "request-context-resolved"
            && row.evidence_policy() == "runtime-derived"
    });
    let digest = product_routes
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}",
                row.operation_name().unwrap_or("missing"),
                row.diagnostics_policy(),
                row.evidence_policy()
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    requirement_row(
        "authorization-posture",
        ready,
        digest,
        "semantic routes must preserve request-context authorization and runtime-derived evidence posture",
    )
}

pub(super) fn support_posture_requirement_row(
    route_inventory: &ForgeServerRouteInventory,
) -> ForgeServerProductOperationRuntimeRequirementRow {
    let product_routes = route_inventory
        .rows()
        .iter()
        .filter(|row| row.operation_name().is_some())
        .collect::<Vec<_>>();
    let ready = product_routes.iter().all(|row| {
        row.support_row()
            .map(|support_row| !support_row.trim().is_empty())
            == Some(true)
    });
    let digest = product_routes
        .iter()
        .map(|row| {
            format!(
                "{}:{}",
                row.operation_name().unwrap_or("missing"),
                row.support_row().unwrap_or("missing")
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    requirement_row(
        "support-posture",
        ready,
        digest,
        "declared support rows must pass through every semantic route intact",
    )
}

pub(super) fn precondition_posture_requirement_row(
    editor_readiness: &ForgeServerProductEditorReadinessCertification,
) -> ForgeServerProductOperationRuntimeRequirementRow {
    let missing_labels = editor_readiness.missing_proof_labels();
    let ready = !missing_labels.iter().any(|label| {
        label == "pressure-shape" || label == "stale-apply-denial" || label == "idempotent-replay"
    });
    requirement_row(
        "precondition-posture",
        ready,
        editor_readiness.canonical_digest(),
        "fixture must prove basis-bound reads, stale denial, and idempotent replay through server-owned contracts",
    )
}

pub(super) fn requirement_row(
    artifact_name: &str,
    ready: bool,
    digest: impl Into<String>,
    detail: impl Into<String>,
) -> ForgeServerProductOperationRuntimeRequirementRow {
    ForgeServerProductOperationRuntimeRequirementRow::new(
        artifact_name,
        if ready {
            ForgeServerProductOperationRuntimeRequirementStatus::Ready
        } else {
            ForgeServerProductOperationRuntimeRequirementStatus::Blocked
        },
        digest,
        detail,
    )
}
