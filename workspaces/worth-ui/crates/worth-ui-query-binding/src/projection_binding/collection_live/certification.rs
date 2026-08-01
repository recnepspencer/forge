use worth_query::facade::{
    installed::{collection, observation},
    runtime,
};

use super::UiLiveCollectionProjection;

type QueryPatch = collection::WorthQueryCollectionPatch;
type QueryReceipt = collection::WorthQueryCollectionPatchApplicationReceipt;

impl UiLiveCollectionProjection {
    pub(crate) fn certification_plan_patch_twins(
        &mut self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> (QueryPatch, QueryPatch) {
        let delivery = self
            .lease
            .drain(workspace)
            .expect("certification collection delivery drains");
        let delta = self
            .lease
            .consumer_invalidation_delta(delivery)
            .expect("certification collection delta derives");
        let admitted = self
            .lease
            .admit_consumer_invalidation_delta(delta, workspace)
            .unwrap_or_else(|stop| {
                panic!(
                    "certification collection invalidation stopped: {:?}",
                    stop.kind()
                )
            });
        self.consumer
            .bind_shared_target(&admitted, workspace)
            .expect("certification consumer binds");
        (
            required_patch(&mut self.consumer, &admitted, workspace),
            required_patch(&mut self.consumer, &admitted, workspace),
        )
    }

    pub(crate) fn certification_plan_patch_for_target(
        &mut self,
        target: &mut Self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> (QueryPatch, QueryPatch) {
        let delivery = self
            .lease
            .drain(workspace)
            .expect("certification shared delivery drains");
        let delta = self
            .lease
            .consumer_invalidation_delta(delivery)
            .expect("certification shared delta derives");
        let admitted = self
            .lease
            .admit_consumer_invalidation_delta(delta, workspace)
            .unwrap_or_else(|stop| {
                panic!(
                    "certification shared invalidation stopped: {:?}",
                    stop.kind()
                )
            });
        self.consumer
            .bind_shared_target(&admitted, workspace)
            .expect("source consumer binds shared target");
        target
            .consumer
            .bind_shared_target(&admitted, workspace)
            .expect("target consumer binds shared target");
        (
            required_patch(&mut self.consumer, &admitted, workspace),
            required_patch(&mut self.consumer, &admitted, workspace),
        )
    }

    #[allow(
        clippy::result_large_err,
        reason = "certification fault injection preserves the exact Query patch denial"
    )]
    pub(crate) fn certification_apply_patch(
        &mut self,
        patch: QueryPatch,
    ) -> Result<QueryReceipt, collection::WorthQueryCollectionDeliveryDenial> {
        self.consumer.apply_patch(patch)
    }

    pub(crate) fn certification_derive_fact(
        &self,
        receipt: &QueryReceipt,
    ) -> crate::UiCollectionProjectionFactReceipt {
        crate::projection_consumption::derive_applied_collection_projection(
            crate::projection_consumption::UiCollectionDerivationContext {
                binding: &self.binding,
                consumer: &self.consumer,
                accesses: &self.accesses,
                budget: self.budget,
            },
            receipt,
        )
    }

    pub(crate) fn certification_row_identities(&self) -> Vec<crate::UiQueryEvidenceReference> {
        self.consumer
            .rows()
            .iter()
            .map(|row| {
                crate::UiQueryEvidenceReference::query_issued(
                    &row.entity_identity().evidence_identity(),
                )
            })
            .collect()
    }
}

fn required_patch(
    consumer: &mut collection::WorthQueryCollectionConsumerWindow,
    admitted: &observation::WorthQueryAdmittedConsumerInvalidation<'_>,
    workspace: &runtime::WorthQueryWorkspace,
) -> QueryPatch {
    match consumer.plan_patch(admitted, workspace) {
        collection::WorthQueryCollectionDeliveryOutcome::Patch(patch) => patch,
        collection::WorthQueryCollectionDeliveryOutcome::NoDelivery(stop) => {
            panic!(
                "certification semantic mutation did not produce a patch: {:?}",
                stop.kind()
            )
        }
    }
}
