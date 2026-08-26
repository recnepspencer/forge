use std::marker::PhantomData;

use super::ApplicationQueryMarkerIdentity;
use crate::portable_identity::WorthQueryPortableTypeIdentity;

#[derive(Clone, Copy)]
struct ApplicationQueryDeclarationMembership;

pub struct ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope> {
    name: &'static str,
    query_type: WorthQueryPortableTypeIdentity,
    parameter_type: WorthQueryPortableTypeIdentity,
    result_type: WorthQueryPortableTypeIdentity,
    scope_type: WorthQueryPortableTypeIdentity,
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
            query_type: Query::QUERY_TYPE_IDENTITY,
            parameter_type: Query::PARAMETER_TYPE_IDENTITY,
            result_type: Query::RESULT_TYPE_IDENTITY,
            scope_type: Query::SCOPE_TYPE_IDENTITY,
            _membership: ApplicationQueryDeclarationMembership,
            _marker: PhantomData,
        }
    }

    pub const fn name(self) -> &'static str {
        self.name
    }

    pub const fn query_type(self) -> WorthQueryPortableTypeIdentity {
        self.query_type
    }

    pub const fn parameter_type(self) -> WorthQueryPortableTypeIdentity {
        self.parameter_type
    }

    pub const fn result_type(self) -> WorthQueryPortableTypeIdentity {
        self.result_type
    }

    pub const fn scope_type(self) -> WorthQueryPortableTypeIdentity {
        self.scope_type
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
