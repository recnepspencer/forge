use super::StoreRecoveryBindingSampleDenial;

pub(super) fn decode_wal_member_payload(
    mut payload: &[u8],
) -> Result<(&[u8], &[u8]), StoreRecoveryBindingSampleDenial> {
    let binding = take_field(&mut payload)?;
    let redo = take_field(&mut payload)?;
    if binding.is_empty() || redo.is_empty() || !payload.is_empty() {
        return Err(StoreRecoveryBindingSampleDenial::InvalidWalMember);
    }
    Ok((binding, redo))
}

fn take_field<'payload>(
    payload: &mut &'payload [u8],
) -> Result<&'payload [u8], StoreRecoveryBindingSampleDenial> {
    let length = payload
        .get(..8)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u64::from_le_bytes)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(StoreRecoveryBindingSampleDenial::InvalidWalMember)?;
    let end = 8_usize
        .checked_add(length)
        .ok_or(StoreRecoveryBindingSampleDenial::InvalidWalMember)?;
    let field = payload
        .get(8..end)
        .ok_or(StoreRecoveryBindingSampleDenial::InvalidWalMember)?;
    *payload = payload
        .get(end..)
        .ok_or(StoreRecoveryBindingSampleDenial::InvalidWalMember)?;
    Ok(field)
}
