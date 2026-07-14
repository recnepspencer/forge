use worth_ui_inspection::UiInspectionQuery;

use crate::evidence::UiInspectionObligationEvidenceReceipt;
use crate::facade::inspection_bridge::{
    admission::{
        assemble_relevance_receipt, assemble_support_receipt, collect_inspection_authority,
        decide_relevance_admission, decide_support_admission, InspectionAuthority,
        RelevanceAdmissionDecision, SupportAdmissionDecision,
    },
    boundary_access::{
        aspect_inspection_boundary, authored_inspection_boundary, graph_inspection_boundary,
        measurement_inspection_boundary, planning_inspection_boundary,
    },
    dispatch::{classify_inspection_dispatch, InspectionDispatchLane},
    obligation_routes::inspect_retained_obligation_query,
    support_routing::inspection_support_report_for,
    UiInspectionReceipt,
};
use crate::facade::WorthUiApp;

pub(crate) fn route_inspection(app: &WorthUiApp, query: UiInspectionQuery) -> UiInspectionReceipt {
    app.lifecycle().record_inspection_query();
    let authority = collect_inspection_authority(app.graph_snapshot().generation().as_u64());
    match classify_inspection_dispatch(&query) {
        InspectionDispatchLane::MeasurementScope => route_measurement_scope(app, query, authority),
        InspectionDispatchLane::PlanningScope => route_planning_scope(app, query, &authority),
        InspectionDispatchLane::ProductRootOrDeclaredSurface => {
            route_product_root(app, query, authority)
        }
        InspectionDispatchLane::AuthoredLookup => route_authored_lookup(app, query, authority),
        InspectionDispatchLane::GraphNodeIdentity => route_graph_node(app, query, authority),
        InspectionDispatchLane::AspectEvidence => route_aspect_evidence(app, query, authority),
        InspectionDispatchLane::RetainedObligation => {
            route_retained_obligation(app, query, authority)
        }
        InspectionDispatchLane::UnsupportedTarget => {
            let admission = query.admit_relevance();
            assemble_relevance_receipt(query, admission, &authority)
        }
    }
}

fn route_measurement_scope(
    app: &WorthUiApp,
    query: UiInspectionQuery,
    authority: InspectionAuthority,
) -> UiInspectionReceipt {
    match decide_relevance_admission(query.clone(), &authority) {
        RelevanceAdmissionDecision::Denied(receipt) => receipt,
        RelevanceAdmissionDecision::Matched(_) => {
            if let Some(receipt) = measurement_inspection_boundary(app).inspect(
                app,
                query.clone(),
                authority
                    .generation
                    .expect("graph-backed inspection has one active generation"),
            ) {
                return receipt;
            }
            let support_report = inspection_support_report_for(app, &query);
            assemble_support_receipt(
                query.clone(),
                query
                    .admit_relevance()
                    .refined_for_support_report(support_report),
                support_report,
                &authority,
            )
        }
    }
}

fn route_planning_scope(
    app: &WorthUiApp,
    query: UiInspectionQuery,
    authority: &InspectionAuthority,
) -> UiInspectionReceipt {
    if let Some(receipt) = planning_inspection_boundary(app).inspect(app, query.clone()) {
        return receipt;
    }
    let admission = query.admit_relevance();
    assemble_relevance_receipt(query, admission, authority)
}

fn route_product_root(
    app: &WorthUiApp,
    query: UiInspectionQuery,
    authority: InspectionAuthority,
) -> UiInspectionReceipt {
    let support_report = inspection_support_report_for(app, &query);
    match decide_support_admission(query.clone(), support_report, app.lifecycle(), &authority) {
        SupportAdmissionDecision::Denied(receipt) => receipt,
        SupportAdmissionDecision::Matched {
            admission,
            support_report,
        } => assemble_support_receipt(query, admission, support_report, &authority),
    }
}

fn route_authored_lookup(
    app: &WorthUiApp,
    query: UiInspectionQuery,
    authority: InspectionAuthority,
) -> UiInspectionReceipt {
    match decide_relevance_admission(query.clone(), &authority) {
        RelevanceAdmissionDecision::Denied(receipt) => receipt,
        RelevanceAdmissionDecision::Matched(_) => {
            app.lifecycle().record_authored_lookup();
            let fallback_query = query.clone();
            authored_inspection_boundary(app)
                .inspect(
                    query,
                    authority
                        .generation
                        .expect("graph-backed inspection has one active generation"),
                )
                .unwrap_or_else(|| {
                    assemble_relevance_receipt(
                        fallback_query.clone(),
                        fallback_query.admit_relevance(),
                        &authority,
                    )
                })
        }
    }
}

fn route_graph_node(
    app: &WorthUiApp,
    query: UiInspectionQuery,
    authority: InspectionAuthority,
) -> UiInspectionReceipt {
    match decide_relevance_admission(query.clone(), &authority) {
        RelevanceAdmissionDecision::Denied(receipt) => receipt,
        RelevanceAdmissionDecision::Matched(_) => graph_inspection_boundary(app)
            .inspect(
                query.clone(),
                authority
                    .generation
                    .expect("graph-backed inspection has one active generation"),
            )
            .unwrap_or_else(|| {
                assemble_relevance_receipt(query.clone(), query.admit_relevance(), &authority)
            }),
    }
}

fn route_aspect_evidence(
    app: &WorthUiApp,
    query: UiInspectionQuery,
    authority: InspectionAuthority,
) -> UiInspectionReceipt {
    let support_report = inspection_support_report_for(app, &query);
    match decide_support_admission(query.clone(), support_report, app.lifecycle(), &authority) {
        SupportAdmissionDecision::Denied(receipt) => receipt,
        SupportAdmissionDecision::Matched {
            admission: _,
            support_report,
        } => aspect_inspection_boundary(app)
            .inspect(
                query.clone(),
                authority
                    .generation
                    .expect("graph-backed inspection has one active generation"),
            )
            .unwrap_or_else(|| {
                assemble_support_receipt(
                    query.clone(),
                    query
                        .admit_relevance()
                        .refined_for_support_report(support_report),
                    support_report,
                    &authority,
                )
            }),
    }
}

fn route_retained_obligation(
    app: &WorthUiApp,
    query: UiInspectionQuery,
    authority: InspectionAuthority,
) -> UiInspectionReceipt {
    match decide_relevance_admission(query.clone(), &authority) {
        RelevanceAdmissionDecision::Denied(receipt) => receipt,
        RelevanceAdmissionDecision::Matched(admission) => {
            if let Some(receipt) = inspect_retained_obligation_query(app, query.clone()) {
                return receipt;
            }
            UiInspectionReceipt::from_obligation(
                query,
                admission,
                authority
                    .generation
                    .expect("graph-backed inspection has one active generation"),
                UiInspectionObligationEvidenceReceipt::new(Box::new([]), Box::new([])),
            )
        }
    }
}
