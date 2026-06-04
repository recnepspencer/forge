use crate::facade::{BridgeWritebackAuthorityOutcome, BridgeWritebackErrorKind};
use crate::routing::canonicalization::digest_string;

pub(in crate::harness::adapter::adapter_impl) struct AdmissionBoundaryAuthorityProof {
    projected_authority_outcome: BridgeWritebackAuthorityOutcome,
    aspect_authority_outcome: BridgeWritebackAuthorityOutcome,
    failure_kind: BridgeWritebackErrorKind,
    failure_digest: String,
    decision_trace_digest: String,
}

pub(in crate::harness::adapter::adapter_impl) struct AdmissionBoundaryShadowProtocolRejection {
    failure_kind: BridgeWritebackErrorKind,
    failure_digest: String,
    projected_admission_record_digest: String,
    aspect_admission_record_digest: String,
    effect_family_mismatch_rejected: bool,
    no_shadow_protocol_admission_record: bool,
    decision_trace_digest: String,
}

impl AdmissionBoundaryAuthorityProof {
    pub(super) fn from_authority_evidence(
        projected_authority_outcome: &BridgeWritebackAuthorityOutcome,
        aspect_authority_outcome: &BridgeWritebackAuthorityOutcome,
        failure_kind: BridgeWritebackErrorKind,
        failure_digest: &str,
    ) -> Self {
        Self {
            projected_authority_outcome: projected_authority_outcome.clone(),
            aspect_authority_outcome: aspect_authority_outcome.clone(),
            failure_kind,
            failure_digest: failure_digest.to_owned(),
            decision_trace_digest: digest_string(
                "bridge-writeback-family-admission-boundary-authority-trace",
                &format!(
                    "projected-authority={}|aspect-authority={}|shadow={:?}",
                    projected_authority_outcome.authoritative_artifact_digest(),
                    aspect_authority_outcome.authoritative_artifact_digest(),
                    failure_kind,
                ),
            )
            .to_string(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_authority_commit_digest(
        &self,
    ) -> &str {
        self.projected_authority_outcome
            .authoritative_artifact_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_authority_commit_digest(&self) -> &str {
        self.aspect_authority_outcome
            .authoritative_artifact_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn distinct_authority_artifacts(&self) -> bool {
        self.projected_authority_commit_digest() != self.aspect_authority_commit_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_authority_outcome(
        &self,
    ) -> &BridgeWritebackAuthorityOutcome {
        &self.projected_authority_outcome
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_authority_outcome(
        &self,
    ) -> &BridgeWritebackAuthorityOutcome {
        &self.aspect_authority_outcome
    }

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
}

impl AdmissionBoundaryShadowProtocolRejection {
    pub(super) fn from_shadow_protocol_error(
        failure_kind: BridgeWritebackErrorKind,
        failure_digest: String,
        projected_admission_record_digest: &str,
        aspect_admission_record_digest: &str,
    ) -> Self {
        let stopped_before_admission =
            failure_kind == BridgeWritebackErrorKind::FamilyBindingMismatch;
        Self {
            failure_kind,
            failure_digest,
            projected_admission_record_digest: projected_admission_record_digest.to_owned(),
            aspect_admission_record_digest: aspect_admission_record_digest.to_owned(),
            effect_family_mismatch_rejected: stopped_before_admission,
            no_shadow_protocol_admission_record: stopped_before_admission,
            decision_trace_digest: digest_string(
                "bridge-writeback-family-admission-boundary-shadow-trace",
                &format!(
                    "shadow={:?}|projected-admission={}|aspect-admission={}",
                    failure_kind, projected_admission_record_digest, aspect_admission_record_digest,
                ),
            )
            .to_string(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn failure_kind(
        &self,
    ) -> BridgeWritebackErrorKind {
        self.failure_kind
    }

    pub(in crate::harness::adapter::adapter_impl) fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_admission_record_digest(
        &self,
    ) -> &str {
        &self.projected_admission_record_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_admission_record_digest(&self) -> &str {
        &self.aspect_admission_record_digest
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_family_mismatch_rejected(
        &self,
    ) -> bool {
        self.effect_family_mismatch_rejected
    }

    pub(in crate::harness::adapter::adapter_impl) fn no_shadow_protocol_admission_record(
        &self,
    ) -> bool {
        self.no_shadow_protocol_admission_record
    }

    pub(in crate::harness::adapter::adapter_impl) fn decision_trace_digest(&self) -> &str {
        &self.decision_trace_digest
    }
}
