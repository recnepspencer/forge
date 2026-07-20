use super::{
    WorthServerAdmittedDurableProductMutation, WorthServerDurableProductMutationConclusion,
    WorthServerDurableProductMutationExecution, WorthServerDurableProductMutationRecoveryHandle,
    WorthServerProductDurabilityCapability,
};

pub trait WorthServerDurableProductMutationExecutor: Send + Sync + 'static {
    fn capability(&self) -> WorthServerProductDurabilityCapability;

    fn execute(
        &self,
        attempt: &WorthServerAdmittedDurableProductMutation,
    ) -> WorthServerDurableProductMutationExecution;

    fn resolve(
        &self,
        recovery: &WorthServerDurableProductMutationRecoveryHandle,
    ) -> WorthServerDurableProductMutationConclusion;
}
