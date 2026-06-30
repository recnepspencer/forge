use std::fs;

use forge_query::facade::consumer_kit::query_consumer_residue_audit;
use forge_query::facade::consumer_kit::ForgeQueryBoundaryAuditSourceSet;

use crate::workload_platform::evidence_lookup_query_consumer_kit::boundary_audit;
use crate::workload_platform::evidence_lookup_query_consumer_kit::closeout::evaluate_consumer_kit_closeout_from_parts;
use crate::workload_platform::evidence_lookup_query_consumer_kit::source_set::{
    evidence_lookup_query_consumer_kit_boundary_sources,
    evidence_lookup_query_consumer_kit_residue_roots,
};
use crate::workload_platform::evidence_lookup_query_consumer_kit::support_snapshot::project_evidence_lookup_query_support_snapshot;
use crate::workload_platform::evidence_lookup_query_surface_matrix::current_evidence_lookup_query_surface_matrix;

use super::super::current_evidence_lookup_query_consumer_kit;

#[test]
fn consumer_residue_audit_blocks_local_query_folklore() {
    let root = std::env::temp_dir().join("worth-spatial-query-consumer-kit-residue-audit");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("temp residue root");
    fs::write(
        root.join("folklore.rs"),
        "struct LocalQueryReport { value: String }\n",
    )
    .expect("folklore fixture");

    let report = query_consumer_residue_audit("worth-spatial.test")
        .required_root(&root)
        .evaluate()
        .expect("residue audit evaluates");

    assert_eq!(report.finding_count(), 1);
    assert_eq!(
        report.findings()[0].residue_class().as_str(),
        "local-query-report"
    );

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn lookup_query_boundary_audit_is_clean() {
    let closeout = current_evidence_lookup_query_consumer_kit().expect("consumer kit closeout");

    assert_eq!(closeout.counters().boundary_audit_finding_count(), 0);
}

#[test]
fn closeout_blocks_on_boundary_audit_source_drift() {
    let matrix = current_evidence_lookup_query_surface_matrix().expect("matrix closes");
    let support_snapshot =
        project_evidence_lookup_query_support_snapshot(&matrix).expect("support snapshot");
    let boundary_sources = evidence_lookup_query_consumer_kit_boundary_sources().source_file(
        "seeded.workspace.write",
        "src/seeded/workspace_write.rs",
        "fn forged_workspace_write(workspace: &mut ForgeQueryWorkspace) { workspace.write(command); }",
    );

    let error = evaluate_consumer_kit_closeout_from_parts(
        matrix,
        support_snapshot,
        boundary_sources,
        evidence_lookup_query_consumer_kit_residue_roots(),
    )
    .expect_err("seeded prohibited seam must fail consumer kit closeout");

    assert_eq!(
        error.kind(),
        super::super::EvidenceLookupQueryConsumerKitErrorKind::BoundaryAudit
    );
    assert!(error.detail().contains("WorkspaceDirectWrite"));
}

#[test]
fn boundary_audit_report_identity_tracks_source_set_expansion_even_when_coverage_is_stable() {
    let baseline = boundary_audit::audit_evidence_lookup_query_hard_prohibitions_for_sources(
        evidence_lookup_query_consumer_kit_boundary_sources(),
    )
    .expect("baseline audit");
    let widened = boundary_audit::audit_evidence_lookup_query_hard_prohibitions_for_sources(
        ForgeQueryBoundaryAuditSourceSet::new("worth-spatial.evidence-lookup")
            .source_file("baseline", "src/baseline.rs", "fn baseline() {}")
            .source_file("extra", "src/extra.rs", "fn extra() {}"),
    )
    .expect("widened audit");

    assert_eq!(baseline.coverage_identity(), widened.coverage_identity());
    assert_ne!(baseline.report_identity(), widened.report_identity());
    assert_ne!(
        baseline.source_labels().len(),
        widened.source_labels().len()
    );
}
