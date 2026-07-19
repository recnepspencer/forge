use super::model::{
    WorthQueryPublicAuthorityOwner as Owner, WorthQueryPublicAuthoritySurfaceClass as Class,
    WorthQueryPublicAuthoritySurfaceRow as Row,
};

const FOUNDATION: &str = "src/facade/exports_foundation.rs";
const POLICY: &str = "src/facade/exports_policy.rs";
const RUNTIME_CORE: &str = "src/facade/exports_runtime_core.rs";
const RUNTIME_PRODUCTS: &str = "src/facade/exports_runtime_products.rs";

pub(super) fn phase_two_authority_surface_rows() -> &'static [Row] {
    PHASE_TWO_ROWS
}

#[rustfmt::skip]
const PHASE_TWO_ROWS: &[Row] = &[
    sealed("QuerySchemaView::basis_authority", "src/schema_view/mod.rs", "basis_authority", RUNTIME_PRODUCTS, "QuerySchemaView", "schema validation", Owner::Identity),
    sealed("WorthQueryReadGraph::schema_basis_authority", "src/runtime/surface/read_composition.rs", "schema_basis_authority", RUNTIME_CORE, "WorthQueryReadGraph", "runtime basis resolution", Owner::Identity),
    sealed("ValidatedQueryArtifact::schema_basis_authority", "src/validation/artifacts/query.rs", "schema_basis_authority", RUNTIME_PRODUCTS, "ValidatedQueryArtifact", "validated planning", Owner::Identity),
    sealed("resolve_runtime_current_snapshot_basis", "src/basis/mod.rs", "resolve_runtime_current_snapshot_basis", FOUNDATION, "resolve_runtime_current_snapshot_basis", "runtime basis resolution", Owner::Identity),
    ordinary("admit_runtime_current_snapshot_basis", "src/basis/mod.rs", "admit_runtime_current_snapshot_basis", FOUNDATION, "admit_runtime_current_snapshot_basis", "external runtime basis admission", Owner::Identity),
    sealed("CanonicalQueryArtifact::authority", "src/canonicalization/artifacts/query.rs", "authority", FOUNDATION, "CanonicalQueryArtifact", "canonical identity continuity", Owner::Identity),
    sealed("ValidatedQueryArtifact::canonical_authority", "src/validation/artifacts/query.rs", "canonical_authority", RUNTIME_PRODUCTS, "ValidatedQueryArtifact", "validated identity continuity", Owner::Identity),
    sealed("PlannedQueryArtifact::canonical_authority", "src/planning/mod.rs", "canonical_authority", POLICY, "PlannedQueryArtifact", "planned identity continuity", Owner::Identity),
    historical("HistoricalEvaluationRequest::retained_snapshot", "retained_snapshot", "HistoricalEvaluationRequest"),
    historical("HistoricalEvaluationRequest::delta_replay", "delta_replay", "HistoricalEvaluationRequest"),
    historical("HistoricalEvaluationRequest::full_reconstruction", "full_reconstruction", "HistoricalEvaluationRequest"),
    historical("HistoricalCapabilityDescriptor::retained_snapshot", "retained_snapshot", "HistoricalCapabilityDescriptor"),
    historical("HistoricalCapabilityDescriptor::delta_replay", "delta_replay", "HistoricalCapabilityDescriptor"),
    historical("HistoricalCapabilityDescriptor::full_reconstruction", "full_reconstruction", "HistoricalCapabilityDescriptor"),
    historical("HistoricalMaterializationDescriptor::retained_snapshot", "retained_snapshot", "HistoricalMaterializationDescriptor"),
    historical("HistoricalMaterializationDescriptor::delta_replay", "delta_replay", "HistoricalMaterializationDescriptor"),
    historical("HistoricalMaterializationDescriptor::full_reconstruction", "full_reconstruction", "HistoricalMaterializationDescriptor"),
];

#[rustfmt::skip]
const fn ordinary(symbol: &'static str, source: &'static str, probe: &'static str, facade: &'static str, facade_probe: &'static str, consumer: &'static str, owner: Owner) -> Row {
    row(symbol, source, probe, facade, facade_probe, consumer, owner, Class::OrdinaryDeclarativeApi)
}

#[rustfmt::skip]
const fn sealed(symbol: &'static str, source: &'static str, probe: &'static str, facade: &'static str, facade_probe: &'static str, consumer: &'static str, owner: Owner) -> Row {
    row(symbol, source, probe, facade, facade_probe, consumer, owner, Class::SealedPhaseApi)
}

#[rustfmt::skip]
const fn historical(symbol: &'static str, probe: &'static str, facade_probe: &'static str) -> Row {
    sealed(symbol, "src/historical/request.rs", probe, FOUNDATION, facade_probe, "historical path admission", Owner::Historical)
}

#[rustfmt::skip]
const fn row(symbol: &'static str, source: &'static str, probe: &'static str, facade: &'static str, facade_probe: &'static str, consumer: &'static str, owner: Owner, class: Class) -> Row {
    Row::new(symbol, source, probe, Some(facade), Some(facade_probe), consumer, owner, class, class, symbol)
}
