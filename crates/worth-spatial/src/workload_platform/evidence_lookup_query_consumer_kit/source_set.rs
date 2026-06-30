use std::path::PathBuf;

use forge_query::facade::consumer_kit::ForgeQueryBoundaryAuditSourceSet;

const MOD_RS: &str = include_str!("mod.rs");
const CLOSEOUT_MOD_RS: &str = include_str!("closeout/mod.rs");
const CLOSEOUT_BINDINGS_RS: &str = include_str!("closeout/bindings.rs");
const CLOSEOUT_DIGEST_RS: &str = include_str!("closeout/digest.rs");
const CLOSEOUT_EVALUATION_RS: &str = include_str!("closeout/evaluation.rs");
const CLOSEOUT_MODEL_RS: &str = include_str!("closeout/model.rs");
const COUNTERS_RS: &str = include_str!("counters.rs");
const ERROR_RS: &str = include_str!("error.rs");
const EVIDENCE_REPORT_RS: &str = include_str!("evidence_report.rs");
const RESIDUE_AUDIT_RS: &str = include_str!("residue_audit.rs");
const ROW_RS: &str = include_str!("row.rs");
const REQUIREMENT_ROW_RS: &str = include_str!("requirement_row.rs");
const SOURCE_SET_RS: &str = include_str!("source_set.rs");
const SUPPORT_PINNING_RS: &str = include_str!("support_pinning.rs");
const SUPPORT_SNAPSHOT_RS: &str = include_str!("support_snapshot.rs");
const BOUNDARY_AUDIT_RS: &str = include_str!("boundary_audit.rs");
const FACADE_RS: &str = include_str!("../../facade/evidence_lookup_query_consumer_kit/mod.rs");

pub(crate) fn evidence_lookup_query_consumer_kit_boundary_sources(
) -> ForgeQueryBoundaryAuditSourceSet {
    ForgeQueryBoundaryAuditSourceSet::new("worth-spatial.evidence-lookup")
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::mod",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/mod.rs",
            MOD_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::closeout::mod",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/closeout/mod.rs",
            CLOSEOUT_MOD_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::closeout::bindings",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/closeout/bindings.rs",
            CLOSEOUT_BINDINGS_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::closeout::digest",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/closeout/digest.rs",
            CLOSEOUT_DIGEST_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::closeout::evaluation",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/closeout/evaluation.rs",
            CLOSEOUT_EVALUATION_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::closeout::model",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/closeout/model.rs",
            CLOSEOUT_MODEL_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::counters",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/counters.rs",
            COUNTERS_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::error",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/error.rs",
            ERROR_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::evidence_report",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/evidence_report.rs",
            EVIDENCE_REPORT_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::residue_audit",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/residue_audit.rs",
            RESIDUE_AUDIT_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::row",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/row.rs",
            ROW_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::requirement_row",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/requirement_row.rs",
            REQUIREMENT_ROW_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::source_set",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/source_set.rs",
            SOURCE_SET_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::support_pinning",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/support_pinning.rs",
            SUPPORT_PINNING_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::support_snapshot",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/support_snapshot.rs",
            SUPPORT_SNAPSHOT_RS,
        )
        .source_file(
            "workload_platform::evidence_lookup_query_consumer_kit::boundary_audit",
            "crates/worth-spatial/src/workload_platform/evidence_lookup_query_consumer_kit/boundary_audit.rs",
            BOUNDARY_AUDIT_RS,
        )
        .source_file(
            "facade::evidence_lookup_query_consumer_kit::mod",
            "crates/worth-spatial/src/facade/evidence_lookup_query_consumer_kit/mod.rs",
            FACADE_RS,
        )
}

pub(crate) fn evidence_lookup_query_consumer_kit_residue_roots() -> Vec<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vec![
        manifest_dir.join("src/workload_platform/evidence_lookup_query_consumer_kit"),
        manifest_dir.join("src/facade/evidence_lookup_query_consumer_kit"),
    ]
}
