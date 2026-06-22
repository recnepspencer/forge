#![allow(dead_code)]

use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopReconstructionEvidenceReceipt,
    PlanarBooleanLoopReconstructionLedger,
};

use super::support::{completed_loop_handoff, RealDegenerateLoopBoundaryProducts, ReplayBranch};

pub(super) struct RealLoopReplayCloseoutProducts {
    pub(super) degenerate: RealDegenerateLoopBoundaryProducts,
    pub(super) decision_log: PlanarBooleanLoopDecisionLog,
    pub(super) ledger: PlanarBooleanLoopReconstructionLedger,
    pub(super) evidence_receipt: PlanarBooleanLoopReconstructionEvidenceReceipt,
    pub(super) ledger_receipt:
        worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedgerReceipt,
}

pub(super) fn real_loop_replay_closeout_products(
    label: &'static str,
    replay_branch: ReplayBranch,
) -> RealLoopReplayCloseoutProducts {
    let handoff = completed_loop_handoff(label, replay_branch);
    let products = handoff
        .products()
        .expect("real loop reconstruction handoff should retain canonical phase products");
    let degenerate = RealDegenerateLoopBoundaryProducts {
        role_products: super::support::RealLoopRoleBoundaryProducts {
            reconstructed: super::support::RealReconstructedLoopProducts {
                candidate: super::support::RealLoopCandidateBoundaryProducts {
                    request: products.request().clone(),
                    source_provenance: products.source_provenance().clone(),
                    continuation_index: products.continuation_index().clone(),
                    split_fragments: products.split_fragments().clone(),
                    walk_candidate_assembly: products.walk_candidate_assembly().clone(),
                    walk_outcomes: products.walk_outcomes().clone(),
                    boundary: products.candidate_boundary().clone(),
                },
                boundary: products.reconstructed_boundary().clone(),
                partition: products.island_partition().clone(),
                split_attribution: products.split_attribution().clone(),
            },
            role_outcomes: products.role_outcomes().clone(),
            containment_postures: products.containment_postures().clone(),
        },
        boundary: products.degenerate_boundary().clone(),
        outcomes: products.degenerate_outcomes().clone(),
    };
    let decision_log = products.decision_log().clone();
    let ledger = products.loop_ledger().clone();
    let ledger_receipt = handoff.loop_ledger_receipt().clone();
    let evidence_receipt = handoff.evidence_receipt().clone();

    RealLoopReplayCloseoutProducts {
        degenerate,
        decision_log,
        ledger,
        evidence_receipt,
        ledger_receipt,
    }
}
