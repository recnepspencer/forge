use schema::facade::platform::authority::touched_graph_conflict_internal::ConflictEvidenceParticipantSource;

struct Wrapper;

impl ConflictEvidenceParticipantSource for Wrapper {
    type EvidenceParticipantDigest = &'static str;

    fn conflict_evidence_participant_digest(&self) -> &Self::EvidenceParticipantDigest {
        &"forbidden"
    }
}

fn main() {}
