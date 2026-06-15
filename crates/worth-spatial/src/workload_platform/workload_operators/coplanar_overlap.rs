use forge_query::facade::ForgeQueryDomainOperatingContext;

use crate::bindings::query_native_planar_local_frame::PlanarLocalFrameCertificateQueryDomain;
use crate::bindings::query_native_planar_overlap::CoplanarOverlapContractQueryDomain;
use crate::bindings::query_native_planar_precision::PlanarPrecisionCertificationQueryDomain;
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_projection::ProjectPointToCertifiedPlane2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_signed_area::CertifiedSignedArea2DQueryDomain;
use crate::bindings::query_native_planar_winding::CertifiedPolygonWinding2DQueryDomain;
use crate::workload_platform::certification_context::WorkloadCertificationContext;
use crate::workload_platform::evidence_ledger::{WorkloadEvidenceRow, WorkloadEvidenceStage};
use crate::workload_platform::projected_overlap_faces::CoplanarOverlapExtractionBundle;

use super::coplanar_overlap_extractions::{
    extraction_summary, operator_digest, CoplanarOverlapOperatorExtraction,
};
use super::coplanar_overlap_receipt::CoplanarOverlapOperatorReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapWorkloadOperator {
    consumed_evidence: Vec<WorkloadEvidenceRow>,
    overlap_extractions: Vec<CoplanarOverlapOperatorExtraction>,
    context_identity: Option<String>,
    context_projection_identity: Option<String>,
    context_motion_identity: Option<String>,
    extraction_bundle_context_identity: Option<String>,
    extraction_bundle_projection_identity: Option<String>,
    extraction_bundle_motion_identity: Option<String>,
}

impl CoplanarOverlapWorkloadOperator {
    pub fn from_consumed_evidence(consumed_evidence: &[WorkloadEvidenceRow]) -> Self {
        Self {
            consumed_evidence: consumed_evidence.to_vec(),
            overlap_extractions: Vec::new(),
            context_identity: None,
            context_projection_identity: None,
            context_motion_identity: None,
            extraction_bundle_context_identity: None,
            extraction_bundle_projection_identity: None,
            extraction_bundle_motion_identity: None,
        }
    }

    pub fn with_certification_context<OC, SC, PC, PRC, WC, AC, PXC, FC>(
        mut self,
        context: &WorkloadCertificationContext<'_, OC, SC, PC, PRC, WC, AC, PXC, FC>,
    ) -> Self
    where
        OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
        PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain>,
        WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
        AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
        PXC: ForgeQueryDomainOperatingContext<PlanarPrecisionCertificationQueryDomain>,
        FC: ForgeQueryDomainOperatingContext<PlanarLocalFrameCertificateQueryDomain>,
    {
        self.context_identity = Some(context.context_identity().to_string());
        self.context_projection_identity = Some(context.projection_stage_identity().to_string());
        self.context_motion_identity =
            Some(context.movement_rotation_posture_identity().to_string());
        self
    }

    pub fn with_extraction_bundle(mut self, bundle: &CoplanarOverlapExtractionBundle) -> Self {
        self.overlap_extractions = bundle
            .receipts()
            .iter()
            .map(CoplanarOverlapOperatorExtraction::from_receipt)
            .collect();
        self.extraction_bundle_context_identity = Some(bundle.context_identity().to_string());
        self.extraction_bundle_projection_identity =
            Some(bundle.projection_stage_identity().to_string());
        self.extraction_bundle_motion_identity =
            Some(bundle.movement_rotation_posture_identity().to_string());
        self
    }

    pub fn execute(self) -> Result<CoplanarOverlapOperatorReceipt, CoplanarOverlapOperatorDenial> {
        require_honest_stage(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::Projection,
        )?;
        require_honest_stage(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::Transform,
        )?;
        require_honest_stage(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::RetainedReplay,
        )?;
        let projection = stage_identity(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::Projection,
        )?;
        let transform = stage_identity(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::Transform,
        )?;
        let retained_replay = stage_identity(
            &self.consumed_evidence,
            RequiredOperatorEvidenceStage::RetainedReplay,
        )?;
        require_context_matches_evidence(
            self.context_identity.as_deref(),
            self.context_projection_identity.as_deref(),
            self.context_motion_identity.as_deref(),
            projection,
            transform,
        )?;
        require_extraction_bundle_matches_context(
            self.extraction_bundle_context_identity.as_deref(),
            self.extraction_bundle_projection_identity.as_deref(),
            self.extraction_bundle_motion_identity.as_deref(),
            self.context_identity.as_deref(),
            self.context_projection_identity.as_deref(),
            self.context_motion_identity.as_deref(),
        )?;
        let extraction_summary = extraction_summary(&self.overlap_extractions)?;
        let operator_digest = operator_digest(
            projection,
            transform,
            retained_replay,
            &extraction_summary.extraction_identities,
            &extraction_summary,
        );
        Ok(CoplanarOverlapOperatorReceipt {
            operator_digest,
            consumed_evidence_identities: consumed_identities(&self.consumed_evidence),
            overlap_extraction_identities: extraction_summary.extraction_identities,
            operator_input_count: self.consumed_evidence.len() + extraction_summary.receipt_count,
            operator_receipt_count: 1,
            overlap_extraction_receipt_count: extraction_summary.receipt_count,
            overlap_candidate_pair_breadth: extraction_summary.candidate_pair_breadth,
            overlap_segment_contacts_certified: extraction_summary.segment_contacts_certified,
            overlap_shared_intervals: extraction_summary.shared_intervals,
            overlap_islands: extraction_summary.overlap_islands,
            overlap_containment_relations: extraction_summary.containment_relations,
            overlap_policy_required_exits: extraction_summary.policy_required_exits,
            overlap_ambiguous_contacts: extraction_summary.ambiguous_contacts,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RequiredOperatorEvidenceStage {
    Projection,
    Transform,
    RetainedReplay,
}

impl RequiredOperatorEvidenceStage {
    fn evidence_stage(self) -> WorkloadEvidenceStage {
        match self {
            Self::Projection => WorkloadEvidenceStage::Projection,
            Self::Transform => WorkloadEvidenceStage::Transform,
            Self::RetainedReplay => WorkloadEvidenceStage::RetainedReplay,
        }
    }

    fn missing_denial(self) -> CoplanarOverlapOperatorDenial {
        match self {
            Self::Projection => CoplanarOverlapOperatorDenial::MissingProjectedWorkload,
            Self::Transform => CoplanarOverlapOperatorDenial::MissingTransformWorkload,
            Self::RetainedReplay => CoplanarOverlapOperatorDenial::MissingRetainedReplayWorkload,
        }
    }

    fn manual_denial(self) -> CoplanarOverlapOperatorDenial {
        match self {
            Self::Projection => CoplanarOverlapOperatorDenial::ManualProjectedWorkload,
            Self::Transform => CoplanarOverlapOperatorDenial::ManualTransformWorkload,
            Self::RetainedReplay => CoplanarOverlapOperatorDenial::ManualRetainedReplayWorkload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapOperatorDenial {
    MissingProjectedWorkload,
    MissingTransformWorkload,
    MissingRetainedReplayWorkload,
    ManualProjectedWorkload,
    ManualTransformWorkload,
    ManualRetainedReplayWorkload,
    SyntheticProjectedWorkload,
    SyntheticTransformWorkload,
    SyntheticRetainedReplayWorkload,
    MissingOverlapExtractionReceipts,
    SyntheticOverlapExtraction,
    MissingCertificationContext,
    MismatchedCertificationContext,
    MismatchedOverlapExtractionBundle,
}

impl CoplanarOverlapOperatorDenial {
    pub fn human_reason(self) -> &'static str {
        match self {
            Self::MissingProjectedWorkload => {
                "coplanar overlap operator requires projected planar workload evidence"
            }
            Self::MissingTransformWorkload => {
                "coplanar overlap operator requires transform workload evidence"
            }
            Self::MissingRetainedReplayWorkload => {
                "coplanar overlap operator requires retained replay workload evidence"
            }
            Self::ManualProjectedWorkload => {
                "coplanar overlap operator rejects hand-filled projection evidence"
            }
            Self::ManualTransformWorkload => {
                "coplanar overlap operator rejects hand-filled transform evidence"
            }
            Self::ManualRetainedReplayWorkload => {
                "coplanar overlap operator rejects hand-filled retained replay evidence"
            }
            Self::SyntheticProjectedWorkload => {
                "coplanar overlap operator requires projected entities and local-basis evidence"
            }
            Self::SyntheticTransformWorkload => {
                "coplanar overlap operator requires real transform step evidence"
            }
            Self::SyntheticRetainedReplayWorkload => {
                "coplanar overlap operator requires retained artifact and replay checkpoint evidence"
            }
            Self::MissingOverlapExtractionReceipts => {
                "coplanar overlap operator requires real overlap extraction receipts"
            }
            Self::SyntheticOverlapExtraction => {
                "coplanar overlap operator requires overlap extraction receipts with candidate pairs and retained overlap facts"
            }
            Self::MissingCertificationContext => {
                "coplanar overlap operator requires a workload certification context compiled from the projected workload and transform receipts"
            }
            Self::MismatchedCertificationContext => {
                "coplanar overlap operator requires certification context, projection evidence, movement posture, and extraction bundle to describe the same workload"
            }
            Self::MismatchedOverlapExtractionBundle => {
                "coplanar overlap operator requires the overlap extraction bundle to be compiled from the same workload certification context"
            }
        }
    }
}

fn require_context_matches_evidence(
    context_identity: Option<&str>,
    context_projection_identity: Option<&str>,
    context_motion_identity: Option<&str>,
    projection_identity: &str,
    transform_identity: &str,
) -> Result<(), CoplanarOverlapOperatorDenial> {
    if context_identity.is_none() {
        return Err(CoplanarOverlapOperatorDenial::MissingCertificationContext);
    }
    if context_projection_identity != Some(projection_identity)
        || context_motion_identity.is_none()
        || transform_identity.is_empty()
    {
        return Err(CoplanarOverlapOperatorDenial::MismatchedCertificationContext);
    }
    Ok(())
}

fn require_extraction_bundle_matches_context(
    bundle_context_identity: Option<&str>,
    bundle_projection_identity: Option<&str>,
    bundle_motion_identity: Option<&str>,
    context_identity: Option<&str>,
    context_projection_identity: Option<&str>,
    context_motion_identity: Option<&str>,
) -> Result<(), CoplanarOverlapOperatorDenial> {
    if bundle_context_identity.is_none() {
        return Ok(());
    }
    if bundle_context_identity == context_identity
        && bundle_projection_identity == context_projection_identity
        && bundle_motion_identity == context_motion_identity
    {
        Ok(())
    } else {
        Err(CoplanarOverlapOperatorDenial::MismatchedOverlapExtractionBundle)
    }
}

fn require_honest_stage(
    consumed_evidence: &[WorkloadEvidenceRow],
    required_stage: RequiredOperatorEvidenceStage,
) -> Result<(), CoplanarOverlapOperatorDenial> {
    let stage = required_stage.evidence_stage();
    let row = consumed_evidence
        .iter()
        .find(|row| row.stage() == stage)
        .ok_or_else(|| required_stage.missing_denial())?;
    if !row.is_receipt_backed() || !row.is_admitted() {
        return Err(required_stage.manual_denial());
    }
    let counters = row.counters();
    match required_stage {
        RequiredOperatorEvidenceStage::Projection
            if counters.projected_entity_count() == 0 || counters.local_basis_part_count() == 0 =>
        {
            Err(CoplanarOverlapOperatorDenial::SyntheticProjectedWorkload)
        }
        RequiredOperatorEvidenceStage::Transform
            if counters.transform_changed_coordinate_count() == 0 =>
        {
            Err(CoplanarOverlapOperatorDenial::SyntheticTransformWorkload)
        }
        RequiredOperatorEvidenceStage::RetainedReplay
            if counters.retained_artifact_count() == 0
                || counters.replay_checkpoint_count() == 0 =>
        {
            Err(CoplanarOverlapOperatorDenial::SyntheticRetainedReplayWorkload)
        }
        _ => Ok(()),
    }
}

fn stage_identity(
    consumed_evidence: &[WorkloadEvidenceRow],
    required_stage: RequiredOperatorEvidenceStage,
) -> Result<&str, CoplanarOverlapOperatorDenial> {
    consumed_evidence
        .iter()
        .find(|row| row.stage() == required_stage.evidence_stage())
        .map(WorkloadEvidenceRow::evidence_identity)
        .ok_or_else(|| required_stage.missing_denial())
}

fn consumed_identities(consumed_evidence: &[WorkloadEvidenceRow]) -> Vec<String> {
    consumed_evidence
        .iter()
        .map(|row| format!("{:?}:{}", row.stage(), row.evidence_identity()))
        .collect()
}
