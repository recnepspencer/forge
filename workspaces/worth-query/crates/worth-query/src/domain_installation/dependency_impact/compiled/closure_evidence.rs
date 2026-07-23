use std::collections::HashSet;

use super::super::compilation::WorthQuerySemanticAspectDependencyCompilationDenialKind;
use super::dependency_locus::WorthQuerySemanticAspectDependencyLocus;
use super::WorthQueryCompiledSemanticAspectDependency;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQuerySemanticDependencyClosureEvidence {
    dependency_count: usize,
    closure_edge_count: usize,
    workflow_graph_edge_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQuerySemanticDependencyEdge {
    WorkflowOutputToConsumer {
        producer_stage: String,
        consumer_stage: String,
    },
}

impl WorthQuerySemanticDependencyClosureEvidence {
    pub const fn dependency_count(self) -> usize {
        self.dependency_count
    }

    pub const fn closure_edge_count(self) -> usize {
        self.closure_edge_count
    }

    pub const fn workflow_graph_edge_count(self) -> usize {
        self.workflow_graph_edge_count
    }

    pub(crate) fn compile(
        dependencies: &[WorthQueryCompiledSemanticAspectDependency],
    ) -> Result<
        (Self, Vec<WorthQuerySemanticDependencyEdge>),
        WorthQuerySemanticAspectDependencyCompilationDenialKind,
    > {
        let loci = dependencies
            .iter()
            .map(|dependency| dependency.locus.clone())
            .collect::<HashSet<_>>();
        if loci.len() != dependencies.len()
            || !loci.contains(&WorthQuerySemanticAspectDependencyLocus::InstalledOperation)
        {
            return Err(
                WorthQuerySemanticAspectDependencyCompilationDenialKind::AmbiguousDependencyGraph,
            );
        }
        let mut closure_edge_count = 0;
        for dependency in dependencies {
            let Some(parent) = dependency.parent_locus() else {
                continue;
            };
            if !loci.contains(&parent) {
                return Err(WorthQuerySemanticAspectDependencyCompilationDenialKind::IncompleteDependencyClosure);
            }
            closure_edge_count += 1;
        }
        let mut workflow_edges = Vec::new();
        for dependency in dependencies {
            let Some((consumer_stage, predecessors)) = dependency.workflow_stage_definition()
            else {
                continue;
            };
            for producer_stage in predecessors {
                let output = WorthQuerySemanticAspectDependencyLocus::WorkflowOutput {
                    stage_identity: producer_stage.clone(),
                };
                if !loci.contains(&output) {
                    return Err(WorthQuerySemanticAspectDependencyCompilationDenialKind::IncompleteDependencyClosure);
                }
                workflow_edges.push(WorthQuerySemanticDependencyEdge::WorkflowOutputToConsumer {
                    producer_stage: producer_stage.clone(),
                    consumer_stage: consumer_stage.to_owned(),
                });
            }
        }
        let evidence = Self {
            dependency_count: dependencies.len(),
            closure_edge_count,
            workflow_graph_edge_count: workflow_edges.len(),
        };
        Ok((evidence, workflow_edges))
    }
}
