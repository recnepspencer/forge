#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ForgeQueryJournalIdentityOperationKind {
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
pub struct ForgeQueryJournalIdentityInventoryRow {
    kind: ForgeQueryJournalIdentityOperationKind,
    path: &'static str,
    required_patterns: &'static [&'static str],
}

impl ForgeQueryJournalIdentityInventoryRow {
    pub const fn new(
        kind: ForgeQueryJournalIdentityOperationKind,
        path: &'static str,
        required_patterns: &'static [&'static str],
    ) -> Self {
        Self {
            kind,
            path,
            required_patterns,
        }
    }

    pub fn kind(&self) -> ForgeQueryJournalIdentityOperationKind {
        self.kind
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn required_patterns(&self) -> &'static [&'static str] {
        self.required_patterns
    }
}

const JOURNAL_IDENTITY_INVENTORY: &[ForgeQueryJournalIdentityInventoryRow] = &[
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::MintCommittedPosition,
        "crates/forge-query/src/runtime/journal_position/identity.rs",
        &[
            "impl ForgeQueryJournalPosition",
            "try_from_commit_identity",
            "relational_commit_id",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::MintPreviewPosition,
        "crates/forge-query/src/runtime/journal_position/identity.rs",
        &["pub(in crate::runtime) fn preview", "preview_identity"],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::AdmitSubmissionLanePosition,
        "crates/forge-query/src/runtime/workspace_submission.rs",
        &["ForgeQueryWorkspaceSubmissionLane", "pub fn submit("],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::CarryWriteReceiptPosition,
        "crates/forge-query/src/runtime/surface/mutation/write_receipt/mod.rs",
        &[
            "impl ForgeQueryWriteReceipt",
            "from_mutation_receipt",
            "journal_position",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::ReadWriteReceiptPosition,
        "crates/forge-query/src/runtime/surface/mutation/write_receipt/accessors.rs",
        &["pub fn journal_position", "ForgeQueryJournalPosition"],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::CarryPreviewReceiptPosition,
        "crates/forge-query/src/runtime/surface/mutation/write_receipt/preview.rs",
        &[
            "ForgeQueryWriteReceipt",
            "preview(",
            "ForgeQueryJournalPosition::preview",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::CarryBatchReceiptPositions,
        "crates/forge-query/src/runtime/surface/mutation/batch_receipt.rs",
        &[
            "ForgeQueryBatchWriteReceipt",
            "journal_positions",
            "journal_position_identity",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::BuildJournalSegmentIdentity,
        "crates/forge-query/src/runtime/journal_replay/segment.rs",
        &[
            "ForgeQueryJournalSegmentIdentity",
            "ForgeQueryEvidenceScope::JournalSegmentIdentity",
            "between(",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::BuildJournalReplayRequest,
        "crates/forge-query/src/runtime/journal_replay/request.rs",
        &[
            "ForgeQueryJournalReplayRequest",
            "with_basis_snapshot",
            "ForgeQueryJournalSegmentIdentity",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::RetainReplayRegistryReceipt,
        "crates/forge-query/src/runtime/journal_replay/registry.rs",
        &[
            "ForgeQueryJournalReplayRegistry",
            "record_write_receipt",
            "entries_for_segment",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::MaterializeReplayOutcome,
        "crates/forge-query/src/runtime/journal_replay/outcome.rs",
        &[
            "ForgeQueryJournalReplayOutcome",
            "ForgeQueryEvidenceScope::JournalReplayOutcome",
            "position_schedule",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::ExposeWorkspaceReplayFacade,
        "crates/forge-query/src/runtime/runtime_journal_replay.rs",
        &[
            "pub fn replay_journal_segment",
            "ForgeQueryJournalReplayRequest",
            "ForgeQueryJournalReplayOutcome",
        ],
    ),
    ForgeQueryJournalIdentityInventoryRow::new(
        ForgeQueryJournalIdentityOperationKind::CertifyJournalIdentityBoundary,
        "crates/forge-query/src/application/support/journal_identity/certification.rs",
        &[
            "ForgeQueryJournalIdentityCertification",
            "from_evidence",
            "ForgeQueryJournalReplaySurfaceEvidence",
        ],
    ),
];

pub fn forge_query_journal_identity_inventory() -> &'static [ForgeQueryJournalIdentityInventoryRow]
{
    JOURNAL_IDENTITY_INVENTORY
}
