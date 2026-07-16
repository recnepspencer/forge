use super::ProtocolCheckInvocation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolInvariantConfigurationDenial {
    ConfigurationRead(String),
    NoConfiguredInvariant,
}

pub fn configured_invariant_count(
    invocation: &ProtocolCheckInvocation,
) -> Result<u64, ProtocolInvariantConfigurationDenial> {
    let configuration =
        std::fs::read_to_string(invocation.configuration_path()).map_err(|error| {
            ProtocolInvariantConfigurationDenial::ConfigurationRead(error.to_string())
        })?;
    let mut count = 0_u64;
    let mut in_invariant_block = false;
    for raw_line in configuration.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("\\*") {
            continue;
        }
        if let Some(remainder) = line.strip_prefix("INVARIANT ") {
            count += u64::from(!remainder.trim().is_empty());
            in_invariant_block = false;
            continue;
        }
        if let Some(remainder) = line.strip_prefix("INVARIANTS ") {
            count += remainder.split_whitespace().count() as u64;
            in_invariant_block = false;
            continue;
        }
        if line == "INVARIANTS" {
            in_invariant_block = true;
            continue;
        }
        if in_invariant_block {
            if is_configuration_directive(line) {
                in_invariant_block = false;
            } else {
                count += 1;
            }
        }
    }
    if count == 0 {
        Err(ProtocolInvariantConfigurationDenial::NoConfiguredInvariant)
    } else {
        Ok(count)
    }
}

fn is_configuration_directive(line: &str) -> bool {
    [
        "SPECIFICATION",
        "CONSTANT",
        "CONSTANTS",
        "PROPERTY",
        "PROPERTIES",
        "CONSTRAINT",
        "CONSTRAINTS",
        "INIT",
        "NEXT",
    ]
    .iter()
    .any(|directive| line == *directive || line.starts_with(&format!("{directive} ")))
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;
    use std::path::PathBuf;

    use crate::ProtocolFamily;

    use super::*;
    use crate::runner::ProtocolCheckBounds;

    #[test]
    fn every_checked_configuration_names_at_least_one_invariant() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bounds =
            ProtocolCheckBounds::new(NonZeroU64::new(1).unwrap(), NonZeroU64::new(1).unwrap());
        for protocol in ProtocolFamily::all() {
            let invocation = ProtocolCheckInvocation::for_checked_protocol(protocol, &root, bounds);
            assert!(configured_invariant_count(&invocation).unwrap() > 0);
        }
    }
}
