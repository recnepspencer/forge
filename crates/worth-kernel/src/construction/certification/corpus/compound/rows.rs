use crate::construction::diagnostics::PrimitiveConstructionBlockingBoundary;
use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionWitnessKind,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

use super::schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundRow {
    scenario_id: String,
    workload_family: PrimitiveConstructionCompoundWorkloadFamily,
    topology_class: PrimitiveConstructionCompoundTopologyClass,
    row_class: PrimitiveConstructionCompoundRowClass,
    direct_digest: String,
    replay_digest: String,
    branch_local_digest: String,
    inspection_digest: Option<String>,
    projection_consumption_digest: Option<String>,
    realization_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
    support_normal_class: Option<PrimitiveSupportNormalClass>,
    normalization_disposition: Option<PrimitiveNormalizationDisposition>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    rejection_class: Option<PrimitiveConstructionRejectionClass>,
    rejection_locality: Option<PrimitiveConstructionRejectionLocality>,
    blocking_boundary: Option<PrimitiveConstructionBlockingBoundary>,
    motion_kind: Option<PrimitiveConstructionCompoundMotionKind>,
    motion_digest: Option<String>,
    grazing_kind: Option<PrimitiveConstructionCompoundGrazingKind>,
    grazing_digest: Option<String>,
    construction_breadth: usize,
    birth_attachment_breadth: usize,
    certification_breadth: usize,
    row_digest: String,
}

impl PrimitiveConstructionCompoundRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        scenario_id: String,
        workload_family: PrimitiveConstructionCompoundWorkloadFamily,
        topology_class: PrimitiveConstructionCompoundTopologyClass,
        row_class: PrimitiveConstructionCompoundRowClass,
        direct_digest: String,
        replay_digest: String,
        branch_local_digest: String,
        inspection_digest: Option<String>,
        projection_consumption_digest: Option<String>,
        realization_strategy: Option<PrimitiveRealizationStrategy>,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: Option<PrimitiveStabilityClass>,
        feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
        support_normal_class: Option<PrimitiveSupportNormalClass>,
        normalization_disposition: Option<PrimitiveNormalizationDisposition>,
        exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
        rejection_class: Option<PrimitiveConstructionRejectionClass>,
        rejection_locality: Option<PrimitiveConstructionRejectionLocality>,
        blocking_boundary: Option<PrimitiveConstructionBlockingBoundary>,
        motion_kind: Option<PrimitiveConstructionCompoundMotionKind>,
        motion_digest: Option<String>,
        grazing_kind: Option<PrimitiveConstructionCompoundGrazingKind>,
        grazing_digest: Option<String>,
        construction_breadth: usize,
        birth_attachment_breadth: usize,
        certification_breadth: usize,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            scenario_id.clone(),
            workload_family.as_str().to_string(),
            topology_class.as_str().to_string(),
            row_class.as_str().to_string(),
            direct_digest.clone(),
            replay_digest.clone(),
            branch_local_digest.clone(),
            inspection_digest.clone().unwrap_or_default(),
            projection_consumption_digest.clone().unwrap_or_default(),
            realization_strategy
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            attempted_realization_strategies
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            stability_class
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            feature_conditioning_class
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            support_normal_class
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            normalization_disposition
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            exhaustion_reason
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            rejection_class
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            rejection_locality
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            blocking_boundary
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            motion_kind
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            motion_digest.clone().unwrap_or_default(),
            grazing_kind
                .map(|v| v.as_str())
                .unwrap_or("none")
                .to_string(),
            grazing_digest.clone().unwrap_or_default(),
            construction_breadth.to_string(),
            birth_attachment_breadth.to_string(),
            certification_breadth.to_string(),
        ]);
        Self {
            scenario_id,
            workload_family,
            topology_class,
            row_class,
            direct_digest,
            replay_digest,
            branch_local_digest,
            inspection_digest,
            projection_consumption_digest,
            realization_strategy,
            attempted_realization_strategies,
            stability_class,
            feature_conditioning_class,
            support_normal_class,
            normalization_disposition,
            exhaustion_reason,
            rejection_class,
            rejection_locality,
            blocking_boundary,
            motion_kind,
            motion_digest,
            grazing_kind,
            grazing_digest,
            construction_breadth,
            birth_attachment_breadth,
            certification_breadth,
            row_digest,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }
    pub fn workload_family(&self) -> PrimitiveConstructionCompoundWorkloadFamily {
        self.workload_family
    }
    pub fn topology_class(&self) -> PrimitiveConstructionCompoundTopologyClass {
        self.topology_class
    }
    pub fn row_class(&self) -> PrimitiveConstructionCompoundRowClass {
        self.row_class
    }
    pub fn direct_digest(&self) -> &str {
        &self.direct_digest
    }
    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }
    pub fn branch_local_digest(&self) -> &str {
        &self.branch_local_digest
    }
    pub fn inspection_digest(&self) -> Option<&str> {
        self.inspection_digest.as_deref()
    }
    pub fn projection_consumption_digest(&self) -> Option<&str> {
        self.projection_consumption_digest.as_deref()
    }
    pub fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.realization_strategy
    }
    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }
    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }
    pub fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass> {
        self.feature_conditioning_class
    }
    pub fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        self.support_normal_class
    }
    pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.normalization_disposition
    }
    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }
    pub fn rejection_class(&self) -> Option<PrimitiveConstructionRejectionClass> {
        self.rejection_class
    }
    pub fn rejection_locality(&self) -> Option<PrimitiveConstructionRejectionLocality> {
        self.rejection_locality
    }
    pub fn blocking_boundary(&self) -> Option<PrimitiveConstructionBlockingBoundary> {
        self.blocking_boundary
    }
    pub fn motion_kind(&self) -> Option<PrimitiveConstructionCompoundMotionKind> {
        self.motion_kind
    }
    pub fn motion_digest(&self) -> Option<&str> {
        self.motion_digest.as_deref()
    }
    pub fn grazing_kind(&self) -> Option<PrimitiveConstructionCompoundGrazingKind> {
        self.grazing_kind
    }
    pub fn grazing_digest(&self) -> Option<&str> {
        self.grazing_digest.as_deref()
    }
    pub fn construction_breadth(&self) -> usize {
        self.construction_breadth
    }
    pub fn birth_attachment_breadth(&self) -> usize {
        self.birth_attachment_breadth
    }
    pub fn certification_breadth(&self) -> usize {
        self.certification_breadth
    }
    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundMotionParityRow {
    scenario_id: String,
    motion_kind: PrimitiveConstructionCompoundMotionKind,
    motion_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionCompoundMotionParityRow {
    pub fn new(
        scenario_id: String,
        motion_kind: PrimitiveConstructionCompoundMotionKind,
        motion_digest: String,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            scenario_id.clone(),
            motion_kind.as_str().to_string(),
            motion_digest.clone(),
        ]);
        Self {
            scenario_id,
            motion_kind,
            motion_digest,
            row_digest,
        }
    }
    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }
    pub fn motion_kind(&self) -> PrimitiveConstructionCompoundMotionKind {
        self.motion_kind
    }
    pub fn motion_digest(&self) -> &str {
        &self.motion_digest
    }
    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundGrazingBoundaryRow {
    scenario_id: String,
    grazing_kind: PrimitiveConstructionCompoundGrazingKind,
    grazing_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionCompoundGrazingBoundaryRow {
    pub fn new(
        scenario_id: String,
        grazing_kind: PrimitiveConstructionCompoundGrazingKind,
        grazing_digest: String,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            scenario_id.clone(),
            grazing_kind.as_str().to_string(),
            grazing_digest.clone(),
        ]);
        Self {
            scenario_id,
            grazing_kind,
            grazing_digest,
            row_digest,
        }
    }
    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }
    pub fn grazing_kind(&self) -> PrimitiveConstructionCompoundGrazingKind {
        self.grazing_kind
    }
    pub fn grazing_digest(&self) -> &str {
        &self.grazing_digest
    }
    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundExhaustionWitnessParityRow {
    scenario_id: String,
    witness_kind: PrimitiveRealizationExhaustionWitnessKind,
    siege_row_digest: String,
    witness_row_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionCompoundExhaustionWitnessParityRow {
    pub fn new(
        scenario_id: String,
        witness_kind: PrimitiveRealizationExhaustionWitnessKind,
        siege_row_digest: String,
        witness_row_digest: String,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            scenario_id.clone(),
            witness_kind.as_str().to_string(),
            siege_row_digest.clone(),
            witness_row_digest.clone(),
        ]);
        Self {
            scenario_id,
            witness_kind,
            siege_row_digest,
            witness_row_digest,
            row_digest,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn witness_kind(&self) -> PrimitiveRealizationExhaustionWitnessKind {
        self.witness_kind
    }

    pub fn siege_row_digest(&self) -> &str {
        &self.siege_row_digest
    }

    pub fn witness_row_digest(&self) -> &str {
        &self.witness_row_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
