use std::collections::BTreeSet;

use worth_foundational::facade::RetentionDeliveryProfile;
use worth_query_installation::facade::{
    WorthQueryArtifactClassification, WorthQueryArtifactDeletionPosture,
    WorthQueryArtifactGovernanceContract, WorthQueryArtifactLegalHoldPosture,
    WorthQueryArtifactRedactionPosture,
};

use super::{WorthQueryProviderCheckpointExport, WorthQueryProviderCheckpointFormat};

#[test]
fn provider_export_digest_binds_every_compatibility_and_governance_axis() {
    let baseline = export(
        "worth.tests.checkpoint",
        1,
        "worth.tests.checkpoint/v1",
        WorthQueryArtifactClassification::Confidential,
        RetentionDeliveryProfile::Durable,
        b"checkpoint-a",
    );
    let variants = [
        export(
            "worth.tests.other-checkpoint",
            1,
            "worth.tests.checkpoint/v1",
            WorthQueryArtifactClassification::Confidential,
            RetentionDeliveryProfile::Durable,
            b"checkpoint-a",
        ),
        export(
            "worth.tests.checkpoint",
            2,
            "worth.tests.checkpoint/v1",
            WorthQueryArtifactClassification::Confidential,
            RetentionDeliveryProfile::Durable,
            b"checkpoint-a",
        ),
        export(
            "worth.tests.checkpoint",
            1,
            "worth.tests.checkpoint/v2",
            WorthQueryArtifactClassification::Confidential,
            RetentionDeliveryProfile::Durable,
            b"checkpoint-a",
        ),
        export(
            "worth.tests.checkpoint",
            1,
            "worth.tests.checkpoint/v1",
            WorthQueryArtifactClassification::Restricted,
            RetentionDeliveryProfile::Durable,
            b"checkpoint-a",
        ),
        export(
            "worth.tests.checkpoint",
            1,
            "worth.tests.checkpoint/v1",
            WorthQueryArtifactClassification::Confidential,
            RetentionDeliveryProfile::Retained,
            b"checkpoint-a",
        ),
        export(
            "worth.tests.checkpoint",
            1,
            "worth.tests.checkpoint/v1",
            WorthQueryArtifactClassification::Confidential,
            RetentionDeliveryProfile::Durable,
            b"checkpoint-b",
        ),
    ];

    let digests = variants
        .iter()
        .map(WorthQueryProviderCheckpointExport::contract_digest)
        .collect::<BTreeSet<_>>();
    assert_eq!(digests.len(), variants.len());
    assert!(digests
        .iter()
        .all(|digest| *digest != baseline.contract_digest()));
    assert_eq!(baseline.payload_bytes(), b"checkpoint-a".len());
    assert_eq!(baseline.payload_digest().len(), 64);
}

#[test]
fn provider_checkpoint_format_rejects_noncanonical_identity_and_zero_version() {
    assert!(WorthQueryProviderCheckpointFormat::new(
        " worth.tests.checkpoint",
        1,
        "worth.tests.checkpoint/v1",
    )
    .is_err());
    assert!(WorthQueryProviderCheckpointFormat::new(
        "worth.tests.checkpoint",
        0,
        "worth.tests.checkpoint/v1",
    )
    .is_err());
    assert!(WorthQueryProviderCheckpointFormat::new(
        "worth.tests.checkpoint",
        1,
        "worth.tests.checkpoint/v1 ",
    )
    .is_err());
}

fn export(
    identity: &str,
    version: u64,
    compatibility: &str,
    classification: WorthQueryArtifactClassification,
    retention: RetentionDeliveryProfile,
    payload: &[u8],
) -> WorthQueryProviderCheckpointExport {
    WorthQueryProviderCheckpointExport::new(
        WorthQueryProviderCheckpointFormat::new(identity, version, compatibility)
            .expect("test checkpoint format should be valid"),
        WorthQueryArtifactGovernanceContract::new(
            ["store-checkpoint-ingestion"],
            classification,
            WorthQueryArtifactRedactionPosture::DomainRedactorRequired,
            retention,
            WorthQueryArtifactDeletionPosture::ExternallyControlled,
            WorthQueryArtifactLegalHoldPosture::DomainControlled,
        ),
        payload.to_vec(),
    )
    .expect("test checkpoint export should construct")
}
