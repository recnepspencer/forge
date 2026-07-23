use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::operation_phase_basis;
use crate::domain_installation::WorthQueryBoundDomainOperation;
use worth_proof::{CanonicalVec, NonEmpty, UniqueVec};

use super::super::compiled::{
    WorthQueryCompiledSemanticAspectDependencyClosure, WorthQuerySemanticAspectDependencyLocus,
    WorthQuerySemanticDependencyClosureEvidence, WorthQuerySemanticDependencyRole,
};
use super::operation_definition::SemanticAspectDependencyCompilation;
use super::{
    WorthQuerySemanticAspectDependencyCompilationDenial,
    WorthQuerySemanticAspectDependencyCompilationDenialKind,
};

impl SemanticAspectDependencyCompilation {
    pub(super) fn finish<D, O, F, L: BasisOperationLane>(
        mut self,
        bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> Result<
        WorthQueryCompiledSemanticAspectDependencyClosure,
        WorthQuerySemanticAspectDependencyCompilationDenial,
    > {
        let bucket_count = WorthQuerySemanticDependencyRole::COUNT
            * WorthQuerySemanticAspectDependencyLocus::KIND_COUNT;
        let mut buckets = (0..bucket_count).map(|_| Vec::new()).collect::<Vec<_>>();
        for dependency in self.dependencies {
            buckets[dependency.canonical_bucket()].push(dependency);
        }
        let canonical_dependencies = buckets.into_iter().flatten().collect::<Vec<_>>();
        let dependency_keys = canonical_dependencies
            .iter()
            .map(|dependency| (dependency.role(), dependency.locus.clone()))
            .collect::<Vec<_>>();
        let (_, canonicality) = match CanonicalVec::try_from_sorted(dependency_keys.clone()) {
            Ok(canonical) => canonical.into_parts(),
            Err(_) => {
                return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                    WorthQuerySemanticAspectDependencyCompilationDenialKind::NonCanonicalClosure,
                    self.counters,
                ));
            }
        };
        let loci = dependency_keys
            .into_iter()
            .map(|(_, locus)| locus)
            .collect();
        let (_, uniqueness) = match UniqueVec::try_from_unique_preserving_order(loci) {
            Ok(unique) => unique.into_parts(),
            Err(_) => {
                return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                    WorthQuerySemanticAspectDependencyCompilationDenialKind::DuplicateDependencyLocus,
                    self.counters,
                ));
            }
        };
        let Ok(dependencies) = NonEmpty::try_from_vec(canonical_dependencies) else {
            return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                WorthQuerySemanticAspectDependencyCompilationDenialKind::EmptyRequiredClosure,
                self.counters,
            ));
        };
        let (closure_evidence, workflow_edges) =
            match WorthQuerySemanticDependencyClosureEvidence::compile(dependencies.as_slice()) {
                Ok(compiled) => compiled,
                Err(kind) => {
                    return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                        kind,
                        self.counters,
                    ));
                }
            };
        self.counters.canonical_traversal_edges = dependencies.as_slice().len();
        self.counters.uniqueness_hash_checks = dependencies.as_slice().len();
        self.counters.compiled_dependency_count = dependencies.as_slice().len();
        self.counters.closure_edges_traversed = closure_evidence.closure_edge_count();
        self.counters.workflow_graph_edges_traversed = closure_evidence.workflow_graph_edge_count();
        Ok(WorthQueryCompiledSemanticAspectDependencyClosure::mint(
            operation_phase_basis(bound.authority_proof()).clone(),
            dependencies,
            canonicality,
            uniqueness,
            self.counters,
            closure_evidence,
            workflow_edges,
        ))
    }
}
