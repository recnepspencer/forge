use worth_store_physical_backend::MAX_OPERATIONAL_CONTROL_PAYLOAD_BYTES;

use super::persisted_record_codec_io::{ControlRecordDecoder, ControlRecordEncoder};
use super::publication_binding_codec::{
    decode_admission_policy, decode_authority_posture, encode_admission_policy,
    encode_authority_posture,
};
use super::{
    OperationalControlRecord, OperationalControlRecordKind, OperationalWorkflowKind,
    PersistedControlRecordDecodeDenial, PersistedOperationalControlRecord,
    PersistedOperationalControlRecordKind, PersistedWorkflowKind,
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

fn encode_kind(
    output: &mut ControlRecordEncoder,
    kind: &OperationalControlRecordKind,
) -> Result<(), OperationalControlEncodingDenial> {
    match kind {
        OperationalControlRecordKind::WorkflowOpened { workflow } => {
            output.u8(1)?;
            output.u8(workflow_tag(*workflow))
        }
        OperationalControlRecordKind::SourceLeasePersisted {
            recovery,
            recovery_object,
        } => {
            output.u8(4)?;
            output.bytes(&recovery.cut_identity())?;
            output.bytes(&recovery_object.digest())?;
            output.u64(recovery_object.bytes())
        }
        OperationalControlRecordKind::BackupMaterializationOpened { plan } => {
            let (platform, path) = plan
                .persisted_path()
                .map_err(|denial| match denial {
                    super::operational_media_path::OperationalMediaPathDenial::AllocationFailed => {
                        OperationalControlEncodingDenial::AllocationFailed
                    }
                    _ => OperationalControlEncodingDenial::RecordTooLarge,
                })?;
            output.u8(11)?;
            output.bytes(&plan.cut_identity())?;
            output.u8(platform)?;
            output.length_prefixed_bytes(&path)?;
            output.u64(plan.buffer_bytes() as u64)
        }
        OperationalControlRecordKind::BackupMaterializationRecorded {
            manifest_digest,
        } => {
            output.u8(5)?;
            output.bytes(manifest_digest)
        }
        OperationalControlRecordKind::IndependentBackupVerificationRecordedAndSourceLeaseReleased {
            verification_identity,
            release,
        } => {
            output.u8(6)?;
            output.bytes(verification_identity)?;
            output.length_prefixed_bytes(release.recovery_bytes())
        }
        OperationalControlRecordKind::BackupAbandoned {
            reason,
            released_source_lease,
        } => {
            output.u8(9)?;
            output.string(reason)?;
            output.u8(1)?;
            output.length_prefixed_bytes(released_source_lease.recovery_bytes())
        }
        OperationalControlRecordKind::AuthorizationConsumed {
            authorization_identity,
            plan_fingerprint,
            operation_tag,
            execution_plan_fingerprint,
            assertion_identity,
            expires_at,
            replay_same_operation_identity,
        } => {
            output.u8(12)?;
            output.bytes(authorization_identity)?;
            output.bytes(plan_fingerprint)?;
            output.u8(*operation_tag)?;
            match execution_plan_fingerprint {
                Some(fingerprint) => {
                    output.u8(1)?;
                    output.bytes(fingerprint)?;
                }
                None => output.u8(0)?,
            }
            output.bytes(assertion_identity)?;
            output.u64(*expires_at)?;
            output.u8(u8::from(*replay_same_operation_identity))
        }
        OperationalControlRecordKind::RepairExecutionOpened {
            authorization_identity, plan_fingerprint, owner_node_count, topology_tag,
        } => { output.u8(13)?; output.bytes(authorization_identity)?;
            output.bytes(plan_fingerprint)?; output.u64(*owner_node_count)?; output.u8(*topology_tag) }
        OperationalControlRecordKind::RepairOwnerReceiptPersisted {
            plan_fingerprint, node_fingerprint, receipt_fingerprint, owner_tag,
        } => { output.u8(14)?; output.bytes(plan_fingerprint)?; output.bytes(node_fingerprint)?;
            output.bytes(receipt_fingerprint)?; output.u8(*owner_tag) }
        OperationalControlRecordKind::RepairOwnerEffectStarted {
            plan_fingerprint, node_fingerprint, owner_tag,
        } => { output.u8(21)?; output.bytes(plan_fingerprint)?;
            output.bytes(node_fingerprint)?; output.u8(*owner_tag) }
        OperationalControlRecordKind::OperationalOwnerReceiptPersisted {
            workflow, plan_fingerprint, receipt_fingerprint, owner_tag,
        } => {
            output.u8(22)?;
            output.u8(workflow_tag(*workflow))?;
            output.bytes(plan_fingerprint)?;
            output.bytes(receipt_fingerprint)?;
            output.u8(*owner_tag)
        }
        OperationalControlRecordKind::RepairDispositionRecorded {
            plan_fingerprint, disposition_tag, disposition_basis,
        } => { output.u8(15)?; output.bytes(plan_fingerprint)?; output.u8(*disposition_tag)?;
            output.bytes(disposition_basis) }
        OperationalControlRecordKind::RecoveryStagingCompleted {
            authorization_identity, plan_fingerprint, execution_plan_fingerprint,
            staged_media_identity,
        } => {
            output.u8(18)?;
            output.bytes(authorization_identity)?;
            output.bytes(plan_fingerprint)?;
            output.bytes(execution_plan_fingerprint)?;
            output.bytes(staged_media_identity)
        }
        OperationalControlRecordKind::RecoveryPublicationPending { binding } => {
            output.u8(16)?;
            output.u8(binding.operation_tag())?;
            output.bytes(&binding.cutover_plan_fingerprint())?;
            output.bytes(&binding.publication_plan_fingerprint())?;
            output.bytes(&binding.publication_identity())?;
            output.bytes(&binding.candidate_media_identity())?;
            output.bytes(&binding.fence_identity())?;
            output.bytes(&binding.fence_plan_fingerprint())?;
            encode_authority_posture(output, binding.authority_posture())?;
            encode_admission_policy(output, binding.admission_policy())
        }
        OperationalControlRecordKind::RecoveryPublicationPrepared { binding } => {
            output.u8(19)?;
            output.u8(binding.operation_tag())?;
            output.bytes(&binding.cutover_plan_fingerprint())?;
            output.bytes(&binding.publication_plan_fingerprint())?;
            output.bytes(&binding.publication_identity())?;
            output.bytes(&binding.candidate_media_identity())?;
            output.bytes(&binding.fence_identity())?;
            output.bytes(&binding.fence_plan_fingerprint())?;
            encode_authority_posture(output, binding.authority_posture())?;
            encode_admission_policy(output, binding.admission_policy())
        }
        OperationalControlRecordKind::RecoveryPublicationDisposition {
            publication_identity, disposition_tag, disposition_basis, observed_authority,
        } => { output.u8(17)?; output.bytes(publication_identity)?; output.u8(*disposition_tag)?;
            output.bytes(disposition_basis)?; output.bytes(&observed_authority.fingerprint()) }
        OperationalControlRecordKind::RecoveryPublicationFenceReleased {
            publication_identity, fence_identity, fence_plan_fingerprint, disposition_tag,
        } => {
            output.u8(20)?;
            output.bytes(publication_identity)?;
            output.bytes(fence_identity)?;
            output.bytes(fence_plan_fingerprint)?;
            output.u8(*disposition_tag)
        }
    }
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
        16 => PersistedOperationalControlRecordKind::RecoveryPublicationPending {
            operation_tag: input.u8()?, cutover_plan_fingerprint: input.array()?,
            publication_plan_fingerprint: input.array()?, publication_identity: input.array()?,
            candidate_media_identity: input.array()?, fence_identity: input.array()?,
            fence_plan_fingerprint: input.array()?,
            authority_posture: decode_authority_posture(input)?,
            admission_policy: decode_admission_policy(input)?,
        },
        17 => PersistedOperationalControlRecordKind::RecoveryPublicationDisposition {
            publication_identity: input.array()?, disposition_tag: input.u8()?,
            disposition_basis: input.array()?,
            observed_authority: input.array()?,
        },
        18 => PersistedOperationalControlRecordKind::RecoveryStagingCompleted {
            authorization_identity: input.array()?, plan_fingerprint: input.array()?,
            execution_plan_fingerprint: input.array()?, staged_media_identity: input.array()?,
        },
        19 => PersistedOperationalControlRecordKind::RecoveryPublicationPrepared {
            operation_tag: input.u8()?, cutover_plan_fingerprint: input.array()?,
            publication_plan_fingerprint: input.array()?, publication_identity: input.array()?,
            candidate_media_identity: input.array()?, fence_identity: input.array()?,
            fence_plan_fingerprint: input.array()?,
            authority_posture: decode_authority_posture(input)?,
            admission_policy: decode_admission_policy(input)?,
        },
        20 => PersistedOperationalControlRecordKind::RecoveryPublicationFenceReleased {
            publication_identity: input.array()?,
            fence_identity: input.array()?,
            fence_plan_fingerprint: input.array()?,
            disposition_tag: input.u8()?,
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
        _ => return Err(PersistedControlRecordDecodeDenial::InvalidEncoding),
    })
}

const fn workflow_tag(kind: OperationalWorkflowKind) -> u8 {
    match kind {
        OperationalWorkflowKind::OfflineInspection => 1,
        OperationalWorkflowKind::Backup => 2,
        OperationalWorkflowKind::Restore => 3,
        OperationalWorkflowKind::PointInTimeRecovery => 4,
        OperationalWorkflowKind::Rollback => 5,
        OperationalWorkflowKind::Repair => 6,
        OperationalWorkflowKind::ReplicaBootstrap => 7,
        OperationalWorkflowKind::ReplicaPromotion => 8,
        OperationalWorkflowKind::ForensicAcquisition => 9,
    }
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
