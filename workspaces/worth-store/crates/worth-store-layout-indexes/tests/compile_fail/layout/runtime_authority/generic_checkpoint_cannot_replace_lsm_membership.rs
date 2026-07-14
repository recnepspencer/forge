use worth_store_lsm_authority::{
    replace_lsm_membership, LsmCompactionMembership, LsmMembershipSession,
};
use worth_store_wal::AdmittedCheckpointPublicationReceipt;

fn bypass(
    session: &mut LsmMembershipSession,
    membership: &LsmCompactionMembership,
    checkpoint: &AdmittedCheckpointPublicationReceipt,
) {
    let _ = replace_lsm_membership(session, membership, checkpoint);
}

fn main() {}
