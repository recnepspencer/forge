use crate::domain_operation::{
    WorthQueryConditionalObservationView, WorthQueryHostConditionalPredicateProvider,
    WorthQueryHostPredicateDecision, WorthQueryHostPredicateFailure,
};

use super::{
    WorthQueryConditionalApplicationOperationDenial,
    WorthQueryConditionalApplicationOperationDenialKind,
    WorthQueryInstalledApplicationConditionalNode,
};

/// Move-only installed host predicate bound to one exact conditional node.
pub struct WorthQueryInstalledHostConditionalProvider<
    Schema,
    ApplicationOperation,
    Input,
    D,
    O,
    F,
    N,
    Provider,
> {
    node: WorthQueryInstalledApplicationConditionalNode<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
        N,
    >,
    provider: Provider,
}

impl<Schema, ApplicationOperation, Input, D, O, F, N>
    WorthQueryInstalledApplicationConditionalNode<Schema, ApplicationOperation, Input, D, O, F, N>
{
    pub fn bind_host_predicate_provider<Provider>(
        self,
        provider: Provider,
    ) -> Result<
        WorthQueryInstalledHostConditionalProvider<
            Schema,
            ApplicationOperation,
            Input,
            D,
            O,
            F,
            N,
            Provider,
        >,
        WorthQueryConditionalApplicationOperationDenial,
    >
    where
        Provider: WorthQueryHostConditionalPredicateProvider<N>,
    {
        validate_provider_identity::<Provider, N>()?;
        Ok(WorthQueryInstalledHostConditionalProvider {
            node: self,
            provider,
        })
    }
}

impl<Schema, ApplicationOperation, Input, D, O, F, N, Provider>
    WorthQueryInstalledHostConditionalProvider<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
        N,
        Provider,
    >
where
    Provider: WorthQueryHostConditionalPredicateProvider<N>,
{
    pub fn node(
        &self,
    ) -> &WorthQueryInstalledApplicationConditionalNode<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
        N,
    > {
        &self.node
    }

    pub fn provider_semantic_identity(&self) -> &'static str {
        Provider::SEMANTIC_IDENTITY
    }

    #[doc(hidden)]
    pub fn evaluate_for_runtime(
        &self,
        observation: WorthQueryConditionalObservationView<'_>,
    ) -> Result<WorthQueryHostPredicateDecision, WorthQueryHostPredicateFailure> {
        self.provider.evaluate(observation)
    }
}

fn validate_provider_identity<Provider, Node>(
) -> Result<(), WorthQueryConditionalApplicationOperationDenial>
where
    Provider: WorthQueryHostConditionalPredicateProvider<Node>,
{
    validate_semantic_identity(Provider::SEMANTIC_IDENTITY)
}

fn validate_semantic_identity(
    identity: &str,
) -> Result<(), WorthQueryConditionalApplicationOperationDenial> {
    if identity.is_empty()
        || identity.trim() != identity
        || identity.chars().any(char::is_whitespace)
    {
        Err(WorthQueryConditionalApplicationOperationDenial::new(
            WorthQueryConditionalApplicationOperationDenialKind::ProviderIdentityInvalid,
            identity,
        ))
    } else {
        Ok(())
    }
}
