use super::configuration::BoundedResidencyConfiguration;

pub(super) fn record_payload(
    configuration: BoundedResidencyConfiguration,
    ordinal: usize,
) -> Result<Vec<u8>, String> {
    let bytes = configuration
        .record_bytes(ordinal)
        .ok_or_else(|| format!("bounded-residency record ordinal {ordinal} is out of range"))?;
    let ordinal = u64::try_from(ordinal)
        .map_err(|_| "bounded-residency record ordinal exceeds u64".to_owned())?;
    let payload = (0..bytes)
        .map(|offset| expected_byte(configuration.seed(), ordinal, offset))
        .collect::<Vec<_>>();
    Ok(payload)
}

pub(super) fn identify_record(
    configuration: BoundedResidencyConfiguration,
    declared_bytes: u64,
    available_payload: &[u8],
) -> Result<usize, String> {
    let ordinal = record_ordinal(available_payload)?;
    let expected_bytes = configuration
        .record_bytes(ordinal)
        .ok_or_else(|| format!("bounded-residency payload declares unknown ordinal {ordinal}"))?;
    if declared_bytes != expected_bytes as u64 {
        return Err(format!(
            "bounded-residency ordinal {ordinal} declares {declared_bytes} bytes; \
             expected {expected_bytes}",
        ));
    }
    verify_record_range(configuration, ordinal, 0, available_payload)?;
    Ok(ordinal)
}

pub(super) fn verify_record_range(
    configuration: BoundedResidencyConfiguration,
    ordinal: usize,
    offset: usize,
    payload: &[u8],
) -> Result<(), String> {
    let record_bytes = configuration
        .record_bytes(ordinal)
        .ok_or_else(|| format!("bounded-residency record ordinal {ordinal} is out of range"))?;
    let end = offset
        .checked_add(payload.len())
        .ok_or_else(|| "bounded-residency record range overflowed usize".to_owned())?;
    if end > record_bytes {
        return Err(format!(
            "bounded-residency ordinal {ordinal} range {offset}..{end} exceeds {record_bytes} bytes"
        ));
    }
    let ordinal_u64 = ordinal as u64;
    if payload.iter().enumerate().any(|(index, byte)| {
        *byte != expected_byte(configuration.seed(), ordinal_u64, offset + index)
    }) {
        return Err(format!(
            "bounded-residency ordinal {ordinal} range {offset}..{end} disagrees with seed truth"
        ));
    }
    Ok(())
}

fn payload_byte(seed: u64, ordinal: u64, offset: u64) -> u8 {
    let mixed = seed
        ^ ordinal.wrapping_add(1).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ offset.wrapping_add(1).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    avalanche(mixed) as u8
}

fn record_ordinal(payload: &[u8]) -> Result<usize, String> {
    let encoded: [u8; 8] = payload
        .get(..8)
        .ok_or_else(|| "bounded-residency payload omitted its ordinal".to_owned())?
        .try_into()
        .expect("eight-byte slice has exact width");
    usize::try_from(u64::from_le_bytes(encoded))
        .map_err(|_| "bounded-residency payload ordinal exceeds usize".to_owned())
}

fn expected_byte(seed: u64, ordinal: u64, offset: usize) -> u8 {
    if offset < 8 {
        ordinal.to_le_bytes()[offset]
    } else {
        payload_byte(seed, ordinal, offset as u64)
    }
}

const fn avalanche(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::{avalanche, expected_byte, payload_byte};

    #[test]
    fn seed_record_and_offset_all_participate_in_payload_identity() {
        let baseline = payload_byte(7, 0, 0);
        assert_ne!(baseline, payload_byte(8, 0, 0));
        assert_ne!(baseline, payload_byte(7, 1, 0));
        assert_ne!(baseline, payload_byte(7, 0, 1));
        assert_ne!(avalanche(1), avalanche(2));
        assert_eq!(
            &[expected_byte(7, 42, 0), expected_byte(7, 42, 1)],
            &[42, 0]
        );
    }
}
