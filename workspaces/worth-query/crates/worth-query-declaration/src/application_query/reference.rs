use std::marker::PhantomData;

use super::ApplicationQueryMarkerIdentity;
use crate::portable_identity::WorthQueryPortableTypeIdentity;

#[derive(Clone, Copy)]
struct ApplicationQueryDeclarationMembership;

pub struct ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope> {
    name: &'static str,
    query_type: &'static str,
    parameter_type: &'static str,
    result_type: &'static str,
    scope_type: &'static str,
    _membership: ApplicationQueryDeclarationMembership,
    _marker: PhantomData<fn(Parameters) -> (Schema, Query, QueryResult, Scope)>,
}

impl<Schema, Query, Parameters, QueryResult, Scope>
    ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>
{
    pub const fn from_declaration() -> Self
    where
        Query: ApplicationQueryMarkerIdentity<
            Schema = Schema,
            Parameters = Parameters,
            QueryResult = QueryResult,
            Scope = Scope,
        >,
    {
        Self {
            name: Query::IDENTIFIER,
            query_type: Query::QUERY_TYPE_NAME,
            parameter_type: Query::PARAMETER_TYPE_NAME,
            result_type: Query::RESULT_TYPE_NAME,
            scope_type: Query::SCOPE_TYPE_NAME,
            _membership: ApplicationQueryDeclarationMembership,
            _marker: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn query_type(&self) -> WorthQueryPortableTypeIdentity {
        WorthQueryPortableTypeIdentity::declared(self.query_type)
    }

    pub const fn parameter_type(&self) -> WorthQueryPortableTypeIdentity {
        WorthQueryPortableTypeIdentity::declared(self.parameter_type)
    }

    pub const fn result_type(&self) -> WorthQueryPortableTypeIdentity {
        WorthQueryPortableTypeIdentity::declared(self.result_type)
    }

    pub const fn scope_type(&self) -> WorthQueryPortableTypeIdentity {
        WorthQueryPortableTypeIdentity::declared(self.scope_type)
    }
}

impl<Schema, Query, Parameters, QueryResult, Scope> Clone
    for ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Schema, Query, Parameters, QueryResult, Scope> Copy
    for ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>
{
}
