use worth_query::facade::{
    domain,
    foundation::{
        ConsumedFieldValueFact, ConsumedNativeValueView, WorthQueryConsumedProjectionAuthority,
    },
};

use crate::{
    WorthUiQueryMeasurementFactObservation, WorthUiQueryMeasurementFactObservationError,
    WorthUiQueryMeasurementRefinementCounters,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSettledMeasurementFactBatch {
    observations: Box<[WorthUiQueryMeasurementFactObservation]>,
    refinement_counters: WorthUiQueryMeasurementRefinementCounters,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiSettledSnapshotSourceGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiSettledSnapshotSourceOrder(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiSettledSnapshotFact {
    settlement_identity: String,
    query_binding_identity: String,
    measurement_facts:
        Result<WorthUiSettledMeasurementFactBatch, WorthUiQueryMeasurementFactObservationError>,
    native_facts: Box<[ConsumedFieldValueFact]>,
    result_state: domain::WorthQueryOperationResultState,
    execution_warning_count: usize,
    projection_warning_count: usize,
    source_generation: Option<WorthUiSettledSnapshotSourceGeneration>,
    source_order: Option<WorthUiSettledSnapshotSourceOrder>,
}

impl WorthUiSettledMeasurementFactBatch {
    fn from_query_authority(
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<Self, WorthUiQueryMeasurementFactObservationError> {
        let (observations, refinement_counters) =
            WorthUiQueryMeasurementFactObservation::from_query_authority(authority)?;
        Ok(Self {
            observations,
            refinement_counters,
        })
    }

    pub fn observations(&self) -> &[WorthUiQueryMeasurementFactObservation] {
        &self.observations
    }

    pub fn refinement_counters(&self) -> WorthUiQueryMeasurementRefinementCounters {
        self.refinement_counters
    }
}

impl WorthUiSettledSnapshotFact {
    pub(super) fn from_settled<D, O, F, L>(
        settled: &domain::WorthQuerySettledDomainProjection<D, O, F, L>,
    ) -> Self
    where
        L: worth_query::facade::foundation::BasisOperationLane,
    {
        Self {
            settlement_identity: settled.identity().to_owned(),
            query_binding_identity: settled.execution_receipt().binding_identity().to_owned(),
            measurement_facts: WorthUiSettledMeasurementFactBatch::from_query_authority(
                settled.authority(),
            ),
            native_facts: settled
                .authority()
                .facts()
                .display_fields()
                .iter()
                .chain(settled.authority().facts().derived_fields())
                .cloned()
                .collect(),
            result_state: settled.result_state(),
            execution_warning_count: settled.warnings().len(),
            projection_warning_count: settled
                .projection_warnings()
                .map_or(0, |warnings| warnings.warning_kinds().len()),
            source_generation: None,
            source_order: None,
        }
    }

    pub(crate) fn attach_source_coordinates(
        &mut self,
        generation: WorthUiSettledSnapshotSourceGeneration,
        order: WorthUiSettledSnapshotSourceOrder,
    ) {
        debug_assert!(self.source_generation.is_none());
        debug_assert!(self.source_order.is_none());
        self.source_generation = Some(generation);
        self.source_order = Some(order);
    }

    pub fn settlement_identity(&self) -> &str {
        &self.settlement_identity
    }

    pub fn query_binding_identity(&self) -> &str {
        &self.query_binding_identity
    }

    pub fn measurement_facts(
        &self,
    ) -> Result<&WorthUiSettledMeasurementFactBatch, WorthUiQueryMeasurementFactObservationError>
    {
        self.measurement_facts.as_ref().map_err(|error| *error)
    }

    pub fn projected_measurement_fact_count(&self) -> usize {
        self.measurement_facts.as_ref().map_or(0, |facts| {
            facts.refinement_counters.projected_measurement_fact_count()
        })
    }

    pub fn native_fact_count(&self) -> usize {
        self.native_facts.len()
    }

    pub fn native_fact(&self, index: usize) -> Option<ConsumedNativeValueView<'_>> {
        self.native_facts.get(index).map(|fact| fact.native_value())
    }

    pub fn result_state(&self) -> domain::WorthQueryOperationResultState {
        self.result_state
    }

    pub fn is_partial(&self) -> bool {
        self.result_state == domain::WorthQueryOperationResultState::Partial
    }

    pub fn execution_warning_count(&self) -> usize {
        self.execution_warning_count
    }

    pub fn projection_warning_count(&self) -> usize {
        self.projection_warning_count
    }

    pub fn warning_count(&self) -> usize {
        self.execution_warning_count + self.projection_warning_count
    }

    pub fn source_generation(&self) -> Option<WorthUiSettledSnapshotSourceGeneration> {
        self.source_generation
    }

    pub fn source_order(&self) -> Option<WorthUiSettledSnapshotSourceOrder> {
        self.source_order
    }
}

impl WorthUiSettledSnapshotSourceGeneration {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl WorthUiSettledSnapshotSourceOrder {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}
