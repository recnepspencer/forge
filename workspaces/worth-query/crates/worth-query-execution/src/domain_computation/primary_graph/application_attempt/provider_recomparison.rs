use super::precondition_binding::WorthQueryBoundMutationPreconditions;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryMutationPreconditionComparisonEvidence {
    expected_version_count: usize,
    expected_fact_count: usize,
    identity: Option<[u8; 32]>,
}

impl WorthQueryMutationPreconditionComparisonEvidence {
    pub const fn expected_version_count(&self) -> usize {
        self.expected_version_count
    }

    pub const fn expected_fact_count(&self) -> usize {
        self.expected_fact_count
    }

    pub const fn identity(&self) -> Option<&[u8; 32]> {
        self.identity.as_ref()
    }
}

pub(super) fn certify_provider_recomparison(
    preconditions: &WorthQueryBoundMutationPreconditions,
) -> WorthQueryMutationPreconditionComparisonEvidence {
    evidence(preconditions)
}

pub(super) fn recover_equivalent_commit_evidence(
    preconditions: &WorthQueryBoundMutationPreconditions,
) -> WorthQueryMutationPreconditionComparisonEvidence {
    evidence(preconditions)
}

fn evidence(
    preconditions: &WorthQueryBoundMutationPreconditions,
) -> WorthQueryMutationPreconditionComparisonEvidence {
    WorthQueryMutationPreconditionComparisonEvidence {
        expected_version_count: preconditions.expected_version_count(),
        expected_fact_count: preconditions.expected_fact_count(),
        identity: match preconditions.identity() {
            Some(identity) => Some(*identity),
            None => None,
        },
    }
}
