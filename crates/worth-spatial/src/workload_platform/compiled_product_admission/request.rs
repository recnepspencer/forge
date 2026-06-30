use crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt;
use crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;
use crate::spatial_compiled_product_family::SpatialCompiledProductConsumer;
use crate::workload_platform::evidence_ledger::SelectedLookupSliceLedger;
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use crate::workload_platform::retained_cancellation_chain::RetainedCancellationChainReceipt;

pub(crate) enum SpatialCompiledProductAdmissionRequest<'a> {
    EvidenceLookupLedger {
        consumer: SpatialCompiledProductConsumer,
        selected_plan: &'a EvidenceLookupSelectedPlan,
        ledger: &'a SelectedLookupSliceLedger,
    },
    EvidenceLookupProduct {
        consumer: SpatialCompiledProductConsumer,
        selected_plan: &'a EvidenceLookupSelectedPlan,
        product: &'a EvidenceLookupIndexProduct,
    },
    RetainedReplay {
        historical: &'a RetainedPlanarHistoricalInspection,
        retained: &'a RetainedPlanarFactsReceipt,
        projection: &'a ProjectionConsumedPlanarFactsReceipt,
    },
    RetainedCancellation {
        receipt: &'a RetainedCancellationChainReceipt,
    },
}

impl<'a> SpatialCompiledProductAdmissionRequest<'a> {
    pub(crate) fn for_evidence_lookup_ledger(
        consumer: SpatialCompiledProductConsumer,
        selected_plan: &'a EvidenceLookupSelectedPlan,
        ledger: &'a SelectedLookupSliceLedger,
    ) -> Self {
        Self::EvidenceLookupLedger {
            consumer,
            selected_plan,
            ledger,
        }
    }

    pub(crate) fn for_evidence_lookup_product(
        consumer: SpatialCompiledProductConsumer,
        selected_plan: &'a EvidenceLookupSelectedPlan,
        product: &'a EvidenceLookupIndexProduct,
    ) -> Self {
        Self::EvidenceLookupProduct {
            consumer,
            selected_plan,
            product,
        }
    }

    pub(crate) fn for_retained_replay(
        historical: &'a RetainedPlanarHistoricalInspection,
        retained: &'a RetainedPlanarFactsReceipt,
        projection: &'a ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        Self::RetainedReplay {
            historical,
            retained,
            projection,
        }
    }

    pub(crate) fn for_retained_cancellation(receipt: &'a RetainedCancellationChainReceipt) -> Self {
        Self::RetainedCancellation { receipt }
    }

    pub(crate) fn consumer(&self) -> SpatialCompiledProductConsumer {
        match self {
            Self::EvidenceLookupLedger { consumer, .. }
            | Self::EvidenceLookupProduct { consumer, .. } => *consumer,
            Self::RetainedReplay { .. } => SpatialCompiledProductConsumer::RetainedReplayParity,
            Self::RetainedCancellation { .. } => {
                SpatialCompiledProductConsumer::RetainedCancellationChain
            }
        }
    }
}
