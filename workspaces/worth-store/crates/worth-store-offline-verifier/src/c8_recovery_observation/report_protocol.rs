use worth_foundational::facade::{
    BoundaryProtocolCompatibilityWindow, BoundaryProtocolIdentity,
    BoundaryProtocolUnsupportedVersion, BoundaryProtocolVersion,
};

pub const RECOVERY_OBSERVER_REPORT_PROTOCOL: BoundaryProtocolIdentity =
    BoundaryProtocolIdentity::new("store.physical.recovery-observer-report");
pub const RECOVERY_OBSERVER_REPORT_VERSION: BoundaryProtocolVersion =
    BoundaryProtocolVersion::new(1);
pub const RECOVERY_OBSERVER_REPORT_COMPATIBILITY_WINDOW: BoundaryProtocolCompatibilityWindow =
    BoundaryProtocolCompatibilityWindow::inclusive(
        BoundaryProtocolVersion::new(1),
        BoundaryProtocolVersion::new(1),
    );

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryObserverDecodeDenial {
    Malformed,
    WrongProtocolFamily,
    UnsupportedVersion(BoundaryProtocolUnsupportedVersion),
    DigestMismatch,
}

pub(super) fn encode_header(bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(
        &(RECOVERY_OBSERVER_REPORT_PROTOCOL.as_str().len() as u64).to_le_bytes(),
    );
    bytes.extend_from_slice(RECOVERY_OBSERVER_REPORT_PROTOCOL.as_str().as_bytes());
    bytes.extend_from_slice(&RECOVERY_OBSERVER_REPORT_VERSION.get().to_le_bytes());
}

pub(super) fn admit_header(bytes: &mut &[u8]) -> Result<(), RecoveryObserverDecodeDenial> {
    let family_length =
        usize::try_from(u64_value(bytes)?).map_err(|_| RecoveryObserverDecodeDenial::Malformed)?;
    let family = take(bytes, family_length)?;
    if family != RECOVERY_OBSERVER_REPORT_PROTOCOL.as_str().as_bytes() {
        return Err(RecoveryObserverDecodeDenial::WrongProtocolFamily);
    }
    let version = BoundaryProtocolVersion::try_new(u32_value(bytes)?)
        .map_err(|_| RecoveryObserverDecodeDenial::Malformed)?;
    RECOVERY_OBSERVER_REPORT_COMPATIBILITY_WINDOW
        .admit(version)
        .map_err(RecoveryObserverDecodeDenial::UnsupportedVersion)?;
    Ok(())
}

pub(super) fn u32_value(bytes: &mut &[u8]) -> Result<u32, RecoveryObserverDecodeDenial> {
    Ok(u32::from_le_bytes(
        take(bytes, 4)?
            .try_into()
            .map_err(|_| RecoveryObserverDecodeDenial::Malformed)?,
    ))
}

pub(super) fn u64_value(bytes: &mut &[u8]) -> Result<u64, RecoveryObserverDecodeDenial> {
    Ok(u64::from_le_bytes(
        take(bytes, 8)?
            .try_into()
            .map_err(|_| RecoveryObserverDecodeDenial::Malformed)?,
    ))
}

pub(super) fn array<const N: usize>(
    bytes: &mut &[u8],
) -> Result<[u8; N], RecoveryObserverDecodeDenial> {
    take(bytes, N)?
        .try_into()
        .map_err(|_| RecoveryObserverDecodeDenial::Malformed)
}

fn take<'a>(bytes: &mut &'a [u8], length: usize) -> Result<&'a [u8], RecoveryObserverDecodeDenial> {
    let (value, remaining) = bytes
        .split_at_checked(length)
        .ok_or(RecoveryObserverDecodeDenial::Malformed)?;
    *bytes = remaining;
    Ok(value)
}
