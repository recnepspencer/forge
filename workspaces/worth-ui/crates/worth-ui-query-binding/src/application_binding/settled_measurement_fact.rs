use worth_query::facade::installed::operation;

use super::WorthUiSnapshotNativeAccess;
use crate::{
    WorthUiAdmittedQueryBindingReference, WorthUiAdmittedQuerySettlementReference,
    WorthUiNativeKeyResolutionCounters, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactObservationError, WorthUiQueryMeasurementRefinementCounters,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSettledMeasurementFactBatch {
    observations: Box<[WorthUiQueryMeasurementFactObservation]>,
    refinement_counters: WorthUiQueryMeasurementRefinementCounters,
    key_resolution_counters: WorthUiNativeKeyResolutionCounters,
    native_access_counters: operation::WorthQueryNativeAccessCounters,
    native_access_binding_counters: Option<WorthUiNativeAccessBindingCounters>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiNativeAccessBindingCounters {
    declared_key_routes: usize,
    declared_key_layout_checks: usize,
    lane_shape_checks: usize,
    fact_scans: usize,
    row_scans: usize,
    path_parses: usize,
    view_registry_inspections: usize,
    domain_registry_inspections: usize,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiSettledSnapshotSourceGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiSettledSnapshotSourceOrder(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiSettledSnapshotFact {
    binding_reference: WorthUiAdmittedQueryBindingReference,
    settlement_reference: WorthUiAdmittedQuerySettlementReference,
    measurement_facts: WorthUiSettledMeasurementFactBatch,
    result_state: operation::WorthQueryOperationResultState,
    execution_warning_count: usize,
    projection_warning_count: usize,
    source_generation: Option<WorthUiSettledSnapshotSourceGeneration>,
    source_order: Option<WorthUiSettledSnapshotSourceOrder>,
}

impl WorthUiSettledMeasurementFactBatch {
    fn from_native_access(
        access: &operation::WorthQueryNativeFieldAccess<'_>,
        key_resolution_counters: WorthUiNativeKeyResolutionCounters,
        native_access_binding_counters: Option<WorthUiNativeAccessBindingCounters>,
    ) -> Result<Self, WorthUiQueryMeasurementFactObservationError> {
        let (observations, refinement_counters) =
            WorthUiQueryMeasurementFactObservation::from_native_access(access)?;
        Ok(Self {
            observations,
            refinement_counters,
            key_resolution_counters,
            native_access_counters: access.counters(),
            native_access_binding_counters,
        })
    }

    pub fn observations(&self) -> &[WorthUiQueryMeasurementFactObservation] {
        &self.observations
    }

    pub fn refinement_counters(&self) -> WorthUiQueryMeasurementRefinementCounters {
        self.refinement_counters
    }

    pub fn key_resolution_counters(&self) -> WorthUiNativeKeyResolutionCounters {
        self.key_resolution_counters
    }

    pub fn native_access_counters(&self) -> operation::WorthQueryNativeAccessCounters {
        self.native_access_counters
    }

    pub fn native_access_binding_counters(&self) -> Option<WorthUiNativeAccessBindingCounters> {
        self.native_access_binding_counters
    }
}

impl WorthUiSettledSnapshotFact {
    pub(crate) fn from_settled<D, O, F, L>(
        settled: &operation::WorthQuerySettledDomainProjection<D, O, F, L>,
        native_access: &WorthUiSnapshotNativeAccess,
        binding_reference: WorthUiAdmittedQueryBindingReference,
        settlement_reference: WorthUiAdmittedQuerySettlementReference,
    ) -> Result<Self, WorthUiQueryMeasurementFactObservationError>
    where
        L: worth_query::facade::foundation::BasisOperationLane,
    {
        let native_access_binding_counters =
            settled.native_access_binding_counters().map(|counters| {
                WorthUiNativeAccessBindingCounters {
                    declared_key_routes: counters.declared_key_routes,
                    declared_key_layout_checks: counters.declared_key_layout_checks,
                    lane_shape_checks: counters.lane_shape_checks,
                    fact_scans: counters.fact_scans,
                    row_scans: counters.row_scans,
                    path_parses: counters.path_parses,
                    view_registry_inspections: counters.view_registry_inspections,
                    domain_registry_inspections: counters.domain_registry_inspections,
                }
            });
        let measurement_facts = settled
            .native_value(native_access.key(), 0)
            .map_err(|denial| {
                WorthUiQueryMeasurementFactObservationError::NativeAccess(Box::new(denial))
            })
            .and_then(|access| {
                WorthUiSettledMeasurementFactBatch::from_native_access(
                    &access,
                    native_access.resolution_counters(),
                    native_access_binding_counters,
                )
            })?;
        Ok(Self {
            binding_reference,
            settlement_reference,
            measurement_facts,
            result_state: settled.result_state(),
            execution_warning_count: settled.warnings().len(),
            projection_warning_count: settled
                .projection_warnings()
                .map_or(0, |warnings| warnings.warning_kinds().len()),
            source_generation: None,
            source_order: None,
        })
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

    pub fn measurement_facts(&self) -> &WorthUiSettledMeasurementFactBatch {
        &self.measurement_facts
    }

    pub fn projected_measurement_fact_count(&self) -> usize {
        self.measurement_facts
            .refinement_counters
            .projected_measurement_fact_count()
    }

    pub fn binding_reference(&self) -> &WorthUiAdmittedQueryBindingReference {
        &self.binding_reference
    }

    pub fn settlement_reference(&self) -> &WorthUiAdmittedQuerySettlementReference {
        &self.settlement_reference
    }

    pub fn result_state(&self) -> operation::WorthQueryOperationResultState {
        self.result_state
    }

    pub fn is_partial(&self) -> bool {
        self.result_state == operation::WorthQueryOperationResultState::Partial
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

impl WorthUiNativeAccessBindingCounters {
    pub fn declared_key_routes(self) -> usize {
        self.declared_key_routes
    }

    pub fn declared_key_layout_checks(self) -> usize {
        self.declared_key_layout_checks
    }

    pub fn lane_shape_checks(self) -> usize {
        self.lane_shape_checks
    }

    pub fn fact_scans(self) -> usize {
        self.fact_scans
    }

    pub fn row_scans(self) -> usize {
        self.row_scans
    }

    pub fn path_parses(self) -> usize {
        self.path_parses
    }

    pub fn view_registry_inspections(self) -> usize {
        self.view_registry_inspections
    }

    pub fn domain_registry_inspections(self) -> usize {
        self.domain_registry_inspections
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
