use std::any::{Any, TypeId};
use std::sync::Arc;

/// Provider-owner declaration of the complete behavior that is relevant to
/// conditional continuity. Bridge compares the associated contract's concrete
/// type and typed `Eq` value; provider implementation identity is intentionally
/// reserved for exact execution affinity.
pub trait BridgeConditionalProviderSemantics: Send + Sync + 'static {
    type SemanticContract: Eq + Send + Sync + 'static;

    fn semantic_contract(&self) -> Self::SemanticContract;
}

#[derive(Clone)]
pub(super) struct BridgeErasedProviderSemanticContract {
    contract_type: TypeId,
    contract: Arc<dyn Any + Send + Sync>,
    equivalent: fn(&dyn Any, &dyn Any) -> bool,
}

impl BridgeErasedProviderSemanticContract {
    pub(super) fn capture<P>(provider: &P) -> Self
    where
        P: BridgeConditionalProviderSemantics,
    {
        fn equivalent<C: Eq + 'static>(left: &dyn Any, right: &dyn Any) -> bool {
            left.downcast_ref::<C>()
                .zip(right.downcast_ref::<C>())
                .is_some_and(|(left, right)| left == right)
        }

        Self {
            contract_type: TypeId::of::<P::SemanticContract>(),
            contract: Arc::new(provider.semantic_contract()),
            equivalent: equivalent::<P::SemanticContract>,
        }
    }

    pub(super) fn is_equivalent_to(&self, candidate: &Self) -> bool {
        self.contract_type == candidate.contract_type
            && (self.equivalent)(self.contract.as_ref(), candidate.contract.as_ref())
    }
}

#[derive(Clone, Default)]
pub(super) struct BridgeConditionalProviderSemanticContracts {
    pub(super) condition: Option<BridgeErasedProviderSemanticContract>,
    pub(super) dependency_comparator: Option<BridgeErasedProviderSemanticContract>,
    pub(super) output_comparator: Option<BridgeErasedProviderSemanticContract>,
    pub(super) reuse_comparator: Option<BridgeErasedProviderSemanticContract>,
    pub(super) trigger: Option<BridgeErasedProviderSemanticContract>,
    pub(super) wake: Option<BridgeErasedProviderSemanticContract>,
    pub(super) compute: Option<BridgeErasedProviderSemanticContract>,
}
