use super::decoding::IdentityReceiptEntry;

#[derive(Debug, Clone, Copy)]
pub(super) struct NonDurableBindings {
    pub(super) no_effect_idempotency: [u8; 32],
    pub(super) dirty_idempotency: [u8; 32],
}

pub(super) fn bind(
    entries: &[IdentityReceiptEntry],
    start_ordinal: usize,
    no_effect_material: [u8; 32],
    dirty_material: [u8; 32],
    dirty_fate: u8,
) -> Result<NonDurableBindings, String> {
    let no_effect = entries
        .first()
        .ok_or_else(|| "C8 identity receipt omitted the proven-no-effect operation".to_owned())?;
    if no_effect.material != no_effect_material || no_effect.fate != 3 || no_effect.record.is_some()
    {
        return Err(format!(
            "C8 non-durable issued fate receipt mismatch at position {start_ordinal}"
        ));
    }
    let dirty = entries
        .get(1)
        .ok_or_else(|| "C8 identity receipt omitted the dirty operation".to_owned())?;
    if dirty.material != dirty_material || dirty.fate != dirty_fate || dirty.record.is_some() {
        return Err(format!(
            "C8 non-durable issued fate receipt mismatch at position {}",
            start_ordinal + 1
        ));
    }
    if entries.len() != 2 {
        return Err("C8 identity receipt contains unexpected non-durable operations".to_owned());
    }
    Ok(NonDurableBindings {
        no_effect_idempotency: no_effect.idempotency,
        dirty_idempotency: dirty.idempotency,
    })
}
