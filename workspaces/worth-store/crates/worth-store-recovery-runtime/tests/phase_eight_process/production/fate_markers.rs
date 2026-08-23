use std::collections::BTreeMap;
use std::process::Output;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IndexedRecoveryFate {
    pub(super) idempotency: [u8; 32],
    pub(super) fate: String,
}

pub(super) fn assert_writer_issued_fates(
    expected: &BTreeMap<[u8; 32], u8>,
    actual: &[IndexedRecoveryFate],
) -> Result<(), String> {
    let mut observed = BTreeMap::new();
    for fate in actual {
        let tag = fate_tag(&fate.fate)?;
        if observed.insert(fate.idempotency, tag).is_some() {
            return Err("recovery emitted a duplicate writer-issued fate identity".to_owned());
        }
        let writer_fate = expected.get(&fate.idempotency).copied();
        let compatible = match (writer_fate, tag) {
            (Some(4), 2 | 4) => true,
            (Some(expected), observed) => expected == observed,
            (None, _) => false,
        };
        if !compatible {
            return Err(format!(
                "recovery emitted a missing or incompatible writer-issued fate for {:?}: expected {:?}, observed {:?}",
                fate.idempotency,
                writer_fate,
                Some(tag),
            ));
        }
    }
    if observed.len() != expected.len() {
        let missing = expected
            .keys()
            .filter(|identity| !observed.contains_key(*identity))
            .count();
        let unexpected = observed
            .keys()
            .filter(|identity| !expected.contains_key(*identity))
            .count();
        return Err(format!(
            "recovery writer-issued fate bijection has {} observed identities, expected {}; missing {}, unexpected {}",
            observed.len(),
            expected.len(),
            missing,
            unexpected
        ));
    }
    Ok(())
}

pub(super) fn indexed_fate_tags(
    actual: &[IndexedRecoveryFate],
) -> Result<BTreeMap<[u8; 32], u8>, String> {
    let mut observed = BTreeMap::new();
    for fate in actual {
        let tag = fate_tag(&fate.fate)?;
        if observed.insert(fate.idempotency, tag).is_some() {
            return Err("recovery emitted a duplicate fate identity".to_owned());
        }
    }
    Ok(observed)
}

pub fn persisted_fate_tags(output: &Output) -> Result<BTreeMap<[u8; 32], u8>, String> {
    indexed_fate_tags(&parse_indexed_recovery_fates(output))
}

fn fate_tag(fate: &str) -> Result<u8, String> {
    match fate {
        "AcknowledgedDurable" => Ok(1),
        "DurableUnacknowledged" => Ok(2),
        "ProvenNoEffect" => Ok(3),
        "Indeterminate" => Ok(4),
        other => Err(format!("unknown recovery fate {other}")),
    }
}

pub(super) fn parse_indexed_recovery_fates(output: &Output) -> Vec<IndexedRecoveryFate> {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    stderr
        .lines()
        .chain(stdout.lines())
        .filter(|line| line.starts_with("C8_RECOVERY_FATE "))
        .map(parse_indexed_recovery_fate)
        .collect()
}

fn parse_indexed_recovery_fate(line: &str) -> IndexedRecoveryFate {
    let mut fields = line.split_whitespace();
    assert_eq!(fields.next(), Some("C8_RECOVERY_FATE"));
    let idempotency = fields.next().expect("indexed fate identity");
    let (name, idempotency) = idempotency
        .split_once('=')
        .expect("indexed fate identity uses name=value");
    assert_eq!(name, "idempotency");
    let fate = fields.next().expect("indexed fate classification");
    let (name, fate) = fate
        .split_once('=')
        .expect("indexed fate classification uses name=value");
    assert_eq!(name, "fate");
    assert!(fields.next().is_none());
    IndexedRecoveryFate {
        idempotency: decode_hex_32(idempotency),
        fate: fate.to_owned(),
    }
}

fn decode_hex_32(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64);
    let mut bytes = [0; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16)
            .expect("indexed fate identity is hexadecimal");
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::{assert_writer_issued_fates, IndexedRecoveryFate};
    use std::collections::BTreeMap;

    fn identity(value: u8) -> [u8; 32] {
        [value; 32]
    }

    fn fate(value: u8, name: &str) -> IndexedRecoveryFate {
        IndexedRecoveryFate {
            idempotency: identity(value),
            fate: name.to_owned(),
        }
    }

    fn expected() -> BTreeMap<[u8; 32], u8> {
        BTreeMap::from([(identity(1), 1), (identity(2), 3)])
    }

    #[test]
    fn writer_fate_bijection_rejects_swapped_fates() {
        let actual = [fate(1, "ProvenNoEffect"), fate(2, "AcknowledgedDurable")];
        assert!(assert_writer_issued_fates(&expected(), &actual).is_err());
    }

    #[test]
    fn writer_fate_bijection_rejects_added_identity() {
        let actual = [
            fate(1, "AcknowledgedDurable"),
            fate(2, "ProvenNoEffect"),
            fate(3, "Indeterminate"),
        ];
        assert!(assert_writer_issued_fates(&expected(), &actual).is_err());
    }

    #[test]
    fn writer_fate_bijection_rejects_deleted_identity() {
        let actual = [fate(1, "AcknowledgedDurable")];
        assert!(assert_writer_issued_fates(&expected(), &actual).is_err());
    }
}
