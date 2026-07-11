use forge_store_physical_format::CheckpointAdjacencyPosture;
use forge_store_physical_integrity::{QuarantineRecord, StoreExecutedIntegrityEvidence};
use forge_store_recovery_physics::{
    IntegrityDamageMap, IntegrityVettedCheckpointRecord, IntegrityVettedPageFrameRecord,
    IntegrityVettedRootManifestRecord, IntegrityVettedSegmentManifestRecord,
    IntegrityVettedWalFrame, PartialPublicationBeforeWalReplayRead,
    RecoveryBlockedByIntegrityDamage, RecoveryIntegrityHandoffReceipt, IntegrityHandoffPayload,
};

use super::s4_recovery_integrity_fixture::{
    inspect_checkpoint_record, inspect_manifest, inspect_page_report, inspect_wal_damage,
    inspect_wal_frame, inspection_envelope, receipt,
};
use super::s4_recovery_physical_fixture::{page_payload_with_record, with_protected_payload_view};

pub(super) fn intact_payload(
    label: &str,
    include_partial_publication_replay_read: bool,
) -> IntegrityHandoffPayload {
    let page_payload = page_payload_with_record(label.as_bytes());
    let page = inspect_page_report(&page_payload);
    let wal = inspect_wal_frame(CheckpointAdjacencyPosture::NotCheckpointAdjacent);
    let checkpoint = inspect_checkpoint_record();
    let manifest = inspect_manifest();
    let (quarantine_record, quarantine_receipt, quarantine_damage) = quarantine_binding();
    let manifest_receipt = receipt(StoreExecutedIntegrityEvidence::authoritative_manifest(
        &manifest,
    ));

    let mut declaration = IntegrityHandoffPayload::declare()
        .root_manifest(
            IntegrityVettedRootManifestRecord::from_manifest_report(
                &manifest,
                manifest_receipt.clone(),
            )
            .unwrap(),
        )
        .segment_manifest(
            IntegrityVettedSegmentManifestRecord::from_manifest_report(&manifest, manifest_receipt)
                .unwrap(),
        )
        .page_frame(
            IntegrityVettedPageFrameRecord::from_page_report(
                &page,
                receipt(StoreExecutedIntegrityEvidence::authoritative_page(&page)),
            )
            .unwrap(),
        )
        .wal_frame(
            IntegrityVettedWalFrame::from_integrity_report(
                &wal,
                receipt(StoreExecutedIntegrityEvidence::authoritative_wal_frame(
                    &wal,
                )),
            )
            .unwrap(),
        )
        .checkpoint_record(
            IntegrityVettedCheckpointRecord::from_integrity_report(
                &checkpoint,
                receipt(
                    StoreExecutedIntegrityEvidence::authoritative_checkpoint_record(&checkpoint),
                ),
            )
            .unwrap(),
        )
        .damage_map(
            IntegrityDamageMap::new()
                .with_recovery_blocking_quarantine(
                    &quarantine_record,
                    quarantine_receipt,
                    &quarantine_damage,
                )
                .unwrap(),
        )
        .inspection_envelope(inspection_envelope(&page_payload));
    if include_partial_publication_replay_read {
        declaration = declaration.partial_publication_before_wal_replay_read(
            partial_publication_before_wal_replay_read(label),
        );
    }
    declaration.seal().unwrap()
}

fn quarantine_binding() -> (
    QuarantineRecord,
    RecoveryIntegrityHandoffReceipt,
    RecoveryBlockedByIntegrityDamage,
) {
    let wal_damage = inspect_wal_damage(CheckpointAdjacencyPosture::NotCheckpointAdjacent);
    let record = forge_store_physical_integrity::PhysicalQuarantineAuthority::seal(
        forge_store_physical_integrity::QuarantineSealRequest::from_executed_finding(
            forge_store_physical_integrity::ExecutedQuarantineFinding::from_wal_frame_denial(
                &wal_damage,
            )
            .unwrap(),
        ),
    )
    .unwrap();
    let evidence =
        forge_store_physical_integrity::PhysicalIntegrityEvidenceAuthority::store_local()
            .materialize(
                forge_store_physical_integrity::StoreExecutedIntegrityEvidence::receipt_evidence(
                    &record,
                ),
                forge_store_physical_integrity::PhysicalIntegrityEvidenceProfile::reduced(),
            )
            .unwrap();
    let receipt =
        forge_store_recovery_physics::RecoveryIntegrityHandoffReceipt::from_quarantine_receipt_evidence(
            &evidence,
        )
        .unwrap();
    let damage = forge_store_recovery_physics::RecoveryBlockedByIntegrityDamage::damaged_wal_frame(
        &wal_damage,
    );
    (record, receipt, damage)
}

fn partial_publication_before_wal_replay_read(
    operation_digest: &str,
) -> PartialPublicationBeforeWalReplayRead {
    let bytes = partial_publication_before_wal_bytes(operation_digest);
    let mut replay_read = None;
    with_protected_payload_view(&bytes, |protected| {
        replay_read = Some(
            PartialPublicationBeforeWalReplayRead::from_protected_physical_bytes(protected)
                .expect("protected fixture bytes encode before-WAL partial publication replay"),
        );
    });
    replay_read.unwrap()
}

fn partial_publication_before_wal_bytes(operation_digest: &str) -> Vec<u8> {
    [
        "forge-store.partial-publication.v1",
        "before-wal-append",
        operation_digest,
    ]
    .join("\n")
    .into_bytes()
}
