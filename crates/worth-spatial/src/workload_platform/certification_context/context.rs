use forge_query::facade::ForgeQueryDomainOperatingContext;
use std::marker::PhantomData;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_planar_local_frame::{
    PlanarLocalFrameCertificateQueryDomain, PlanarLocalFrameCertificateQueryWorld,
};
use crate::bindings::query_native_planar_overlap::{
    CoplanarOverlapContractContracts, CoplanarOverlapContractQueryDomain,
    CoplanarOverlapContractQueryWorld,
};
use crate::bindings::query_native_planar_precision::{
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use crate::bindings::query_native_planar_predicate::{
    PlanarPredicateAuthorityQueryDomain, PlanarPredicateAuthorityQueryWorld,
};
use crate::bindings::query_native_planar_projection::{
    ProjectPointToCertifiedPlane2DQueryDomain, ProjectPointToCertifiedPlane2DQueryWorld,
};
use crate::bindings::query_native_planar_segment_segment::{
    CertifiedSegmentSegment2DQueryDomain, CertifiedSegmentSegment2DQueryWorld,
};
use crate::bindings::query_native_planar_signed_area::{
    CertifiedSignedArea2DContracts, CertifiedSignedArea2DQueryDomain,
    CertifiedSignedArea2DQueryWorld,
};
use crate::bindings::query_native_planar_winding::{
    CertifiedPolygonWinding2DContracts, CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld,
};
use crate::planar_contracts::local_frame::PlanarLocalFrameCertificateReceipt;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;
use crate::workload_platform::transform_workload::TransformReceiptSet;

use super::analysis_surface::CertifiedAnalysisSurface;
use super::contracts::WorkloadCertificationContextContracts;
use super::denial::{WorkloadCertificationContextDenial, WorkloadCertificationContextDenialKind};
use super::motion_binding::WorkloadMotionBinding;
use super::precision_policy::WorkloadPrecisionPolicy;

pub struct WorkloadCertificationContext<
    'a,
    OC = CoplanarOverlapContractQueryWorld,
    SC = CertifiedSegmentSegment2DQueryWorld,
    PC = PlanarPredicateAuthorityQueryWorld,
    PRC = ProjectPointToCertifiedPlane2DQueryWorld,
    WC = CertifiedPolygonWinding2DQueryWorld,
    AC = CertifiedSignedArea2DQueryWorld,
    PXC = PlanarPrecisionCertificationQueryWorld,
    FC = PlanarLocalFrameCertificateQueryWorld,
> where
    OC: ForgeQueryDomainOperatingContext<CoplanarOverlapContractQueryDomain> + Clone,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain> + Clone,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain> + Clone,
    PRC: ForgeQueryDomainOperatingContext<ProjectPointToCertifiedPlane2DQueryDomain> + Clone,
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain> + Clone,
    AC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain> + Clone,
    PXC: ForgeQueryDomainOperatingContext<PlanarPrecisionCertificationQueryDomain> + Clone,
    FC: ForgeQueryDomainOperatingContext<PlanarLocalFrameCertificateQueryDomain> + Clone,
{
    projected_workload: ProjectedPlanarWorkload,
    context_identity: String,
    projection_stage_identity: String,
    analysis_surface: CertifiedAnalysisSurface,
    topology_neighborhood_identity: String,
    frame_identity: String,
    precision_policy: WorkloadPrecisionPolicy,
    motion_binding: WorkloadMotionBinding,
    precision_receipt: PlanarPrecisionCertificateReceipt,
    local_frame_receipt: PlanarLocalFrameCertificateReceipt,
    contracts: WorkloadCertificationContextContracts<OC, SC, PC, PRC, WC, AC, PXC, FC>,
    lifetime: PhantomData<&'a ()>,
}

impl WorkloadCertificationContext<'static> {
    pub fn from_projected_workload(projected: &ProjectedPlanarWorkload) -> WorkloadContextBuilder {
        WorkloadContextBuilder::new(projected)
    }
}

pub struct WorkloadContextBuilder {
    projected_workload: ProjectedPlanarWorkload,
    transform_receipts: Option<TransformReceiptSet>,
    precision_policy: WorkloadPrecisionPolicy,
}

impl WorkloadContextBuilder {
    fn new(projected: &ProjectedPlanarWorkload) -> Self {
        Self {
            projected_workload: projected.clone(),
            transform_receipts: None,
            precision_policy: WorkloadPrecisionPolicy::LocalFeatureScale,
        }
    }

    pub fn with_transform_receipts(mut self, receipts: &TransformReceiptSet) -> Self {
        self.transform_receipts = Some(receipts.clone());
        self
    }

    pub fn with_precision_policy(mut self, policy: WorkloadPrecisionPolicy) -> Self {
        self.precision_policy = policy;
        self
    }

    pub fn compile<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>(
        self,
        contracts: WorkloadCertificationContextContracts<OC, SC, PC, PRC, WC, AC, PXC, FC>,
    ) -> Result<
        WorkloadCertificationContext<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>,
        WorkloadCertificationContextDenial,
    >
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
        let projection_stage_identity = self
            .projected_workload
            .receipts()
            .stage_identity()
            .receipt_identity()
            .to_string();
        let transform_receipts = self.transform_receipts.ok_or_else(|| {
            WorkloadCertificationContextDenial::new(
                WorkloadCertificationContextDenialKind::MissingTransformReceipts,
                "workload certification context requires transform receipts",
            )
        })?;
        let motion_binding = WorkloadMotionBinding::from_transform_receipts(
            &projection_stage_identity,
            &transform_receipts,
        )?;
        compile_context(
            self.projected_workload,
            projection_stage_identity,
            self.precision_policy,
            motion_binding,
            contracts,
        )
    }
}

impl<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>
    WorkloadCertificationContext<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>
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
    pub fn with_motion_binding(
        &self,
        motion_binding: WorkloadMotionBinding,
    ) -> Result<Self, WorkloadCertificationContextDenial> {
        if motion_binding.projected_workload_identity() != self.projection_stage_identity() {
            return Err(WorkloadCertificationContextDenial::new(
                WorkloadCertificationContextDenialKind::MismatchedMotionBinding,
                "workload certification context cannot rebind motion from another projected workload",
            ));
        }
        compile_context(
            self.projected_workload.clone(),
            self.projection_stage_identity.clone(),
            self.precision_policy,
            motion_binding,
            self.contracts_ref(),
        )
    }

    pub(crate) fn contracts_ref(
        &self,
    ) -> WorkloadCertificationContextContracts<OC, SC, PC, PRC, WC, AC, PXC, FC> {
        self.contracts.clone()
    }
}

impl<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>
    WorkloadCertificationContext<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>
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
    pub fn projection_handle(
        &self,
    ) -> &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        ProjectPointToCertifiedPlane2DQueryDomain,
        PRC,
    > {
        &self.contracts.projection_handle
    }

    pub fn winding_contracts(&self) -> &CertifiedPolygonWinding2DContracts<WC, SC, PC> {
        &self.contracts.winding_contracts
    }

    pub fn signed_area_contracts(&self) -> &CertifiedSignedArea2DContracts<AC> {
        &self.contracts.signed_area_contracts
    }

    pub fn overlap_contracts(&self) -> &CoplanarOverlapContractContracts<OC, SC, PC> {
        &self.contracts.overlap_contracts
    }
    pub fn context_identity(&self) -> &str {
        &self.context_identity
    }

    pub fn projected_workload(&self) -> &ProjectedPlanarWorkload {
        &self.projected_workload
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }

    pub fn analysis_surface(&self) -> &CertifiedAnalysisSurface {
        &self.analysis_surface
    }

    pub fn topology_neighborhood_identity(&self) -> &str {
        &self.topology_neighborhood_identity
    }

    pub fn frame_identity(&self) -> &str {
        &self.frame_identity
    }

    pub fn motion_binding(&self) -> &WorkloadMotionBinding {
        &self.motion_binding
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        self.motion_binding.movement_rotation_posture_identity()
    }

    pub fn precision_receipt(&self) -> &PlanarPrecisionCertificateReceipt {
        &self.precision_receipt
    }

    pub fn local_frame_receipt(&self) -> &PlanarLocalFrameCertificateReceipt {
        &self.local_frame_receipt
    }
}

fn compile_context<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>(
    projected_workload: ProjectedPlanarWorkload,
    projection_stage_identity: String,
    precision_policy: WorkloadPrecisionPolicy,
    motion_binding: WorkloadMotionBinding,
    contracts: WorkloadCertificationContextContracts<OC, SC, PC, PRC, WC, AC, PXC, FC>,
) -> Result<
    WorkloadCertificationContext<'a, OC, SC, PC, PRC, WC, AC, PXC, FC>,
    WorkloadCertificationContextDenial,
>
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
    let analysis_surface = CertifiedAnalysisSurface::from_projected_workload(&projected_workload);
    let topology_neighborhood_identity =
        topology_neighborhood_identity(&projection_stage_identity, &analysis_surface);
    let frame_identity = frame_identity(&projection_stage_identity, &analysis_surface);
    let precision_receipt = precision_policy.certify_precision(
        &frame_identity,
        &topology_neighborhood_identity,
        &motion_binding,
        &contracts,
    )?;
    let local_frame_receipt = precision_policy.certify_local_frame(
        &frame_identity,
        &motion_binding,
        &precision_receipt,
        &contracts,
    )?;
    let context_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            format!("projection-stage:{projection_stage_identity}"),
            format!("analysis-surface:{}", analysis_surface.surface_identity()),
            format!("topology-neighborhood:{topology_neighborhood_identity}"),
            format!("motion:{}", motion_binding.motion_binding_identity()),
            format!("precision:{}", precision_receipt.fact_digest()),
            format!("local-frame:{}", local_frame_receipt.fact_digest()),
        ],
    );
    Ok(WorkloadCertificationContext {
        projected_workload,
        context_identity,
        projection_stage_identity,
        analysis_surface,
        topology_neighborhood_identity,
        frame_identity,
        precision_policy,
        motion_binding,
        precision_receipt,
        local_frame_receipt,
        contracts,
        lifetime: PhantomData,
    })
}

fn frame_identity(
    projection_stage_identity: &str,
    analysis_surface: &CertifiedAnalysisSurface,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            format!("analysis-frame-projection:{projection_stage_identity}"),
            format!("analysis-surface:{}", analysis_surface.surface_identity()),
        ],
    )
}

fn topology_neighborhood_identity(
    projection_stage_identity: &str,
    analysis_surface: &CertifiedAnalysisSurface,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            format!("projection-stage:{projection_stage_identity}"),
            format!(
                "topology-query-surface:{}",
                analysis_surface.topology_query_surface_identity()
            ),
            format!(
                "workload-local-basis:{}",
                analysis_surface.workload_local_basis_identity()
            ),
        ],
    )
}
