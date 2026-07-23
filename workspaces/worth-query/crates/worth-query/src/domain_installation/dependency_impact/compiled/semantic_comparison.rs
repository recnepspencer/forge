use super::WorthQueryCompiledSemanticAspectDependencyClosure;

/// Describes whether two compiled closures carry the same Query semantics.
///
/// This report carries no phase proof, checked basis, or readmission method. It
/// can explain replay convergence but cannot authorize reuse or impact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDependencyClosureSemanticComparison {
    subject_dependency_count: usize,
    candidate_dependency_count: usize,
    roles_and_loci_match: bool,
    source_semantics_match: bool,
    compilation_counters_match: bool,
    closure_evidence_match: bool,
    workflow_edges_match: bool,
}

impl WorthQueryDependencyClosureSemanticComparison {
    pub(super) fn compare(
        subject: &WorthQueryCompiledSemanticAspectDependencyClosure,
        candidate: &WorthQueryCompiledSemanticAspectDependencyClosure,
    ) -> Self {
        let subject_dependencies = subject.dependencies();
        let candidate_dependencies = candidate.dependencies();
        let same_width = subject_dependencies.len() == candidate_dependencies.len();
        Self {
            subject_dependency_count: subject_dependencies.len(),
            candidate_dependency_count: candidate_dependencies.len(),
            roles_and_loci_match: same_width
                && subject_dependencies
                    .iter()
                    .zip(candidate_dependencies)
                    .all(|(subject, candidate)| subject.semantic_role_and_locus_eq(candidate)),
            source_semantics_match: same_width
                && subject_dependencies
                    .iter()
                    .zip(candidate_dependencies)
                    .all(|(subject, candidate)| subject.semantic_source_eq(candidate)),
            compilation_counters_match: subject.counters() == candidate.counters(),
            closure_evidence_match: subject.closure_evidence() == candidate.closure_evidence(),
            workflow_edges_match: subject.workflow_edges() == candidate.workflow_edges(),
        }
    }

    pub const fn subject_dependency_count(self) -> usize {
        self.subject_dependency_count
    }

    pub const fn candidate_dependency_count(self) -> usize {
        self.candidate_dependency_count
    }

    pub const fn roles_and_loci_match(self) -> bool {
        self.roles_and_loci_match
    }

    /// Includes conditional observations, graph/read/effect evidence,
    /// identity-evolution lineage, and semantic output/result-state evidence.
    pub const fn source_semantics_match(self) -> bool {
        self.source_semantics_match
    }

    pub const fn compilation_counters_match(self) -> bool {
        self.compilation_counters_match
    }

    pub const fn closure_evidence_match(self) -> bool {
        self.closure_evidence_match
    }

    pub const fn workflow_edges_match(self) -> bool {
        self.workflow_edges_match
    }

    pub const fn is_converged(self) -> bool {
        self.subject_dependency_count == self.candidate_dependency_count
            && self.roles_and_loci_match
            && self.source_semantics_match
            && self.compilation_counters_match
            && self.closure_evidence_match
            && self.workflow_edges_match
    }
}
