use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQueryBoundDomainOperation;

use super::super::compiled::{
    WorthQueryCompiledSemanticAspectDependency,
    WorthQuerySemanticAspectDependencyCompilationCounters, WorthQuerySemanticAspectDependencyLocus,
    WorthQuerySemanticAspectDependencySource, WorthQuerySemanticDependencyRole,
};
use super::{
    WorthQuerySemanticAspectDependencyCompilationDenial,
    WorthQuerySemanticAspectDependencyCompilationDenialKind,
};

pub(super) struct SemanticAspectDependencyCompilation {
    pub(super) dependencies: Vec<WorthQueryCompiledSemanticAspectDependency>,
    pub(super) counters: WorthQuerySemanticAspectDependencyCompilationCounters,
    pub(super) conditional_declarations: std::collections::HashMap<
        worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    >,
    pub(super) conditional_order:
        Vec<worth_query_installation::facade::WorthQueryConditionalNodeLocation>,
}

impl SemanticAspectDependencyCompilation {
    pub(super) fn from_bound<D, O, F, L: BasisOperationLane>(
        bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
        counters: WorthQuerySemanticAspectDependencyCompilationCounters,
    ) -> Result<Self, WorthQuerySemanticAspectDependencyCompilationDenial> {
        let mut compilation = Self {
            dependencies: Vec::new(),
            counters: WorthQuerySemanticAspectDependencyCompilationCounters {
                installed_definition_visits: 1,
                ..counters
            },
            conditional_declarations: std::collections::HashMap::new(),
            conditional_order: Vec::new(),
        };
        if let Err(kind) = compilation.push_installed_definition(bound.definition()) {
            return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                kind,
                compilation.counters,
            ));
        }
        Ok(compilation)
    }

    fn push_installed_definition(
        &mut self,
        definition: &worth_query_installation::facade::WorthQueryPortableDomainOperationDefinition,
    ) -> Result<(), WorthQuerySemanticAspectDependencyCompilationDenialKind> {
        use worth_query_installation::facade::WorthQueryOperationWorkflowContract;

        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::InstalledOperation,
                WorthQuerySemanticDependencyRole::OperationalIdentity,
                WorthQuerySemanticAspectDependencySource::InstalledOperationIdentity {
                    identity: definition.identity().clone(),
                    canonical_identity: definition.canonical_identity().to_owned(),
                },
            ));
        self.push_native_projection(
            WorthQuerySemanticAspectDependencyLocus::OperationNativeProjection,
            &definition.semantics().native_projection,
        );
        self.push_collection_contract(&definition.semantics().collection);
        self.push_installed_semantic_contracts(definition.semantics());
        for graph_read in definition.semantics().graph_reads.roles() {
            self.counters.graph_read_role_visits += 1;
            let role = super::graph_read_access::WorthQueryCompiledGraphReadAccess::from_declared(
                graph_read.access,
            )
            .dependency_role();
            for (projection_ordinal, projection) in graph_read.semantic_reads.iter().enumerate() {
                self.push_graph_projection(
                    WorthQuerySemanticAspectDependencyLocus::GraphReadNativeProjection {
                        graph_read_role: graph_read.role.clone(),
                        projection_ordinal,
                    },
                    projection,
                    role,
                );
            }
        }
        for node in &definition.semantics().conditional_nodes {
            let location =
                worth_query_installation::facade::WorthQueryConditionalNodeLocation::operation(
                    node.identity(),
                )
                .map_err(|_| {
                    WorthQuerySemanticAspectDependencyCompilationDenialKind::InvalidInstalledConditionalLocation
                })?;
            self.push_conditional(location, node);
        }
        if let WorthQueryOperationWorkflowContract::Declared(workflow) =
            &definition.semantics().workflow
        {
            for stage in workflow.stages() {
                self.dependencies
                    .push(WorthQueryCompiledSemanticAspectDependency::new(
                        WorthQuerySemanticAspectDependencyLocus::WorkflowStage {
                            stage_identity: stage.identity().to_owned(),
                        },
                        WorthQuerySemanticDependencyRole::SupportAndLifecycle,
                        WorthQuerySemanticAspectDependencySource::WorkflowStage {
                            predecessors: stage.predecessors().to_vec(),
                        },
                    ));
                for graph_read_role in &stage.semantics().graph_read_roles {
                    self.dependencies
                        .push(WorthQueryCompiledSemanticAspectDependency::new(
                            WorthQuerySemanticAspectDependencyLocus::WorkflowStageRead {
                                stage_identity: stage.identity().to_owned(),
                                graph_read_role: graph_read_role.clone(),
                            },
                            WorthQuerySemanticDependencyRole::ProjectedValue,
                            WorthQuerySemanticAspectDependencySource::WorkflowStageRead {
                                graph_read_role: graph_read_role.clone(),
                            },
                        ));
                    self.counters.workflow_stage_read_edges += 1;
                }
                for node in &stage.semantics().conditional_nodes {
                    let location = worth_query_installation::facade::WorthQueryConditionalNodeLocation::workflow_stage(
                        stage.identity(),
                        node.identity(),
                    )
                    .map_err(|_| {
                        WorthQuerySemanticAspectDependencyCompilationDenialKind::InvalidInstalledConditionalLocation
                    })?;
                    self.push_conditional(location, node);
                }
            }
        }
        Ok(())
    }

    fn push_native_projection(
        &mut self,
        locus: WorthQuerySemanticAspectDependencyLocus,
        projection: &worth_query_installation::facade::WorthQueryOperationNativeProjectionContract,
    ) {
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                locus,
                WorthQuerySemanticDependencyRole::ProjectedValue,
                WorthQuerySemanticAspectDependencySource::NativeProjection(projection.clone()),
            ));
        self.counters.native_projection_edges += 1;
    }

    fn push_graph_projection(
        &mut self,
        locus: WorthQuerySemanticAspectDependencyLocus,
        projection: &worth_query_installation::facade::WorthQueryOperationNativeProjectionContract,
        role: WorthQuerySemanticDependencyRole,
    ) {
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                locus,
                role,
                WorthQuerySemanticAspectDependencySource::NativeProjection(projection.clone()),
            ));
        self.counters.native_projection_edges += 1;
    }
}
