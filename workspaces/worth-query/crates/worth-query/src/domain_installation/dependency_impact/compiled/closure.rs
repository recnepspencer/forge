use crate::domain_installation::operation_authority_chain::WorthQueryOperationAuthorityBasis;
use worth_proof::{CanonicalOrder, NonEmpty, Proof, StructuralProofAuthority, Uniqueness};

use super::{
    WorthQueryCompiledSemanticAspectDependency,
    WorthQuerySemanticAspectDependencyCompilationCounters,
};

pub struct WorthQueryCompiledSemanticAspectDependencyClosure {
    pub(crate) affinity: WorthQueryOperationAuthorityBasis,
    dependencies: NonEmpty<WorthQueryCompiledSemanticAspectDependency>,
    pub(crate) _canonical_dependencies: Proof<CanonicalOrder, StructuralProofAuthority>,
    pub(crate) _unique_dependencies: Proof<Uniqueness, StructuralProofAuthority>,
    counters: WorthQuerySemanticAspectDependencyCompilationCounters,
    impact_index: super::impact_index::WorthQuerySemanticImpactIndex,
    closure_evidence: super::closure_evidence::WorthQuerySemanticDependencyClosureEvidence,
    workflow_edges: Vec<super::closure_evidence::WorthQuerySemanticDependencyEdge>,
    invalidation_manifest: super::WorthQueryInstalledInvalidationManifest,
}

impl WorthQueryCompiledSemanticAspectDependencyClosure {
    pub(crate) fn mint(
        affinity: WorthQueryOperationAuthorityBasis,
        dependencies: NonEmpty<WorthQueryCompiledSemanticAspectDependency>,
        canonical_dependencies: Proof<CanonicalOrder, StructuralProofAuthority>,
        unique_dependencies: Proof<Uniqueness, StructuralProofAuthority>,
        mut counters: WorthQuerySemanticAspectDependencyCompilationCounters,
        closure_evidence: super::closure_evidence::WorthQuerySemanticDependencyClosureEvidence,
        workflow_edges: Vec<super::closure_evidence::WorthQuerySemanticDependencyEdge>,
    ) -> Self {
        let impact_index =
            super::impact_index::WorthQuerySemanticImpactIndex::compile(dependencies.as_slice());
        let invalidation_manifest = super::WorthQueryInstalledInvalidationManifest::compile(
            &affinity,
            dependencies.as_slice(),
            &impact_index,
        );
        counters.impact_index_entries = impact_index.entry_count();
        counters.impact_index_dependency_visits = dependencies.as_slice().len();
        counters.impact_mask_propagation_edges = impact_index.mask_propagation_edges();
        Self {
            affinity,
            dependencies,
            _canonical_dependencies: canonical_dependencies,
            _unique_dependencies: unique_dependencies,
            counters,
            impact_index,
            closure_evidence,
            workflow_edges,
            invalidation_manifest,
        }
    }

    pub fn dependencies(&self) -> &[WorthQueryCompiledSemanticAspectDependency] {
        self.dependencies.as_slice()
    }

    pub const fn counters(&self) -> WorthQuerySemanticAspectDependencyCompilationCounters {
        self.counters
    }

    /// Expanded width used by the linear compilation claim. This counts
    /// compiled dependency records, every materialized impact-index entry,
    /// every indexed dependency visit, and every workflow mask/graph edge.
    pub const fn measured_compilation_width(&self) -> usize {
        self.counters.compiled_dependency_count
            + self.counters.impact_index_dependency_visits
            + self.counters.impact_index_entries
            + self.counters.impact_mask_propagation_edges
            + self.counters.workflow_graph_edges_traversed
    }

    pub const fn closure_evidence(
        &self,
    ) -> super::closure_evidence::WorthQuerySemanticDependencyClosureEvidence {
        self.closure_evidence
    }

    pub fn workflow_edges(&self) -> &[super::closure_evidence::WorthQuerySemanticDependencyEdge] {
        &self.workflow_edges
    }

    pub fn bound_operation_identity(&self) -> &str {
        &self.affinity.operation_identity
    }

    pub const fn invalidation_manifest(&self) -> &super::WorthQueryInstalledInvalidationManifest {
        &self.invalidation_manifest
    }

    pub const fn installation_generation(&self) -> u64 {
        self.affinity.installation_generation
    }

    pub fn basis_identity(&self) -> &str {
        &self.affinity.basis_identity
    }

    pub fn compare_semantics(
        &self,
        candidate: &Self,
    ) -> super::WorthQueryDependencyClosureSemanticComparison {
        super::WorthQueryDependencyClosureSemanticComparison::compare(self, candidate)
    }

    pub(crate) fn converges_with(&self, candidate: &Self) -> bool {
        self.compare_semantics(candidate).is_converged()
    }

    pub(crate) fn contains_workflow_effect_receipt(&self, identity: &str) -> bool {
        self.impact_index.contains_workflow_effect_receipt(identity)
    }

    pub(in crate::domain_installation::dependency_impact) fn indexed_semantic_impact(
        &self,
        change: &worth_runtime_bridge::facade::BridgeSemanticAspectChange,
    ) -> super::impact_index::WorthQueryIndexedImpact {
        self.impact_index.semantic_roles(change)
    }

    pub(in crate::domain_installation::dependency_impact) fn contains_conditional_dependency(
        &self,
        location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        ordinal: usize,
    ) -> bool {
        self.impact_index.contains_conditional(location, ordinal)
    }

    pub(in crate::domain_installation::dependency_impact) fn conditional_consequence_roles(
        &self,
        location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        ordinal: usize,
    ) -> Option<Vec<super::WorthQuerySemanticDependencyRole>> {
        self.impact_index
            .conditional_consequence_roles(location, ordinal)
    }

    pub(in crate::domain_installation::dependency_impact) const fn has_structural_membership_dependency(
        &self,
    ) -> bool {
        self.impact_index.structural_membership()
    }
}
