use crate::admission_digest::hash_parts;

use super::counters::BasisEligibilityCounters;
use super::lower_runtime::LowerRuntimeBoundBasis;
use super::scoping::{
    ScopedBasisProof, ScopedCertificationBasis, ScopedInspectionBasis, ScopedMaterializationBasis,
    ScopedMutationPreparationBasis, ScopedObservationBasis, ScopedPreviewCloseoutBasis,
    ScopedReplayBasis, ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
};
use super::support::{basis_lifecycle_support_matrix, BasisLifecycleSupportMatrix};
use super::taxonomy::{BasisAuthorityPosture, BasisFamily, BasisLifecyclePosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisUseReceiptKind {
    Observation,
    MutationPreparation,
    Replay,
    Inspection,
    Materialization,
    SubscriptionDeclaration,
    SubscriptionActivation,
    PreviewCloseout,
    Certification,
}

impl BasisUseReceiptKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Observation => "observation",
            Self::MutationPreparation => "mutation_preparation",
            Self::Replay => "replay",
            Self::Inspection => "inspection",
            Self::Materialization => "materialization",
            Self::SubscriptionDeclaration => "subscription_declaration",
            Self::SubscriptionActivation => "subscription_activation",
            Self::PreviewCloseout => "preview_closeout",
            Self::Certification => "certification",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisNextTransition {
    LaterInspection,
    Materialization,
    EffectPlan,
    ProjectionConsumption,
    TemporalExtensionDeferred,
    AsyncResourceExtensionDeferred,
    StoreBackedReplayDeferred,
    DurableReloadDeferred,
    Certification,
}

impl BasisNextTransition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LaterInspection => "later_inspection",
            Self::Materialization => "materialization",
            Self::EffectPlan => "effect_plan",
            Self::ProjectionConsumption => "projection_consumption",
            Self::TemporalExtensionDeferred => "temporal_extension_deferred",
            Self::AsyncResourceExtensionDeferred => "async_resource_extension_deferred",
            Self::StoreBackedReplayDeferred => "store_backed_replay_deferred",
            Self::DurableReloadDeferred => "durable_reload_deferred",
            Self::Certification => "certification",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisUseReceipt {
    kind: BasisUseReceiptKind,
    basis_family: BasisFamily,
    authority: BasisAuthorityPosture,
    lifecycle: BasisLifecyclePosture,
    capability_digest: String,
    scoped_basis_digest: String,
    lower_runtime_basis_digest: String,
    lower_runtime_binding_digest: String,
    lower_runtime_evidence_digest: String,
    readmission_trace_digest: String,
    permitted_next_transitions: Vec<BasisNextTransition>,
    receipt_digest: String,
    counters: BasisEligibilityCounters,
}

impl BasisUseReceipt {
    fn new<S: ScopedBasisProof>(
        kind: BasisUseReceiptKind,
        bound_basis: LowerRuntimeBoundBasis<S>,
    ) -> Self {
        let scoped = bound_basis.scoped_basis();
        let permitted_next_transitions = transitions_for(kind);
        let counters = BasisEligibilityCounters::receipt_emission(
            bound_basis.counters().retained_evidence_lookup_width(),
        );
        let receipt_digest = hash_parts(&[
            "basis_use_receipt_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("family:{}", scoped.family().as_str()),
            format!("authority:{}", scoped.authority().as_str()),
            format!("lifecycle:{}", scoped.lifecycle().as_str()),
            format!("capability:{}", scoped.capability_digest()),
            format!("scoped:{}", scoped.scoped_basis_digest()),
            format!("lower_runtime_basis:{}", bound_basis.basis_digest()),
            format!(
                "lower_runtime_binding:{}",
                bound_basis.lower_runtime_binding_digest()
            ),
            format!("evidence:{}", bound_basis.evidence_digest()),
            format!(
                "readmission_trace:{}",
                bound_basis.readmission_trace().trace_digest()
            ),
            format!(
                "transitions:{}",
                transition_digest(&permitted_next_transitions)
            ),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            kind,
            basis_family: scoped.family(),
            authority: scoped.authority(),
            lifecycle: scoped.lifecycle(),
            capability_digest: scoped.capability_digest().to_string(),
            scoped_basis_digest: scoped.scoped_basis_digest().to_string(),
            lower_runtime_basis_digest: bound_basis.basis_digest().to_string(),
            lower_runtime_binding_digest: bound_basis.lower_runtime_binding_digest().to_string(),
            lower_runtime_evidence_digest: bound_basis.evidence_digest().to_string(),
            readmission_trace_digest: bound_basis.readmission_trace().trace_digest().to_string(),
            permitted_next_transitions,
            receipt_digest,
            counters,
        }
    }

    pub fn kind(&self) -> BasisUseReceiptKind {
        self.kind
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn authority(&self) -> BasisAuthorityPosture {
        self.authority
    }

    pub fn lifecycle(&self) -> BasisLifecyclePosture {
        self.lifecycle
    }

    pub fn capability_digest(&self) -> &str {
        &self.capability_digest
    }

    pub fn scoped_basis_digest(&self) -> &str {
        &self.scoped_basis_digest
    }

    pub fn lower_runtime_basis_digest(&self) -> &str {
        &self.lower_runtime_basis_digest
    }

    pub fn lower_runtime_binding_digest(&self) -> &str {
        &self.lower_runtime_binding_digest
    }

    pub fn lower_runtime_evidence_digest(&self) -> &str {
        &self.lower_runtime_evidence_digest
    }

    pub fn readmission_trace_digest(&self) -> &str {
        &self.readmission_trace_digest
    }

    pub fn permitted_next_transitions(&self) -> &[BasisNextTransition] {
        &self.permitted_next_transitions
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelfDescribingBasisEnvelope {
    receipt: BasisUseReceipt,
    support_matrix_digest: String,
    structured_warnings: Vec<String>,
    integrity_digest: String,
    envelope_digest: String,
    counters: BasisEligibilityCounters,
}

impl SelfDescribingBasisEnvelope {
    fn new(receipt: BasisUseReceipt, support_matrix: BasisLifecycleSupportMatrix) -> Self {
        let structured_warnings = warnings_for(&receipt);
        let integrity_digest = hash_parts(&[
            "basis_envelope_integrity_v1".to_string(),
            format!("receipt:{}", receipt.receipt_digest()),
            format!("support:{}", support_matrix.matrix_digest()),
            format!("warnings:{}", structured_warnings.len()),
        ]);
        let envelope_digest = hash_parts(&[
            "self_describing_basis_envelope_v1".to_string(),
            format!("receipt:{}", receipt.receipt_digest()),
            format!("integrity:{integrity_digest}"),
        ]);
        Self {
            receipt,
            support_matrix_digest: support_matrix.matrix_digest().to_string(),
            structured_warnings,
            integrity_digest,
            envelope_digest,
            counters: BasisEligibilityCounters::envelope_materialization(),
        }
    }

    pub fn receipt(&self) -> &BasisUseReceipt {
        &self.receipt
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn structured_warnings(&self) -> &[String] {
        &self.structured_warnings
    }

    pub fn lifecycle(&self) -> BasisLifecyclePosture {
        self.receipt.lifecycle()
    }

    pub fn readmission_trace_digest(&self) -> &str {
        self.receipt.readmission_trace_digest()
    }

    pub fn integrity_digest(&self) -> &str {
        &self.integrity_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

pub fn emit_observation_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedObservationBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::Observation, bound_basis)
}

pub fn emit_mutation_preparation_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedMutationPreparationBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::MutationPreparation, bound_basis)
}

pub fn emit_replay_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedReplayBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::Replay, bound_basis)
}

pub fn emit_inspection_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedInspectionBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::Inspection, bound_basis)
}

pub fn emit_materialization_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedMaterializationBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::Materialization, bound_basis)
}

pub fn emit_subscription_declaration_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedSubscriptionDeclarationBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::SubscriptionDeclaration, bound_basis)
}

pub fn emit_subscription_activation_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedSubscriptionActivationBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::SubscriptionActivation, bound_basis)
}

pub fn emit_preview_closeout_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedPreviewCloseoutBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::PreviewCloseout, bound_basis)
}

pub fn emit_certification_basis_receipt(
    bound_basis: LowerRuntimeBoundBasis<ScopedCertificationBasis>,
) -> BasisUseReceipt {
    BasisUseReceipt::new(BasisUseReceiptKind::Certification, bound_basis)
}

pub fn envelope_basis_use(receipt: BasisUseReceipt) -> SelfDescribingBasisEnvelope {
    SelfDescribingBasisEnvelope::new(receipt, basis_lifecycle_support_matrix())
}

fn transitions_for(kind: BasisUseReceiptKind) -> Vec<BasisNextTransition> {
    use BasisNextTransition::*;
    match kind {
        BasisUseReceiptKind::Observation => vec![
            LaterInspection,
            ProjectionConsumption,
            TemporalExtensionDeferred,
            AsyncResourceExtensionDeferred,
        ],
        BasisUseReceiptKind::MutationPreparation => vec![EffectPlan, LaterInspection],
        BasisUseReceiptKind::Replay => vec![LaterInspection, StoreBackedReplayDeferred],
        BasisUseReceiptKind::Inspection => vec![Materialization, Certification],
        BasisUseReceiptKind::Materialization => vec![ProjectionConsumption],
        BasisUseReceiptKind::SubscriptionDeclaration => {
            vec![ProjectionConsumption, AsyncResourceExtensionDeferred]
        }
        BasisUseReceiptKind::SubscriptionActivation => {
            vec![ProjectionConsumption, DurableReloadDeferred]
        }
        BasisUseReceiptKind::PreviewCloseout => vec![LaterInspection, Materialization],
        BasisUseReceiptKind::Certification => vec![Certification],
    }
}

fn transition_digest(transitions: &[BasisNextTransition]) -> String {
    hash_parts(
        &transitions
            .iter()
            .map(|transition| transition.as_str().to_string())
            .collect::<Vec<_>>(),
    )
}

fn warnings_for(receipt: &BasisUseReceipt) -> Vec<String> {
    let mut warnings = Vec::new();
    if receipt
        .permitted_next_transitions()
        .iter()
        .any(|transition| transition.as_str().ends_with("_deferred"))
    {
        warnings
            .push("deferred future-neighbor transition remains unsupported in 9.3.2".to_string());
    }
    warnings
}

#[cfg(test)]
mod tests;
