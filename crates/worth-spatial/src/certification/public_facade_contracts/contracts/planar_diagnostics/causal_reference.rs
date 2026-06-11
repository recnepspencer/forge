use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, CausalInspection,
    CausalInspectionMaterializationPolicy, EqualityPredicate, QueryObservationReceipt,
    QuerySchemaView, ScalarPredicateValue, SchemaFieldKind, SchemaFieldView,
};
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticCausalEvidence;

use super::causal_runtime::diagnostic_causal_query_runtime;

pub(crate) fn causal_reference(world: &'static str) -> PlanarDiagnosticCausalEvidence {
    let read_receipt = query_read_receipt(world);
    let observation = QueryObservationReceipt::from_read_receipt(read_receipt.receipt());
    let plan = CausalInspection::for_observation(observation)
        .why_replayed()
        .reference_only()
        .materialization(CausalInspectionMaterializationPolicy::DigestReferenceOnly)
        .plan()
        .expect("reference-rich causal inspection plan");
    PlanarDiagnosticCausalEvidence::from_query_causal_inspection_plan(&plan)
}

fn query_read_receipt(world: &'static str) -> forge_query::facade::ForgeQueryReadResult {
    let mut workspace = diagnostic_causal_query_runtime()
        .workspace(format!("planar-diagnostic-causal-{world}"))
        .expect("diagnostic causal workspace");
    workspace
        .compose_read(|read| {
            read.explicit_broad_search_collection(
                "planar-diagnostic",
                diagnostic_query_schema(world),
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id").expect("identity selector"),
                        )
                        .project(
                            AspectFieldSelector::new("diagnostic", "world")
                                .expect("diagnostic world selector"),
                        )
                        .where_equal(
                            EqualityPredicate::new(
                                "diagnostic",
                                "world",
                                ScalarPredicateValue::String(world.to_string()),
                            )
                            .expect("diagnostic world predicate"),
                        )
                },
                |shape| {
                    shape
                        .field(
                            AuthoredResultShapeField::new("identity", "id", "id")
                                .expect("identity result shape"),
                        )
                        .field(
                            AuthoredResultShapeField::new("diagnostic", "world", "world")
                                .expect("diagnostic world result shape"),
                        )
                },
            )
        })
        .expect("diagnostic causal read receipt")
}

fn diagnostic_query_schema(world: &'static str) -> QuerySchemaView {
    QuerySchemaView::new(
        format!("planar-diagnostic-causal-schema-{world}"),
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String)
                .text_predicate_queryable(),
            SchemaFieldView::new("diagnostic", "world", SchemaFieldKind::String)
                .text_predicate_queryable(),
        ],
        [],
    )
}
