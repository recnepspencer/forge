use worth_query_declaration::facade::{
    application_query::ApplicationQueryParameterSet, application_schema::ApplicationSchema,
};
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

use super::{
    affinity::validate_continuation_affinity, authority::WorthQueryApplicationQueryContinuation,
};
use crate::domain_computation::primary_graph::{
    application_query::{
        graph_read_plan_binding::WorthQueryAdmittedContinuationState,
        WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationQueryAccessContext,
        WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
        WorthQueryApplicationQueryControls, WorthQueryApplicationQueryResumeControls,
    },
    WorthQueryPrimaryGraphApplicationRuntime,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn readmit_application_query_continuation<
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
        continuation: WorthQueryApplicationQueryContinuation<
            Schema,
            Query,
            Parameters,
            QueryResult,
            Scope,
        >,
        controls: WorthQueryApplicationQueryResumeControls<'a>,
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
        let affinity =
            validate_continuation_affinity(self, query, access.scope().entity_id(), continuation)?;
        let query_controls = WorthQueryApplicationQueryControls::continuation_resume(
            affinity.basis_version,
            controls,
        );
        let (parameters, query_controls) =
            self.prepare_application_query_admission(query, access, parameters, query_controls)?;
        if !parameters
            .canonical_basis()
            .is_equivalent_to(&affinity.parameter_basis)
        {
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::ContinuationParameterMismatch,
                query.name(),
            ));
        }
        let mut plan =
            self.finish_application_query_admission(query, access, parameters, query_controls)?;
        plan.continuation_state = Some(WorthQueryAdmittedContinuationState {
            expected_generation: affinity.index_generation,
            boundary: affinity.boundary,
            page_ordinal: affinity.page_ordinal,
        });
        Ok(plan)
    }
}

fn denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}
