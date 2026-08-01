use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRequest,
    application_query::ApplicationQueryParameterSet, application_schema::ApplicationSchema,
};
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

mod finalization;
mod governed_access;
mod obligation_progression;

pub(in crate::domain_computation::primary_graph::application_query) use governed_access::{
    prepare_governed_access, prepare_retained_governance, WorthQueryGovernanceAdmission,
};

use super::{
    disclosure::WorthQueryApplicationQueryGovernance, WorthQueryAdmittedApplicationQueryPlan,
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryAdmissionDenial,
    WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryControls,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn admit_application_query<
        'a,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >(
        &'a self,
        query: &'a WorthQueryInstalledApplicationQuery<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        access: &WorthQueryApplicationQueryAccessContext<
            'a,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationQueryControls<'a, Schema>,
    ) -> Result<
        WorthQueryAdmittedApplicationQueryPlan<
            'a,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        let (parameters, controls) =
            self.prepare_application_query_admission(query, parameters, controls)?;
        self.finish_application_query_admission(query, access, parameters, controls, None)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn admit_governed_application_query<
        'a,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Capability,
        Operation,
        Input,
    >(
        &'a self,
        query: &'a WorthQueryInstalledApplicationQuery<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        access: &WorthQueryApplicationQueryAccessContext<
            'a,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        capability: crate::domain_computation::authorization::WorthQueryPreparedApplicationCapabilityAccess<
            Schema,
            Capability,
            Operation,
            Input,
        >,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationQueryControls<'a, Schema>,
    ) -> Result<
        WorthQueryAdmittedApplicationQueryPlan<
            'a,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        WorthQueryApplicationQueryAdmissionDenial,
    >
    where
        Capability: 'static,
        Operation: 'static,
        Input: ApplicationCapabilityRequest<Schema, Capability, Scope = Scope> + 'static,
    {
        let pending = prepare_governed_access(self, query, access, capability, &controls)?;
        let (parameters, controls) =
            self.prepare_application_query_admission(query, parameters, controls)?;
        self.finish_application_query_admission(query, access, parameters, controls, Some(pending))
    }

    pub(super) fn readmit_application_query_live<
        'a,
        Query,
        Parameters,
        QueryResult,
        Principal: 'static,
        PrincipalIdentity: 'static,
        Scope: 'static,
    >(
        &'a self,
        query: &'a WorthQueryInstalledApplicationQuery<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        access: &WorthQueryApplicationQueryAccessContext<
            'a,
            Schema,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        governance: WorthQueryApplicationQueryGovernance,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationQueryControls<'a, Schema>,
    ) -> Result<
        WorthQueryAdmittedApplicationQueryPlan<
            'a,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        let (parameters, controls) =
            self.prepare_application_query_admission(query, parameters, controls)?;
        if !governance.computation_matches(
            self.runtime.authority_identity(),
            query.identity(),
            parameters.identity(),
            access.principal().principal_entity_id(),
            access.scope().entity_id(),
        ) {
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::DisclosureAuthorizationMismatch,
                query.name(),
            ));
        }
        let pending = prepare_retained_governance(governance.into_pending());
        self.finish_application_query_admission(query, access, parameters, controls, pending)
    }
}

fn denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}
