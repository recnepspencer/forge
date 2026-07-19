#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryJournalIdentityOperationKind {
    MintCommittedPosition,
    MintPreviewPosition,
    AdmitSubmissionLanePosition,
    CarryWriteReceiptPosition,
    ReadWriteReceiptPosition,
    CarryPreviewReceiptPosition,
    CarryBatchReceiptPositions,
    BuildJournalSegmentIdentity,
    BuildJournalReplayRequest,
    RetainReplayRegistryReceipt,
    MaterializeReplayOutcome,
    ExposeWorkspaceReplayFacade,
    CertifyJournalIdentityBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryJournalIdentityInventoryRow {
    kind: WorthQueryJournalIdentityOperationKind,
    path: &'static str,
    required_patterns: &'static [&'static str],
}

impl WorthQueryJournalIdentityInventoryRow {
    pub const fn new(
        kind: WorthQueryJournalIdentityOperationKind,
        path: &'static str,
        required_patterns: &'static [&'static str],
    ) -> Self {
        Self {
            kind,
            path,
            required_patterns,
        }
    }

    pub fn kind(&self) -> WorthQueryJournalIdentityOperationKind {
        self.kind
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn required_patterns(&self) -> &'static [&'static str] {
        self.required_patterns
    }
}

const JOURNAL_IDENTITY_INVENTORY: &[WorthQueryJournalIdentityInventoryRow] = &[
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::MintCommittedPosition,
        "crates/worth-query/src/runtime/journal_position/identity.rs",
        &[
            "impl WorthQueryJournalPosition",
            "try_from_commit_identity",
            "relational_commit_id",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::MintPreviewPosition,
        "crates/worth-query/src/runtime/journal_position/identity.rs",
        &["pub(in crate::runtime) fn preview", "preview_identity"],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::AdmitSubmissionLanePosition,
        "crates/worth-query/src/runtime/workspace_submission.rs",
        &["WorthQueryWorkspaceSubmissionLane", "pub fn submit("],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::CarryWriteReceiptPosition,
        "crates/worth-query/src/runtime/surface/mutation/write_receipt/mod.rs",
        &[
            "impl WorthQueryWriteReceipt",
            "from_mutation_receipt",
            "journal_position",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::ReadWriteReceiptPosition,
        "crates/worth-query/src/runtime/surface/mutation/write_receipt/accessors.rs",
        &["pub fn journal_position", "WorthQueryJournalPosition"],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::CarryPreviewReceiptPosition,
        "crates/worth-query/src/runtime/surface/mutation/write_receipt/preview.rs",
        &[
            "WorthQueryWriteReceipt",
            "preview(",
            "WorthQueryJournalPosition::preview",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::CarryBatchReceiptPositions,
        "crates/worth-query/src/runtime/surface/mutation/batch_receipt.rs",
        &[
            "WorthQueryBatchWriteReceipt",
            "journal_positions",
            "journal_position_identity",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::BuildJournalSegmentIdentity,
        "crates/worth-query/src/runtime/journal_replay/segment.rs",
        &[
            "WorthQueryJournalSegmentIdentity",
            "WorthQueryEvidenceScope::JournalSegmentIdentity",
            "between(",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::BuildJournalReplayRequest,
        "crates/worth-query/src/runtime/journal_replay/request.rs",
        &[
            "WorthQueryJournalReplayRequest",
            "with_basis_snapshot",
            "WorthQueryJournalSegmentIdentity",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::RetainReplayRegistryReceipt,
        "crates/worth-query/src/runtime/journal_replay/registry.rs",
        &[
            "WorthQueryJournalReplayRegistry",
            "record_write_receipt",
            "entries_for_segment",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::MaterializeReplayOutcome,
        "crates/worth-query/src/runtime/journal_replay/outcome.rs",
        &[
            "WorthQueryJournalReplayOutcome",
            "WorthQueryEvidenceScope::JournalReplayOutcome",
            "position_schedule",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::ExposeWorkspaceReplayFacade,
        "crates/worth-query/src/runtime/runtime_journal_replay.rs",
        &[
            "pub fn replay_journal_segment",
            "WorthQueryJournalReplayRequest",
            "WorthQueryJournalReplayOutcome",
        ],
    ),
    WorthQueryJournalIdentityInventoryRow::new(
        WorthQueryJournalIdentityOperationKind::CertifyJournalIdentityBoundary,
        "crates/worth-query/src/application/support/journal_identity/certification.rs",
        &[
            "WorthQueryJournalIdentityCertification",
            "from_evidence",
            "WorthQueryJournalReplaySurfaceEvidence",
        ],
    ),
];

pub fn worth_query_journal_identity_inventory() -> &'static [WorthQueryJournalIdentityInventoryRow]
{
    JOURNAL_IDENTITY_INVENTORY
}
