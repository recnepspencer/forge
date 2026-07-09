use crate::identity::hash_parts;

use super::super::migration::BasisLifecycleMigrationSurface;
use super::super::proofs::{BasisIntentDenial, DeniedBasisCapability};
use super::super::scoping::ScopedBasisProof;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecycleAdapterOutcome {
    ScopedCapability,
    AdvisoryEligibility,
    TypedDenial,
}

impl BasisLifecycleAdapterOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ScopedCapability => "scoped_capability",
            Self::AdvisoryEligibility => "advisory_eligibility",
            Self::TypedDenial => "typed_denial",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleAdapterProof {
    surface: BasisLifecycleMigrationSurface,
    entrypoint: &'static str,
    target_lifecycle_phase: &'static str,
    operation_lane: &'static str,
    outcome: BasisLifecycleAdapterOutcome,
    source_digest: String,
    lifecycle_proof_digest: String,
    adapter_proof_digest: String,
}

impl BasisLifecycleAdapterProof {
    pub(super) fn new(
        surface: BasisLifecycleMigrationSurface,
        entrypoint: &'static str,
        target_lifecycle_phase: &'static str,
        operation_lane: &'static str,
        outcome: BasisLifecycleAdapterOutcome,
        source_digest: String,
        lifecycle_proof_digest: String,
    ) -> Self {
        let adapter_proof_digest = hash_parts(&[
            "basis_lifecycle_adapter_proof_v1".to_string(),
            format!("surface:{}", surface.as_str()),
            format!("entrypoint:{entrypoint}"),
            format!("phase:{target_lifecycle_phase}"),
            format!("lane:{operation_lane}"),
            format!("outcome:{}", outcome.as_str()),
            format!("source:{source_digest}"),
            format!("lifecycle:{lifecycle_proof_digest}"),
        ]);
        Self {
            surface,
            entrypoint,
            target_lifecycle_phase,
            operation_lane,
            outcome,
            source_digest,
            lifecycle_proof_digest,
            adapter_proof_digest,
        }
    }

    pub fn surface(&self) -> BasisLifecycleMigrationSurface {
        self.surface
    }

    pub fn entrypoint(&self) -> &'static str {
        self.entrypoint
    }

    pub fn target_lifecycle_phase(&self) -> &'static str {
        self.target_lifecycle_phase
    }

    pub fn operation_lane(&self) -> &'static str {
        self.operation_lane
    }

    pub fn outcome(&self) -> BasisLifecycleAdapterOutcome {
        self.outcome
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn lifecycle_proof_digest(&self) -> &str {
        &self.lifecycle_proof_digest
    }

    pub fn adapter_proof_digest(&self) -> &str {
        &self.adapter_proof_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisLifecycleAdapterError {
    IntentDenied(BasisIntentDenial),
    CapabilityDenied(DeniedBasisCapability),
}

impl From<BasisIntentDenial> for BasisLifecycleAdapterError {
    fn from(value: BasisIntentDenial) -> Self {
        Self::IntentDenied(value)
    }
}

impl From<DeniedBasisCapability> for BasisLifecycleAdapterError {
    fn from(value: DeniedBasisCapability) -> Self {
        Self::CapabilityDenied(value)
    }
}

pub(super) fn scoped_proof(
    surface: BasisLifecycleMigrationSurface,
    entrypoint: &'static str,
    target_lifecycle_phase: &'static str,
    operation_lane: &'static str,
    source_digest: String,
    scoped: &impl ScopedBasisProof,
) -> BasisLifecycleAdapterProof {
    BasisLifecycleAdapterProof::new(
        surface,
        entrypoint,
        target_lifecycle_phase,
        operation_lane,
        BasisLifecycleAdapterOutcome::ScopedCapability,
        source_digest,
        scoped.scoped_basis_digest().to_string(),
    )
}

pub(super) fn advisory_proof(
    surface: BasisLifecycleMigrationSurface,
    entrypoint: &'static str,
    target_lifecycle_phase: &'static str,
    operation_lane: &'static str,
    source_digest: String,
    trace_digest: &str,
) -> BasisLifecycleAdapterProof {
    BasisLifecycleAdapterProof::new(
        surface,
        entrypoint,
        target_lifecycle_phase,
        operation_lane,
        BasisLifecycleAdapterOutcome::AdvisoryEligibility,
        source_digest,
        trace_digest.to_string(),
    )
}

pub(super) fn typed_denial_proof(
    surface: BasisLifecycleMigrationSurface,
    entrypoint: &'static str,
    target_lifecycle_phase: &'static str,
    operation_lane: &'static str,
    source_digest: String,
    message: &'static str,
) -> BasisLifecycleAdapterProof {
    let lifecycle_proof_digest = hash_parts(&[
        "basis_lifecycle_adapter_denial_v1".to_string(),
        format!("surface:{}", surface.as_str()),
        format!("entrypoint:{entrypoint}"),
        format!("lane:{operation_lane}"),
        format!("message:{message}"),
    ]);
    BasisLifecycleAdapterProof::new(
        surface,
        entrypoint,
        target_lifecycle_phase,
        operation_lane,
        BasisLifecycleAdapterOutcome::TypedDenial,
        source_digest,
        lifecycle_proof_digest,
    )
}

pub(super) fn source_digest(
    source_kind: &'static str,
    identity: &str,
    evidence: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    let mut parts = vec![
        "basis_lifecycle_legacy_source_v1".to_string(),
        format!("kind:{source_kind}"),
        format!("identity:{identity}"),
    ];
    parts.extend(
        evidence
            .into_iter()
            .map(|part| format!("evidence:{}", part.as_ref())),
    );
    hash_parts(&parts)
}
