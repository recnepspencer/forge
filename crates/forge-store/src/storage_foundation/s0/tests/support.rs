use crate::storage_foundation::s0::*;

pub(super) fn digest(label: &str) -> S0StableDigest {
    S0StableDigest::new(label).unwrap()
}

pub(super) fn metadata(label: &str) -> S0NondeterministicMetadata {
    S0NondeterministicMetadata::excluded(
        "excluded-from-digest",
        Some(format!("C:/tmp/{label}")),
        Some(format!("host-{label}")),
    )
    .unwrap()
}

pub(super) fn input_digest(label: &str) -> S0InputFileDigest {
    S0InputFileDigest::new(format!("input:{label}")).unwrap()
}

pub(super) fn scan_root(path: &str) -> S0DeclaredScanRoot {
    S0DeclaredScanRoot::new(path, "phase-1 test root").unwrap()
}

pub(super) fn matched_file(path: &str, digest: &str, byte_count: u64) -> S0MatchedInputFile {
    S0MatchedInputFile::new(
        path,
        S0InputFileKind::RoadmapDoc,
        input_digest(digest),
        byte_count,
    )
    .unwrap()
}

pub(super) fn matched_file_with_kind(
    path: &str,
    kind: S0InputFileKind,
    digest: &str,
    byte_count: u64,
) -> S0MatchedInputFile {
    S0MatchedInputFile::new(path, kind, input_digest(digest), byte_count).unwrap()
}

pub(super) fn evidence_ref(label: &str) -> S0EvidenceRef {
    S0EvidenceRef::new(
        S0ArtifactKind::S0EvidenceBundle,
        S0StableDigest::new(format!("evidence:{label}")).unwrap(),
    )
}

pub(super) fn terminology_input(path: &str, contents: &str) -> TerminologyScanInputFile {
    TerminologyScanInputFile::new(path, contents).unwrap()
}

pub(super) fn terminology_scope(path: &str) -> TerminologyScanScope {
    TerminologyScanScope::new(path).unwrap()
}

pub(super) fn semantic_cleanup_row() -> MilestonePhysicalStatusRow {
    MilestonePhysicalStatusRow::new(
        "13.3",
        "semantic trust closeout",
        "_docs/forge-store/milestone-13.3-closeout.md",
        "Shipped store capability reclassification test",
        vec!["subscription-support trust".to_string()],
        vec![
            SemanticPhysicalClaimFamily::SubscriptionSupport,
            SemanticPhysicalClaimFamily::PhysicalSubstrate,
            SemanticPhysicalClaimFamily::PhysicalIntegrity,
        ],
        S0PhysicalStatus::PhysicalDebt,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::BootstrapPhysical,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        None,
        None,
        vec![
            BackendForbiddenClaim::new(BackendForbiddenClaimKind::PhysicalPersistence, "S1")
                .unwrap(),
            BackendForbiddenClaim::new(BackendForbiddenClaimKind::PlatformGradeDurability, "S12")
                .unwrap(),
        ],
        vec![
            Roadmap2SequenceId::new("S1").unwrap(),
            Roadmap2SequenceId::new("S3").unwrap(),
            Roadmap2SequenceId::new("S12").unwrap(),
        ],
        vec!["qualify physical database posture".to_string()],
        None,
    )
    .unwrap()
}

pub(super) fn verified_complexity_report() -> S0ComplexityContractReport {
    S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        S0RequiredArtifactSet::canonical_complexity_contracts()
            .into_iter()
            .map(|name| S0ComplexityContract::verified(name.as_str(), 0, 0)),
    )
}

pub(super) fn release_lane_inputs(
    source_revision: &str,
) -> (
    S0AuditInputManifest,
    TerminologyRiskReport,
    ReleaseClaimReport,
) {
    let manifest = S0AuditInputManifest::new(
        source_revision,
        vec![scan_root(".github")],
        vec![matched_file_with_kind(
            ".github/workflows/release.yml",
            S0InputFileKind::Workflow,
            "workflow",
            64,
        )],
    )
    .unwrap();
    let terminology_report = TerminologyRiskReport::scan(
        source_revision,
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("release-lane"),
        &TerminologyScanPlan::new(vec![terminology_scope(".github")]).unwrap(),
        &manifest,
        &[terminology_input(
            ".github/workflows/release.yml",
            "semantic durability release lane\n",
        )],
        &[TerminologyAllowlistEntry::new(
            ".github/workflows/release.yml",
            1,
            "durability",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap()],
    )
    .unwrap();
    let release_report = ReleaseClaimReport::from_terminology_report(
        &ReleaseClaimScanPlan::new(vec![".github/workflows/release.yml".to_string()]).unwrap(),
        &terminology_report,
    )
    .unwrap();
    (manifest, terminology_report, release_report)
}

pub(super) fn milestone_sequence_for_13_3() -> RoadmapSequenceStatusMatrix {
    RoadmapSequenceStatusMatrix::new(
        vec![MilestoneStatusDeclaration::new(
            "13.3",
            MilestoneSpecStatus::Closed,
            MilestoneCloseoutStatus::Closed,
            vec![evidence_ref("13.2")],
        )
        .unwrap()],
        vec![],
    )
    .unwrap()
}
