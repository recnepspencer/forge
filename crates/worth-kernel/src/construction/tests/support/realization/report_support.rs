use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::tests::support::realization::{
    prepare_realization_snapshot, PrimitiveConstructionRealizationSnapshot,
};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionRealizationExhaustionStatus {
    NotApplicable,
    NotExhausted,
    Exhausted,
}

impl PrimitiveConstructionRealizationExhaustionStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::NotExhausted => "not_exhausted",
            Self::Exhausted => "exhausted",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PrimitiveConstructionRealizationReportView {
    snapshot: PrimitiveConstructionRealizationSnapshot,
    report_digest: String,
}

impl PrimitiveConstructionRealizationReportView {
    pub(crate) fn from_snapshot(snapshot: &PrimitiveConstructionRealizationSnapshot) -> Self {
        let report_digest = digest_owned_parts(&[
            snapshot.family().as_str().to_string(),
            snapshot.admitted().to_string(),
            snapshot
                .selected_strategy()
                .map(PrimitiveRealizationStrategy::as_str)
                .unwrap_or("none")
                .to_string(),
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
            snapshot
                .canonical_artifact_digest()
                .unwrap_or("none")
                .to_string(),
            exhaustion_status(snapshot).as_str().to_string(),
            snapshot.realization_digest().to_string(),
        ]);
        Self {
            snapshot: snapshot.clone(),
            report_digest,
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.snapshot.family()
    }

    pub(crate) fn admitted(&self) -> bool {
        self.snapshot.admitted()
    }

    pub(crate) fn selected_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.snapshot.selected_strategy()
    }

    pub(crate) fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.snapshot.attempted_strategies()
    }

    pub(crate) fn attempted_realization_strategy_count(&self) -> usize {
        self.snapshot.attempted_strategies().len()
    }

    pub(crate) fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.snapshot.stability_class()
    }

    pub(crate) fn canonical_artifact_digest(&self) -> Option<&str> {
        self.snapshot.canonical_artifact_digest()
    }

    pub(crate) fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.snapshot.exhaustion_reason()
    }

    pub(crate) fn conditioning_witness(&self) -> Option<&PrimitiveConditioningWitness> {
        self.snapshot.conditioning_witness()
    }

    pub(crate) fn status(&self) -> PrimitiveConstructionRealizationExhaustionStatus {
        exhaustion_status(&self.snapshot)
    }

    pub(crate) fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub(crate) fn prepare_primitive_construction_realization_strategy_report(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionRealizationReportView {
    PrimitiveConstructionRealizationReportView::from_snapshot(&prepare_realization_snapshot(
        intent.into_request(),
    ))
}

pub(crate) fn prepare_primitive_construction_conditioning_witness_report(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionRealizationReportView {
    PrimitiveConstructionRealizationReportView::from_snapshot(&prepare_realization_snapshot(
        intent.into_request(),
    ))
}

pub(crate) fn prepare_primitive_construction_stability_class_report(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionRealizationReportView {
    PrimitiveConstructionRealizationReportView::from_snapshot(&prepare_realization_snapshot(
        intent.into_request(),
    ))
}

pub(crate) fn prepare_primitive_construction_realization_exhaustion_report(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionRealizationReportView {
    PrimitiveConstructionRealizationReportView::from_snapshot(&prepare_realization_snapshot(
        intent.into_request(),
    ))
}

fn exhaustion_status(
    snapshot: &PrimitiveConstructionRealizationSnapshot,
) -> PrimitiveConstructionRealizationExhaustionStatus {
    if snapshot.exhaustion_reason().is_some() {
        PrimitiveConstructionRealizationExhaustionStatus::Exhausted
    } else if snapshot.stability_class() == Some(PrimitiveStabilityClass::StableAfterEscalation) {
        PrimitiveConstructionRealizationExhaustionStatus::NotExhausted
    } else if snapshot.stability_class() == Some(PrimitiveStabilityClass::StableDirect) {
        PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
    } else if snapshot.conditioning_witness().is_some() {
        PrimitiveConstructionRealizationExhaustionStatus::NotExhausted
    } else {
        PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
    }
}
