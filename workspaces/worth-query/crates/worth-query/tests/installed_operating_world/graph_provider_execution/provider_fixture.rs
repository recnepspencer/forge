use std::sync::{Arc, Mutex};

use worth_query::facade::domain;

use super::graph_projection_material;

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
}

impl SelectiveProvider {
    pub(super) fn new(log: &Arc<Mutex<Vec<&'static str>>>, fail_at: Option<FailAt>) -> Self {
        Self {
            log: Arc::clone(log),
            fail_at,
            expected_commit_roles: None,
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
}

impl<G> domain::WorthQueryGraphParticipationProvider<G> for SelectiveProvider {
    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        super::super::installed_operation_fixture::execution_resource_support()
    }

    fn observe(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed(self.contact_label("observe", FailAt::Observe)?))
    }

    fn project(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        self.log.lock().unwrap().push("project");
        if self.fail_at == Some(FailAt::Project) {
            Err(domain::WorthQueryGraphProviderFailure::new("project"))
        } else {
            Ok(call.projected(
                "project",
                graph_projection_material("graph-provider-execution-projection"),
            ))
        }
    }

    fn touch_effect(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed(self.contact_label("touch", FailAt::Touch)?))
    }
}

impl domain::WorthQueryGraphCommitProvider<SharedCommit> for SelectiveProvider {
    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        super::super::installed_operation_fixture::execution_resource_support()
    }

    fn admit_commit(
        &self,
        call: &domain::WorthQueryGraphCommitCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
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
