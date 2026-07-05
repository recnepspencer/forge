use super::binding::PlanarBooleanOverlapReadinessLoopLedgerBinding;
use super::counters::PlanarBooleanOverlapRegionExtractionRequestCounters;
use super::denial::PlanarBooleanOverlapRegionExtractionRequestDenial;
use super::identity::overlap_region_extraction_request_identity;
use super::input::PlanarBooleanOverlapRegionExtractionRequestInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionExtractionRequest {
    request_identity: String,
    readiness_loop_ledger_binding: PlanarBooleanOverlapReadinessLoopLedgerBinding,
    counters: PlanarBooleanOverlapRegionExtractionRequestCounters,
}

impl PlanarBooleanOverlapRegionExtractionRequest {
    pub fn admit(
        input: PlanarBooleanOverlapRegionExtractionRequestInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapRegionExtractionRequestDenial> {
        let mut counters = PlanarBooleanOverlapRegionExtractionRequestCounters::default();
        let binding =
            PlanarBooleanOverlapReadinessLoopLedgerBinding::admit(&input).map_err(|denial| {
                PlanarBooleanOverlapRegionExtractionRequestDenial::from_binding_denial(
                    denial, counters,
                )
            })?;
        counters.consumed_readiness_binding();
        counters.consumed_loop_ledger_rows(binding.loop_ledger_row_identities().len());
        let request_identity = overlap_region_extraction_request_identity(&binding, counters);
        Ok(Self {
            request_identity,
            readiness_loop_ledger_binding: binding,
            counters,
        })
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn readiness_loop_ledger_binding(&self) -> &PlanarBooleanOverlapReadinessLoopLedgerBinding {
        &self.readiness_loop_ledger_binding
    }

    pub fn counters(&self) -> PlanarBooleanOverlapRegionExtractionRequestCounters {
        self.counters
    }

    pub fn certifies_overlap_region_extraction_request(&self) -> bool {
        !self.request_identity.is_empty()
            && !self
                .readiness_loop_ledger_binding
                .binding_identity()
                .is_empty()
            && self.counters.readiness_bindings_consumed() == 1
    }
}
