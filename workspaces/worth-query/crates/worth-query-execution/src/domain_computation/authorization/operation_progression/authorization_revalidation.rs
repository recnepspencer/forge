//! Exact operation-session authorization revalidation.

use worth_query_installation::facade::ApplicationSchema;

use crate::domain_computation::authorization::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryRetainedAuthorizationDecisionFacts, WorthQueryRetainedCapabilityAuthorization,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession;

pub(in crate::domain_computation::authorization) struct WorthQueryOperationAuthorizationRevalidation<
    'a,
    Schema,
> {
    _permit: &'a super::WorthQueryCapabilityTransitionPermit,
    runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    authorization: WorthQueryRetainedCapabilityAuthorization,
    graph_work: WorthQueryManagedGraphWorkSession,
}

impl<'a, Schema> WorthQueryOperationAuthorizationRevalidation<'a, Schema>
where
    Schema: ApplicationSchema,
{
    pub(in crate::domain_computation::authorization) fn bind(
        permit: &'a super::WorthQueryCapabilityTransitionPermit,
        runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        authorization: WorthQueryRetainedCapabilityAuthorization,
        graph_work: WorthQueryManagedGraphWorkSession,
    ) -> Self {
        Self {
            _permit: permit,
            runtime,
            authorization,
            graph_work,
        }
    }

    pub(in crate::domain_computation::authorization) fn revalidate(
        mut self,
        subject: &str,
    ) -> Result<
        (
            WorthQueryRetainedAuthorizationDecisionFacts,
            WorthQueryManagedGraphWorkSession,
        ),
        WorthQueryOperationAuthorizationDenial,
    > {
        self.runtime
            .refresh_capability_authorization_for_graph_work(
                &mut self.authorization,
                &self.graph_work,
            )?;
        self.graph_work
            .set_retained_decision_facts(self.authorization.exact_fact_count());
        let session = self.graph_work.identity();
        let authorization =
            WorthQueryRetainedAuthorizationDecisionFacts::capability(self.authorization);
        if !authorization.belongs_to_session(session) {
            return Err(WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                subject,
            ));
        }
        Ok((authorization, self.graph_work))
    }
}
