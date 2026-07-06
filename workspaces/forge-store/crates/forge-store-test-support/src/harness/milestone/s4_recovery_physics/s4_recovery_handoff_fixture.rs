use forge_store_physical_format::CheckpointAdjacencyPosture;
use forge_store_physical_integrity::StoreExecutedIntegrityEvidence;
use forge_store_recovery_physics::{
    IntegrityDamageMap, IntegrityVettedCheckpointRecord, IntegrityVettedPageFrameRecord,
    IntegrityVettedRootManifestRecord, IntegrityVettedSegmentManifestRecord,
    IntegrityVettedWalFrame, PartialPublicationBeforeWalReplayRead, S4IntegrityHandoffPayload,
};

use super::s4_recovery_integrity_fixture::{
    inspect_checkpoint_record, inspect_manifest, inspect_page_report, inspect_wal_frame,
    inspection_envelope, quarantine_summary, receipt,
};
use super::s4_recovery_physical_fixture::{page_payload_with_record, with_protected_payload_view};

pub(super) fn intact_payload(
    label: &str,
    include_partial_publication_replay_read: bool,
) -> S4IntegrityHandoffPayload {
    let page_payload = page_payload_with_record(label.as_bytes());
    let page = inspect_page_report(&page_payload);
    let wal = inspect_wal_frame(CheckpointAdjacencyPosture::NotCheckpointAdjacent);
    let checkpoint = inspect_checkpoint_record();
    let manifest = inspect_manifest();
    let manifest_receipt = receipt(StoreExecutedIntegrityEvidence::authoritative_manifest(
        &manifest,
    ));

    let mut declaration = S4IntegrityHandoffPayload::declare()
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
        .damage_map(IntegrityDamageMap::new().with_quarantine_summary(quarantine_summary(&page)))
        .inspection_envelope(inspection_envelope(&page_payload));
    if include_partial_publication_replay_read {
        declaration = declaration.partial_publication_before_wal_replay_read(
            partial_publication_before_wal_replay_read(label),
        );
    }
    declaration.seal().unwrap()
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
