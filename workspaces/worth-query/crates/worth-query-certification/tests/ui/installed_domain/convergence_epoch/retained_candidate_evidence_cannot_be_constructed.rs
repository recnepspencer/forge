use worth_query_host::facade::convergence_epoch::WorthQueryRetainedConvergenceCandidateEvidence;

fn forge() -> WorthQueryRetainedConvergenceCandidateEvidence {
    WorthQueryRetainedConvergenceCandidateEvidence {
        occurrence_identity: "forged-occurrence".into(),
        state_identity: "forged-state".into(),
        report_evidence_identity: "forged-report".into(),
    }
}

fn main() {}
