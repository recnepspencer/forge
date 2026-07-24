use std::sync::{Arc, Mutex};

use worth_query::facade::domain;

use super::graph_read_material;

#[derive(Clone, Copy, Debug)]
pub(super) struct RemoteA;
#[derive(Clone, Copy, Debug)]
pub(super) struct RemoteB;
#[derive(Clone, Copy, Debug)]
pub(super) struct SharedCommit;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FailAt {
    Observe,
    Project,
    Touch,
    Commit,
}

pub(super) struct SelectiveProvider {
    log: Arc<Mutex<Vec<&'static str>>>,
    fail_at: Option<FailAt>,
    expected_commit_roles: Option<Vec<&'static str>>,
    resource_support: domain::WorthQueryExecutionResourceSupport,
}

impl SelectiveProvider {
    pub(super) fn new(log: &Arc<Mutex<Vec<&'static str>>>, fail_at: Option<FailAt>) -> Self {
        Self {
            log: Arc::clone(log),
            fail_at,
            expected_commit_roles: None,
            resource_support: super::super::installed_operation_fixture::execution_resource_support(
            ),
        }
    }

    pub(super) fn new_with_support(
        log: &Arc<Mutex<Vec<&'static str>>>,
        resource_support: domain::WorthQueryExecutionResourceSupport,
    ) -> Self {
        Self {
            log: Arc::clone(log),
            fail_at: None,
            expected_commit_roles: None,
            resource_support,
        }
    }

    pub(super) fn commit(
        log: &Arc<Mutex<Vec<&'static str>>>,
        fail_at: Option<FailAt>,
        expected_roles: Vec<&'static str>,
    ) -> Self {
        Self {
            log: Arc::clone(log),
            fail_at,
            expected_commit_roles: Some(expected_roles),
            resource_support: super::super::installed_operation_fixture::execution_resource_support(
            ),
        }
    }

    pub(super) fn commit_with_support(
        log: &Arc<Mutex<Vec<&'static str>>>,
        expected_roles: Vec<&'static str>,
        resource_support: domain::WorthQueryExecutionResourceSupport,
    ) -> Self {
        Self {
            log: Arc::clone(log),
            fail_at: None,
            expected_commit_roles: Some(expected_roles),
            resource_support,
        }
    }

    fn contact_label(
        &self,
        label: &'static str,
        kind: FailAt,
    ) -> Result<&'static str, domain::WorthQueryGraphProviderFailure> {
        self.log.lock().unwrap().push(label);
        if self.fail_at == Some(kind) {
            Err(domain::WorthQueryGraphProviderFailure::new(label))
        } else {
            Ok(label)
        }
    }

    fn assert_graph_call_resources(call: &domain::WorthQueryGraphProviderCall) {
        assert_eq!(call.execution_resources().strategy(), "fixture-bounded");
        assert_eq!(
            call.resource_envelope().cancellation_safe_point().as_str(),
            "fixture-chunk-boundary"
        );
    }
}

impl<G> domain::WorthQueryGraphParticipationProvider<G> for SelectiveProvider {
    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        self.resource_support.clone()
    }

    fn observe(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Self::assert_graph_call_resources(call);
        Ok(call.completed(self.contact_label("observe", FailAt::Observe)?))
    }

    fn project(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Self::assert_graph_call_resources(call);
        self.log.lock().unwrap().push("project");
        if self.fail_at == Some(FailAt::Project) {
            Err(domain::WorthQueryGraphProviderFailure::new("project"))
        } else {
            call.projected(
                "project",
                graph_read_material("graph-provider-execution-projection"),
            )
        }
    }

    fn touch_effect(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Self::assert_graph_call_resources(call);
        Ok(call.completed(self.contact_label("touch", FailAt::Touch)?))
    }
}

impl domain::WorthQueryGraphCommitProvider<SharedCommit> for SelectiveProvider {
    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        self.resource_support.clone()
    }

    fn admit_commit(
        &self,
        call: &domain::WorthQueryGraphCommitCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        assert_eq!(call.execution_resources().strategy(), "fixture-bounded");
        assert_eq!(
            call.resource_envelope().cancellation_safe_point().as_str(),
            "fixture-chunk-boundary"
        );
        assert_eq!(
            call.graph_roles(),
            self.expected_commit_roles
                .as_deref()
                .expect("commit fixture declares the exact mutating role set")
        );
        Ok(call.completed(self.contact_label("commit", FailAt::Commit)?))
    }
}

pub(super) struct ReceiptOnlyProvider;

impl domain::WorthQueryGraphParticipationProvider<RemoteA> for ReceiptOnlyProvider {
    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        super::super::installed_operation_fixture::execution_resource_support()
    }

    fn observe(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("observe"))
    }

    fn project(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("dishonest-projection-receipt"))
    }

    fn touch_effect(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("touch"))
    }
}
