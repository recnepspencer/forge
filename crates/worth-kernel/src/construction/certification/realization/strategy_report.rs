use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
use worth_geom::facade::{PrimitiveRealizationStrategy, PrimitiveStabilityClass};

use super::snapshot::prepare_realization_snapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRealizationStrategyReport {
    family: PrimitiveConstructionFamily,
    admitted: bool,
    selected_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    canonical_artifact_digest: Option<String>,
    report_digest: String,
}

impl PrimitiveConstructionRealizationStrategyReport {
    pub(crate) fn from_snapshot(
        snapshot: &super::snapshot::PrimitiveConstructionRealizationSnapshot,
    ) -> Self {
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
                .canonical_artifact_digest()
                .unwrap_or_default()
                .to_string(),
            snapshot.realization_digest().to_string(),
        ]);
        Self {
            family: snapshot.family(),
            admitted: snapshot.admitted(),
            selected_strategy: snapshot.selected_strategy(),
            attempted_strategies: snapshot.attempted_strategies().to_vec(),
            stability_class: snapshot.stability_class(),
            canonical_artifact_digest: snapshot.canonical_artifact_digest().map(str::to_string),
            report_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn admitted(&self) -> bool {
        self.admitted
    }

    pub fn selected_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.selected_strategy
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub fn canonical_artifact_digest(&self) -> Option<&str> {
        self.canonical_artifact_digest.as_deref()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_realization_strategy_report(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionRealizationStrategyReport {
    PrimitiveConstructionRealizationStrategyReport::from_snapshot(&prepare_realization_snapshot(
        intent.into_request(),
    ))
}
