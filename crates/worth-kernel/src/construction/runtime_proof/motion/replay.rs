use crate::construction::certification::motion::{
    prepare_primitive_construction_move_witness_resolution_report_with_catalog,
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
    prepare_primitive_construction_rotate_witness_resolution_report_with_catalog,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionReport,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::{PrimitiveConstructionFamily, PrimitiveConstructionIntent};
use crate::spatial_intent::{
    MoveSpatialIntent, PointsTowardSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
};
use worth_spatial::facade::witness_catalog::SpatialWitnessCatalog;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionReplayParityReport {
    kind: PrimitiveConstructionMotionWitnessResolutionKind,
    family: PrimitiveConstructionFamily,
    direct_report: PrimitiveConstructionMotionWitnessResolutionReport,
    replay_report: PrimitiveConstructionMotionWitnessResolutionReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionMotionReplayParityReport {
    fn new(
        kind: PrimitiveConstructionMotionWitnessResolutionKind,
        family: PrimitiveConstructionFamily,
        direct_report: PrimitiveConstructionMotionWitnessResolutionReport,
        replay_report: PrimitiveConstructionMotionWitnessResolutionReport,
    ) -> Self {
        let parity_verified = direct_report == replay_report;
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ReplayIdentity,
            &[
                format!("{kind:?}"),
                family.as_str().to_string(),
                direct_report.report_digest().to_string(),
                replay_report.report_digest().to_string(),
                parity_verified.to_string(),
            ],
        );
        Self {
            kind,
            family,
            direct_report,
            replay_report,
            parity_verified,
            report_digest,
        }
    }

    #[cfg(test)]
    pub fn direct_report(&self) -> &PrimitiveConstructionMotionWitnessResolutionReport {
        &self.direct_report
    }
    #[cfg(test)]
    pub fn replay_report(&self) -> &PrimitiveConstructionMotionWitnessResolutionReport {
        &self.replay_report
    }
    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_move_replay_parity_report(
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionMotionReplayParityReport {
    prepare_primitive_construction_move_replay_parity_report_with_catalog(
        intent,
        &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_move_replay_parity_report_with_catalog(
    intent: MoveSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> PrimitiveConstructionMotionReplayParityReport {
    prepare_replay_report(
        prepare_primitive_construction_move_witness_resolution_report_with_catalog(
            intent.clone(),
            catalog,
        ),
        prepare_primitive_construction_move_witness_resolution_report_with_catalog(intent, catalog),
    )
}

#[cfg(test)]
pub fn prepare_primitive_construction_rotate_replay_parity_report(
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionMotionReplayParityReport {
    prepare_primitive_construction_rotate_replay_parity_report_with_catalog(
        intent,
        &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_rotate_replay_parity_report_with_catalog(
    intent: RotateSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> PrimitiveConstructionMotionReplayParityReport {
    prepare_replay_report(
        prepare_primitive_construction_rotate_witness_resolution_report_with_catalog(
            intent.clone(),
            catalog,
        ),
        prepare_primitive_construction_rotate_witness_resolution_report_with_catalog(
            intent, catalog,
        ),
    )
}

pub fn prepare_primitive_construction_reorient_replay_parity_report(
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionMotionReplayParityReport {
    prepare_primitive_construction_reorient_replay_parity_report_with_catalog(
        intent,
        &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
    )
}

pub fn prepare_primitive_construction_reorient_replay_parity_report_with_catalog(
    intent: ReorientSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> PrimitiveConstructionMotionReplayParityReport {
    prepare_replay_report(
        prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
            intent.clone(),
            catalog,
        ),
        prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
            intent, catalog,
        ),
    )
}

pub fn prepare_primitive_construction_points_toward_replay_parity_report_with_catalog(
    intent: PointsTowardSpatialIntent<PrimitiveConstructionIntent>,
    catalog: &impl SpatialWitnessCatalog,
) -> PrimitiveConstructionMotionReplayParityReport {
    prepare_replay_report(
        prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
            intent.clone(),
            catalog,
        ),
        prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
            intent, catalog,
        ),
    )
}

fn prepare_replay_report(
    direct_report: PrimitiveConstructionMotionWitnessResolutionReport,
    replay_report: PrimitiveConstructionMotionWitnessResolutionReport,
) -> PrimitiveConstructionMotionReplayParityReport {
    PrimitiveConstructionMotionReplayParityReport::new(
        direct_report.kind(),
        direct_report.subject_family(),
        direct_report,
        replay_report,
    )
}

#[cfg(test)]
#[path = "replay_tests.rs"]
mod tests;
