use worth_kernel::workload_composition::{
    CompletedBooleanLoopReconstructionHandoff, CompletedBooleanLoopReconstructionProducts,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanLoopSummumBonumProofBranch {
    Original,
    Replayed,
    Shared,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanLoopSummumBonumProofRowKind {
    LoopLedgerReceipt,
    LoopEvidenceReceipt,
    ReplayParityReceipt,
    ReplayParityRow,
    RuntimeRegistrationProof,
    WorkloadStageIndex,
    DownstreamLoopConsumption,
    WalkOutcomeSet,
    AdmittedLoopCandidateSet,
    DeniedLoopCandidateSet,
    ReconstructedLoopSet,
    BornLoopSet,
    IslandPartition,
    SplitAttribution,
    RoleOutcomeSet,
    ContainmentPostureSet,
    DegenerateOutcomeSet,
    DecisionLog,
    LoopLedger,
    WalkOutcomeRow,
    AdmittedLoopCandidateRow,
    DeniedLoopCandidateRow,
    ReconstructedLoopRow,
    BornLoopRow,
    IslandPartitionRow,
    SplitAttributionRow,
    RoleOutcomeRow,
    ContainmentPostureRow,
    DegenerateOutcomeRow,
    LedgerRow,
    PublicContractFenceRow,
    AntiTheatreGuard,
    AntiTheatreFence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarBooleanLoopSummumBonumCloseoutProofRow {
    branch: PlanarBooleanLoopSummumBonumProofBranch,
    kind: PlanarBooleanLoopSummumBonumProofRowKind,
    identity: String,
    trace_identity: Option<String>,
}

impl PlanarBooleanLoopSummumBonumCloseoutProofRow {
    pub(crate) fn new(
        branch: PlanarBooleanLoopSummumBonumProofBranch,
        kind: PlanarBooleanLoopSummumBonumProofRowKind,
        identity: impl Into<String>,
    ) -> Self {
        Self {
            branch,
            kind,
            identity: identity.into(),
            trace_identity: None,
        }
    }

    pub(crate) fn with_trace(
        branch: PlanarBooleanLoopSummumBonumProofBranch,
        kind: PlanarBooleanLoopSummumBonumProofRowKind,
        identity: impl Into<String>,
        trace_identity: impl Into<String>,
    ) -> Self {
        Self {
            branch,
            kind,
            identity: identity.into(),
            trace_identity: Some(trace_identity.into()),
        }
    }

    pub(crate) fn branch(&self) -> PlanarBooleanLoopSummumBonumProofBranch {
        self.branch
    }

    pub(crate) fn kind(&self) -> PlanarBooleanLoopSummumBonumProofRowKind {
        self.kind
    }

    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn trace_identity(&self) -> Option<&str> {
        self.trace_identity.as_deref()
    }
}

pub(crate) fn collect_branch_proof_rows(
    branch: PlanarBooleanLoopSummumBonumProofBranch,
    handoff: &CompletedBooleanLoopReconstructionHandoff,
    products: &CompletedBooleanLoopReconstructionProducts,
) -> Vec<PlanarBooleanLoopSummumBonumCloseoutProofRow> {
    let mut rows = vec![
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::LoopLedgerReceipt,
            handoff.loop_ledger_receipt().receipt_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::LoopEvidenceReceipt,
            handoff.evidence_receipt().receipt_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::RuntimeRegistrationProof,
            handoff.runtime_registration_proof().proof_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::WorkloadStageIndex,
            handoff.workload_stage_index_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::DownstreamLoopConsumption,
            handoff
                .loop_ledger_receipt()
                .downstream_consumption_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::WalkOutcomeSet,
            products.walk_outcomes().walk_outcome_set_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::AdmittedLoopCandidateSet,
            products
                .candidate_boundary()
                .loop_candidates()
                .loop_candidate_set_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::DeniedLoopCandidateSet,
            products
                .candidate_boundary()
                .denied_loop_candidates()
                .denied_loop_candidate_set_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::ReconstructedLoopSet,
            products
                .reconstructed_boundary()
                .reconstructed_loops()
                .reconstructed_loop_set_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::BornLoopSet,
            products
                .reconstructed_boundary()
                .born_loops()
                .born_loop_set_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::IslandPartition,
            products.island_partition().partition_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::SplitAttribution,
            products.split_attribution().attribution_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::RoleOutcomeSet,
            products.role_outcomes().role_outcome_set_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::ContainmentPostureSet,
            products
                .containment_postures()
                .containment_evidence_posture_set_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::DegenerateOutcomeSet,
            products
                .degenerate_outcomes()
                .degenerate_loop_outcome_set_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::DecisionLog,
            products.decision_log().decision_log_identity(),
        ),
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::LoopLedger,
            products.loop_ledger().ledger_identity(),
        ),
    ];
    rows.extend(products.walk_outcomes().rows().iter().map(|row| {
        PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::WalkOutcomeRow,
            row.walk_outcome_identity(),
            row.source_loop_identity(),
        )
    }));
    rows.extend(
        products
            .candidate_boundary()
            .loop_candidates()
            .rows()
            .iter()
            .map(|row| {
                PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
                    branch,
                    PlanarBooleanLoopSummumBonumProofRowKind::AdmittedLoopCandidateRow,
                    row.loop_candidate_identity(),
                    row.walk_outcome_identity(),
                )
            }),
    );
    rows.extend(
        products
            .candidate_boundary()
            .denied_loop_candidates()
            .rows()
            .iter()
            .map(|row| {
                PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
                    branch,
                    PlanarBooleanLoopSummumBonumProofRowKind::DeniedLoopCandidateRow,
                    row.denied_loop_candidate_identity(),
                    row.walk_outcome_identity(),
                )
            }),
    );
    rows.extend(
        products
            .reconstructed_boundary()
            .reconstructed_loops()
            .rows()
            .iter()
            .map(|row| {
                PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
                    branch,
                    PlanarBooleanLoopSummumBonumProofRowKind::ReconstructedLoopRow,
                    row.reconstructed_loop_identity(),
                    row.loop_candidate_identity(),
                )
            }),
    );
    rows.extend(
        products
            .reconstructed_boundary()
            .born_loops()
            .rows()
            .iter()
            .map(|row| {
                PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
                    branch,
                    PlanarBooleanLoopSummumBonumProofRowKind::BornLoopRow,
                    row.born_loop_identity(),
                    row.loop_candidate_identity(),
                )
            }),
    );
    rows.extend(products.island_partition().rows().iter().map(|row| {
        PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::IslandPartitionRow,
            row.island_identity(),
            row.source_loop_identity(),
        )
    }));
    rows.extend(products.split_attribution().rows().iter().map(|row| {
        PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::SplitAttributionRow,
            row.attribution_identity(),
            row.source_loop_identity(),
        )
    }));
    rows.extend(products.role_outcomes().rows().iter().map(|row| {
        PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::RoleOutcomeRow,
            row.role_outcome_identity(),
            row.loop_identity(),
        )
    }));
    rows.extend(products.containment_postures().rows().iter().map(|row| {
        PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::ContainmentPostureRow,
            row.containment_posture_identity(),
            row.loop_identity(),
        )
    }));
    rows.extend(products.degenerate_outcomes().rows().iter().map(|row| {
        PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::DegenerateOutcomeRow,
            row.degenerate_loop_outcome_identity(),
            row.loop_identity(),
        )
    }));
    rows.extend(products.loop_ledger().rows().iter().map(|row| {
        PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
            branch,
            PlanarBooleanLoopSummumBonumProofRowKind::LedgerRow,
            row.ledger_row_identity(),
            row.canonical_loop_identity(),
        )
    }));
    rows
}
