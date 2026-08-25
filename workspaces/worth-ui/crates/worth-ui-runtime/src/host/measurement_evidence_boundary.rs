use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity, WorthUiHostCapabilityReport,
    WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::UiMeasurementResult;
use std::cell::RefCell;
use std::rc::Rc;

use super::{
    admit_fresh_host_evidence, construct_freshness_witness, normalize_host_observation,
    observe_host_measurement, UiHostMeasurementEvidenceDenial, UiHostMeasurementFreshnessWitness,
    UiHostMeasurementNeed, UiHostMeasurementNormalizationContext,
};

#[derive(Debug, Default)]
pub(crate) struct UiHostMeasurementSourceAuthority {
    next_source_order: u64,
}

impl UiHostMeasurementSourceAuthority {
    pub(crate) fn advance_past(&mut self, retained: &UiMeasurementResult) {
        let (_, _, retained_order) = retained.host_source_position();
        self.next_source_order = self.next_source_order.max(retained_order);
    }

    fn seal(
        &mut self,
        result: &mut UiMeasurementResult,
    ) -> Result<(), UiHostMeasurementEvidenceDenial> {
        let source_order = self
            .next_source_order
            .checked_add(1)
            .ok_or(UiHostMeasurementEvidenceDenial::SourceOrderExhausted)?;
        self.next_source_order = source_order;
        result.seal_host_source_position(
            source_order,
            result.evidence_generation().as_u64(),
            source_order,
        );
        Ok(())
    }
}

/// Persistent capability for host observation and source-coordinate admission.
#[derive(Clone, Debug)]
pub struct WorthUiHostMeasurementCollector {
    authority: Rc<RefCell<UiHostMeasurementSourceAuthority>>,
}

pub struct UiHostMeasurementCollectionInput<'a> {
    pub identity: UiMeasurementRequestIdentity,
    pub evidence_family: UiMeasurementEvidenceFamily,
    pub need: UiHostMeasurementNeed,
    pub capability_report: &'a WorthUiHostCapabilityReport,
    pub evidence_generation: UiEvidenceAuthorityGeneration,
    pub normalization_context: UiHostMeasurementNormalizationContext,
}

impl WorthUiHostMeasurementCollector {
    pub(crate) fn new(authority: Rc<RefCell<UiHostMeasurementSourceAuthority>>) -> Self {
        Self { authority }
    }

    pub(crate) fn for_internal_proof() -> Self {
        Self::new(Rc::new(RefCell::new(Default::default())))
    }

    pub(crate) fn collect<A: WorthUiMeasurementHostAdapter + ?Sized>(
        &self,
        adapter: &A,
        input: UiHostMeasurementCollectionInput<'_>,
    ) -> Result<UiMeasurementResult, UiHostMeasurementEvidenceDenial> {
        collect_host_measurement_evidence(&mut self.authority.borrow_mut(), adapter, input)
    }

    /// Collect, normalize, freshness-admit, and source-position one host fact.
    pub(crate) fn collect_admitted<A: WorthUiMeasurementHostAdapter + ?Sized>(
        &self,
        adapter: &A,
        input: UiHostMeasurementCollectionInput<'_>,
    ) -> Result<crate::host::UiAdmittedHostMeasurement, UiHostMeasurementEvidenceDenial> {
        self.collect(adapter, input)
            .map(crate::host::UiAdmittedHostMeasurement::from_collected)
    }
}

pub(crate) fn collect_host_measurement_evidence<A: WorthUiMeasurementHostAdapter + ?Sized>(
    source: &mut UiHostMeasurementSourceAuthority,
    adapter: &A,
    input: UiHostMeasurementCollectionInput<'_>,
) -> Result<UiMeasurementResult, UiHostMeasurementEvidenceDenial> {
    let UiHostMeasurementCollectionInput {
        identity,
        evidence_family,
        need,
        capability_report,
        evidence_generation,
        normalization_context,
    } = input;
    let observation =
        observe_host_measurement(adapter, identity, evidence_family, need, capability_report)?;
    let mut normalized =
        normalize_host_observation(observation, evidence_generation, normalization_context)?;
    let freshness_witness = construct_freshness_witness(
        evidence_generation,
        normalization_context.assumption_profile(),
    );
    admit_fresh_host_evidence(&normalized, freshness_witness)?;
    source.seal(&mut normalized)?;
    Ok(normalized)
}

pub fn admit_current_host_measurement_evidence<'a>(
    result: &'a UiMeasurementResult,
    freshness_witness: UiHostMeasurementFreshnessWitness,
) -> Result<crate::evidence::UiCurrentMeasurementResult<'a>, UiHostMeasurementEvidenceDenial> {
    admit_fresh_host_evidence(result, freshness_witness)
}
