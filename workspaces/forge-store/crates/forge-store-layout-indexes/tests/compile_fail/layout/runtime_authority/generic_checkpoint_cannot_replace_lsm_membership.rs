use forge_store_lsm_authority::{LsmCompactionMembership, LsmMembershipSession};
use forge_store_wal::AdmittedCheckpointPublicationReceipt;

fn bypass(
    session: &mut LsmMembershipSession,
    membership: &LsmCompactionMembership,
    checkpoint: &AdmittedCheckpointPublicationReceipt,
) {
    let _ = session.replace(membership, checkpoint);
}

fn main() {}
