use forge_query::facade::runtime::{
    admit_graph_read_access_for_family, admit_graph_read_access_for_family_in_authority,
    derive_graph_read_access_requirements, plan_admitted_graph_read_access_for_family,
    plan_admitted_graph_read_access_for_family_in_authority,
    try_derive_graph_read_access_requirements, ForgeQueryAdmittedGraphReadAccessPlan,
    ForgeQueryEphemeralGraphIndexReceipt, ForgeQueryGraphReadAccessAdmission,
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessComplexityCounters,
    ForgeQueryGraphReadAccessCostEstimate, ForgeQueryGraphReadAccessDenial,
    ForgeQueryGraphReadAccessDenialKind, ForgeQueryGraphReadAccessPlanConsumption,
    ForgeQueryGraphReadAccessReceiptSummary, ForgeQueryGraphReadAccessRequirementCounters,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadAccessRequirementRow,
    ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadCostEvidence,
    ForgeQueryGraphReadOperationCapabilityRequirementKind,
    ForgeQueryGraphReadOperationUnsupportedDenialKind, ForgeQueryGraphReadStreamingReceipt,
    ForgeQueryLiveGraphReadAccessReceipt, ForgeQueryPersistentGraphIndexRequirementRow,
};
use std::sync::OnceLock;

use super::capability_row::{
    QueryGraphReadAccessCapabilityKind, QueryGraphReadAccessCapabilityRow,
    QueryGraphReadAccessCapabilitySurface,
};
use super::cost_counters::QueryGraphReadCostCounterField;
use super::receipt_fields::QueryGraphReadReceiptField;

pub(super) fn query_graph_read_access_capability_rows(
) -> &'static [QueryGraphReadAccessCapabilityRow] {
    static CAPABILITY_ROWS: OnceLock<Vec<QueryGraphReadAccessCapabilityRow>> = OnceLock::new();
    CAPABILITY_ROWS.get_or_init(build_query_graph_read_access_capability_rows)
}

pub(super) fn anchor_query_graph_read_access_symbols() {
    let _ = derive_graph_read_access_requirements;
    let _ = try_derive_graph_read_access_requirements;
    let _ = admit_graph_read_access_for_family;
    let _ = admit_graph_read_access_for_family_in_authority;
    let _ = plan_admitted_graph_read_access_for_family;
    let _ = plan_admitted_graph_read_access_for_family_in_authority;
}

fn build_query_graph_read_access_capability_rows() -> Vec<QueryGraphReadAccessCapabilityRow> {
    let mut rows = Vec::new();
    push_function_rows(&mut rows);
    push_type_rows(&mut rows);
    push_admission_posture_rows(&mut rows);
    push_denial_kind_rows(&mut rows);
    push_requirement_kind_rows(&mut rows);
    push_receipt_field_rows(&mut rows);
    push_cost_counter_rows(&mut rows);
    push_later_pressure_rows(&mut rows);
    rows
}

fn push_function_rows(rows: &mut Vec<QueryGraphReadAccessCapabilityRow>) {
    rows.extend([
        row("derive_graph_read_access_requirements", function_surface()),
        row(
            "try_derive_graph_read_access_requirements",
            function_surface(),
        ),
        row("admit_graph_read_access_for_family", function_surface()),
        row(
            "admit_graph_read_access_for_family_in_authority",
            function_surface(),
        ),
        row(
            "plan_admitted_graph_read_access_for_family",
            function_surface(),
        ),
        row(
            "plan_admitted_graph_read_access_for_family_in_authority",
            function_surface(),
        ),
    ]);
}

fn push_type_rows(rows: &mut Vec<QueryGraphReadAccessCapabilityRow>) {
    rows.extend([
        type_row::<ForgeQueryAdmittedGraphReadAccessPlan>("ForgeQueryAdmittedGraphReadAccessPlan"),
        type_row::<ForgeQueryGraphReadAccessAdmission>("ForgeQueryGraphReadAccessAdmission"),
        type_row::<ForgeQueryGraphReadAccessDenial>("ForgeQueryGraphReadAccessDenial"),
        type_row::<ForgeQueryGraphReadAccessRequirementRow>(
            "ForgeQueryGraphReadAccessRequirementRow",
        ),
        type_row::<ForgeQueryGraphReadAccessRequirementSet>(
            "ForgeQueryGraphReadAccessRequirementSet",
        ),
        type_row::<ForgeQueryGraphReadAccessRequirementCounters>(
            "ForgeQueryGraphReadAccessRequirementCounters",
        ),
        type_row::<ForgeQueryGraphReadAccessPlanConsumption>(
            "ForgeQueryGraphReadAccessPlanConsumption",
        ),
        type_row::<ForgeQueryGraphReadAccessComplexityCounters>(
            "ForgeQueryGraphReadAccessComplexityCounters",
        ),
        type_row::<ForgeQueryGraphReadAccessReceiptSummary>(
            "ForgeQueryGraphReadAccessReceiptSummary",
        ),
        type_row::<ForgeQueryEphemeralGraphIndexReceipt>("ForgeQueryEphemeralGraphIndexReceipt"),
        type_row::<ForgeQueryGraphReadStreamingReceipt>("ForgeQueryGraphReadStreamingReceipt"),
        type_row::<ForgeQueryLiveGraphReadAccessReceipt>("ForgeQueryLiveGraphReadAccessReceipt"),
        type_row::<ForgeQueryPersistentGraphIndexRequirementRow>(
            "ForgeQueryPersistentGraphIndexRequirementRow",
        ),
        type_row::<ForgeQueryGraphReadCostEvidence>("ForgeQueryGraphReadCostEvidence"),
        type_row::<ForgeQueryGraphReadAccessCostEstimate>("ForgeQueryGraphReadAccessCostEstimate"),
    ]);
}

fn push_admission_posture_rows(rows: &mut Vec<QueryGraphReadAccessCapabilityRow>) {
    rows.extend(
        ForgeQueryGraphReadAccessAdmissionPosture::ALL
            .iter()
            .map(|posture| {
                QueryGraphReadAccessCapabilityRow::from_query_owned_surface(
                    QueryGraphReadAccessCapabilityKind::AdmissionPosture,
                    posture.as_str(),
                    QueryGraphReadAccessCapabilitySurface::GraphReadAccessRuntime,
                )
            }),
    );
}

fn push_denial_kind_rows(rows: &mut Vec<QueryGraphReadAccessCapabilityRow>) {
    rows.extend(ForgeQueryGraphReadAccessDenialKind::ALL.iter().map(|kind| {
        QueryGraphReadAccessCapabilityRow::from_query_owned_surface(
            QueryGraphReadAccessCapabilityKind::DenialKind,
            kind.as_str(),
            QueryGraphReadAccessCapabilitySurface::GraphReadAccessRuntime,
        )
    }));
}

fn push_requirement_kind_rows(rows: &mut Vec<QueryGraphReadAccessCapabilityRow>) {
    rows.extend(
        ForgeQueryGraphReadAccessRequirementKind::all()
            .iter()
            .map(|kind| {
                QueryGraphReadAccessCapabilityRow::from_query_owned_surface(
                    QueryGraphReadAccessCapabilityKind::RequirementKind,
                    kind.as_str(),
                    QueryGraphReadAccessCapabilitySurface::GraphReadAccessRuntime,
                )
            }),
    );
}

fn push_receipt_field_rows(rows: &mut Vec<QueryGraphReadAccessCapabilityRow>) {
    rows.extend(QueryGraphReadReceiptField::ALL.iter().map(|field| {
        QueryGraphReadAccessCapabilityRow::from_query_owned_surface(
            QueryGraphReadAccessCapabilityKind::ReceiptField,
            field.query_label(),
            QueryGraphReadAccessCapabilitySurface::ReadReceiptAccessor,
        )
    }));
}

fn push_cost_counter_rows(rows: &mut Vec<QueryGraphReadAccessCapabilityRow>) {
    rows.extend(QueryGraphReadCostCounterField::ALL.iter().map(|field| {
        QueryGraphReadAccessCapabilityRow::from_query_owned_surface(
            QueryGraphReadAccessCapabilityKind::CostCounter,
            field.query_label(),
            QueryGraphReadAccessCapabilitySurface::GraphReadAccessRuntime,
        )
    }));
}

fn push_later_pressure_rows(rows: &mut Vec<QueryGraphReadAccessCapabilityRow>) {
    rows.extend([
        capability_gap_pressure_type_row::<ForgeQueryGraphReadOperationCapabilityRequirementKind>(
            "ForgeQueryGraphReadOperationCapabilityRequirementKind",
        ),
        capability_gap_pressure_type_row::<ForgeQueryGraphReadOperationUnsupportedDenialKind>(
            "ForgeQueryGraphReadOperationUnsupportedDenialKind",
        ),
    ]);
}

fn row(
    query_label: &'static str,
    surface: QueryGraphReadAccessCapabilitySurface,
) -> QueryGraphReadAccessCapabilityRow {
    QueryGraphReadAccessCapabilityRow::from_query_owned_surface(
        QueryGraphReadAccessCapabilityKind::Function,
        query_label,
        surface,
    )
}

fn type_row<T>(query_label: &'static str) -> QueryGraphReadAccessCapabilityRow {
    let _ = std::mem::size_of::<T>();
    QueryGraphReadAccessCapabilityRow::from_query_owned_surface(
        QueryGraphReadAccessCapabilityKind::Type,
        query_label,
        QueryGraphReadAccessCapabilitySurface::RuntimeFacade,
    )
}

fn capability_gap_pressure_type_row<T>(
    query_label: &'static str,
) -> QueryGraphReadAccessCapabilityRow {
    let _ = std::mem::size_of::<T>();
    QueryGraphReadAccessCapabilityRow::from_query_owned_surface(
        QueryGraphReadAccessCapabilityKind::CapabilityGapPressure,
        query_label,
        QueryGraphReadAccessCapabilitySurface::RuntimeFacade,
    )
}

fn function_surface() -> QueryGraphReadAccessCapabilitySurface {
    QueryGraphReadAccessCapabilitySurface::RuntimeFacade
}
