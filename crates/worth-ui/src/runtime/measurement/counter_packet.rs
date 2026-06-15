use super::certification::WorthUiCounterCaptureRichness;
use super::counter_family::WorthUiRuntimeCounterFamily;
use super::denial::WorthUiMeasurementCertificationDenial;
use super::frame_cost_counter::{WorthUiCounterValueKind, WorthUiFrameCostCounter};
use super::measurement_boundary::WorthUiMeasurementBoundary;
use super::query_evidence::WorthUiMeasurementQueryEvidence;
use super::replay_digest::packet_digest;
use super::{WorthUiCertifiedMeasurementPacket, WorthUiComplexityContract};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthUiMeasurementCounterPacket {
    family: WorthUiRuntimeCounterFamily,
    boundary: WorthUiMeasurementBoundary,
    capture_richness: WorthUiCounterCaptureRichness,
    rows: Vec<WorthUiFrameCostCounter>,
    query_evidence: Vec<WorthUiMeasurementQueryEvidence>,
    replay_digest: u64,
    active_plan_digest: u64,
}

#[derive(Debug, Clone)]
pub struct WorthUiCounterPacketBuilder {
    family: WorthUiRuntimeCounterFamily,
    boundary: WorthUiMeasurementBoundary,
    capture_richness: WorthUiCounterCaptureRichness,
    rows: Vec<WorthUiFrameCostCounter>,
    query_evidence: Vec<WorthUiMeasurementQueryEvidence>,
    active_plan_digest: u64,
}

impl WorthUiCounterPacketBuilder {
    pub(crate) fn new(
        family: WorthUiRuntimeCounterFamily,
        boundary: WorthUiMeasurementBoundary,
    ) -> Self {
        Self {
            family,
            boundary,
            capture_richness: WorthUiCounterCaptureRichness::Standard,
            rows: Vec::new(),
            query_evidence: Vec::new(),
            active_plan_digest: 0,
        }
    }

    pub fn record(mut self, counter: WorthUiFrameCostCounter) -> Self {
        self.rows.push(counter);
        self
    }

    pub fn with_query_evidence(mut self, evidence: WorthUiMeasurementQueryEvidence) -> Self {
        self.query_evidence.push(evidence);
        self
    }

    pub fn with_active_plan_digest(mut self, active_plan_digest: u64) -> Self {
        self.active_plan_digest = active_plan_digest;
        self
    }

    pub fn with_capture_richness(
        mut self,
        capture_richness: WorthUiCounterCaptureRichness,
    ) -> Self {
        self.capture_richness = capture_richness;
        self
    }

    pub fn seal(
        mut self,
    ) -> Result<WorthUiMeasurementCounterPacket, WorthUiMeasurementCertificationDenial> {
        if self.family.allowed_boundary() != self.boundary {
            return Err(WorthUiMeasurementCertificationDenial::CounterFamilyBoundaryMismatch);
        }
        if self
            .rows
            .iter()
            .any(|row| row.value_kind() == WorthUiCounterValueKind::UnattributedWorkBucket)
        {
            return Err(WorthUiMeasurementCertificationDenial::UnattributedWorkBucket);
        }
        if self
            .rows
            .iter()
            .any(|row| !counter_name_matches_family(self.family, row.name()))
        {
            return Err(
                WorthUiMeasurementCertificationDenial::CounterNameDoesNotMatchFamilyBoundary,
            );
        }
        if self
            .rows
            .iter()
            .any(|row| !is_foundational_label(row.name()))
        {
            return Err(WorthUiMeasurementCertificationDenial::InvalidFoundationalCounterName);
        }
        self.rows.sort();
        if self
            .rows
            .windows(2)
            .any(|window| window[0].name() == window[1].name())
        {
            return Err(WorthUiMeasurementCertificationDenial::DuplicateCounterName);
        }
        self.query_evidence.sort();
        let replay_digest = packet_digest(
            self.family,
            self.boundary,
            self.capture_richness,
            self.active_plan_digest,
            &self.rows,
            &self.query_evidence,
        );
        Ok(WorthUiMeasurementCounterPacket {
            family: self.family,
            boundary: self.boundary,
            capture_richness: self.capture_richness,
            rows: self.rows,
            query_evidence: self.query_evidence,
            replay_digest,
            active_plan_digest: self.active_plan_digest,
        })
    }
}

fn counter_name_matches_family(family: WorthUiRuntimeCounterFamily, name: &str) -> bool {
    name.strip_prefix(family.token())
        .is_some_and(|remainder| remainder.starts_with('.'))
}

pub(crate) fn is_foundational_label(name: &str) -> bool {
    !name.trim().is_empty()
        && name.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '.' | '_' | '-')
        })
}

impl WorthUiMeasurementCounterPacket {
    pub fn family(&self) -> WorthUiRuntimeCounterFamily {
        self.family
    }

    pub fn boundary(&self) -> WorthUiMeasurementBoundary {
        self.boundary
    }

    pub fn counters(&self) -> &[WorthUiFrameCostCounter] {
        &self.rows
    }

    pub fn query_evidence(&self) -> &[WorthUiMeasurementQueryEvidence] {
        &self.query_evidence
    }

    pub fn capture_richness(&self) -> WorthUiCounterCaptureRichness {
        self.capture_richness
    }

    pub fn replay_digest(&self) -> u64 {
        self.replay_digest
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn certify_against(
        self,
        contract: WorthUiComplexityContract,
    ) -> Result<WorthUiCertifiedMeasurementPacket, WorthUiMeasurementCertificationDenial> {
        WorthUiCertifiedMeasurementPacket::certify(self, contract)
    }
}
