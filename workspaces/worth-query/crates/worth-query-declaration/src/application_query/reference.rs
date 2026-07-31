use std::marker::PhantomData;

pub struct ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope> {
    name: &'static str,
    _marker: PhantomData<fn(Parameters) -> (Schema, Query, QueryResult, Scope)>,
}

impl<Schema, Query, Parameters, QueryResult, Scope>
    ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>
{
    #[doc(hidden)]
    pub const fn from_schema_identifier(name: &'static str) -> Self {
        Self {
            name,
            _marker: PhantomData,
        }
    }

    pub const fn name(self) -> &'static str {
        self.name
    }

    pub fn query_type(self) -> &'static str {
        std::any::type_name::<Query>()
    }

    pub fn parameter_type(self) -> &'static str {
        std::any::type_name::<Parameters>()
    }

    pub fn result_type(self) -> &'static str {
        std::any::type_name::<QueryResult>()
    }

    pub fn scope_type(self) -> &'static str {
        std::any::type_name::<Scope>()
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
