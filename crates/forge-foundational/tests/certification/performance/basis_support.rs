use forge_foundational::{
    canonicalization, performance, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalComparisonOutcome, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalEquivalenceBasis,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

pub fn authoritative_claim() -> forge_foundational::FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("authoritative claim should build")
}

pub fn exact_compare(
    left: forge_foundational::CanonicalBasisReadyArtifact,
    right: forge_foundational::CanonicalBasisReadyArtifact,
) -> CanonicalComparisonOutcome {
    let ready = match forge_foundational::prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        left,
        right,
    ) {
        forge_proof::TransitionOutcome::Success(ready) => ready,
        _ => panic!("expected comparison readiness"),
    };
    forge_foundational::compare_canonical_basis(&ready)
}

pub fn derive_digest(
    ready: forge_foundational::CanonicalBasisReadyArtifact,
) -> CanonicalDerivedDigest {
    let digest_ready = match canonicalization()
        .digest()
        .for_sequence(ready, CanonicalDigestAlgorithmId::test_stable_fixture())
    {
        forge_proof::TransitionOutcome::Success(ready) => ready,
        _ => panic!("expected digest readiness"),
    };
    canonicalization().digest().derive(digest_ready)
}

pub fn performance_text_entry(
    kind: CanonicalBasisEntryKind,
    locus: &str,
    value: &str,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Performance,
        forge_foundational::CanonicalBasisLocus::Named(locus.into()),
        kind,
        forge_foundational::CanonicalBasisValue::ExactText(value.into()),
    )
}

pub fn assert_entries_present(entries: &[CanonicalBasisEntry], expected: &[CanonicalBasisEntry]) {
    for entry in expected {
        assert!(
            entries.contains(entry),
            "expected canonical entry missing: {entry:?}"
        );
    }
}
