use topology::facade::PlanarBooleanLoopBlueprintRegistry;
use worth_kernel::replay_undo_consumer_cutover::{
    ReplayUndoMilestoneTwelvePublicCloseout, ReplayUndoMilestoneTwelvePublicCloseoutErrorKind,
    ReplayUndoMilestoneTwelvePublicCloseoutInput,
};
use worth_kernel::replay_undo_inventory::current_replay_undo_inventory_report;

use super::metaboss_support::MetabossEventExtractionSubject;
use super::real_handoff_support::{packet_backed_replay_undo_chain_for_branch, ReplayBranch};

pub(crate) fn assert_public_closeout_rejects_mismatched_proof_products() {
    let (matrix, validators) = PlanarBooleanLoopBlueprintRegistry::phase_2()
        .into_classification_matrix_and_validator_plan();
    let subject = MetabossEventExtractionSubject::certify_event_carrier(
        "phase13 public closeout proof product parity",
    );
    let foreign_subject = MetabossEventExtractionSubject::certify_event_carrier(
        "phase13 foreign public closeout proof product parity",
    );
    let chain = packet_backed_replay_undo_chain_for_branch(
        &subject,
        ReplayBranch::Original,
        &matrix,
        &validators,
    )
    .expect("ordinary replay/undo chain");
    let foreign_chain = packet_backed_replay_undo_chain_for_branch(
        &foreign_subject,
        ReplayBranch::Original,
        &matrix,
        &validators,
    )
    .expect("foreign ordinary replay/undo chain");
    assert_ne!(
        chain
            .consumer_cutover_closeout()
            .transaction_packet_identity(),
        foreign_chain
            .hard_deletion_closeout()
            .milestone_thirteen_seed()
            .transaction_packet_identity(),
        "regression fixture must exercise different proof chains"
    );
    assert_ne!(
        chain.consumer_cutover_closeout().replay_scope_identity(),
        foreign_chain
            .hard_deletion_closeout()
            .milestone_thirteen_seed()
            .replay_scope_identity(),
        "regression fixture must pressure replay-scope proof parity"
    );
    assert_ne!(
        chain.consumer_cutover_closeout().undo_scope_identity(),
        foreign_chain
            .hard_deletion_closeout()
            .milestone_thirteen_seed()
            .undo_scope_identity(),
        "regression fixture must pressure undo-scope proof parity"
    );

    let inventory = current_replay_undo_inventory_report().expect("replay/undo inventory");
    let same_chain_input =
        ReplayUndoMilestoneTwelvePublicCloseoutInput::from_replay_undo_boundary(&chain, &inventory)
            .expect("same-chain public closeout input");
    let same_chain_closeout = ReplayUndoMilestoneTwelvePublicCloseout::publish(same_chain_input)
        .expect("same-chain public closeout");
    assert_eq!(
        same_chain_closeout.milestone_thirteen_seed(),
        chain.hard_deletion_closeout().milestone_thirteen_seed()
    );
    assert_eq!(
        same_chain_closeout.transaction_packet_identity(),
        chain
            .consumer_cutover_closeout()
            .transaction_packet_identity()
    );
    assert_eq!(
        same_chain_closeout.replay_scope_identity(),
        chain.consumer_cutover_closeout().replay_scope_identity()
    );
    assert_eq!(
        same_chain_closeout.undo_scope_identity(),
        chain.consumer_cutover_closeout().undo_scope_identity()
    );

    let error = match ReplayUndoMilestoneTwelvePublicCloseoutInput::from_parts(
        chain.consumer_cutover_closeout(),
        foreign_chain.hard_deletion_closeout(),
        &inventory,
    ) {
        Ok(_) => {
            panic!("public closeout must reject mismatched cutover and hard-deletion products")
        }
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        &ReplayUndoMilestoneTwelvePublicCloseoutErrorKind::MismatchedProofProducts
    );
}
