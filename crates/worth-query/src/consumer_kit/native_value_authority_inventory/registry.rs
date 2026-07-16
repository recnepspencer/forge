use std::sync::OnceLock;

use super::registry_helpers::{contract_projection, unvalidated};
use super::{
    WorthQueryNativeValueAuthorityClass as Class, WorthQueryNativeValueAuthorityRow as Row,
    WorthQueryNativeValueDisposition as Disposition,
};

pub(super) const PREDICATE: &str = "src/authoring/predicate.rs";
pub(super) const PREDICATE_OPERAND: &str = "src/authoring/predicate_operand.rs";
pub(super) const SCHEMA_FIELD: &str = "src/schema_view/field.rs";
pub(super) const MUTATION_ASPECT: &str = "src/runtime/mutation/aspect.rs";
pub(super) const DESIRED_ASPECT: &str = "src/runtime/mutation/native_intent/desired_aspect.rs";
pub(super) const CONSUMED_FACTS: &str = "src/projection_consumption/consumed/facts.rs";
pub(super) const CONSUMED_FIELD_VALUE: &str =
    "src/projection_consumption/consumed/field_value_fact.rs";
pub(super) const LIVE_SURFACE: &str = "src/runtime/surface/live.rs";

pub(super) const PREDICATE_EXPORTS: &[&str] = &[
    "src/facade/exports_foundation.rs",
    "src/facade/exports_read.rs",
    "src/facade/exports_aggregate.rs",
    "src/facade/exports_live_capability.rs",
    "src/facade/exports_history.rs",
    "src/facade/exports_comparison.rs",
];
pub(super) const SCHEMA_EXPORTS: &[&str] = &[
    "src/facade/exports_runtime_products.rs",
    "src/facade/exports_read.rs",
    "src/facade/exports_aggregate.rs",
    "src/facade/exports_live_capability.rs",
    "src/facade/exports_history.rs",
    "src/facade/exports_comparison.rs",
];
pub(super) const AUTHORED_VALUE_EXPORTS: &[&str] = &[
    "src/facade/exports_workflow.rs",
    "src/facade/exports_domain.rs",
    "src/facade/exports_runtime.rs",
    "src/facade/exports_mutation.rs",
    "src/facade/exports_preview.rs",
];

pub fn worth_query_native_value_authority_rows() -> &'static [Row] {
    static ROWS: OnceLock<Vec<Row>> = OnceLock::new();
    ROWS.get_or_init(|| {
        [
            CORE_ROWS,
            super::registry_phase22::ROWS,
            super::registry_phase23::ROWS,
            super::registry_phase24::ROWS,
        ]
        .into_iter()
        .flatten()
        .cloned()
        .collect()
    })
}

const CORE_ROWS: &[Row] = &[
    contract_projection(
        "WorthQueryAdmittedNativeFieldFamily",
        "src/runtime/graph_read_access/schema_reference_evidence/admitted_field_kind.rs",
        &["src/facade/exports_runtime_core.rs"],
        &["graph-read evidence", "selectivity shape"],
        "phase-27-contract-derived-schema",
    ),
    unvalidated(
        "WorthQueryAuthoredAspectValue",
        "src/runtime/mutation/aspect/authored_value.rs",
        AUTHORED_VALUE_EXPORTS,
        &[
            "ordinary mutation",
            "workflow",
            "preview",
            "domain extensions",
        ],
        "phase-26-native-mutation-authoring",
    ),
    unvalidated(
        "WorthQueryAuthoredAspectMutation",
        "src/runtime/mutation/aspect/authored_mutation.rs",
        &["src/facade/exports_runtime.rs"],
        &["mutation admission", "lowering", "receipts"],
        "phase-26-contract-validated-successor",
    ),
    Row::new(
        "WorthQueryDesiredAspectValue",
        DESIRED_ASPECT,
        &[],
        &["mutation parsing", "mutation identity"],
        Class::ProofBearingCarrier,
        Disposition::PreserveWithProof,
        "phase-26-contract-validated-successor",
    ),
    Row::new(
        "canonical_aspect_value",
        "src/projection_consumption/certification/oracle/value_terms.rs",
        &[],
        &["independent projection-consumption certification oracle"],
        Class::IndependentCertificationOracle,
        Disposition::PreserveAsIndependentOracle,
        "phase-29-foundational-value-identity",
    ),
];
