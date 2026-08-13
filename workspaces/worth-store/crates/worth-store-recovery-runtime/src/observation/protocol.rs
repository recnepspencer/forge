use worth_foundational::facade::{
    BoundaryProtocolCompatibilityWindow, BoundaryProtocolIdentity,
    BoundaryProtocolUnsupportedVersion, BoundaryProtocolVersion,
};

pub const RECOVERY_REPORT_PROTOCOL: BoundaryProtocolIdentity =
    BoundaryProtocolIdentity::new("store.physical.recovery-report");
pub const RECOVERY_REPORT_VERSION: BoundaryProtocolVersion = BoundaryProtocolVersion::new(1);
pub const RECOVERY_REPORT_COMPATIBILITY_WINDOW: BoundaryProtocolCompatibilityWindow =
    BoundaryProtocolCompatibilityWindow::inclusive(
        BoundaryProtocolVersion::new(1),
        BoundaryProtocolVersion::new(1),
    );

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryReportDecodeDenial {
    Malformed,
    WrongProtocolFamily,
    UnsupportedVersion(BoundaryProtocolUnsupportedVersion),
    DigestMismatch,
}

pub(super) fn admit_header(bytes: &mut &[u8]) -> Result<(), RecoveryReportDecodeDenial> {
    let family = field(bytes)?;
    if family != RECOVERY_REPORT_PROTOCOL.as_str().as_bytes() {
        return Err(RecoveryReportDecodeDenial::WrongProtocolFamily);
    }
    let version = BoundaryProtocolVersion::try_new(u32_value(bytes)?)
        .map_err(|_| RecoveryReportDecodeDenial::Malformed)?;
    RECOVERY_REPORT_COMPATIBILITY_WINDOW
        .admit(version)
        .map_err(RecoveryReportDecodeDenial::UnsupportedVersion)?;
    Ok(())
}

pub(super) fn encode_header(bytes: &mut Vec<u8>) {
    encode_field(bytes, RECOVERY_REPORT_PROTOCOL.as_str().as_bytes());
    bytes.extend_from_slice(&RECOVERY_REPORT_VERSION.get().to_le_bytes());
}

pub(super) fn encode_field(bytes: &mut Vec<u8>, field: &[u8]) {
    bytes.extend_from_slice(&(field.len() as u64).to_le_bytes());
    bytes.extend_from_slice(field);
}

pub(super) fn byte(bytes: &mut &[u8]) -> Result<u8, RecoveryReportDecodeDenial> {
    Ok(take(bytes, 1)?[0])
}

pub(super) fn u32_value(bytes: &mut &[u8]) -> Result<u32, RecoveryReportDecodeDenial> {
    Ok(u32::from_le_bytes(
        take(bytes, 4)?
            .try_into()
            .map_err(|_| RecoveryReportDecodeDenial::Malformed)?,
    ))
}

pub(super) fn u64_value(bytes: &mut &[u8]) -> Result<u64, RecoveryReportDecodeDenial> {
    Ok(u64::from_le_bytes(
        take(bytes, 8)?
            .try_into()
            .map_err(|_| RecoveryReportDecodeDenial::Malformed)?,
    ))
}

pub(super) fn array<const N: usize>(
    bytes: &mut &[u8],
) -> Result<[u8; N], RecoveryReportDecodeDenial> {
    take(bytes, N)?
        .try_into()
        .map_err(|_| RecoveryReportDecodeDenial::Malformed)
}

fn field<'a>(bytes: &mut &'a [u8]) -> Result<&'a [u8], RecoveryReportDecodeDenial> {
    let length =
        usize::try_from(u64_value(bytes)?).map_err(|_| RecoveryReportDecodeDenial::Malformed)?;
    take(bytes, length)
}

fn take<'a>(bytes: &mut &'a [u8], length: usize) -> Result<&'a [u8], RecoveryReportDecodeDenial> {
    let (value, remaining) = bytes
        .split_at_checked(length)
        .ok_or(RecoveryReportDecodeDenial::Malformed)?;
    *bytes = remaining;
    Ok(value)
}
