#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBranchLocalStateKind {
    Candidate,
    Staged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchLocalStateDefinition {
    kind: FoundationalBranchLocalStateKind,
    name: &'static str,
    intended_use: &'static str,
    must_not_mean: &'static str,
}

impl FoundationalBranchLocalStateDefinition {
    const fn new(
        kind: FoundationalBranchLocalStateKind,
        name: &'static str,
        intended_use: &'static str,
        must_not_mean: &'static str,
    ) -> Self {
        Self {
            kind,
            name,
            intended_use,
            must_not_mean,
        }
    }

    pub const fn kind(&self) -> FoundationalBranchLocalStateKind {
        self.kind
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intended_use(&self) -> &'static str {
        self.intended_use
    }

    pub const fn must_not_mean(&self) -> &'static str {
        self.must_not_mean
    }
}

const CANDIDATE_STATE_DEFINITION: FoundationalBranchLocalStateDefinition =
    FoundationalBranchLocalStateDefinition::new(
        FoundationalBranchLocalStateKind::Candidate,
        "candidate",
        "branch-local work that has not yet been staged as a stronger branch-local snapshot",
        "merge meaning, committed authority, or receipt evidence",
    );
const STAGED_STATE_DEFINITION: FoundationalBranchLocalStateDefinition =
    FoundationalBranchLocalStateDefinition::new(
        FoundationalBranchLocalStateKind::Staged,
        "staged",
        "branch-local work that is ready for later merge planning while still remaining non-authoritative",
        "merge verdicts, committed authority, or receipt evidence",
    );

pub const fn foundational_branch_local_state_definitions(
) -> [FoundationalBranchLocalStateDefinition; 2] {
    [CANDIDATE_STATE_DEFINITION, STAGED_STATE_DEFINITION]
}

pub(crate) const fn candidate_state_definition() -> &'static FoundationalBranchLocalStateDefinition
{
    &CANDIDATE_STATE_DEFINITION
}

pub(crate) const fn staged_state_definition() -> &'static FoundationalBranchLocalStateDefinition {
    &STAGED_STATE_DEFINITION
}
