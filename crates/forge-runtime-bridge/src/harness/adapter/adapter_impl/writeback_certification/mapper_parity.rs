use crate::facade::{BridgeWritebackError, BridgeWritebackErrorKind, BridgeWritebackReplayBundle};
use crate::routing::canonicalization::digest_string;
use crate::writeback::{BridgeDerivedWritebackEffect, BridgeWritebackExecutionRecord};

pub(in crate::harness::adapter::adapter_impl) struct WritebackMapperParityMatrixEvidence<'a> {
    pub projected_effect: &'a BridgeDerivedWritebackEffect,
    pub aspect_effect: &'a BridgeDerivedWritebackEffect,
    pub projected_replay_bundle: &'a BridgeWritebackReplayBundle,
    pub aspect_replay_bundle: &'a BridgeWritebackReplayBundle,
    pub projected_mapper_envelope_retained: bool,
    pub aspect_mapper_envelope_retained: bool,
    pub projected_mapped_input_retained: bool,
    pub aspect_mapped_input_retained: bool,
    pub projected_execution_record: &'a BridgeWritebackExecutionRecord,
    pub aspect_execution_record: &'a BridgeWritebackExecutionRecord,
    pub projected_admission_record_digest: &'a str,
    pub aspect_admission_record_digest: &'a str,
    pub shadow_protocol_error: &'a BridgeWritebackError,
}

pub(in crate::harness::adapter::adapter_impl) struct WritebackMapperParityMatrix {
    projected_family: MapperParityFamilyEvidence,
    aspect_family: MapperParityFamilyEvidence,
    mapper_parity_proof: MapperParityProof,
    shadow_protocol_rejection: MapperParityShadowProtocolRejection,
}

pub(in crate::harness::adapter::adapter_impl) struct MapperParityFamilyEvidence {
    effect: BridgeDerivedWritebackEffect,
    replay_bundle: BridgeWritebackReplayBundle,
}

pub(in crate::harness::adapter::adapter_impl) struct MapperParityProof {
    projected_mapper_envelope_retained: bool,
    aspect_mapper_envelope_retained: bool,
    projected_mapped_input_retained: bool,
    aspect_mapped_input_retained: bool,
    projected_family_mapper_record_digest: Option<String>,
    aspect_family_mapper_record_digest: Option<String>,
    projected_family_execution_record_digest: String,
    aspect_family_execution_record_digest: String,
    projected_admission_record_digest: String,
    aspect_admission_record_digest: String,
    decision_trace_digest: String,
}

pub(in crate::harness::adapter::adapter_impl) struct MapperParityShadowProtocolRejection {
    failure_kind: BridgeWritebackErrorKind,
    failure_digest: String,
    decision_trace_digest: String,
    effect_family_mismatch_rejected: bool,
    no_shadow_protocol_mapper_envelope_retained: bool,
}

impl WritebackMapperParityMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn from_mapper_parity_evidence(
        evidence: WritebackMapperParityMatrixEvidence<'_>,
    ) -> Self {
        let projected_family = MapperParityFamilyEvidence::from_effect(
            evidence.projected_effect,
            evidence.projected_replay_bundle,
        );
        let aspect_family = MapperParityFamilyEvidence::from_effect(
            evidence.aspect_effect,
            evidence.aspect_replay_bundle,
        );
        let projected_mapper_record_digest = evidence
            .projected_execution_record
            .mapper_record_digest()
            .map(ToOwned::to_owned);
        let aspect_mapper_record_digest = evidence
            .aspect_execution_record
            .mapper_record_digest()
            .map(ToOwned::to_owned);
        let projected_execution_record_digest =
            evidence.projected_execution_record.digest().to_owned();
        let aspect_execution_record_digest = evidence.aspect_execution_record.digest().to_owned();
        let failure_kind = evidence.shadow_protocol_error.kind();
        let shadow_protocol_stopped_before_mapper = evidence.shadow_protocol_error.kind()
            == BridgeWritebackErrorKind::FamilyBindingMismatch;
        Self {
            mapper_parity_proof: MapperParityProof {
                projected_mapper_envelope_retained: evidence.projected_mapper_envelope_retained,
                aspect_mapper_envelope_retained: evidence.aspect_mapper_envelope_retained,
                projected_mapped_input_retained: evidence.projected_mapped_input_retained,
                aspect_mapped_input_retained: evidence.aspect_mapped_input_retained,
                projected_family_mapper_record_digest: projected_mapper_record_digest.clone(),
                aspect_family_mapper_record_digest: aspect_mapper_record_digest.clone(),
                projected_family_execution_record_digest: projected_execution_record_digest.clone(),
                aspect_family_execution_record_digest: aspect_execution_record_digest.clone(),
                projected_admission_record_digest: evidence.projected_admission_record_digest.to_owned(),
                aspect_admission_record_digest: evidence.aspect_admission_record_digest.to_owned(),
                decision_trace_digest: digest_string(
                    "bridge-writeback-family-mapper-parity-trace",
                    &format!(
                        "projected-admission={}|aspect-admission={}|projected-mapper={}|aspect-mapper={}|projected-execution={}|aspect-execution={}",
                        evidence.projected_admission_record_digest,
                        evidence.aspect_admission_record_digest,
                        projected_mapper_record_digest.as_deref().unwrap_or("none"),
                        aspect_mapper_record_digest.as_deref().unwrap_or("none"),
                        projected_execution_record_digest,
                        aspect_execution_record_digest,
                    ),
                )
                .to_string(),
            },
            shadow_protocol_rejection: MapperParityShadowProtocolRejection {
                failure_kind: failure_kind.clone(),
                failure_digest: digest_string(
                    "bridge-writeback-family-mapper-parity-shadow-protocol",
                    &evidence.shadow_protocol_error.to_string(),
                )
                .to_string(),
                decision_trace_digest: digest_string(
                    "bridge-writeback-family-mapper-parity-shadow-trace",
                    &format!(
                        "shadow={}|projected-admission={}|aspect-admission={}",
                        format!("{failure_kind:?}"),
                        evidence.projected_admission_record_digest,
                        evidence.aspect_admission_record_digest,
                    ),
                )
                .to_string(),
                effect_family_mismatch_rejected: shadow_protocol_stopped_before_mapper,
                no_shadow_protocol_mapper_envelope_retained: shadow_protocol_stopped_before_mapper,
            },
            projected_family,
            aspect_family,
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_family(
        &self,
    ) -> &MapperParityFamilyEvidence {
        &self.projected_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family(
        &self,
    ) -> &MapperParityFamilyEvidence {
        &self.aspect_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn mapper_parity_proof(
        &self,
    ) -> &MapperParityProof {
        &self.mapper_parity_proof
    }

    pub(in crate::harness::adapter::adapter_impl) fn shadow_protocol_rejection(
        &self,
    ) -> &MapperParityShadowProtocolRejection {
        &self.shadow_protocol_rejection
    }
}

impl MapperParityFamilyEvidence {
    fn from_effect(
        effect: &BridgeDerivedWritebackEffect,
        replay_bundle: &BridgeWritebackReplayBundle,
    ) -> Self {
        Self {
            effect: effect.clone(),
            replay_bundle: replay_bundle.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect(
        &self,
    ) -> &BridgeDerivedWritebackEffect {
        &self.effect
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle(
        &self,
    ) -> &BridgeWritebackReplayBundle {
        &self.replay_bundle
    }

    pub(in crate::harness::adapter::adapter_impl) fn writeback_effect_artifact_digest(
        &self,
    ) -> &str {
        self.effect.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_digest(&self) -> &str {
        self.effect.effect_intent_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_intent_patch_canonical_basis(
        &self,
    ) -> &str {
        self.effect.effect_intent().patch_canonical_basis()
    }

    pub(in crate::harness::adapter::adapter_impl) fn causality_digest(&self) -> &str {
        self.effect.causality_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn mapped_input_digest(&self) -> &str {
        self.effect.mapped_input_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn mapper_envelope_digest(&self) -> &str {
        self.effect.mapper_envelope_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn replay_bundle_digest(&self) -> &str {
        self.replay_bundle.digest()
    }
}

impl MapperParityProof {
    pub(in crate::harness::adapter::adapter_impl) fn projected_mapper_envelope_retained(
        &self,
    ) -> bool {
        self.projected_mapper_envelope_retained
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_mapper_envelope_retained(
        &self,
    ) -> bool {
        self.aspect_mapper_envelope_retained
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_mapped_input_retained(
        &self,
    ) -> bool {
        self.projected_mapped_input_retained
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_mapped_input_retained(&self) -> bool {
        self.aspect_mapped_input_retained
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_family_mapper_record_digest(
        &self,
    ) -> Option<&str> {
        self.projected_family_mapper_record_digest.as_deref()
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family_mapper_record_digest(
        &self,
    ) -> Option<&str> {
        self.aspect_family_mapper_record_digest.as_deref()
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_family_execution_record_digest(
        &self,
    ) -> &str {
        &self.projected_family_execution_record_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family_execution_record_digest(
        &self,
    ) -> &str {
        &self.aspect_family_execution_record_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_admission_record_digest(
        &self,
    ) -> &str {
        &self.projected_admission_record_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_admission_record_digest(&self) -> &str {
        &self.aspect_admission_record_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn decision_trace_digest(&self) -> &str {
        &self.decision_trace_digest
    }
}

impl MapperParityShadowProtocolRejection {
    pub(in crate::harness::adapter::adapter_impl) fn failure_kind(
        &self,
    ) -> BridgeWritebackErrorKind {
        self.failure_kind
    }

    pub(in crate::harness::adapter::adapter_impl) fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn decision_trace_digest(&self) -> &str {
        &self.decision_trace_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_family_mismatch_rejected(
        &self,
    ) -> bool {
        self.effect_family_mismatch_rejected
    }

    pub(in crate::harness::adapter::adapter_impl) fn no_shadow_protocol_mapper_envelope_retained(
        &self,
    ) -> bool {
        self.no_shadow_protocol_mapper_envelope_retained
    }
}
