use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    WorthQueryDomainHandleDenial, WorthQueryDomainInstallationGeneration,
    WorthQueryInstalledDomainAuthority,
};
use crate::domain_installation::WorthQueryInstalledOperationGraphBinding;

type InstalledOperationMarker<D, O, F> = fn() -> (D, O, F);

#[derive(Debug)]
pub struct WorthQueryInstalledDomainOperation<D, O, F> {
    domain_authority: Arc<WorthQueryInstalledDomainAuthority>,
    operation_authority:
        Arc<worth_query_installation::facade::WorthQueryInstalledDomainOperationAuthority>,
    workflow_graph: Option<Arc<super::super::WorthQueryInstalledWorkflowGraph>>,
    graph_bindings: Vec<WorthQueryInstalledOperationGraphBinding>,
    marker: PhantomData<InstalledOperationMarker<D, O, F>>,
}

impl<D, O, F> WorthQueryInstalledDomainOperation<D, O, F> {
    pub(crate) fn mint(
        domain_authority: Arc<WorthQueryInstalledDomainAuthority>,
        operation_authority: Arc<
            worth_query_installation::facade::WorthQueryInstalledDomainOperationAuthority,
        >,
        workflow_graph: Option<Arc<super::super::WorthQueryInstalledWorkflowGraph>>,
        graph_bindings: Vec<WorthQueryInstalledOperationGraphBinding>,
    ) -> Self {
        Self {
            domain_authority,
            operation_authority,
            workflow_graph,
            graph_bindings,
            marker: PhantomData,
        }
    }

    pub fn definition(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryPortableDomainOperationDefinition {
        self.operation_authority.definition()
    }

    pub fn operation_slot(&self) -> String {
        self.operation_authority.operation_slot()
    }

    pub fn domain_owner(&self) -> &str {
        self.domain_authority.domain_owner()
    }

    pub fn installation_generation(&self) -> WorthQueryDomainInstallationGeneration {
        self.domain_authority.installation_generation()
    }

    pub(crate) fn semantic_correspondence_candidate<G: 'static>(
        &self,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
        graph: &super::super::WorthQueryInstalledGraphParticipation<G>,
        source_record_identity: Option<
            worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
        >,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
        worth_runtime_bridge::facade::BridgeCorrespondenceDenial,
    > {
        if !self.domain_authority.is_current_installation_generation() {
            return Err(
                worth_runtime_bridge::facade::BridgeCorrespondenceDenial::without_admission(
                    worth_runtime_bridge::facade::BridgeCorrespondenceDenialKind::StaleQueryInstallation,
                ),
            );
        }
        let query_dependency = self
            .operation_authority
            .conditional_dependency(location, dependency_ordinal)
            .map_err(|_| {
                worth_runtime_bridge::facade::BridgeCorrespondenceDenial::without_admission(
                    worth_runtime_bridge::facade::BridgeCorrespondenceDenialKind::PortableDependencyNotOwnedByOperation,
                )
            })?;
        let dependency = query_dependency.dependency();
        let graph_role = dependency.graph_read_role().as_str();
        let graph_binding = self.graph_bindings.iter().find(|binding| {
            binding.role == graph_role && binding.graph_marker == std::any::TypeId::of::<G>()
        });
        if graph_binding.is_none()
            || graph.record.definition.role != graph_role
            || graph.record.runtime_authority != self.domain_authority.runtime_authority().as_u64()
        {
            return Err(
                worth_runtime_bridge::facade::BridgeCorrespondenceDenial::without_admission(
                    worth_runtime_bridge::facade::BridgeCorrespondenceDenialKind::GraphParticipationNotOwnedByOperation,
                ),
            );
        }
        worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate::from_query_authority(
            query_dependency,
            Arc::clone(&graph.record.installation_authority),
            source_record_identity,
        )
    }

    pub fn semantic_correspondence_registration<G: 'static>(
        &self,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
        graph: &super::super::WorthQueryInstalledGraphParticipation<G>,
        source_record_identity: Option<
            worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
        >,
        targets: Vec<worth_runtime_bridge::facade::BridgeSignalAspectTargetDeclaration>,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeSemanticCorrespondenceRegistration,
        worth_runtime_bridge::facade::BridgeCorrespondenceDenial,
    > {
        let dependency = self.semantic_correspondence_candidate(
            location,
            dependency_ordinal,
            graph,
            source_record_identity,
        )?;
        worth_runtime_bridge::facade::BridgeSemanticCorrespondenceRegistration::new(
            dependency, targets,
        )
    }

    pub(crate) fn domain_authority(&self) -> &Arc<WorthQueryInstalledDomainAuthority> {
        &self.domain_authority
    }

    pub(crate) fn operation_authority(
        &self,
    ) -> &Arc<worth_query_installation::facade::WorthQueryInstalledDomainOperationAuthority> {
        &self.operation_authority
    }

    pub(crate) fn workflow_graph(
        &self,
    ) -> Option<&Arc<super::super::WorthQueryInstalledWorkflowGraph>> {
        self.workflow_graph.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainOperationLookupCounters {
    pub authority_checks: usize,
    pub indexed_operation_lookups: usize,
    pub package_content_scans: usize,
    pub planning_steps: usize,
    pub lower_runtime_contacts: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledDomainOperationLookupDenialKind {
    DomainAuthority,
    OperationNotInstalled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainOperationLookupDenial {
    kind: WorthQueryInstalledDomainOperationLookupDenialKind,
    domain_denial: Option<WorthQueryDomainHandleDenial>,
    counters: WorthQueryInstalledDomainOperationLookupCounters,
}

impl WorthQueryInstalledDomainOperationLookupDenial {
    pub(crate) fn domain(denial: WorthQueryDomainHandleDenial) -> Self {
        Self {
            kind: WorthQueryInstalledDomainOperationLookupDenialKind::DomainAuthority,
            domain_denial: Some(denial),
            counters: WorthQueryInstalledDomainOperationLookupCounters {
                authority_checks: 1,
                ..WorthQueryInstalledDomainOperationLookupCounters::default()
            },
        }
    }

    pub(crate) fn operation_not_installed() -> Self {
        Self {
            kind: WorthQueryInstalledDomainOperationLookupDenialKind::OperationNotInstalled,
            domain_denial: None,
            counters: WorthQueryInstalledDomainOperationLookupCounters {
                authority_checks: 1,
                indexed_operation_lookups: 1,
                ..WorthQueryInstalledDomainOperationLookupCounters::default()
            },
        }
    }

    pub fn kind(&self) -> WorthQueryInstalledDomainOperationLookupDenialKind {
        self.kind
    }

    pub fn domain_denial(&self) -> Option<&WorthQueryDomainHandleDenial> {
        self.domain_denial.as_ref()
    }

    pub fn counters(&self) -> WorthQueryInstalledDomainOperationLookupCounters {
        self.counters
    }
}
