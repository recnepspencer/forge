use super::persisted_record_codec::OperationalControlEncodingDenial;
use super::persisted_record_codec_io::{ControlRecordDecoder, ControlRecordEncoder};
use super::PersistedControlRecordDecodeDenial;

pub(super) fn encode_authority_posture(
    output: &mut ControlRecordEncoder,
    posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
) -> Result<(), OperationalControlEncodingDenial> {
    output.bytes(&posture.verification_identity())?;
    for region in posture.regions() {
        output.bytes(&region.identity())?;
        output.u64(region.count())?;
    }
    Ok(())
}

pub(super) fn decode_authority_posture(
    input: &mut ControlRecordDecoder<'_>,
) -> Result<
    worth_store_authority::RecoveryAuthorityAdmissionPosture,
    PersistedControlRecordDecodeDenial,
> {
    let verification_identity = input.array()?;
    let mut regions = [worth_store_authority::RecoveryAuthorityRegionPosture::observed([0; 32], 0)
        .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)?; 5];
    for region in &mut regions {
        *region = worth_store_authority::RecoveryAuthorityRegionPosture::observed(
            input.array()?,
            input.u64()?,
        )
        .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)?;
    }
    worth_store_authority::RecoveryAuthorityAdmissionPosture::from_independent_post_verification(
        verification_identity,
        regions,
    )
    .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)
}

pub(super) fn encode_admission_policy(
    output: &mut ControlRecordEncoder,
    policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
) -> Result<(), OperationalControlEncodingDenial> {
    output.u8(match policy.kind() {
        worth_store_authority::RecoveryAuthorityAdmissionPolicyKind::FullyTrustedOnly => 1,
        worth_store_authority::RecoveryAuthorityAdmissionPolicyKind::ExactDeclaredResidualPosture => 2,
    })?;
    output.bytes(&policy.admitted_posture_identity())?;
    output.bytes(&policy.decision_basis())
}

pub(super) fn decode_admission_policy(
    input: &mut ControlRecordDecoder<'_>,
) -> Result<
    worth_store_authority::RecoveryAuthorityAdmissionPolicy,
    PersistedControlRecordDecodeDenial,
> {
    let kind = match input.u8()? {
        1 => worth_store_authority::RecoveryAuthorityAdmissionPolicyKind::FullyTrustedOnly,
        2 => worth_store_authority::RecoveryAuthorityAdmissionPolicyKind::ExactDeclaredResidualPosture,
        _ => return Err(PersistedControlRecordDecodeDenial::InvalidEncoding),
    };
    worth_store_authority::RecoveryAuthorityAdmissionPolicy::from_persisted(
        kind,
        input.array()?,
        input.array()?,
    )
    .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)
}
