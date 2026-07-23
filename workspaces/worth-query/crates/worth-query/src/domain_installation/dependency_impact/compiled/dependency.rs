use super::dependency_locus::WorthQuerySemanticAspectDependencyLocus;
use super::dependency_source::{
    WorthQuerySemanticAspectDependencySource, WorthQuerySemanticAspectDependencyView,
};
use super::WorthQuerySemanticDependencyRole;

#[derive(Clone, Debug)]
pub struct WorthQueryCompiledSemanticAspectDependency {
    pub(crate) locus: WorthQuerySemanticAspectDependencyLocus,
    role: WorthQuerySemanticDependencyRole,
    pub(crate) source: WorthQuerySemanticAspectDependencySource,
}

impl WorthQueryCompiledSemanticAspectDependency {
    pub(crate) fn new(
        locus: WorthQuerySemanticAspectDependencyLocus,
        role: WorthQuerySemanticDependencyRole,
        source: WorthQuerySemanticAspectDependencySource,
    ) -> Self {
        Self {
            locus,
            role,
            source,
        }
    }

    pub const fn role(&self) -> WorthQuerySemanticDependencyRole {
        self.role
    }

    pub fn installed_operation_identity(
        &self,
    ) -> Option<&worth_query_installation::facade::WorthQueryDomainOperationIdentity> {
        match &self.source {
            WorthQuerySemanticAspectDependencySource::InstalledOperationIdentity {
                identity,
                ..
            } => Some(identity),
            _ => None,
        }
    }

    pub fn installed_operation_canonical_identity(&self) -> Option<&str> {
        match &self.source {
            WorthQuerySemanticAspectDependencySource::InstalledOperationIdentity {
                canonical_identity,
                ..
            } => Some(canonical_identity),
            _ => None,
        }
    }

    pub fn source(&self) -> WorthQuerySemanticAspectDependencyView<'_> {
        match &self.source {
            WorthQuerySemanticAspectDependencySource::InstalledOperationIdentity {
                identity,
                canonical_identity,
            } => WorthQuerySemanticAspectDependencyView::InstalledOperationIdentity {
                identity,
                canonical_identity,
            },
            WorthQuerySemanticAspectDependencySource::NativeProjection(projection) => {
                WorthQuerySemanticAspectDependencyView::NativeProjection(projection)
            }
            WorthQuerySemanticAspectDependencySource::CollectionField(field) => {
                WorthQuerySemanticAspectDependencyView::CollectionField(field)
            }
            WorthQuerySemanticAspectDependencySource::CollectionWindowPolicy(policy) => {
                WorthQuerySemanticAspectDependencyView::CollectionWindowPolicy(*policy)
            }
            WorthQuerySemanticAspectDependencySource::ResultShape(shape) => {
                WorthQuerySemanticAspectDependencyView::ResultShape(shape)
            }
            WorthQuerySemanticAspectDependencySource::TouchGraphRole(role) => {
                WorthQuerySemanticAspectDependencyView::TouchGraphRole(role)
            }
            WorthQuerySemanticAspectDependencySource::TouchScope(scope) => {
                WorthQuerySemanticAspectDependencyView::TouchScope(scope)
            }
            WorthQuerySemanticAspectDependencySource::EffectFamily(family) => {
                WorthQuerySemanticAspectDependencyView::EffectFamily(*family)
            }
            WorthQuerySemanticAspectDependencySource::InstalledInvariant(invariant) => {
                WorthQuerySemanticAspectDependencyView::InstalledInvariant(invariant)
            }
            WorthQuerySemanticAspectDependencySource::ReplayContract(contract) => {
                WorthQuerySemanticAspectDependencyView::ReplayContract(*contract)
            }
            WorthQuerySemanticAspectDependencySource::LineageContract(contract) => {
                WorthQuerySemanticAspectDependencyView::LineageContract(*contract)
            }
            WorthQuerySemanticAspectDependencySource::SupportContract(contract) => {
                WorthQuerySemanticAspectDependencyView::SupportContract(*contract)
            }
            WorthQuerySemanticAspectDependencySource::WorkflowStageRead { graph_read_role } => {
                WorthQuerySemanticAspectDependencyView::WorkflowStageRead { graph_read_role }
            }
            WorthQuerySemanticAspectDependencySource::WorkflowStage { predecessors } => {
                WorthQuerySemanticAspectDependencyView::WorkflowStage { predecessors }
            }
            WorthQuerySemanticAspectDependencySource::ConditionalNodeContract(declaration) => {
                WorthQuerySemanticAspectDependencyView::ConditionalNodeContract(declaration)
            }
            WorthQuerySemanticAspectDependencySource::ConditionalTruth(dependency) => {
                WorthQuerySemanticAspectDependencyView::ConditionalTruth(dependency)
            }
            WorthQuerySemanticAspectDependencySource::RealizedGraphCall {
                role,
                call_kind,
                evidence_identity,
                projection_result_digest,
                commit_graph_roles,
            } => WorthQuerySemanticAspectDependencyView::RealizedGraphCall {
                role,
                call_kind: *call_kind,
                evidence_identity,
                projection_result_digest: projection_result_digest.as_deref(),
                commit_graph_roles,
            },
            WorthQuerySemanticAspectDependencySource::RealizedWorkflowRead(evidence) => {
                WorthQuerySemanticAspectDependencyView::RealizedWorkflowRead(evidence)
            }
            WorthQuerySemanticAspectDependencySource::RealizedConditionalOutcome {
                class,
                signal_projection,
                observations,
            } => WorthQuerySemanticAspectDependencyView::RealizedConditionalOutcome {
                class: *class,
                signal_projection,
                observations,
            },
            WorthQuerySemanticAspectDependencySource::RealizedDirectOutput {
                execution,
                publication,
            } => WorthQuerySemanticAspectDependencyView::RealizedDirectOutput {
                execution,
                publication,
            },
            WorthQuerySemanticAspectDependencySource::RealizedWorkflowEffect(evidence) => {
                WorthQuerySemanticAspectDependencyView::RealizedWorkflowEffect(evidence)
            }
            WorthQuerySemanticAspectDependencySource::RealizedWorkflowInvariant(evidence) => {
                WorthQuerySemanticAspectDependencyView::RealizedWorkflowInvariant(evidence)
            }
            WorthQuerySemanticAspectDependencySource::RealizedWorkflowLineage(evidence) => {
                WorthQuerySemanticAspectDependencyView::RealizedWorkflowLineage(evidence)
            }
            WorthQuerySemanticAspectDependencySource::RealizedWorkflowOutput {
                receipt_identity,
                semantic_output,
                result_state,
            } => WorthQuerySemanticAspectDependencyView::RealizedWorkflowOutput {
                receipt_identity,
                semantic_output,
                result_state: *result_state,
            },
        }
    }

    pub(crate) fn conditional_location(
        &self,
    ) -> Option<&worth_query_installation::facade::WorthQueryConditionalNodeLocation> {
        match &self.locus {
            WorthQuerySemanticAspectDependencyLocus::ConditionalNode { location }
            | WorthQuerySemanticAspectDependencyLocus::ConditionalTruth { location, .. }
            | WorthQuerySemanticAspectDependencyLocus::ConditionalOutcome { location } => {
                Some(location)
            }
            _ => None,
        }
    }
}

impl WorthQueryCompiledSemanticAspectDependency {
    pub(crate) fn canonical_bucket(&self) -> usize {
        self.role.canonical_ordinal() * WorthQuerySemanticAspectDependencyLocus::KIND_COUNT
            + self.locus.kind_ordinal()
    }

    pub(super) fn semantic_role_and_locus_eq(&self, candidate: &Self) -> bool {
        self.role == candidate.role && self.locus == candidate.locus
    }

    pub(super) fn semantic_source_eq(&self, candidate: &Self) -> bool {
        self.source.semantic_replay_eq(&candidate.source)
    }

    pub(crate) fn parent_locus(&self) -> Option<WorthQuerySemanticAspectDependencyLocus> {
        use WorthQuerySemanticAspectDependencyLocus as Locus;
        match &self.locus {
            Locus::InstalledOperation => None,
            Locus::WorkflowStage { .. } => Some(Locus::InstalledOperation),
            Locus::WorkflowStageRead { stage_identity, .. }
            | Locus::WorkflowGraphCall { stage_identity, .. }
            | Locus::WorkflowPrimaryRead { stage_identity, .. }
            | Locus::WorkflowEffect { stage_identity, .. }
            | Locus::WorkflowInvariant { stage_identity, .. }
            | Locus::WorkflowLineage { stage_identity, .. }
            | Locus::WorkflowOutput { stage_identity } => Some(Locus::WorkflowStage {
                stage_identity: stage_identity.clone(),
            }),
            Locus::ConditionalNode { location } => location
                .stage_identity()
                .map(|stage_identity| Locus::WorkflowStage {
                    stage_identity: stage_identity.to_owned(),
                })
                .or(Some(Locus::InstalledOperation)),
            Locus::ConditionalTruth { location, .. } | Locus::ConditionalOutcome { location } => {
                Some(Locus::ConditionalNode {
                    location: location.clone(),
                })
            }
            _ => Some(Locus::InstalledOperation),
        }
    }

    pub(crate) fn workflow_stage_definition(&self) -> Option<(&str, &[String])> {
        match (&self.locus, &self.source) {
            (
                WorthQuerySemanticAspectDependencyLocus::WorkflowStage { stage_identity },
                WorthQuerySemanticAspectDependencySource::WorkflowStage { predecessors },
            ) => Some((stage_identity, predecessors)),
            _ => None,
        }
    }
}
