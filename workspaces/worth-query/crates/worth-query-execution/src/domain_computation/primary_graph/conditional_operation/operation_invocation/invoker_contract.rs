use crate::domain_computation::primary_graph::{
    WorthQueryApplicationEffectProgramBuilder,
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryTemporalInvocationFailureKind {
    ProjectionRejected,
    InvocationRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTemporalInvocationFailure {
    kind: WorthQueryTemporalInvocationFailureKind,
    detail: String,
}

impl WorthQueryTemporalInvocationFailure {
    pub fn new(kind: WorthQueryTemporalInvocationFailureKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryTemporalInvocationFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Application behavior invoked only after Query has freshly admitted the
/// exact installed operation. The invoker describes decision reads and domain
/// effects; Query owns temporal-intent advancement and the final commit.
pub trait WorthQueryTemporalOperationInvoker<Schema, Operation, Input, Scope>:
    Send + Sync + 'static
{
    const SEMANTIC_IDENTITY: &'static str;
    type Projection: 'static;

    fn preconditions(
        &self,
        _input: &Input,
    ) -> worth_query_declaration::facade::application_schema::TypedMutationPreconditions<
        Schema,
        Operation,
        Scope,
    > {
        Default::default()
    }

    fn project(
        &self,
        input: &Input,
        reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
            '_,
            '_,
            Schema,
            Operation,
        >,
        scope: &WorthQueryInvariantEntityIdentity<Schema, Scope>,
    ) -> Result<Self::Projection, WorthQueryTemporalInvocationFailure>;

    fn apply(
        &self,
        input: Input,
        projection: Self::Projection,
        effects: &mut WorthQueryApplicationEffectProgramBuilder<Schema, Operation, Input, Scope>,
    ) -> Result<(), WorthQueryTemporalInvocationFailure>;
}
