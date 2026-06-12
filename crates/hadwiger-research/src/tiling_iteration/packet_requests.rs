use crate::research_cockpit::ResearchCockpitSession;

use super::packet_artifacts::TilingIterationPacketKind;
use super::packet_errors::{require_iteration_non_empty, TilingIterationError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingIterationPacketRequest {
    packet_id: String,
    packet_kind: TilingIterationPacketKind,
    cockpit_session: Option<ResearchCockpitSession>,
    evidence_basis: Vec<String>,
    required_checker_lanes: Vec<String>,
    expected_information_gain: Option<String>,
    reactivation_obligations: Vec<String>,
}

impl TilingIterationPacketRequest {
    pub fn lower_bound_obstruction(
        packet_id: impl Into<String>,
    ) -> TilingIterationPacketRequestBuilder {
        TilingIterationPacketRequestBuilder::new(
            packet_id,
            TilingIterationPacketKind::LowerBoundObstruction,
        )
    }

    pub fn upper_bound_periodic_quotient(
        packet_id: impl Into<String>,
    ) -> TilingIterationPacketRequestBuilder {
        TilingIterationPacketRequestBuilder::new(
            packet_id,
            TilingIterationPacketKind::UpperBoundPeriodicQuotient,
        )
    }

    pub(crate) fn packet_id(&self) -> &str {
        &self.packet_id
    }

    pub fn packet_kind(&self) -> TilingIterationPacketKind {
        self.packet_kind
    }

    pub fn cockpit_session(&self) -> Option<&ResearchCockpitSession> {
        self.cockpit_session.as_ref()
    }

    pub fn required_checker_lanes(&self) -> &[String] {
        &self.required_checker_lanes
    }

    pub fn evidence_basis(&self) -> &[String] {
        &self.evidence_basis
    }

    pub fn expected_information_gain(&self) -> Option<&str> {
        self.expected_information_gain.as_deref()
    }

    pub fn reactivation_obligations(&self) -> &[String] {
        &self.reactivation_obligations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingIterationPacketRequestBuilder {
    packet_id: String,
    packet_kind: TilingIterationPacketKind,
    cockpit_session: Option<ResearchCockpitSession>,
    evidence_basis: Vec<String>,
    required_checker_lanes: Vec<String>,
    reactivation_obligations: Vec<String>,
}

impl TilingIterationPacketRequestBuilder {
    fn new(packet_id: impl Into<String>, packet_kind: TilingIterationPacketKind) -> Self {
        Self {
            packet_id: packet_id.into(),
            packet_kind,
            cockpit_session: None,
            evidence_basis: Vec::new(),
            required_checker_lanes: Vec::new(),
            reactivation_obligations: Vec::new(),
        }
    }

    pub fn from_cockpit_session(mut self, session: &ResearchCockpitSession) -> Self {
        self.cockpit_session = Some(session.clone());
        self
    }

    pub fn with_required_checker_lane(self, lane: impl Into<String>) -> Self {
        self.try_with_required_checker_lane(lane)
            .expect("required checker lane must be non-empty")
    }

    pub fn try_with_required_checker_lane(
        mut self,
        lane: impl Into<String>,
    ) -> Result<Self, TilingIterationError> {
        let lane = require_iteration_non_empty(lane, "required_checker_lane")?;
        self.required_checker_lanes.push(lane);
        Ok(self)
    }

    pub fn with_evidence_basis(self, basis: impl Into<String>) -> Self {
        self.try_with_evidence_basis(basis)
            .expect("evidence basis must be non-empty")
    }

    pub fn try_with_evidence_basis(
        mut self,
        basis: impl Into<String>,
    ) -> Result<Self, TilingIterationError> {
        let basis = require_iteration_non_empty(basis, "evidence_basis")?;
        self.evidence_basis.push(basis);
        Ok(self)
    }

    pub fn with_reactivation_obligation(self, obligation: impl Into<String>) -> Self {
        self.try_with_reactivation_obligation(obligation)
            .expect("reactivation obligation must be non-empty")
    }

    pub fn try_with_reactivation_obligation(
        mut self,
        obligation: impl Into<String>,
    ) -> Result<Self, TilingIterationError> {
        let obligation = require_iteration_non_empty(obligation, "reactivation_obligation")?;
        self.reactivation_obligations.push(obligation);
        Ok(self)
    }

    pub fn with_expected_information_gain(
        self,
        gain: impl Into<String>,
    ) -> Result<TilingIterationPacketRequest, TilingIterationError> {
        let packet_id = require_iteration_non_empty(self.packet_id, "packet_id")?;
        let expected_information_gain =
            require_iteration_non_empty(gain, "expected_information_gain")?;
        if self.required_checker_lanes.is_empty() {
            return Err(TilingIterationError::MissingRequiredCheckerLane);
        }
        if self.evidence_basis.is_empty() {
            return Err(TilingIterationError::MissingEvidenceBasis);
        }
        if self.reactivation_obligations.is_empty() {
            return Err(TilingIterationError::MissingReactivationObligation);
        }
        let mut evidence_basis = self.evidence_basis;
        evidence_basis.sort();
        evidence_basis.dedup();
        let mut required_checker_lanes = self.required_checker_lanes;
        required_checker_lanes.sort();
        required_checker_lanes.dedup();
        let mut reactivation_obligations = self.reactivation_obligations;
        reactivation_obligations.sort();
        reactivation_obligations.dedup();
        Ok(TilingIterationPacketRequest {
            packet_id,
            packet_kind: self.packet_kind,
            cockpit_session: self.cockpit_session,
            evidence_basis,
            required_checker_lanes,
            expected_information_gain: Some(expected_information_gain),
            reactivation_obligations,
        })
    }
}
