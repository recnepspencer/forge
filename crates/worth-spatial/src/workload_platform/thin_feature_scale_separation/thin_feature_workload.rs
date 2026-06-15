use std::collections::BTreeSet;

use super::platform_projection_evidence::require_platform_projection_matches_ledger;
use super::stage_evidence::{
    require_platform_stage_evidence, require_thin_feature_topology_breadth, stage_counters,
    workload_identity,
};
use super::thin_feature_counters::{
    ThinFeatureScaleSeparationCounterInput, ThinFeatureScaleSeparationCounters,
};
use super::thin_feature_digest::thin_feature_digest;
use super::thin_feature_evidence_counts::{
    precision_escalation_count, precision_witnesses_cover_required_scales_and_basis,
    tiny_rotation_count,
};
use super::thin_feature_policy::{
    ThinFeatureEvidenceIntegrity, ThinFeaturePredicateCertification, ThinFeatureScalePolicy,
    ThinFeatureScaleSeparationWorkloadError, ThinFeatureTinyRotationPressure,
};
use super::thin_feature_receipt::ThinFeatureScaleSeparationReceipt;
use crate::planar_contracts::local_frame::PlanarLocalFrameCertificateReceipt;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;

pub struct ThinFeatureScaleSeparationWorkload<'a> {
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
    precision: Option<&'a PlanarPrecisionCertificateReceipt>,
    precision_scale_witnesses: Vec<&'a PlanarPrecisionCertificateReceipt>,
    platform_projection: Option<&'a ProjectedPlanarWorkload>,
    local_frame: Option<&'a PlanarLocalFrameCertificateReceipt>,
    projection_consumption: Option<&'a ProjectionConsumedPlanarFactsReceipt>,
    required_thin_feature_count: usize,
    required_local_scale_orders: BTreeSet<i32>,
    required_world_magnitude_order: i32,
    tiny_rotation_pressure: ThinFeatureTinyRotationPressure,
    predicate_certification: ThinFeaturePredicateCertification,
    scale_policy: ThinFeatureScalePolicy,
    evidence_integrity: ThinFeatureEvidenceIntegrity,
}

impl<'a> ThinFeatureScaleSeparationWorkload<'a> {
    pub fn from_platform_evidence(evidence_ledger: &'a CompleteWorkloadEvidenceLedger) -> Self {
        Self {
            evidence_ledger,
            precision: None,
            precision_scale_witnesses: Vec::new(),
            platform_projection: None,
            local_frame: None,
            projection_consumption: None,
            required_thin_feature_count: 12,
            required_local_scale_orders: BTreeSet::from([-12, -9, -6]),
            required_world_magnitude_order: 12,
            tiny_rotation_pressure: ThinFeatureTinyRotationPressure::RequiredAndSupported,
            predicate_certification: ThinFeaturePredicateCertification::Certified,
            scale_policy: ThinFeatureScalePolicy::Admit,
            evidence_integrity: ThinFeatureEvidenceIntegrity::Consistent,
        }
    }

    pub fn with_precision_receipt(
        mut self,
        receipt: &'a PlanarPrecisionCertificateReceipt,
    ) -> Self {
        self.precision = Some(receipt);
        self.precision_scale_witnesses.push(receipt);
        self
    }

    pub fn with_precision_scale_witness(
        mut self,
        receipt: &'a PlanarPrecisionCertificateReceipt,
    ) -> Self {
        self.precision_scale_witnesses.push(receipt);
        self
    }

    pub fn with_platform_projection(mut self, projected: &'a ProjectedPlanarWorkload) -> Self {
        self.platform_projection = Some(projected);
        self
    }

    pub fn with_local_frame_receipt(
        mut self,
        receipt: &'a PlanarLocalFrameCertificateReceipt,
    ) -> Self {
        self.local_frame = Some(receipt);
        self
    }

    pub fn with_projection_consumption_receipt(
        mut self,
        receipt: &'a ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.projection_consumption = Some(receipt);
        self
    }

    pub fn with_required_thin_feature_count(mut self, count: usize) -> Self {
        self.required_thin_feature_count = count;
        self
    }

    pub fn with_required_local_scale_orders<const N: usize>(mut self, orders: [i32; N]) -> Self {
        self.required_local_scale_orders = BTreeSet::from(orders);
        self
    }

    pub fn with_required_world_magnitude_order(mut self, order: i32) -> Self {
        self.required_world_magnitude_order = order;
        self
    }

    pub fn requiring_tiny_rotation_pressure(
        mut self,
        pressure: ThinFeatureTinyRotationPressure,
    ) -> Self {
        self.tiny_rotation_pressure = pressure;
        self
    }

    pub fn requiring_predicate_certification(
        mut self,
        certification: ThinFeaturePredicateCertification,
    ) -> Self {
        self.predicate_certification = certification;
        self
    }

    pub fn requiring_scale_policy(mut self, policy: ThinFeatureScalePolicy) -> Self {
        self.scale_policy = policy;
        self
    }

    pub fn requiring_evidence_integrity(mut self, integrity: ThinFeatureEvidenceIntegrity) -> Self {
        self.evidence_integrity = integrity;
        self
    }

    pub fn certify(
        self,
    ) -> Result<ThinFeatureScaleSeparationReceipt, ThinFeatureScaleSeparationWorkloadError> {
        self.require_predicate_certification()?;
        self.require_scale_policy()?;
        self.require_tiny_rotation_pressure()?;
        self.require_evidence_integrity()?;

        let precision = self.required_precision_receipt()?;
        let platform_projection = self.required_platform_projection()?;
        let local_frame = self.required_local_frame_receipt()?;
        let projection_consumption = self.required_projection_consumption_receipt()?;
        let platform_projection_identity =
            require_platform_projection_matches_ledger(self.evidence_ledger, platform_projection)?;
        self.require_precision_and_frame_alignment(precision, local_frame)?;
        self.require_projection_consumption_alignment(local_frame, projection_consumption)?;

        let counters =
            self.thin_feature_counters(precision, local_frame, projection_consumption)?;
        let workload_identity = workload_identity(self.evidence_ledger)?;
        let precision_identity = precision.fact_digest().to_string();
        let local_frame_identity = local_frame.fact_digest().to_string();
        let projection_consumption_identity = projection_consumption
            .projection_consumption_digest()
            .to_string();
        let projection_consumed_local_frame_identity = projection_consumption
            .basis()
            .materialization_basis_identity()
            .to_string();
        let local_scale_orders = self.local_scale_orders();
        let thin_feature_digest = thin_feature_digest(
            &workload_identity,
            &precision_identity,
            &platform_projection_identity,
            &local_frame_identity,
            &projection_consumption_identity,
            &projection_consumed_local_frame_identity,
            &local_scale_orders,
            self.required_world_magnitude_order,
            counters,
        );

        Ok(ThinFeatureScaleSeparationReceipt::new(
            thin_feature_digest,
            workload_identity,
            precision_identity,
            platform_projection_identity,
            local_frame_identity,
            projection_consumption_identity,
            projection_consumed_local_frame_identity,
            local_scale_orders,
            self.required_world_magnitude_order,
            counters,
        ))
    }

    fn required_precision_receipt(
        &self,
    ) -> Result<&PlanarPrecisionCertificateReceipt, ThinFeatureScaleSeparationWorkloadError> {
        self.precision
            .ok_or(ThinFeatureScaleSeparationWorkloadError::MissingPrecisionEvidence)
    }

    fn required_local_frame_receipt(
        &self,
    ) -> Result<&PlanarLocalFrameCertificateReceipt, ThinFeatureScaleSeparationWorkloadError> {
        self.local_frame
            .ok_or(ThinFeatureScaleSeparationWorkloadError::MissingLocalFrameEvidence)
    }

    fn required_platform_projection(
        &self,
    ) -> Result<&ProjectedPlanarWorkload, ThinFeatureScaleSeparationWorkloadError> {
        self.platform_projection
            .ok_or(ThinFeatureScaleSeparationWorkloadError::MissingPlatformProjectionEvidence)
    }

    fn required_projection_consumption_receipt(
        &self,
    ) -> Result<&ProjectionConsumedPlanarFactsReceipt, ThinFeatureScaleSeparationWorkloadError>
    {
        self.projection_consumption
            .ok_or(ThinFeatureScaleSeparationWorkloadError::MissingProjectionConsumedBasis)
    }

    fn require_precision_and_frame_alignment(
        &self,
        precision: &PlanarPrecisionCertificateReceipt,
        local_frame: &PlanarLocalFrameCertificateReceipt,
    ) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        if local_frame.precision_fact_digest() != precision.fact_digest() {
            return Err(ThinFeatureScaleSeparationWorkloadError::IntegrityMismatch {
                stage: WorkloadEvidenceStage::Projection,
            });
        }
        if local_frame.basis().local_feature_scale_order()
            != precision.basis().local_feature_scale_order()
            || local_frame.basis().world_magnitude_order()
                != precision.basis().world_magnitude_order()
        {
            return Err(ThinFeatureScaleSeparationWorkloadError::PrecisionBasisFailure);
        }
        Ok(())
    }

    fn require_projection_consumption_alignment(
        &self,
        local_frame: &PlanarLocalFrameCertificateReceipt,
        projection_consumption: &ProjectionConsumedPlanarFactsReceipt,
    ) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        if projection_consumption
            .basis()
            .materialization_basis_identity()
            != local_frame.fact_digest()
        {
            return Err(ThinFeatureScaleSeparationWorkloadError::IntegrityMismatch {
                stage: WorkloadEvidenceStage::Projection,
            });
        }
        Ok(())
    }

    fn thin_feature_counters(
        &self,
        precision: &PlanarPrecisionCertificateReceipt,
        local_frame: &PlanarLocalFrameCertificateReceipt,
        projection_consumption: &ProjectionConsumedPlanarFactsReceipt,
    ) -> Result<ThinFeatureScaleSeparationCounters, ThinFeatureScaleSeparationWorkloadError> {
        let topology = stage_counters(self.evidence_ledger, WorkloadEvidenceStage::Topology)?;
        let binding = stage_counters(self.evidence_ledger, WorkloadEvidenceStage::GeometryBinding)?;
        let support = stage_counters(self.evidence_ledger, WorkloadEvidenceStage::SurfaceSupport)?;
        let projection = stage_counters(self.evidence_ledger, WorkloadEvidenceStage::Projection)?;
        let transform = stage_counters(self.evidence_ledger, WorkloadEvidenceStage::Transform)?;
        let diagnostics = stage_counters(self.evidence_ledger, WorkloadEvidenceStage::Diagnostics)?;
        let response = stage_counters(self.evidence_ledger, WorkloadEvidenceStage::Response)?;

        require_platform_stage_evidence(
            topology,
            binding,
            support,
            projection,
            transform,
            diagnostics,
            response,
        )?;
        require_thin_feature_topology_breadth(self.required_thin_feature_count, topology)?;
        self.require_precision_scale_evidence(precision, local_frame)?;
        self.require_world_magnitude_floor(precision, local_frame)?;

        Ok(ThinFeatureScaleSeparationCounters::new(
            ThinFeatureScaleSeparationCounterInput {
                thin_feature_count: self.required_thin_feature_count,
                local_scale_order_count: self.required_local_scale_orders.len(),
                world_magnitude_order_count: 1,
                precision_escalation_count: precision_escalation_count(
                    &self.precision_scale_witnesses,
                ),
                local_basis_part_count: projection.local_basis_part_count(),
                projected_entity_count: projection.projected_entity_count(),
                transform_step_count: transform.transform_step_count(),
                tiny_rotation_pressure_count: tiny_rotation_count(self.tiny_rotation_pressure),
                projection_consumed_basis_count: projection_consumption
                    .counters()
                    .projection_receipts_consumed(),
                diagnostic_count: diagnostics.diagnostic_count(),
                user_outcome_count: response.user_outcome_count(),
            },
        ))
    }

    fn require_precision_scale_evidence(
        &self,
        precision: &PlanarPrecisionCertificateReceipt,
        local_frame: &PlanarLocalFrameCertificateReceipt,
    ) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        if self.required_local_scale_orders.len() < 3
            || !precision_witnesses_cover_required_scales_and_basis(
                precision,
                &self.precision_scale_witnesses,
                &self.required_local_scale_orders,
            )
            || !self
                .required_local_scale_orders
                .contains(&local_frame.basis().local_feature_scale_order())
        {
            return Err(ThinFeatureScaleSeparationWorkloadError::PrecisionBasisFailure);
        }
        Ok(())
    }

    fn require_world_magnitude_floor(
        &self,
        precision: &PlanarPrecisionCertificateReceipt,
        local_frame: &PlanarLocalFrameCertificateReceipt,
    ) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        if self.required_world_magnitude_order < 12
            || precision.basis().world_magnitude_order() < self.required_world_magnitude_order
            || local_frame.basis().world_magnitude_order() < self.required_world_magnitude_order
        {
            return Err(ThinFeatureScaleSeparationWorkloadError::PrecisionBasisFailure);
        }
        Ok(())
    }

    fn require_tiny_rotation_pressure(
        &self,
    ) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        match self.tiny_rotation_pressure {
            ThinFeatureTinyRotationPressure::RequiredAndSupported => Ok(()),
            ThinFeatureTinyRotationPressure::Unsupported => {
                Err(ThinFeatureScaleSeparationWorkloadError::UnsupportedTinyRotationPosture)
            }
        }
    }

    fn require_predicate_certification(
        &self,
    ) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        match self.predicate_certification {
            ThinFeaturePredicateCertification::Certified => Ok(()),
            ThinFeaturePredicateCertification::Uncertain => {
                Err(ThinFeatureScaleSeparationWorkloadError::PredicateUncertain)
            }
        }
    }

    fn require_scale_policy(&self) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        match self.scale_policy {
            ThinFeatureScalePolicy::Admit => Ok(()),
            ThinFeatureScalePolicy::RequiresUserDecision => {
                Err(ThinFeatureScaleSeparationWorkloadError::PolicyRequired)
            }
        }
    }

    fn require_evidence_integrity(&self) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        match self.evidence_integrity {
            ThinFeatureEvidenceIntegrity::Consistent => Ok(()),
            ThinFeatureEvidenceIntegrity::MismatchedLocalFrameProjection { stage } => {
                Err(ThinFeatureScaleSeparationWorkloadError::IntegrityMismatch { stage })
            }
        }
    }

    fn local_scale_orders(&self) -> Vec<i32> {
        self.required_local_scale_orders.iter().copied().collect()
    }
}
