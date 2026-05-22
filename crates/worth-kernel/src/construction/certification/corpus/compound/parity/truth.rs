use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

use super::super::builder::{
    build_exhaustion_witness_parity_report_from_siege, build_grazing_boundary_report_from_siege,
    build_motion_parity_report_from_siege,
};
use super::super::ordering_report::PrimitiveConstructionCompoundOrderingParityReport;
use super::super::report::{
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundMotionParityReport,
};
use super::registry::{compound_parity_registry, PrimitiveConstructionCompoundParityRegistry};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundParityCanonicalTruth {
    siege: PrimitiveConstructionCompoundAdversarialSiegeReport,
    registry: PrimitiveConstructionCompoundParityRegistry,
    truth_digest: String,
}

impl PrimitiveConstructionCompoundParityCanonicalTruth {
    pub fn from_siege(siege: &PrimitiveConstructionCompoundAdversarialSiegeReport) -> Self {
        let registry = compound_parity_registry();
        let truth_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &[
                siege.report_digest().to_string(),
                registry.registry_digest().to_string(),
            ],
        );
        Self {
            siege: siege.clone(),
            registry,
            truth_digest,
        }
    }

    pub fn siege(&self) -> &PrimitiveConstructionCompoundAdversarialSiegeReport {
        &self.siege
    }

    pub fn truth_digest(&self) -> &str {
        &self.truth_digest
    }

    pub fn expected_ordering(&self) -> PrimitiveConstructionCompoundOrderingParityReport {
        PrimitiveConstructionCompoundOrderingParityReport::new(self.siege.lane_reports().to_vec())
    }

    pub fn expected_motion(
        &self,
    ) -> Result<
        PrimitiveConstructionCompoundMotionParityReport,
        super::super::builder::PrimitiveConstructionCompoundAdversarialSiegeError,
    > {
        build_motion_parity_report_from_siege(&self.siege)
    }

    pub fn expected_grazing(
        &self,
    ) -> Result<
        PrimitiveConstructionCompoundGrazingBoundaryReport,
        super::super::builder::PrimitiveConstructionCompoundAdversarialSiegeError,
    > {
        build_grazing_boundary_report_from_siege(&self.siege)
    }

    pub fn expected_exhaustion(
        &self,
    ) -> Result<
        PrimitiveConstructionCompoundExhaustionWitnessParityReport,
        super::super::builder::PrimitiveConstructionCompoundAdversarialSiegeError,
    > {
        build_exhaustion_witness_parity_report_from_siege(&self.siege)
    }

    pub fn ordering_matches(
        &self,
        report: &PrimitiveConstructionCompoundOrderingParityReport,
    ) -> bool {
        *report == self.expected_ordering()
    }

    pub fn motion_matches(
        &self,
        report: &PrimitiveConstructionCompoundMotionParityReport,
    ) -> Result<bool, super::super::builder::PrimitiveConstructionCompoundAdversarialSiegeError>
    {
        Ok(*report == self.expected_motion()?)
    }

    pub fn grazing_matches(
        &self,
        report: &PrimitiveConstructionCompoundGrazingBoundaryReport,
    ) -> Result<bool, super::super::builder::PrimitiveConstructionCompoundAdversarialSiegeError>
    {
        Ok(*report == self.expected_grazing()?)
    }

    pub fn exhaustion_matches(
        &self,
        report: &PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    ) -> Result<bool, super::super::builder::PrimitiveConstructionCompoundAdversarialSiegeError>
    {
        Ok(*report == self.expected_exhaustion()?)
    }
}
