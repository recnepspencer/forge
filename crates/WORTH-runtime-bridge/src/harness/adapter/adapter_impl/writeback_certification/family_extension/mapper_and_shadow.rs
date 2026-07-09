use crate::facade::{BridgeWritebackError, BridgeWritebackErrorKind};
use crate::writeback::{BridgeWritebackExecutionRecord, BridgeWritebackFamilyAdmissionRecord};

use super::WritebackFamilyExtensionMatrixEvidence;

pub(in crate::harness::adapter::adapter_impl) struct FamilyExtensionMapperParityProof {
    projected_mapper_envelope_retained: bool,
    aspect_mapper_envelope_retained: bool,
    projected_mapped_input_retained: bool,
    aspect_mapped_input_retained: bool,
    projected_execution_record: BridgeWritebackExecutionRecord,
    aspect_execution_record: BridgeWritebackExecutionRecord,
    projected_admission_record: BridgeWritebackFamilyAdmissionRecord,
    aspect_admission_record: BridgeWritebackFamilyAdmissionRecord,
}

pub(in crate::harness::adapter::adapter_impl) struct FamilyExtensionShadowProtocolRejection {
    error: BridgeWritebackError,
    projected_admission_record: BridgeWritebackFamilyAdmissionRecord,
    aspect_admission_record: BridgeWritebackFamilyAdmissionRecord,
}

impl FamilyExtensionMapperParityProof {
    pub(super) fn from_mapper_evidence(
        evidence: &WritebackFamilyExtensionMatrixEvidence<'_>,
    ) -> Self {
        Self {
            projected_mapper_envelope_retained: evidence.projected_mapper_envelope_retained,
            aspect_mapper_envelope_retained: evidence.aspect_mapper_envelope_retained,
            projected_mapped_input_retained: evidence.projected_mapped_input_retained,
            aspect_mapped_input_retained: evidence.aspect_mapped_input_retained,
            projected_execution_record: evidence.projected_execution_record.clone(),
            aspect_execution_record: evidence.aspect_execution_record.clone(),
            projected_admission_record: evidence.projected_admission_record.clone(),
            aspect_admission_record: evidence.aspect_admission_record.clone(),
        }
    }

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
        self.projected_execution_record.mapper_record_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family_mapper_record_digest(
        &self,
    ) -> Option<&str> {
        self.aspect_execution_record.mapper_record_digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_family_execution_record_digest(
        &self,
    ) -> &str {
        self.projected_execution_record.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family_execution_record_digest(
        &self,
    ) -> &str {
        self.aspect_execution_record.digest()
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_execution_record(
        &self,
    ) -> &BridgeWritebackExecutionRecord {
        &self.projected_execution_record
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_execution_record(
        &self,
    ) -> &BridgeWritebackExecutionRecord {
        &self.aspect_execution_record
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_admission_record(
        &self,
    ) -> &BridgeWritebackFamilyAdmissionRecord {
        &self.projected_admission_record
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_admission_record(
        &self,
    ) -> &BridgeWritebackFamilyAdmissionRecord {
        &self.aspect_admission_record
    }
}

impl FamilyExtensionShadowProtocolRejection {
    pub(super) fn from_shadow_error(
        error: &BridgeWritebackError,
        projected_admission_record: &BridgeWritebackFamilyAdmissionRecord,
        aspect_admission_record: &BridgeWritebackFamilyAdmissionRecord,
    ) -> Self {
        Self {
            error: error.clone(),
            projected_admission_record: projected_admission_record.clone(),
            aspect_admission_record: aspect_admission_record.clone(),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn failure_kind(
        &self,
    ) -> BridgeWritebackErrorKind {
        self.error.kind()
    }

    pub(in crate::harness::adapter::adapter_impl) fn effect_family_mismatch_rejected(
        &self,
    ) -> bool {
        self.error.kind() == BridgeWritebackErrorKind::FamilyBindingMismatch
    }

    pub(in crate::harness::adapter::adapter_impl) fn no_shadow_protocol_mapper_envelope_retained(
        &self,
    ) -> bool {
        self.error.kind() == BridgeWritebackErrorKind::FamilyBindingMismatch
    }

    pub(in crate::harness::adapter::adapter_impl) fn error(&self) -> &BridgeWritebackError {
        &self.error
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_admission_record(
        &self,
    ) -> &BridgeWritebackFamilyAdmissionRecord {
        &self.projected_admission_record
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_admission_record(
        &self,
    ) -> &BridgeWritebackFamilyAdmissionRecord {
        &self.aspect_admission_record
    }
}
