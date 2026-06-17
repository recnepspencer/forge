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
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceStage, WorkloadEvidenceStageLinkSet,
};
use crate::workload_platform::projected_overlap_faces::CoplanarOverlapExtractionBundle;

use super::coplanar_overlap_extractions::{
    extraction_summary, operator_digest, CoplanarOverlapOperatorExtraction,
};
use super::coplanar_overlap_receipt::CoplanarOverlapOperatorReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapWorkloadOperator {
    consumed_stage_links: WorkloadEvidenceStageLinkSet,
    overlap_extractions: Vec<CoplanarOverlapOperatorExtraction>,
    context_binding: Option<OperatorCertificationContextBinding>,
    extraction_bundle_binding: Option<OperatorExtractionBundleBinding>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperatorCertificationContextBinding {
    context_identity: String,
    projection_stage_identity: String,
    transform_stage_identity: String,
    motion_posture_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OperatorExtractionBundleBinding {
    context_identity: String,
    projection_stage_identity: String,
    motion_posture_identity: String,
}

impl CoplanarOverlapWorkloadOperator {
    pub fn from_stage_links(consumed_stage_links: &WorkloadEvidenceStageLinkSet) -> Self {
        Self {
            consumed_stage_links: consumed_stage_links.clone(),
            overlap_extractions: Vec::new(),
            context_binding: None,
            extraction_bundle_binding: None,
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
        self.context_binding = Some(OperatorCertificationContextBinding {
            context_identity: context.context_identity().to_string(),
            projection_stage_identity: context.projection_stage_identity().to_string(),
            transform_stage_identity: context
                .motion_binding()
                .transform_stage_identity()
                .to_string(),
            motion_posture_identity: context.movement_rotation_posture_identity().to_string(),
        });
        self
    }

    pub fn with_extraction_bundle(mut self, bundle: &CoplanarOverlapExtractionBundle) -> Self {
        self.overlap_extractions = bundle
            .receipts()
            .iter()
            .map(CoplanarOverlapOperatorExtraction::from_receipt)
            .collect();
        self.extraction_bundle_binding = Some(OperatorExtractionBundleBinding {
            context_identity: bundle.context_identity().to_string(),
            projection_stage_identity: bundle.projection_stage_identity().to_string(),
            motion_posture_identity: bundle.movement_rotation_posture_identity().to_string(),
        });
        self
    }

    pub fn execute(self) -> Result<CoplanarOverlapOperatorReceipt, CoplanarOverlapOperatorDenial> {
        require_honest_stage(
            &self.consumed_stage_links,
            RequiredOperatorEvidenceStage::Projection,
        )?;
        require_honest_stage(
            &self.consumed_stage_links,
            RequiredOperatorEvidenceStage::Transform,
        )?;
        require_honest_stage(
            &self.consumed_stage_links,
            RequiredOperatorEvidenceStage::RetainedReplay,
        )?;
        let projection = stage_identity(
            &self.consumed_stage_links,
            RequiredOperatorEvidenceStage::Projection,
        )?;
        let transform = stage_identity(
            &self.consumed_stage_links,
            RequiredOperatorEvidenceStage::Transform,
        )?;
        let retained_replay = stage_identity(
            &self.consumed_stage_links,
            RequiredOperatorEvidenceStage::RetainedReplay,
        )?;
        require_context_matches_evidence(self.context_binding.as_ref(), projection, transform)?;
        require_extraction_bundle_matches_context(
            self.extraction_bundle_binding.as_ref(),
            self.context_binding.as_ref(),
        )?;
        let extraction_summary = extraction_summary(&self.overlap_extractions)?;
        let operator_digest = operator_digest(
            projection,
            transform,
            retained_replay,
            &extraction_summary.extraction_identities,
            &extraction_summary,
        );
        let operator_input_count =
            self.consumed_stage_links.links().len() + extraction_summary.receipt_count;
        Ok(CoplanarOverlapOperatorReceipt {
            operator_digest,
            consumed_evidence_identities: consumed_identities(&self.consumed_stage_links),
            consumed_stage_links: self.consumed_stage_links,
            overlap_extraction_identities: extraction_summary.extraction_identities,
            operator_input_count,
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
    context_binding: Option<&OperatorCertificationContextBinding>,
    projection_identity: &str,
    transform_identity: &str,
) -> Result<(), CoplanarOverlapOperatorDenial> {
    let Some(context_binding) = context_binding else {
        return Err(CoplanarOverlapOperatorDenial::MissingCertificationContext);
    };
    if context_binding.projection_stage_identity != projection_identity
        || context_binding.transform_stage_identity != transform_identity
        || context_binding.motion_posture_identity.is_empty()
    {
        return Err(CoplanarOverlapOperatorDenial::MismatchedCertificationContext);
    }
    Ok(())
}

fn require_extraction_bundle_matches_context(
    bundle_binding: Option<&OperatorExtractionBundleBinding>,
    context_binding: Option<&OperatorCertificationContextBinding>,
) -> Result<(), CoplanarOverlapOperatorDenial> {
    let Some(bundle_binding) = bundle_binding else {
        return Ok(());
    };
    if context_binding.is_some_and(|context_binding| {
        bundle_binding.context_identity == context_binding.context_identity
            && bundle_binding.projection_stage_identity == context_binding.projection_stage_identity
            && bundle_binding.motion_posture_identity == context_binding.motion_posture_identity
    }) {
        Ok(())
    } else {
        Err(CoplanarOverlapOperatorDenial::MismatchedOverlapExtractionBundle)
    }
}

fn require_honest_stage(
    consumed_stage_links: &WorkloadEvidenceStageLinkSet,
    required_stage: RequiredOperatorEvidenceStage,
) -> Result<(), CoplanarOverlapOperatorDenial> {
    let link = consumed_stage_links
        .link_for_stage(required_stage.evidence_stage())
        .ok_or_else(|| required_stage.missing_denial())?;
    let counters = link.counters();
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
    consumed_stage_links: &WorkloadEvidenceStageLinkSet,
    required_stage: RequiredOperatorEvidenceStage,
) -> Result<&str, CoplanarOverlapOperatorDenial> {
    consumed_stage_links
        .link_for_stage(required_stage.evidence_stage())
        .map(|link| link.evidence_identity())
        .ok_or_else(|| required_stage.missing_denial())
}

fn consumed_identities(consumed_stage_links: &WorkloadEvidenceStageLinkSet) -> Vec<String> {
    consumed_stage_links
        .links()
        .iter()
        .map(|link| link.evidence_identity().to_string())
        .collect()
}
