use forge_store_certification::{
    S51CertificationCloseoutInput, S51CertificationEvidencePolicy, S51CloseoutPerformanceRows,
};
use forge_store_physical_certification::S51SecurityScopeHarnessReplayTranscript;

fn main() {
    let rows: Vec<S51CloseoutPerformanceRows> = Vec::new();
    let transcripts: Vec<S51SecurityScopeHarnessReplayTranscript> = Vec::new();
    let _input = S51CertificationCloseoutInput::from_replay_and_security_scope(
        rows,
        transcripts,
        todo!(),
        S51CertificationEvidencePolicy::counter_backed_foundational(),
    );
}
