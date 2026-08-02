use worth_store_physical_backend::MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES;

use super::persisted_record_codec_io::{ControlRecordDecoder, ControlRecordEncoder};
use super::persisted_record_encoding::encode_kind;
use super::{
    OperationalControlRecord, PersistedControlRecordDecodeDenial,
    PersistedOperationalControlRecord, PersistedOperationalControlRecordKind,
    PersistedWorkflowKind,
};

const CONTROL_RECORD_MAGIC: [u8; 8] = *b"WSCTL004";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalControlEncodingDenial {
    AllocationFailed,
    RecordTooLarge,
}

pub(crate) fn encode_control_record(
    record: &OperationalControlRecord,
) -> Result<Vec<u8>, OperationalControlEncodingDenial> {
    let mut output = ControlRecordEncoder::new();
    output.bytes(&CONTROL_RECORD_MAGIC)?;
    output.bytes(&record.authority_identity().fingerprint())?;
    output.string(record.operation_id().as_str())?;
    output.string(record.transition_id().as_str())?;
    encode_kind(&mut output, record.kind())?;
    Ok(output.finish())
}

pub(crate) fn decode_control_record(
    bytes: &[u8],
) -> Result<PersistedOperationalControlRecord, PersistedControlRecordDecodeDenial> {
    if bytes.len() > MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES {
        return Err(PersistedControlRecordDecodeDenial::InvalidEncoding);
    }
    let mut input = ControlRecordDecoder::new(bytes);
    if input.array::<8>()? != CONTROL_RECORD_MAGIC {
        return Err(PersistedControlRecordDecodeDenial::InvalidEncoding);
    }
    let authority_identity_fingerprint = input.array()?;
    let operation_id = input.string()?;
    let transition_id = input.string()?;
    let kind = decode_kind(&mut input)?;
    input.require_eof()?;
    Ok(PersistedOperationalControlRecord {
        authority_identity_fingerprint,
        operation_id,
        transition_id,
        kind,
    })
}

fn decode_kind(
    input: &mut ControlRecordDecoder<'_>,
) -> Result<PersistedOperationalControlRecordKind, PersistedControlRecordDecodeDenial> {
    Ok(match input.u8()? {
        1 => PersistedOperationalControlRecordKind::WorkflowOpened {
            workflow: workflow_from_tag(input.u8()?)?,
        },
        4 => PersistedOperationalControlRecordKind::SourceLeasePersisted {
            cut_identity: input.array()?,
            object_digest: input.array()?,
            object_bytes: input.u64()?,
        },
        11 => PersistedOperationalControlRecordKind::BackupMaterializationOpened {
            cut_identity: input.array()?,
            target_platform: input.u8()?,
            target_bytes: input.length_prefixed_bytes()?,
            buffer_bytes: input.u64()?,
        },
        5 => PersistedOperationalControlRecordKind::BackupMaterializationRecorded {
            manifest_digest: input.array()?,
        },
        6 => PersistedOperationalControlRecordKind::IndependentBackupVerificationRecordedAndSourceLeaseReleased {
            verification_identity: input.array()?,
            release_recovery_bytes: input.length_prefixed_bytes()?,
        },
        9 => PersistedOperationalControlRecordKind::BackupAbandoned {
            reason: input.string()?,
            released_source_lease: match input.u8()? {
                1 => input.length_prefixed_bytes()?,
                _ => return Err(PersistedControlRecordDecodeDenial::InvalidEncoding),
            },
        },
        12 => PersistedOperationalControlRecordKind::AuthorizationConsumed {
            authorization_identity: input.array()?,
            plan_fingerprint: input.array()?,
            operation_tag: input.u8()?,
            execution_plan_fingerprint: match input.u8()? {
                0 => None,
                1 => Some(input.array()?),
                _ => return Err(PersistedControlRecordDecodeDenial::InvalidEncoding),
            },
            assertion_identity: input.array()?,
            expires_at: input.u64()?,
            replay_same_operation_identity: match input.u8()? {
                0 => false,
                1 => true,
                _ => return Err(PersistedControlRecordDecodeDenial::InvalidEncoding),
            },
        },
        13 => PersistedOperationalControlRecordKind::RepairExecutionOpened {
            authorization_identity: input.array()?, plan_fingerprint: input.array()?,
            owner_node_count: input.u64()?, topology_tag: input.u8()?,
        },
        14 => PersistedOperationalControlRecordKind::RepairOwnerReceiptPersisted {
            plan_fingerprint: input.array()?, node_fingerprint: input.array()?,
            receipt_fingerprint: input.array()?, owner_tag: input.u8()?,
        },
        15 => PersistedOperationalControlRecordKind::RepairDispositionRecorded {
            plan_fingerprint: input.array()?, disposition_tag: input.u8()?,
            disposition_basis: input.array()?,
        },
        18 => PersistedOperationalControlRecordKind::RecoveryStagingCompleted {
            authorization_identity: input.array()?, plan_fingerprint: input.array()?,
            execution_plan_fingerprint: input.array()?, staged_media_identity: input.array()?,
        },
        21 => PersistedOperationalControlRecordKind::RepairOwnerEffectStarted {
            plan_fingerprint: input.array()?,
            node_fingerprint: input.array()?,
            owner_tag: input.u8()?,
        },
        22 => PersistedOperationalControlRecordKind::OperationalOwnerReceiptPersisted {
            workflow: workflow_from_tag(input.u8()?)?,
            plan_fingerprint: input.array()?,
            receipt_fingerprint: input.array()?,
            owner_tag: input.u8()?,
        },
        23 => PersistedOperationalControlRecordKind::ReplicaBootstrapTransferRecorded {
            authorization_plan_fingerprint: input.array()?,
            execution_plan_fingerprint: input.array()?,
            receipt_identity: input.array()?,
            durable_target_identity: input.array()?,
            source_lease_identity: input.array()?,
            source_bytes_read: input.u64()?,
            output_bytes_written: input.u64()?,
            backend_requests: input.u64()?,
            maximum_resident_buffer_bytes: input.u64()?,
        },
        24 => PersistedOperationalControlRecordKind::ReplicaPromotionFenceRecorded {
            authorization_plan_fingerprint: input.array()?,
            execution_plan_fingerprint: input.array()?,
            fence_identity: input.array()?,
            promoted_epoch: input.u64()?,
        },
        25 => PersistedOperationalControlRecordKind::ReplicaPromotionRecorded {
            authorization_plan_fingerprint: input.array()?,
            execution_plan_fingerprint: input.array()?,
            receipt_identity: input.array()?,
            fence_identity: input.array()?,
            promoted_epoch: input.u64()?,
        },
        26 => PersistedOperationalControlRecordKind::ReplicaBootstrapCompleted {
            receipt_identity: input.array()?,
            verification_identity: input.array()?,
            source_lease_identity: input.array()?,
        },
        27 => PersistedOperationalControlRecordKind::ReplicaBootstrapAbandoned {
            receipt_identity: input.array()?,
            reason: input.string()?,
            source_lease_identity: input.array()?,
        },
        28 => PersistedOperationalControlRecordKind::ReplicaPromotionPublished {
            receipt_identity: input.array()?, verification_identity: input.array()?,
            publication_identity: input.array()?, target_identity: input.array()?,
            promoted_epoch: input.u64()?,
        },
        29 => PersistedOperationalControlRecordKind::ReplicaPromotionReadmitted {
            publication_identity: input.array()?, serve_lease_identity: input.array()?,
            serving_epoch: input.u64()?,
        },
        30 => PersistedOperationalControlRecordKind::OldPrimaryRejoinPlanned {
            promotion_receipt_identity: input.array()?,
            rejoin_plan_fingerprint: input.array()?, disposition_tag: input.u8()?,
        },
        31 => PersistedOperationalControlRecordKind::OldPrimaryRejoinCompleted {
            rejoin_plan_fingerprint: input.array()?,
            rejoin_receipt_identity: input.array()?,
            forensic_retention_identity: input.array()?,
            rebootstrap_target_identity: input.array()?,
            disposition_tag: input.u8()?,
        },
        _ => return Err(PersistedControlRecordDecodeDenial::InvalidEncoding),
    })
}

const fn workflow_from_tag(
    tag: u8,
) -> Result<PersistedWorkflowKind, PersistedControlRecordDecodeDenial> {
    Ok(match tag {
        1 => PersistedWorkflowKind::OfflineInspection,
        2 => PersistedWorkflowKind::Backup,
        3 => PersistedWorkflowKind::Restore,
        4 => PersistedWorkflowKind::PointInTimeRecovery,
        5 => PersistedWorkflowKind::Rollback,
        6 => PersistedWorkflowKind::Repair,
        7 => PersistedWorkflowKind::ReplicaBootstrap,
        8 => PersistedWorkflowKind::ReplicaPromotion,
        9 => PersistedWorkflowKind::ForensicAcquisition,
        _ => return Err(PersistedControlRecordDecodeDenial::InvalidEncoding),
    })
}
