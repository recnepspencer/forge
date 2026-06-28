#[path = "s4_foundational_evidence_support.rs"]
mod evidence_support;

use forge_store_recovery_physics::{
    RecoveryEvidenceCanonicalBasis, RecoveryEvidenceConstructionSource, RecoveryEvidenceDenial,
    RecoveryEvidencePayloadKind, RecoveryEvidenceRichness, RecoveryPhysicsEvidenceSource,
};

#[test]
fn non_executed_and_non_foundational_payload_sources_are_denied() {
    let denied_sources = [
        (
            RecoveryEvidenceConstructionSource::PlannedRecovery,
            RecoveryEvidenceDenial::PlannedRecoveryCannotMaterializeEvidence,
        ),
        (
            RecoveryEvidenceConstructionSource::CopiedReceiptFields,
            RecoveryEvidenceDenial::CopiedReceiptFieldsCannotMaterializeEvidence,
        ),
        (
            RecoveryEvidenceConstructionSource::LogExcerpt,
            RecoveryEvidenceDenial::LogExcerptCannotMaterializeEvidence,
        ),
        (
            RecoveryEvidenceConstructionSource::SameRunSelfComparison,
            RecoveryEvidenceDenial::SameRunSelfComparisonCannotMaterializeEvidence,
        ),
    ];
    for (source, denial) in denied_sources {
        assert_eq!(
            RecoveryPhysicsEvidenceSource::deny_non_executed_source(source),
            denial
        );
    }

    let denied_payloads = [
        (
            RecoveryEvidencePayloadKind::RawBytes,
            RecoveryEvidenceDenial::RawBytesCannotMaterializeEvidence,
        ),
        (
            RecoveryEvidencePayloadKind::JsonShapedPayload,
            RecoveryEvidenceDenial::JsonPayloadCannotMaterializeEvidence,
        ),
        (
            RecoveryEvidencePayloadKind::DebugString,
            RecoveryEvidenceDenial::DebugStringCannotMaterializeEvidence,
        ),
        (
            RecoveryEvidencePayloadKind::DisplayName,
            RecoveryEvidenceDenial::DisplayNameCannotMaterializeEvidence,
        ),
        (
            RecoveryEvidencePayloadKind::ProducerPrivateName,
            RecoveryEvidenceDenial::ProducerPrivateNameCannotMaterializeEvidence,
        ),
    ];
    for (payload, denial) in denied_payloads {
        assert_eq!(
            RecoveryPhysicsEvidenceSource::deny_payload_kind(payload),
            Some(denial)
        );
    }
}

#[test]
fn reduced_richness_profile_preserves_recovery_truth() {
    let source = evidence_support::verified_source();
    let full = RecoveryEvidenceCanonicalBasis::full(&source).unwrap();
    let reduced = RecoveryEvidenceCanonicalBasis::reduced_from(
        &full,
        source.recovered_state().recovered_physical_root(),
        source.recovered_state().recovered_physical_root(),
    )
    .unwrap();

    assert_eq!(reduced.richness(), RecoveryEvidenceRichness::Reduced);
    assert_eq!(reduced.digest(), full.digest());
    assert_eq!(
        RecoveryEvidenceCanonicalBasis::reduced_from(&full, "root-a", "root-b").unwrap_err(),
        RecoveryEvidenceDenial::ProfileReductionChangedRecoveryTruth
    );
    assert_eq!(source.payload().len(), 5);
    assert_eq!(
        source.authority().epoch(),
        forge_foundational::BoundaryEpoch::new(4)
    );
}
