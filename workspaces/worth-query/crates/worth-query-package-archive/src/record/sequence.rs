use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::decode_budget::RecordDecodeAttempt;
use std::cmp::Ordering;

pub(super) fn write_sequence<T>(
    output: &mut dyn BinaryEncodingSink,
    values: &[T],
    mut write: impl FnMut(&mut dyn BinaryEncodingSink, &T) -> Result<(), Denial>,
) -> Result<(), Denial> {
    let count = u32::try_from(values.len()).map_err(|_| Denial::new(Kind::NumericWidthExceeded))?;
    output.claim_nested_entries(count)?;
    output.u32(count)?;
    for value in values {
        write(output, value)?;
    }
    Ok(())
}

pub(super) fn decode_sequence<'a, T>(
    input: &mut BinaryInput<'a>,
    budget: &mut RecordDecodeAttempt,
    minimum_entry_bytes: usize,
    mut decode: impl FnMut(&mut BinaryInput<'a>, &mut RecordDecodeAttempt) -> Result<T, Denial>,
) -> Result<Vec<T>, Denial> {
    let count = input.u32()?;
    budget.claim_nested_entries(u64::from(count))?;
    let minimum_bytes = usize::try_from(count)
        .ok()
        .and_then(|count| count.checked_mul(minimum_entry_bytes))
        .ok_or_else(|| Denial::new(Kind::Truncated))?;
    if minimum_bytes > input.remaining_len() {
        return Err(Denial::new(Kind::Truncated));
    }
    let capacity = usize::try_from(count).map_err(|_| Denial::new(Kind::NumericWidthExceeded))?;
    let mut values = Vec::with_capacity(capacity);
    for _ in 0..count {
        values.push(decode(input, budget)?);
    }
    Ok(values)
}

pub(super) fn require_canonical_sequence<T: Ord>(values: &[T]) -> Result<(), Denial> {
    if values.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(Denial::new(Kind::NonCanonicalRecordSequence));
    }
    Ok(())
}

pub(super) fn require_canonical_sequence_by<T, K: Ord + ?Sized>(
    values: &[T],
    key: impl Fn(&T) -> &K,
) -> Result<(), Denial> {
    if values.windows(2).any(|pair| key(&pair[0]) >= key(&pair[1])) {
        return Err(Denial::new(Kind::NonCanonicalRecordSequence));
    }
    Ok(())
}

pub(super) fn require_canonical_sequence_by_order<T>(
    values: &[T],
    compare: impl Fn(&T, &T) -> Ordering,
) -> Result<(), Denial> {
    if values
        .windows(2)
        .any(|pair| compare(&pair[0], &pair[1]) != Ordering::Less)
    {
        return Err(Denial::new(Kind::NonCanonicalRecordSequence));
    }
    Ok(())
}
