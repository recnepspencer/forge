use std::num::NonZeroU64;

use worth_store::physical_runtime::{
    PhysicalWorkFilesystemCapabilityEvidence, PhysicalWorkFilesystemCapabilityObservation,
    PhysicalWorkFilesystemLocationEvidence, PhysicalWorkFilesystemProfileEvidence,
    PhysicalWorkFilesystemProfileParts, PhysicalWorkFilesystemSupportEvidence,
};

const PREFIX: &str = "C5_1_FILESYSTEM_PROFILE ";

pub(super) fn parse(lines: &[String]) -> Result<PhysicalWorkFilesystemProfileEvidence, String> {
    let matching = lines
        .iter()
        .filter(|line| line.starts_with(PREFIX))
        .collect::<Vec<_>>();
    let [line] = matching.as_slice() else {
        return Err(format!(
            "expected one `{PREFIX}` marker, found {}",
            matching.len()
        ));
    };
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 9 {
        return Err(format!("malformed filesystem profile marker `{line}`"));
    }
    PhysicalWorkFilesystemProfileEvidence::from_parts(PhysicalWorkFilesystemProfileParts {
        root_identity: fixed_hex(fields[1], "filesystem root identity")?,
        volume_identity: fixed_hex(fields[2], "filesystem volume identity")?,
        filesystem_type: decode_text(fields[3], "filesystem type")?.into_boxed_str(),
        allocation_granularity: NonZeroU64::new(number(fields[4], "allocation granularity")?)
            .ok_or_else(|| "filesystem allocation granularity cannot be zero".to_owned())?,
        location: location(fields[5])?,
        removable: boolean(fields[6], "filesystem removable posture")?,
        read_only: boolean(fields[7], "filesystem read-only posture")?,
        capabilities: capabilities(fields[8])?,
    })
    .map_err(|denial| format!("filesystem profile evidence denied: {denial:?}"))
}

fn capabilities(
    encoded: &str,
) -> Result<Box<[PhysicalWorkFilesystemCapabilityObservation]>, String> {
    if encoded.len() != PhysicalWorkFilesystemCapabilityEvidence::ALL.len() || !encoded.is_ascii() {
        return Err("filesystem capability support vector has the wrong breadth".into());
    }
    PhysicalWorkFilesystemCapabilityEvidence::ALL
        .into_iter()
        .zip(encoded.bytes())
        .map(|(capability, support)| {
            Ok(PhysicalWorkFilesystemCapabilityObservation::new(
                capability,
                match support {
                    b'S' => PhysicalWorkFilesystemSupportEvidence::Supported,
                    b'U' => PhysicalWorkFilesystemSupportEvidence::Unsupported,
                    b'I' => PhysicalWorkFilesystemSupportEvidence::Indeterminate,
                    _ => return Err("filesystem support vector contains an unknown state"),
                },
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Vec::into_boxed_slice)
        .map_err(str::to_owned)
}

fn location(encoded: &str) -> Result<PhysicalWorkFilesystemLocationEvidence, String> {
    match encoded {
        "local" => Ok(PhysicalWorkFilesystemLocationEvidence::Local),
        "remote" => Ok(PhysicalWorkFilesystemLocationEvidence::Remote),
        "unknown" => Ok(PhysicalWorkFilesystemLocationEvidence::Unknown),
        _ => Err(format!("unknown filesystem location `{encoded}`")),
    }
}

fn fixed_hex<const N: usize>(encoded: &str, label: &str) -> Result<[u8; N], String> {
    let bytes = decode_hex(encoded, label)?;
    bytes
        .try_into()
        .map_err(|_| format!("{label} must contain exactly {N} bytes"))
}

fn decode_text(encoded: &str, label: &str) -> Result<String, String> {
    String::from_utf8(decode_hex(encoded, label)?)
        .map_err(|_| format!("{label} is not valid UTF-8"))
}

fn decode_hex(encoded: &str, label: &str) -> Result<Vec<u8>, String> {
    if !encoded.len().is_multiple_of(2) || !encoded.is_ascii() {
        return Err(format!("{label} must contain hexadecimal byte pairs"));
    }
    encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let pair = std::str::from_utf8(pair).expect("ASCII was checked");
            u8::from_str_radix(pair, 16)
                .map_err(|_| format!("{label} contains non-hexadecimal data"))
        })
        .collect()
}

fn boolean(encoded: &str, label: &str) -> Result<bool, String> {
    match encoded {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("{label} must be `true` or `false`")),
    }
}

fn number<T: std::str::FromStr>(encoded: &str, label: &str) -> Result<T, String> {
    encoded
        .parse()
        .map_err(|_| format!("{label} is not a valid number"))
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn missing_or_short_profile_is_rejected() {
        assert!(parse(&[]).is_err());
        assert!(parse(&["C5_1_FILESYSTEM_PROFILE 00".to_owned()]).is_err());
    }
}
