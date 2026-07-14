use worth_store_layout_indexes::{
    lsm_publication_runtime, BaselineLsmRunPublicationAdmission, PreparedLsmCompaction,
};
use worth_store_lsm_authority::LsmMembershipSession;
use worth_store_wal::AdmittedCheckpointPublicationReceipt;

fn bypass(
    session: &mut LsmMembershipSession,
    admission: BaselineLsmRunPublicationAdmission,
    prepared: PreparedLsmCompaction,
    manifest: AdmittedCheckpointPublicationReceipt,
) {
    let _ = lsm_publication_runtime().publish(session, admission, prepared, todo!(), manifest);
}

fn main() {}
