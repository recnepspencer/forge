use super::super::foundation::{WorkflowAuthorityTargetFamily, WorkflowBasisFamily};
use super::super::performance::WorkflowInspectionCounters;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowStalenessOutcome {
    StillFresh,
    StaleDenied,
    ExplicitRebindRequired,
}

impl WorkflowStalenessOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StillFresh => "still_fresh",
            Self::StaleDenied => "stale_denied",
            Self::ExplicitRebindRequired => "explicit_rebind_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowExplicitRebindArtifact {
    declaration_digest: String,
    basis_family: WorkflowBasisFamily,
    basis_digest: String,
    authority_target_family: WorkflowAuthorityTargetFamily,
    rebind_reason: &'static str,
    digest: String,
}

impl WorkflowExplicitRebindArtifact {
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn rebind_reason(&self) -> &'static str {
        self.rebind_reason
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowInspectionFailureClass {
    UnsupportedInspectionFamily,
    RelationalInspectionMismatch,
    NonAuthoritativeOutcomeForbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowInspectionError {
    failure_class: WorkflowInspectionFailureClass,
    message: &'static str,
    counters: WorkflowInspectionCounters,
}

impl WorkflowInspectionError {
    #[cfg(test)]
    pub(super) fn new(
        failure_class: WorkflowInspectionFailureClass,
        message: &'static str,
        counters: WorkflowInspectionCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> &WorkflowInspectionFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &WorkflowInspectionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConflictInspectionFamily {
    MergeWorkflowNarrow,
}

impl ConflictInspectionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MergeWorkflowNarrow => "merge_workflow_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MergeClassAdmission {
    ExecutionAdmissible,
    ExecutionDenied,
}

impl MergeClassAdmission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExecutionAdmissible => "execution_admissible",
            Self::ExecutionDenied => "execution_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PostMergeInspectionFamily {
    AuthoritativeOutcomeNarrow,
}

impl PostMergeInspectionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeOutcomeNarrow => "authoritative_outcome_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowAuthorityOutcomeFamily {
    MutationLoweringAdmitted,
    MergeLoweringAdmitted,
    WritebackLoweringAdmitted,
}

impl WorkflowAuthorityOutcomeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MutationLoweringAdmitted => "mutation_lowering_admitted",
            Self::MergeLoweringAdmitted => "merge_lowering_admitted",
            Self::WritebackLoweringAdmitted => "writeback_lowering_admitted",
        }
    }
}
