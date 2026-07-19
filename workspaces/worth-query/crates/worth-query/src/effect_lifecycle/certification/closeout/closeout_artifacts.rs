use crate::effect_lifecycle::counters::EffectLifecycleCounters;
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExecutionCertificationLane {
    MutationReceiptSurface,
    WritebackReceiptSurface,
    BatchReceiptSurface,
    AdvisorySurface,
    DeferredSurface,
    DeniedSurface,
    MismatchDetectionSurface,
    ProofShapeSurface,
    PerformanceSurface,
    SupportAndDxSurface,
    CompileFailBoundary,
    SeededReplayParity,
    HostileExecutionSurface,
}

impl EffectExecutionCertificationLane {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MutationReceiptSurface => "mutation_receipt_surface",
            Self::WritebackReceiptSurface => "writeback_receipt_surface",
            Self::BatchReceiptSurface => "batch_receipt_surface",
            Self::AdvisorySurface => "advisory_surface",
            Self::DeferredSurface => "deferred_surface",
            Self::DeniedSurface => "denied_surface",
            Self::MismatchDetectionSurface => "mismatch_detection_surface",
            Self::ProofShapeSurface => "proof_shape_surface",
            Self::PerformanceSurface => "performance_surface",
            Self::SupportAndDxSurface => "support_and_dx_surface",
            Self::CompileFailBoundary => "compile_fail_boundary",
            Self::SeededReplayParity => "seeded_replay_parity",
            Self::HostileExecutionSurface => "hostile_execution_surface",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectExecutionCertificationRow {
    lane: EffectExecutionCertificationLane,
    evidence_digest: String,
    evidence_detail: String,
    counter_snapshot_digest: String,
    failure_digest: Option<String>,
    row_digest: String,
}

impl EffectExecutionCertificationRow {
    pub(super) fn new(
        lane: EffectExecutionCertificationLane,
        evidence_digest: String,
        evidence_detail: String,
        counters: &EffectLifecycleCounters,
        failure_digest: Option<String>,
    ) -> Self {
        let counter_snapshot_digest = counters.counter_for_reporting().to_string();
        let row_digest = hash_parts(&[
            "effect_execution_certification_row_v1".to_string(),
            format!("lane:{}", lane.as_str()),
            format!("evidence:{evidence_digest}"),
            format!("detail:{evidence_detail}"),
            format!("counters:{counter_snapshot_digest}"),
            format!("failure:{}", failure_digest.as_deref().unwrap_or("none")),
        ]);
        Self {
            lane,
            evidence_digest,
            evidence_detail,
            counter_snapshot_digest,
            failure_digest,
            row_digest,
        }
    }

    pub fn lane(&self) -> EffectExecutionCertificationLane {
        self.lane
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn evidence_detail(&self) -> &str {
        &self.evidence_detail
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn failure_digest(&self) -> Option<&str> {
        self.failure_digest.as_deref()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectExecutionCertificationOutputDigest {
    output_name: &'static str,
    digest: String,
}

impl EffectExecutionCertificationOutputDigest {
    pub(super) fn certified(output_name: &'static str, digest: String) -> Self {
        Self {
            output_name,
            digest,
        }
    }

    pub fn output_name(&self) -> &'static str {
        self.output_name
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectExecutionCertificationBundle {
    rows: Vec<EffectExecutionCertificationRow>,
    outputs: Vec<EffectExecutionCertificationOutputDigest>,
    seeded_bundle_digest: String,
    phase4_bundle_digest: String,
    certification_bundle_digest: String,
}

impl EffectExecutionCertificationBundle {
    pub(super) fn new(
        rows: Vec<EffectExecutionCertificationRow>,
        outputs: Vec<EffectExecutionCertificationOutputDigest>,
        seeded_bundle_digest: String,
        phase4_bundle_digest: String,
    ) -> Self {
        let certification_bundle_digest =
            hash_parts(
                &std::iter::once("effect_execution_certification_bundle_v1".to_string())
                    .chain(std::iter::once(format!("seeded:{seeded_bundle_digest}")))
                    .chain(std::iter::once(format!("phase4:{phase4_bundle_digest}")))
                    .chain(rows.iter().map(|row| format!("row:{}", row.row_digest())))
                    .chain(outputs.iter().map(|output| {
                        format!("output:{}:{}", output.output_name(), output.digest())
                    }))
                    .collect::<Vec<_>>(),
            );
        Self {
            rows,
            outputs,
            seeded_bundle_digest,
            phase4_bundle_digest,
            certification_bundle_digest,
        }
    }

    pub fn rows(&self) -> &[EffectExecutionCertificationRow] {
        &self.rows
    }

    pub fn outputs(&self) -> &[EffectExecutionCertificationOutputDigest] {
        &self.outputs
    }

    pub fn output_digest(&self, output_name: &str) -> Option<&str> {
        self.outputs
            .iter()
            .find(|output| output.output_name() == output_name)
            .map(|output| output.digest())
    }

    pub fn seeded_bundle_digest(&self) -> &str {
        &self.seeded_bundle_digest
    }

    pub fn phase4_bundle_digest(&self) -> &str {
        &self.phase4_bundle_digest
    }

    pub fn certification_bundle_digest(&self) -> &str {
        &self.certification_bundle_digest
    }
}
