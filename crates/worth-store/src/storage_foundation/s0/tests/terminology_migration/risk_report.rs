use super::super::support::{
    digest, matched_file_with_kind, metadata, scan_root, terminology_input, terminology_scope,
};
use crate::storage_foundation::s0::{
    S0AuditInputManifest, S0InputFileKind, TerminologyAllowedUse, TerminologyAllowlistEntry,
    TerminologyCleanupRejection, TerminologyRiskReport, TerminologyScanPlan,
};

#[test]
fn phase1_terminology_report_json_rejects_tampered_rows() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store")],
        vec![matched_file_with_kind(
            "_docs/worth-store/worth_store_roadmap.md",
            S0InputFileKind::RoadmapDoc,
            "roadmap",
            64,
        )],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![terminology_scope(
        "_docs/worth-store/worth_store_roadmap.md",
    )])
    .unwrap();
    let allowlist = vec![TerminologyAllowlistEntry::new(
        "_docs/worth-store/worth_store_roadmap.md",
        1,
        "database",
        TerminologyAllowedUse::AllowedSemanticUse,
    )
    .unwrap()];
    let report = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("terms-json"),
        &plan,
        &manifest,
        &[terminology_input(
            "_docs/worth-store/worth_store_roadmap.md",
            "database semantics only\n",
        )],
        &allowlist,
    )
    .unwrap();
    let mut json: serde_json::Value =
        serde_json::from_slice(&report.to_canonical_json_bytes().unwrap()).unwrap();
    json["rows"][0]["notes"] = serde_json::Value::String("tampered".into());
    let bytes = serde_json::to_vec(&json).unwrap();

    let error = TerminologyRiskReport::validate_canonical_json_bytes(&bytes)
        .expect_err("tampering must stale the digest");

    assert_eq!(
        error,
        TerminologyCleanupRejection::DeterministicDigestMismatch
    );
}
