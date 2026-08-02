//! Publication session closeout cannot be promoted to visible blob generations:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobPublicationSessionCloseout, BlobVisibleGeneration};
//!
//! fn requires_visible(_: BlobVisibleGeneration) {}
//!
//! let closeout: BlobPublicationSessionCloseout = todo!();
//! requires_visible(closeout);
//! ```
//! Publication WAL records cannot be promoted to visible blob generations:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobPublicationWalRecord, BlobVisibleGeneration};
//!
//! fn requires_visible(_: BlobVisibleGeneration) {}
//!
//! let record: BlobPublicationWalRecord = todo!();
//! requires_visible(record);
//! ```
//! Copied durable publication declarations cannot append blob publication WAL records:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobPublicationWalRecord;
//! use worth_store_wal::PublicationDeclaration;
//!
//! let declaration: PublicationDeclaration = todo!();
//! let _record = BlobPublicationWalRecord::append(declaration);
//! ```
//! Raw crash labels cannot drive blob publication recovery:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobPublicationCrashPoint, BlobPublicationRecoveryReplay};
//!
//! let _replay = BlobPublicationRecoveryReplay::recover(
//!     BlobPublicationCrashPoint::AfterChunkWrite,
//! );
//! ```
//! Generic partial-publication classifications cannot drive blob publication recovery:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobPublicationRecoveryEvidence, LogicalContentDigest,
//! };
//! use worth_store_recovery_physics::PartialPublicationClassification;
//!
//! let digest: LogicalContentDigest = todo!();
//! let classification: PartialPublicationClassification = todo!();
//! let _evidence = BlobPublicationRecoveryEvidence::chunk_write_replayed(
//!     &digest,
//!     classification,
//! );
//! ```
//! Generic partial-publication classifications cannot mint pre-WAL blob replay evidence:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobPublicationPreWalReplayEvidence, LogicalContentDigest,
//! };
//! use worth_store_recovery_physics::PartialPublicationClassification;
//!
//! let digest: LogicalContentDigest = todo!();
//! let classification: PartialPublicationClassification = todo!();
//! let _evidence = BlobPublicationPreWalReplayEvidence::from_chunk_write_replay(
//!     &digest,
//!     &classification,
//! );
//! ```
//! Blob publication replay byte materializers are not public authority:
//! ```compile_fail
//! use worth_store_blob_chunks::{
//!     BlobPublicationPreWalReplayEvidence, LogicalContentDigest,
//! };
//!
//! let digest: LogicalContentDigest = todo!();
//! let _bytes = BlobPublicationPreWalReplayEvidence::chunk_write_persisted_replay_bytes(
//!     &digest,
//! );
//! ```
//! Raw pre-WAL partial-publication bytes are not a public replay authority:
//! ```compile_fail
//! use worth_store_recovery_physics::PartialPublicationPersistedBytes;
//!
//! let _bytes = PartialPublicationPersistedBytes::before_wal_append("copied-operation");
//! ```
//! Raw replay bytes alone cannot mint a replayed crash-edge witness:
//! ```compile_fail
//! use worth_store_recovery_physics::{
//!     PartialPublicationPersistedBytes, PartialPublicationReplayedCrashEdge,
//! };
//!
//! let bytes = PartialPublicationPersistedBytes::from_bytes(Vec::new());
//! let _edge = PartialPublicationReplayedCrashEdge::from_replayed_store_bytes(bytes);
//! ```
//! Replay-read source assembly is not a public authority surface:
//! ```compile_fail
//! use worth_store_recovery_physics::PartialPublicationReplayReadSource;
//!
//! let _source: PartialPublicationReplayReadSource = todo!();
//! ```
//! Raw persisted bytes cannot be readmitted as a before-WAL replay witness:
//! ```compile_fail
//! use worth_store_recovery_physics::{
//!     PartialPublicationPersistedBytes, PartialPublicationReplayReadWitness,
//! };
//!
//! let bytes = PartialPublicationPersistedBytes::from_bytes(Vec::new());
//! let _witness = PartialPublicationReplayReadWitness::readmitted_before_wal_append(bytes);
//! ```
//! Replay-read records cannot be constructed outside the recovery replay gate:
//! ```compile_fail
//! use worth_store_recovery_physics::{
//!     PartialPublicationPersistedBytes, PartialPublicationReplayReadRecord, RecoveryReplayEntryGate,
//! };
//!
//! let replay_entry: RecoveryReplayEntryGate = todo!();
//! let bytes = PartialPublicationPersistedBytes::from_bytes(Vec::new());
//! let _record = PartialPublicationReplayReadRecord::from_replay_entry_gate(&replay_entry, bytes);
//! ```
//! Recovery replay gates do not accept caller-supplied partial-publication bytes:
//! ```compile_fail
//! use worth_store_recovery_physics::{
//!     PartialPublicationPersistedBytes, RecoveryReplayEntryGate,
//! };
//!
//! let replay_entry: RecoveryReplayEntryGate = todo!();
//! let bytes = PartialPublicationPersistedBytes::from_bytes(Vec::new());
//! let _record = replay_entry.read_partial_publication_persisted_bytes(bytes);
//! ```
//! Recovery replay gates do not accept copied operation digest strings:
//! ```compile_fail
//! use worth_store_recovery_physics::RecoveryReplayEntryGate;
//!
//! let replay_entry: RecoveryReplayEntryGate = todo!();
//! let _record = replay_entry.read_partial_publication_before_wal_append("copied-operation");
//! ```
//! Recovery entry admission cannot be mutated with copied pre-WAL replay identity:
//! ```compile_fail
//! use worth_store_recovery_physics::RecoveryEntryAdmission;
//!
//! let admission: RecoveryEntryAdmission = todo!();
//! let _entry = admission.with_partial_publication_before_wal_replay_read("copied-operation");
//! ```
//! S.4 integrity handoff declarations cannot attach copied pre-WAL operation
//! identity:
//! ```compile_fail
//! use worth_store_recovery_physics::IntegrityHandoffPayload;
//!
//! let _payload = IntegrityHandoffPayload::declare()
//!     .partial_publication_before_wal_operation_digest("copied-operation");
//! ```
//! S.4 integrity handoff declarations cannot treat copied operation identity as
//! a sealed replay-read payload:
//! ```compile_fail
//! use worth_store_recovery_physics::IntegrityHandoffPayload;
//!
//! let _payload = IntegrityHandoffPayload::declare()
//!     .partial_publication_before_wal_replay_read("copied-operation");
//! ```
//! Protected physical bytes cannot be forged from caller-owned raw byte slices:
//! ```compile_fail
//! use worth_store_physical_integrity::ProtectedPhysicalByteView;
//! use worth_store_recovery_physics::PartialPublicationBeforeWalReplayRead;
//!
//! let raw: &[u8] = b"worth-store.partial-publication.v1\nbefore-wal-append\ncopied";
//! let protected = ProtectedPhysicalByteView { bytes: raw };
//! let _read = PartialPublicationBeforeWalReplayRead::from_integrity_checked_frame(protected);
//! ```
//! Copied crash-edge representation cannot mint replayed crash-edge authority:
//! ```compile_fail
//! use worth_store_recovery_physics::{
//!     PartialPublicationCrashEdge, PartialPublicationReplayedCrashEdge, RecoveryReplayEntryGate,
//! };
//!
//! let replay_entry: RecoveryReplayEntryGate = todo!();
//! let _edge = PartialPublicationReplayedCrashEdge::from_recovery_replay_read(
//!     &replay_entry,
//!     PartialPublicationCrashEdge::before_wal_append("copied-operation"),
//! );
//! ```
//! Copied operation identity cannot directly mint a replay-read artifact:
//! ```compile_fail
//! use worth_store_recovery_physics::{
//!     PartialPublicationReplayReadArtifact, RecoveryReplayEntryGate,
//! };
//!
//! let replay_entry: RecoveryReplayEntryGate = todo!();
//! let _artifact = PartialPublicationReplayReadArtifact::phase_test_before_wal_append(
//!     &replay_entry,
//!     "copied-operation",
//! );
//! ```
//! Copied operation identity cannot mint a lower before-WAL replay-read payload:
//! ```compile_fail
//! use worth_store_recovery_physics::PartialPublicationBeforeWalReplayRead;
//!
//! let _read =
//!     PartialPublicationBeforeWalReplayRead::phase_test_from_operation_digest("copied-operation");
//! ```
//! Lower before-WAL replay-read payloads cannot be synthesized by callers:
//! ```compile_fail
//! use worth_store_recovery_physics::{
//!     PartialPublicationBeforeWalReplayRead, PartialPublicationPersistedBytes,
//! };
//!
//! let _read = PartialPublicationBeforeWalReplayRead {
//!     persisted_bytes: PartialPublicationPersistedBytes::during_checkpoint_cutover("copied"),
//!     _seal: todo!(),
//! };
//! ```
//! Registry observations cannot be promoted to visible blob generations:
//! ```compile_fail
//! use worth_store_blob_chunks::{BlobGenerationObservation, BlobVisibleGeneration};
//!
//! fn requires_visible(_: BlobVisibleGeneration) {}
//!
//! let observation: BlobGenerationObservation<'_> = todo!();
//! requires_visible(observation);
//! ```
//! Published blob generations cannot be synthesized from raw fields:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobGenerationPublished;
//!
//! let _forged = BlobGenerationPublished {
//!     object_id: todo!(),
//!     generation: todo!(),
//!     chunk_tree_root: todo!(),
//!     logical_content_digest: todo!(),
//!     classification: todo!(),
//!     publication_declaration: todo!(),
//!     replay_classification_digest: todo!(),
//!     replay_counters: todo!(),
//!     staging_identity: todo!(),
//!     security_metadata: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Visible blob generations cannot be synthesized from raw fields:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobVisibleGeneration;
//!
//! let _forged = BlobVisibleGeneration {
//!     object_id: todo!(),
//!     generation: todo!(),
//!     chunk_tree_root: todo!(),
//!     logical_content_digest: todo!(),
//!     classification: todo!(),
//!     counters: todo!(),
//! };
//! ```
//! Counter receipt identities cannot be synthesized from copied strings:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobPublicationCounterReceiptIdentity;
//!
//! let _forged = BlobPublicationCounterReceiptIdentity {
//!     value: "copied-counter-receipt".to_owned(),
//! };
//! ```
//! Recovery operation digests cannot be synthesized from copied strings:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobPublicationRecoveryOperationDigest;
//!
//! let _forged = BlobPublicationRecoveryOperationDigest {
//!     value: "copied-operation".to_owned(),
//! };
//! ```
//! Pre-WAL replay evidence cannot be synthesized from raw fields:
//! ```compile_fail
//! use worth_store_blob_chunks::BlobPublicationPreWalReplayEvidence;
//!
//! let _forged = BlobPublicationPreWalReplayEvidence {
//!     operation_digest: "copied-operation".to_owned(),
//!     classification_digest: "copied-classification".to_owned(),
//!     counters: todo!(),
//! };
//! ```
