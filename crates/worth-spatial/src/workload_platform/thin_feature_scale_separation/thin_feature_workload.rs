use std::collections::BTreeSet;

use worth_primitives::{truth_digest_parts, TruthDigestScope};

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
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

pub struct ThinFeatureScaleSeparationWorkload<'a> {
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
    precision: Option<&'a PlanarPrecisionCertificateReceipt>,
    precision_scale_witnesses: Vec<&'a PlanarPrecisionCertificateReceipt>,
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
        let local_frame = self.required_local_frame_receipt()?;
        let projection_consumption = self.required_projection_consumption_receipt()?;
        self.require_precision_and_frame_alignment(precision, local_frame)?;
        self.require_projection_consumption_alignment(local_frame, projection_consumption)?;

        let counters =
            self.thin_feature_counters(precision, local_frame, projection_consumption)?;
        let workload_identity = self.workload_identity()?;
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
        let topology = self.stage_counters(WorkloadEvidenceStage::Topology)?;
        let binding = self.stage_counters(WorkloadEvidenceStage::GeometryBinding)?;
        let support = self.stage_counters(WorkloadEvidenceStage::SurfaceSupport)?;
        let projection = self.stage_counters(WorkloadEvidenceStage::Projection)?;
        let transform = self.stage_counters(WorkloadEvidenceStage::Transform)?;
        let diagnostics = self.stage_counters(WorkloadEvidenceStage::Diagnostics)?;
        let response = self.stage_counters(WorkloadEvidenceStage::Response)?;

        self.require_platform_stage_evidence(
            topology,
            binding,
            support,
            projection,
            transform,
            diagnostics,
            response,
        )?;
        self.require_thin_feature_topology_breadth(topology)?;
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

    fn require_platform_stage_evidence(
        &self,
        topology: WorkloadEvidenceStageCounters,
        binding: WorkloadEvidenceStageCounters,
        support: WorkloadEvidenceStageCounters,
        projection: WorkloadEvidenceStageCounters,
        transform: WorkloadEvidenceStageCounters,
        diagnostics: WorkloadEvidenceStageCounters,
        response: WorkloadEvidenceStageCounters,
    ) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        if topology.topology_face_count() == 0 || binding.binding_target_count() == 0 {
            return Err(ThinFeatureScaleSeparationWorkloadError::MissingTopologyEvidence);
        }
        if support.surface_support_count() == 0 {
            return Err(ThinFeatureScaleSeparationWorkloadError::MissingSurfaceSupportEvidence);
        }
        if projection.local_basis_part_count() == 0 || projection.projected_entity_count() == 0 {
            return Err(ThinFeatureScaleSeparationWorkloadError::MissingProjectionEvidence);
        }
        if transform.transform_step_count() == 0 {
            return Err(ThinFeatureScaleSeparationWorkloadError::MissingTransformEvidence);
        }
        if diagnostics.diagnostic_count() == 0 || response.user_outcome_count() == 0 {
            return Err(ThinFeatureScaleSeparationWorkloadError::MissingResponseEvidence);
        }
        Ok(())
    }

    fn require_thin_feature_topology_breadth(
        &self,
        topology: WorkloadEvidenceStageCounters,
    ) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
        if self.required_thin_feature_count < 12
            || topology.topology_relation_count() < self.required_thin_feature_count
        {
            return Err(ThinFeatureScaleSeparationWorkloadError::MissingTopologyEvidence);
        }
        Ok(())
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

    fn stage_counters(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Result<WorkloadEvidenceStageCounters, ThinFeatureScaleSeparationWorkloadError> {
        self.evidence_ledger
            .row_for_stage(stage)
            .filter(|row| row.is_receipt_backed() && row.is_admitted())
            .map(|row| row.counters())
            .ok_or(ThinFeatureScaleSeparationWorkloadError::MissingReceiptBackedStage(stage))
    }

    fn workload_identity(&self) -> Result<String, ThinFeatureScaleSeparationWorkloadError> {
        let mut parts = Vec::new();
        for stage in WorkloadEvidenceStage::AUTHORITY_STAGES {
            let row = self
                .evidence_ledger
                .row_for_stage(stage)
                .filter(|row| row.is_receipt_backed() && row.is_admitted())
                .ok_or(ThinFeatureScaleSeparationWorkloadError::MissingReceiptBackedStage(stage))?;
            parts.push(format!("{stage:?}:{}", row.evidence_identity()));
        }
        Ok(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "thin-feature-scale-separation-ledger".to_string(),
                parts.join("|"),
            ],
        ))
    }

    fn local_scale_orders(&self) -> Vec<i32> {
        self.required_local_scale_orders.iter().copied().collect()
    }
}
