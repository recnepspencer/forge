use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    WorthQueryDomainHandleDenial, WorthQueryDomainInstallationGeneration,
    WorthQueryInstalledDomainAuthority,
};
use crate::domain_installation::WorthQueryInstalledOperationGraphBinding;

mod bridge_correspondence;

type InstalledOperationMarker<D, O, F> = fn() -> (D, O, F);

#[derive(Debug)]
pub struct WorthQueryInstalledDomainOperation<D, O, F> {
    domain_authority: Arc<WorthQueryInstalledDomainAuthority>,
    operation_authority:
        Arc<worth_query_installation::facade::WorthQueryInstalledDomainOperationAuthority>,
    workflow_graph: Option<Arc<super::super::WorthQueryInstalledWorkflowGraph>>,
    graph_bindings: Vec<WorthQueryInstalledOperationGraphBinding>,
    lookup_counters: WorthQueryInstalledDomainOperationLookupCounters,
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
        lookup_counters: WorthQueryInstalledDomainOperationLookupCounters,
    ) -> Self {
        Self {
            domain_authority,
            operation_authority,
            workflow_graph,
            graph_bindings,
            lookup_counters,
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

    pub const fn lookup_counters(&self) -> WorthQueryInstalledDomainOperationLookupCounters {
        self.lookup_counters
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
    pub graph_binding_lookups: usize,
    pub graph_bindings_retained: usize,
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
