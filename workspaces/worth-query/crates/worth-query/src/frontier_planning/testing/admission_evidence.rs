use super::{
    FrontierAwarePlan, FrontierBreadthPrediction, FrontierDisjointnessClass, FrontierPostureDigest,
    FrontierPredictionDriftOutcome, FrontierSurfaceDigest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParallelAdmissionDecision {
    disjointness_class: FrontierDisjointnessClass,
    predicted_breadth: FrontierBreadthPrediction,
    packet_count: usize,
}

impl ParallelAdmissionDecision {
    pub fn disjointness_class(&self) -> &FrontierDisjointnessClass {
        &self.disjointness_class
    }

    pub fn predicted_breadth(&self) -> &FrontierBreadthPrediction {
        &self.predicted_breadth
    }

    pub fn packet_count(&self) -> usize {
        self.packet_count
    }

    pub(in crate::frontier_planning::testing) fn from_frontier_plan(
        plan: &FrontierAwarePlan,
    ) -> Self {
        Self {
            disjointness_class: plan.disjointness_class().clone(),
            predicted_breadth: plan.predicted_breadth().clone(),
            packet_count: plan.packet_set().packets().len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SerialFallbackReason {
    DeterministicAdmissionDenied,
    PredictionDriftRequiresSerialRoute,
    SerialExecutor,
    BelowMinStageWidth,
    BelowPolicyWorkThreshold,
    ValidationHeavyStage,
    BelowFullParallelThreshold,
    FullParallelUnsupportedByMutableEngine,
}

impl SerialFallbackReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeterministicAdmissionDenied => "deterministic_admission_denied",
            Self::PredictionDriftRequiresSerialRoute => "prediction_drift_requires_serial_route",
            Self::SerialExecutor => "serial_executor",
            Self::BelowMinStageWidth => "below_min_stage_width",
            Self::BelowPolicyWorkThreshold => "below_policy_work_threshold",
            Self::ValidationHeavyStage => "validation_heavy_stage",
            Self::BelowFullParallelThreshold => "below_full_parallel_threshold",
            Self::FullParallelUnsupportedByMutableEngine => {
                "full_parallel_unsupported_by_mutable_engine"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontierRouteEvidence {
    basis_digest: String,
    pub(in crate::frontier_planning::testing) surface_digest: FrontierSurfaceDigest,
    pub(in crate::frontier_planning::testing) drift_outcome: FrontierPredictionDriftOutcome,
    pub(in crate::frontier_planning::testing) disjointness_class: Option<FrontierDisjointnessClass>,
    serial_fallback_reason: Option<SerialFallbackReason>,
}

impl FrontierRouteEvidence {
    pub(crate) fn parallel_admission(
        basis_digest: String,
        surface_digest: FrontierSurfaceDigest,
        disjointness_class: FrontierDisjointnessClass,
    ) -> Self {
        Self {
            basis_digest,
            surface_digest,
            drift_outcome: FrontierPredictionDriftOutcome::WithinBudget,
            disjointness_class: Some(disjointness_class),
            serial_fallback_reason: None,
        }
    }

    pub(crate) fn serial_fallback(
        basis_digest: String,
        surface_digest: FrontierSurfaceDigest,
        reason: SerialFallbackReason,
        drift_outcome: FrontierPredictionDriftOutcome,
    ) -> Self {
        Self {
            basis_digest,
            surface_digest,
            drift_outcome,
            disjointness_class: None,
            serial_fallback_reason: Some(reason),
        }
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.surface_digest
    }

    pub fn drift_outcome(&self) -> &FrontierPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn serial_fallback_reason(&self) -> Option<&SerialFallbackReason> {
        self.serial_fallback_reason.as_ref()
    }

    pub(in crate::frontier_planning::testing) fn route_posture_digest(
        &self,
        frontier_plan: &FrontierAwarePlan,
    ) -> FrontierPostureDigest {
        FrontierPostureDigest::from_parts(&[
            format!(
                "frontier_plan_posture:{}",
                frontier_plan.report().posture_digest().as_str()
            ),
            format!("evidence_basis:{}", self.basis_digest),
            format!("frontier_surface:{}", self.surface_digest.as_str()),
            format!("drift_outcome:{}", self.drift_outcome.as_str()),
            format!(
                "disjointness:{}",
                self.disjointness_class
                    .as_ref()
                    .map(FrontierDisjointnessClass::as_str)
                    .unwrap_or("none")
            ),
            format!(
                "serial_fallback_reason:{}",
                self.serial_fallback_reason
                    .as_ref()
                    .map(SerialFallbackReason::as_str)
                    .unwrap_or("none")
            ),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParallelAdmissionEvidence {
    route_evidence: FrontierRouteEvidence,
}

impl ParallelAdmissionEvidence {
    pub fn basis_digest(&self) -> &str {
        self.route_evidence.basis_digest()
    }

    pub fn surface_digest(&self) -> &FrontierSurfaceDigest {
        self.route_evidence.surface_digest()
    }

    pub(crate) fn from_surface(
        basis_digest: impl Into<String>,
        surface_digest: FrontierSurfaceDigest,
        disjointness_class: FrontierDisjointnessClass,
    ) -> Self {
        Self {
            route_evidence: FrontierRouteEvidence::parallel_admission(
                basis_digest.into(),
                surface_digest,
                disjointness_class,
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_surface_with_drift_for_test(
        basis_digest: impl Into<String>,
        surface_digest: FrontierSurfaceDigest,
        disjointness_class: FrontierDisjointnessClass,
        drift_outcome: FrontierPredictionDriftOutcome,
    ) -> Self {
        Self {
            route_evidence: FrontierRouteEvidence {
                basis_digest: basis_digest.into(),
                surface_digest,
                drift_outcome,
                disjointness_class: Some(disjointness_class),
                serial_fallback_reason: None,
            },
        }
    }

    pub(in crate::frontier_planning::testing) fn route_evidence(&self) -> &FrontierRouteEvidence {
        &self.route_evidence
    }

    #[cfg(test)]
    pub(crate) fn route_posture_digest_for_test(
        &self,
        frontier_plan: &FrontierAwarePlan,
    ) -> FrontierPostureDigest {
        self.route_evidence.route_posture_digest(frontier_plan)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParallelAdmissionBundleEvidence {
    basis_digest: String,
    bundle_surface_digest: FrontierSurfaceDigest,
    route_evidences: Vec<ParallelAdmissionEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ParallelAdmissionBundleEvidenceError {
    EmptyRouteEvidence,
    MixedBasisDigest {
        expected_basis_digest: String,
        found_basis_digest: String,
    },
}

impl ParallelAdmissionBundleEvidence {
    pub(crate) fn from_routes(
        bundle_surface_digest: FrontierSurfaceDigest,
        route_evidences: Vec<ParallelAdmissionEvidence>,
    ) -> Result<Self, ParallelAdmissionBundleEvidenceError> {
        let first = route_evidences
            .first()
            .ok_or(ParallelAdmissionBundleEvidenceError::EmptyRouteEvidence)?;
        let expected_basis_digest = first.basis_digest().to_string();
        for route in route_evidences.iter().skip(1) {
            let found_basis_digest = route.basis_digest();
            if found_basis_digest != expected_basis_digest {
                return Err(ParallelAdmissionBundleEvidenceError::MixedBasisDigest {
                    expected_basis_digest,
                    found_basis_digest: found_basis_digest.to_string(),
                });
            }
        }

        Ok(Self {
            basis_digest: expected_basis_digest,
            bundle_surface_digest,
            route_evidences,
        })
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn bundle_surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.bundle_surface_digest
    }

    pub fn route_evidences(&self) -> &[ParallelAdmissionEvidence] {
        &self.route_evidences
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialFallbackEvidence {
    route_evidence: FrontierRouteEvidence,
}

impl SerialFallbackEvidence {
    pub fn basis_digest(&self) -> &str {
        self.route_evidence.basis_digest()
    }

    pub fn surface_digest(&self) -> &FrontierSurfaceDigest {
        self.route_evidence.surface_digest()
    }

    pub fn drift_outcome(&self) -> &FrontierPredictionDriftOutcome {
        self.route_evidence.drift_outcome()
    }

    pub fn reason(&self) -> &SerialFallbackReason {
        self.route_evidence
            .serial_fallback_reason()
            .expect("serial fallback evidence must carry fallback reason")
    }

    pub(crate) fn from_surface(
        basis_digest: impl Into<String>,
        surface_digest: FrontierSurfaceDigest,
        reason: SerialFallbackReason,
        drift_outcome: FrontierPredictionDriftOutcome,
    ) -> Self {
        Self {
            route_evidence: FrontierRouteEvidence::serial_fallback(
                basis_digest.into(),
                surface_digest,
                reason,
                drift_outcome,
            ),
        }
    }

    pub(in crate::frontier_planning::testing) fn route_evidence(&self) -> &FrontierRouteEvidence {
        &self.route_evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialFallbackBundleEvidence {
    basis_digest: String,
    bundle_surface_digest: FrontierSurfaceDigest,
    route_evidences: Vec<SerialFallbackEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SerialFallbackBundleEvidenceError {
    EmptyRouteEvidence,
    MixedBasisDigest {
        expected_basis_digest: String,
        found_basis_digest: String,
    },
}

impl SerialFallbackBundleEvidence {
    pub(crate) fn from_routes(
        bundle_surface_digest: FrontierSurfaceDigest,
        route_evidences: Vec<SerialFallbackEvidence>,
    ) -> Result<Self, SerialFallbackBundleEvidenceError> {
        let first = route_evidences
            .first()
            .ok_or(SerialFallbackBundleEvidenceError::EmptyRouteEvidence)?;
        let expected_basis_digest = first.basis_digest().to_string();
        for route in route_evidences.iter().skip(1) {
            let found_basis_digest = route.basis_digest();
            if found_basis_digest != expected_basis_digest {
                return Err(SerialFallbackBundleEvidenceError::MixedBasisDigest {
                    expected_basis_digest,
                    found_basis_digest: found_basis_digest.to_string(),
                });
            }
        }

        Ok(Self {
            basis_digest: expected_basis_digest,
            bundle_surface_digest,
            route_evidences,
        })
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn bundle_surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.bundle_surface_digest
    }

    pub fn route_evidences(&self) -> &[SerialFallbackEvidence] {
        &self.route_evidences
    }
}
