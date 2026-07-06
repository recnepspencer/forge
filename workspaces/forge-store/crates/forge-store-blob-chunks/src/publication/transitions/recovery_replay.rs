use super::super::classification::recovered_state::classify_recovered_state;
use super::super::types::recovery_types::{
    BlobPublicationRecoveryEvidence, BlobPublicationRecoveryReplay,
};
use super::super::BlobPublicationCounterSnapshot;

pub(crate) fn recover(evidence: BlobPublicationRecoveryEvidence) -> BlobPublicationRecoveryReplay {
    let counters = BlobPublicationCounterSnapshot::start().with_recovered_state();
    let recovered_state = classify_recovered_state(evidence.crash_point(), counters);
    BlobPublicationRecoveryReplay {
        evidence,
        recovered_state,
    }
}