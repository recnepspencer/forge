use super::anti_theatre_closeout::certify_anti_theatre_closeout;
use super::chain_rows::{
    collect_branch_proof_rows, PlanarBooleanLoopSummumBonumCloseoutProofRow,
    PlanarBooleanLoopSummumBonumProofBranch as Branch,
    PlanarBooleanLoopSummumBonumProofRowKind as Kind,
};
use super::public_contract_closeout::certify_public_contract_closeout;
use super::public_contract_support::proof_rows::PlanarBooleanLoopPublicContractProofRowKind;
use super::workload_evidence_support::certified_real_loop_replay_closeout_chain;
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopReplayParityRow, PlanarBooleanLoopReplayParityRowKind,
};

pub(crate) struct PlanarBooleanLoopSummumBonumCloseoutProofBundle {
    pub(crate) replay_parity_identity: String,
    pub(crate) public_contract_proof_identity: String,
    pub(crate) anti_theatre_proof_identity: String,
    pub(crate) proof_rows: Vec<PlanarBooleanLoopSummumBonumCloseoutProofRow>,
}

impl PlanarBooleanLoopSummumBonumCloseoutProofBundle {
    pub(crate) fn row(
        &self,
        branch: Branch,
        kind: Kind,
        identity: &str,
    ) -> Option<&PlanarBooleanLoopSummumBonumCloseoutProofRow> {
        self.proof_rows
            .iter()
            .find(|row| row.branch() == branch && row.kind() == kind && row.identity() == identity)
    }

    pub(crate) fn row_with_trace(
        &self,
        branch: Branch,
        kind: Kind,
        identity: &str,
        trace_identity: &str,
    ) -> Option<&PlanarBooleanLoopSummumBonumCloseoutProofRow> {
        self.proof_rows.iter().find(|row| {
            row.branch() == branch
                && row.kind() == kind
                && row.identity() == identity
                && row.trace_identity() == Some(trace_identity)
        })
    }

    pub(crate) fn rows_for(
        &self,
        branch: Branch,
        kind: Kind,
    ) -> impl Iterator<Item = &PlanarBooleanLoopSummumBonumCloseoutProofRow> {
        self.proof_rows
            .iter()
            .filter(move |row| row.branch() == branch && row.kind() == kind)
    }
}

pub(crate) fn certify_planar_boolean_loop_reconstruction_summum_bonum_closeout(
) -> PlanarBooleanLoopSummumBonumCloseoutProofBundle {
    let chain =
        certified_real_loop_replay_closeout_chain("phase7.4 loop reconstruction metaboss closeout");
    let original_products = chain
        .original
        .products()
        .expect("real closeout handoff should expose canonical loop products");
    chain
        .replayed
        .products()
        .expect("replayed closeout handoff should expose canonical loop products");

    let public_contract = certify_public_contract_closeout(&chain.original);
    let anti_theatre = certify_anti_theatre_closeout(&chain.original, &public_contract);

    let replayed_products = chain
        .replayed
        .products()
        .expect("replayed closeout handoff should expose canonical loop products");

    let mut proof_rows =
        collect_branch_proof_rows(Branch::Original, &chain.original, original_products);
    proof_rows.extend(collect_branch_proof_rows(
        Branch::Replayed,
        &chain.replayed,
        replayed_products,
    ));
    proof_rows.extend(chain.replay_parity.rows().iter().map(
        |row: &PlanarBooleanLoopReplayParityRow| {
            let identity = format!("{}=>{}", row.original_identity(), row.replayed_identity());
            let trace = replay_trace_label(row.kind());
            PlanarBooleanLoopSummumBonumCloseoutProofRow::with_trace(
                Branch::Shared,
                Kind::ReplayParityRow,
                identity,
                trace,
            )
        },
    ));
    proof_rows.extend(public_contract.rows().iter().map(|row| {
        PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
            Branch::Shared,
            Kind::PublicContractFenceRow,
            format!("{:?}:{}", row.kind(), row.identity()),
        )
    }));
    proof_rows.extend(
        anti_theatre
            .rows()
            .iter()
            .filter_map(|row| match row.kind() {
                PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreGuard => {
                    Some(PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
                        Branch::Shared,
                        Kind::AntiTheatreGuard,
                        row.identity(),
                    ))
                }
                PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreFence => {
                    Some(PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
                        Branch::Shared,
                        Kind::AntiTheatreFence,
                        row.identity(),
                    ))
                }
                _ => None,
            }),
    );
    proof_rows.push(PlanarBooleanLoopSummumBonumCloseoutProofRow::new(
        Branch::Shared,
        Kind::ReplayParityReceipt,
        chain.replay_parity.replay_identity(),
    ));

    PlanarBooleanLoopSummumBonumCloseoutProofBundle {
        replay_parity_identity: chain.replay_parity.replay_identity().to_string(),
        public_contract_proof_identity: public_contract.proof_identity().to_string(),
        anti_theatre_proof_identity: anti_theatre.proof_identity().to_string(),
        proof_rows,
    }
}

fn replay_trace_label(kind: PlanarBooleanLoopReplayParityRowKind) -> &'static str {
    match kind {
        PlanarBooleanLoopReplayParityRowKind::LoopEvidenceReceipt => "loop-evidence-receipt",
        PlanarBooleanLoopReplayParityRowKind::ReconstructedLoopSet => "reconstructed-loop-set",
        PlanarBooleanLoopReplayParityRowKind::BornLoopSet => "born-loop-set",
        PlanarBooleanLoopReplayParityRowKind::IslandPartition => "island-partition",
        PlanarBooleanLoopReplayParityRowKind::SplitAttribution => "split-attribution",
        PlanarBooleanLoopReplayParityRowKind::RoleOutcomeSet => "role-outcome-set",
        PlanarBooleanLoopReplayParityRowKind::DegenerateOutcomeSet => "degenerate-outcome-set",
        PlanarBooleanLoopReplayParityRowKind::DecisionLog => "decision-log",
        PlanarBooleanLoopReplayParityRowKind::LoopLedgerReceipt => "loop-ledger-receipt",
        PlanarBooleanLoopReplayParityRowKind::DownstreamConsumption => "downstream-consumption",
        PlanarBooleanLoopReplayParityRowKind::RetainedReplayCheckpoint => {
            "retained-replay-checkpoint"
        }
    }
}
