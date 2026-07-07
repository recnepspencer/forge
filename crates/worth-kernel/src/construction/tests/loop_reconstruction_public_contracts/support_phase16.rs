use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopReconstructionEvidenceReceipt,
};

use super::support::{completed_loop_handoff, RealDegenerateLoopBoundaryProducts, ReplayBranch};

pub(super) struct RealLoopReplayCloseoutProducts {
    pub(super) degenerate: RealDegenerateLoopBoundaryProducts,
    pub(super) decision_log: PlanarBooleanLoopDecisionLog,
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
                boundary: products.reconstructed_boundary().clone(),
                partition: products.island_partition().clone(),
                split_attribution: products.split_attribution().clone(),
            },
            role_outcomes: products.role_outcomes().clone(),
        },
        outcomes: products.degenerate_outcomes().clone(),
    };
    let decision_log = products.decision_log().clone();
    let ledger_receipt = handoff.loop_ledger_receipt().clone();
    let evidence_receipt = handoff.evidence_receipt().clone();

    RealLoopReplayCloseoutProducts {
        degenerate,
        decision_log,
        evidence_receipt,
        ledger_receipt,
    }
}
