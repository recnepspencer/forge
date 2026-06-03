use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

use super::snapshot::prepare_realization_snapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionRealizationExhaustionStatus {
    NotApplicable,
    NotExhausted,
    Exhausted,
}

impl PrimitiveConstructionRealizationExhaustionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::NotExhausted => "not_exhausted",
            Self::Exhausted => "exhausted",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRealizationExhaustionReport {
    family: PrimitiveConstructionFamily,
    status: PrimitiveConstructionRealizationExhaustionStatus,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    conditioning_witness: Option<PrimitiveConditioningWitness>,
    report_digest: String,
}

impl PrimitiveConstructionRealizationExhaustionReport {
    pub(crate) fn from_snapshot(
        snapshot: &super::snapshot::PrimitiveConstructionRealizationSnapshot,
    ) -> Self {
        let status = if snapshot.exhaustion_reason().is_some() {
            PrimitiveConstructionRealizationExhaustionStatus::Exhausted
        } else if snapshot.stability_class() == Some(PrimitiveStabilityClass::StableAfterEscalation)
        {
            PrimitiveConstructionRealizationExhaustionStatus::NotExhausted
        } else if snapshot.stability_class() == Some(PrimitiveStabilityClass::StableDirect) {
            PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
        } else if snapshot.conditioning_witness().is_some() {
            PrimitiveConstructionRealizationExhaustionStatus::NotExhausted
        } else {
            PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
        };
        let report_digest = digest_owned_parts(&[
            snapshot.family().as_str().to_string(),
            status.as_str().to_string(),
            snapshot
                .attempted_strategies()
                .iter()
                .map(|strategy| strategy.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            snapshot
                .stability_class()
                .map(PrimitiveStabilityClass::as_str)
                .unwrap_or("none")
                .to_string(),
            snapshot
                .exhaustion_reason()
                .map(PrimitiveRealizationExhaustionReason::as_str)
                .unwrap_or("none")
                .to_string(),
            snapshot.realization_digest().to_string(),
        ]);
        Self {
            family: snapshot.family(),
            status,
            attempted_strategies: snapshot.attempted_strategies().to_vec(),
            stability_class: snapshot.stability_class(),
            exhaustion_reason: snapshot.exhaustion_reason(),
            conditioning_witness: snapshot.conditioning_witness().cloned(),
            report_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn status(&self) -> PrimitiveConstructionRealizationExhaustionStatus {
        self.status
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub fn conditioning_witness(&self) -> Option<&PrimitiveConditioningWitness> {
        self.conditioning_witness.as_ref()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_realization_exhaustion_report(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionRealizationExhaustionReport {
    PrimitiveConstructionRealizationExhaustionReport::from_snapshot(&prepare_realization_snapshot(
        intent.into_request(),
    ))
}
