mod axes;
mod builders;
mod bundle;
mod digests;
mod fixtures {
    pub(super) mod canonical;
    pub(super) mod saved_query;
    pub(super) mod views;
}
mod lanes {
    pub(super) mod builders;
    pub(super) mod bundle_parts;
}
mod row;
mod tests;

pub const MILESTONE_NINE_FIVE_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "named-scope-table-retained-derived-parity",
    "template-detail-live-artifact-parity",
    "retained-vs-live-projection-contract-distinctness",
    "grouped-view-family-preserved-reuse-distinctness",
    "grouped-ordinary-vs-preserved-reuse-distinctness",
    "public-bridge-bootstrap-fixed-under-template-composition",
];

pub const MILESTONE_NINE_FIVE_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "grouped-preserved-reuse-basis-erasure-denied",
    "inspector-target-preserved-reuse-downcast-denied",
];
