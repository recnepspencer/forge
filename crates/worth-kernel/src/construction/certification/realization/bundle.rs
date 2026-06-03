use crate::construction::intent::PrimitiveConstructionIntent;

use super::{
    snapshot::{prepare_realization_snapshot, PrimitiveConstructionRealizationSnapshot},
    PrimitiveConstructionConditioningWitnessReport,
    PrimitiveConstructionRealizationExhaustionReport,
    PrimitiveConstructionRealizationStrategyReport, PrimitiveConstructionStabilityClassReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRealizationReportBundle {
    strategy_report: PrimitiveConstructionRealizationStrategyReport,
    conditioning_witness_report: PrimitiveConstructionConditioningWitnessReport,
    stability_class_report: PrimitiveConstructionStabilityClassReport,
    exhaustion_report: PrimitiveConstructionRealizationExhaustionReport,
}

impl PrimitiveConstructionRealizationReportBundle {
    fn from_snapshot(snapshot: PrimitiveConstructionRealizationSnapshot) -> Self {
        let strategy_report =
            PrimitiveConstructionRealizationStrategyReport::from_snapshot(&snapshot);
        let conditioning_witness_report =
            PrimitiveConstructionConditioningWitnessReport::from_snapshot(&snapshot);
        let stability_class_report =
            PrimitiveConstructionStabilityClassReport::from_snapshot(&snapshot);
        let exhaustion_report =
            PrimitiveConstructionRealizationExhaustionReport::from_snapshot(&snapshot);
        Self {
            strategy_report,
            conditioning_witness_report,
            stability_class_report,
            exhaustion_report,
        }
    }

    pub fn strategy_report(&self) -> &PrimitiveConstructionRealizationStrategyReport {
        &self.strategy_report
    }

    pub fn conditioning_witness_report(&self) -> &PrimitiveConstructionConditioningWitnessReport {
        &self.conditioning_witness_report
    }

    pub fn stability_class_report(&self) -> &PrimitiveConstructionStabilityClassReport {
        &self.stability_class_report
    }

    pub fn exhaustion_report(&self) -> &PrimitiveConstructionRealizationExhaustionReport {
        &self.exhaustion_report
    }
}

pub fn prepare_primitive_construction_realization_report_bundle(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionRealizationReportBundle {
    PrimitiveConstructionRealizationReportBundle::from_snapshot(prepare_realization_snapshot(
        intent.into_request(),
    ))
}
