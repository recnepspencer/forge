use schema::facade::platform::authority::touched_graph_conflict::ConflictEvidenceParticipantSource;

struct Wrapper;

impl ConflictEvidenceParticipantSource for Wrapper {
    fn conflict_evidence_participant_digest(&self) -> &str {
        "copied-text"
    }
}

fn main() {}
