use worth_store_formal_models::{ModeledSourceCandidateRole, SourcePrecedenceAction};

pub(in crate::courtroom::protocol_models) fn execute_ordinary_source_precedence(
) -> Vec<SourcePrecedenceAction> {
    execute_ordinary_source_precedence_traces()
        .into_iter()
        .flatten()
        .collect()
}

pub(in crate::courtroom::protocol_models) fn execute_ordinary_source_precedence_traces(
) -> Vec<Vec<SourcePrecedenceAction>> {
    use ModeledSourceCandidateRole as Role;
    use SourcePrecedenceAction as Action;

    vec![
        vec![
            Action::CandidateDiscovered {
                discovery_order: 1,
                role: Role::CheckpointBase,
            },
            Action::CandidateAdmitted { discovery_order: 1 },
            Action::SourceSelected,
        ],
        vec![
            Action::CandidateDiscovered {
                discovery_order: 2,
                role: Role::PageSkipApply,
            },
            Action::CandidateAdvisoryOnly { discovery_order: 2 },
            Action::CandidateDiscovered {
                discovery_order: 3,
                role: Role::ResidueDiscoveryOnly,
            },
            Action::CandidateRejected { discovery_order: 3 },
            Action::ContradictionPreserved,
        ],
        vec![
            Action::CandidateDiscovered {
                discovery_order: 4,
                role: Role::CompactionVisibility,
            },
            Action::CandidateRejected { discovery_order: 4 },
            Action::CandidateDiscovered {
                discovery_order: 5,
                role: Role::RecoveryBlocked,
            },
            Action::SourceQuarantined,
            Action::SourceDenied,
        ],
    ]
}

pub(in crate::courtroom::protocol_models) fn replay_quarantined_source_guard(
    _seed: u64,
) -> Vec<SourcePrecedenceAction> {
    execute_ordinary_source_precedence_traces()
        .into_iter()
        .nth(2)
        .expect("source quarantine trace")
}
