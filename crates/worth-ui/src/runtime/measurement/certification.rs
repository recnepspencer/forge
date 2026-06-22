use forge_foundational::{
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture,
};

use super::counter_family::{WorthUiCounterAuthority, WorthUiRuntimeCounterFamily};
use super::counter_packet::is_foundational_label;
use super::counter_packet::WorthUiMeasurementCounterPacket;
use super::denial::WorthUiMeasurementCertificationDenial;
use super::measurement_boundary::WorthUiMeasurementBoundary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthUiCounterCaptureRichness {
    Minimal,
    Standard,
    Full,
    Support,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthUiComplexityContract {
    name: &'static str,
    required_boundary: Option<WorthUiMeasurementBoundary>,
    required_family: Option<WorthUiRuntimeCounterFamily>,
    foundational_boundary: FoundationalPerformanceBoundary,
    evidence_strength: FoundationalPerformanceEvidenceStrength,
    breadth_locality: FoundationalPerformanceBreadthLocalityPosture,
    access_pattern: FoundationalPerformanceAccessPatternPosture,
    execution_temperature: FoundationalPerformanceExecutionTemperature,
    freshness_retention: FoundationalPerformanceFreshnessRetentionPosture,
    fallback_debt: FoundationalPerformanceFallbackDebtPosture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthUiCertifiedMeasurementPacket {
    packet: WorthUiMeasurementCounterPacket,
    contract: WorthUiComplexityContract,
}

impl WorthUiComplexityContract {
    pub fn hot_path(name: &'static str) -> Self {
        Self {
            name,
            required_boundary: None,
            required_family: None,
            foundational_boundary: FoundationalPerformanceBoundary::AuthoritativeExecution,
            evidence_strength:
                FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            breadth_locality: FoundationalPerformanceBreadthLocalityPosture::PointLocal,
            access_pattern: FoundationalPerformanceAccessPatternPosture::PointLookup,
            execution_temperature: FoundationalPerformanceExecutionTemperature::HotPath,
            freshness_retention:
                FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent,
            fallback_debt: FoundationalPerformanceFallbackDebtPosture::Verified,
        }
    }

    pub fn requires_boundary(mut self, boundary: WorthUiMeasurementBoundary) -> Self {
        self.required_boundary = Some(boundary);
        self
    }

    pub fn requires_counter_family(mut self, family: WorthUiRuntimeCounterFamily) -> Self {
        self.required_family = Some(family);
        self
    }

    pub fn foundational_boundary(mut self, boundary: FoundationalPerformanceBoundary) -> Self {
        self.foundational_boundary = boundary;
        self
    }

    pub fn evidence_strength(
        mut self,
        evidence_strength: FoundationalPerformanceEvidenceStrength,
    ) -> Self {
        self.evidence_strength = evidence_strength;
        self
    }

    pub fn breadth_locality(
        mut self,
        breadth_locality: FoundationalPerformanceBreadthLocalityPosture,
    ) -> Self {
        self.breadth_locality = breadth_locality;
        self
    }

    pub fn access_pattern(
        mut self,
        access_pattern: FoundationalPerformanceAccessPatternPosture,
    ) -> Self {
        self.access_pattern = access_pattern;
        self
    }

    pub fn execution_temperature(
        mut self,
        execution_temperature: FoundationalPerformanceExecutionTemperature,
    ) -> Self {
        self.execution_temperature = execution_temperature;
        self
    }

    pub fn freshness_retention(
        mut self,
        freshness_retention: FoundationalPerformanceFreshnessRetentionPosture,
    ) -> Self {
        self.freshness_retention = freshness_retention;
        self
    }

    pub fn fallback_debt(
        mut self,
        fallback_debt: FoundationalPerformanceFallbackDebtPosture,
    ) -> Self {
        self.fallback_debt = fallback_debt;
        self
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn required_boundary(&self) -> Option<WorthUiMeasurementBoundary> {
        self.required_boundary
    }

    pub fn required_family(&self) -> Option<WorthUiRuntimeCounterFamily> {
        self.required_family
    }

    pub fn foundational_boundary_value(&self) -> FoundationalPerformanceBoundary {
        self.foundational_boundary
    }

    pub fn evidence_strength_value(&self) -> FoundationalPerformanceEvidenceStrength {
        self.evidence_strength
    }

    pub fn breadth_locality_value(&self) -> FoundationalPerformanceBreadthLocalityPosture {
        self.breadth_locality
    }

    pub fn access_pattern_value(&self) -> FoundationalPerformanceAccessPatternPosture {
        self.access_pattern
    }

    pub fn execution_temperature_value(&self) -> FoundationalPerformanceExecutionTemperature {
        self.execution_temperature
    }

    pub fn freshness_retention_value(&self) -> FoundationalPerformanceFreshnessRetentionPosture {
        self.freshness_retention
    }

    pub fn fallback_debt_value(&self) -> FoundationalPerformanceFallbackDebtPosture {
        self.fallback_debt
    }
}

impl WorthUiCertifiedMeasurementPacket {
    pub(crate) fn certify(
        packet: WorthUiMeasurementCounterPacket,
        contract: WorthUiComplexityContract,
    ) -> Result<Self, WorthUiMeasurementCertificationDenial> {
        let required_boundary = contract
            .required_boundary
            .ok_or(WorthUiMeasurementCertificationDenial::HotPathWithoutMeasurementBoundary)?;
        if required_boundary != packet.boundary() {
            return Err(WorthUiMeasurementCertificationDenial::BoundaryNotRequiredByContract);
        }
        let required_family = contract
            .required_family
            .ok_or(WorthUiMeasurementCertificationDenial::CounterFamilyNotRequiredByContract)?;
        if required_family != packet.family() {
            return Err(WorthUiMeasurementCertificationDenial::CounterFamilyNotRequiredByContract);
        }
        if packet.boundary().foundational_boundary() != contract.foundational_boundary {
            return Err(WorthUiMeasurementCertificationDenial::FoundationalBoundaryMismatch);
        }
        if !is_foundational_label(contract.name) {
            return Err(WorthUiMeasurementCertificationDenial::InvalidFoundationalContractName);
        }
        if contract.evidence_strength
            != FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt
        {
            return Err(
                WorthUiMeasurementCertificationDenial::FoundationalEvidenceStrengthMismatch,
            );
        }
        if packet.counters().is_empty() {
            return Err(WorthUiMeasurementCertificationDenial::EmptyCounterPacket);
        }
        if !packet
            .counters()
            .iter()
            .any(|counter| counter.certifies_execution_work())
        {
            return Err(WorthUiMeasurementCertificationDenial::ElapsedTimeOnlyFrameCost);
        }
        if !packet
            .counters()
            .iter()
            .any(|counter| counter.certifies_execution_work() && counter.value() > 0)
        {
            return Err(
                WorthUiMeasurementCertificationDenial::MissingNonzeroWorthUiCounterEvidence,
            );
        }
        if packet.family().authority() == WorthUiCounterAuthority::ForgeQueryEvidence
            && packet.query_evidence().is_empty()
        {
            return Err(WorthUiMeasurementCertificationDenial::MissingQueryEvidence);
        }
        Ok(Self { packet, contract })
    }

    pub fn packet(&self) -> &WorthUiMeasurementCounterPacket {
        &self.packet
    }

    pub fn contract(&self) -> &WorthUiComplexityContract {
        &self.contract
    }

    pub fn replay_digest(&self) -> u64 {
        self.packet.replay_digest()
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.packet.active_plan_digest()
    }
}
