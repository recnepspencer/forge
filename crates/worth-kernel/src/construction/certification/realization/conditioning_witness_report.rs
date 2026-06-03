use super::snapshot::prepare_realization_snapshot;
use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionConditioningWitnessReport {
    family: PrimitiveConstructionFamily,
    admitted: bool,
    selected_strategy: Option<PrimitiveRealizationStrategy>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    conditioning_witness: Option<PrimitiveConditioningWitness>,
    report_digest: String,
}

impl PrimitiveConstructionConditioningWitnessReport {
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
                .exhaustion_reason()
                .map(PrimitiveRealizationExhaustionReason::as_str)
                .unwrap_or("none")
                .to_string(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.coordinate_magnitude().to_bits().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.feature_size().to_bits().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.condition_number().to_bits().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.machine_epsilon_at_scale().to_bits().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.precision_headroom_ratio().to_bits().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| {
                    witness
                        .minimum_support_normal_magnitude()
                        .to_bits()
                        .to_string()
                })
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.normalization_scale_applied().to_bits().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.feature_size_collapsed().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.needs_local_transform().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| {
                    witness
                        .support_normal_headroom_ratio()
                        .to_bits()
                        .to_string()
                })
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.feature_conditioning_class().as_str().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.support_normal_class().as_str().to_string())
                .unwrap_or_default(),
            snapshot
                .conditioning_witness()
                .map(|witness| witness.normalization_disposition().as_str().to_string())
                .unwrap_or_default(),
        ]);
        Self {
            family: snapshot.family(),
            admitted: snapshot.admitted(),
            selected_strategy: snapshot.selected_strategy(),
            exhaustion_reason: snapshot.exhaustion_reason(),
            conditioning_witness: snapshot.conditioning_witness().cloned(),
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

pub fn prepare_primitive_construction_conditioning_witness_report(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionConditioningWitnessReport {
    PrimitiveConstructionConditioningWitnessReport::from_snapshot(&prepare_realization_snapshot(
        intent.into_request(),
    ))
}
