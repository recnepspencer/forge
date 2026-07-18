use std::path::PathBuf;

use worth_store_test_support::structural_preflight::{
    StructuralPredicate, StructuralPreflightEvidence, STRUCTURAL_PREFLIGHT_BUNDLE_ENV,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S10StructuralPreflightDenial {
    MissingBundleEnvironment,
    BundleReadFailed(PathBuf),
    BundleDecodeFailed(PathBuf),
    FailedPredicate(StructuralPredicate),
    MissingPredicate(StructuralPredicate),
    InvalidEvidenceIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10StructuralPreflightEvidence {
    dependency_boundary_identity: [u8; 32],
    inventory_identity: [u8; 32],
    preservation_identity: [u8; 32],
    admitted_residue_identity: [u8; 32],
    evidence_identity: [u8; 32],
}

pub fn require_s10_structural_preflight(
) -> Result<S10StructuralPreflightEvidence, S10StructuralPreflightDenial> {
    let path = std::env::var_os(STRUCTURAL_PREFLIGHT_BUNDLE_ENV)
        .map(PathBuf::from)
        .ok_or(S10StructuralPreflightDenial::MissingBundleEnvironment)?;
    let bytes = std::fs::read(&path)
        .map_err(|_| S10StructuralPreflightDenial::BundleReadFailed(path.clone()))?;
    let bundle: StructuralPreflightEvidence = serde_json::from_slice(&bytes)
        .map_err(|_| S10StructuralPreflightDenial::BundleDecodeFailed(path.clone()))?;
    if let Some(failure) = bundle.failures().first() {
        return Err(S10StructuralPreflightDenial::FailedPredicate(
            failure.predicate,
        ));
    }
    Ok(S10StructuralPreflightEvidence {
        dependency_boundary_identity: required_identity(
            &bundle,
            StructuralPredicate::Dependency,
        )?,
        inventory_identity: required_identity(&bundle, StructuralPredicate::Inventory)?,
        preservation_identity: required_identity(&bundle, StructuralPredicate::Preservation)?,
        admitted_residue_identity: required_identity(
            &bundle,
            StructuralPredicate::AdmittedResidue,
        )?,
        evidence_identity: decode_identity(&bundle.evidence_identity.0)
            .ok_or(S10StructuralPreflightDenial::InvalidEvidenceIdentity)?,
    })
}

impl S10StructuralPreflightEvidence {
    pub const fn dependency_boundary_identity(self) -> [u8; 32] {
        self.dependency_boundary_identity
    }
    pub const fn inventory_identity(self) -> [u8; 32] {
        self.inventory_identity
    }
    pub const fn preservation_identity(self) -> [u8; 32] {
        self.preservation_identity
    }
    pub const fn admitted_residue_identity(self) -> [u8; 32] {
        self.admitted_residue_identity
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
}

fn required_identity(
    bundle: &StructuralPreflightEvidence,
    predicate: StructuralPredicate,
) -> Result<[u8; 32], S10StructuralPreflightDenial> {
    bundle
        .passed_identity(predicate)
        .and_then(decode_identity)
        .ok_or(S10StructuralPreflightDenial::MissingPredicate(predicate))
}

fn decode_identity(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 {
        return None;
    }
    let mut bytes = [0; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        bytes[index] = (hex_digit(pair[0])? << 4) | hex_digit(pair[1])?;
    }
    Some(bytes)
}

const fn hex_digit(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_behavioral_execution_without_preflight_bundle_denies() {
        let previous = std::env::var_os(STRUCTURAL_PREFLIGHT_BUNDLE_ENV);
        std::env::remove_var(STRUCTURAL_PREFLIGHT_BUNDLE_ENV);
        assert_eq!(
            require_s10_structural_preflight(),
            Err(S10StructuralPreflightDenial::MissingBundleEnvironment)
        );
        if let Some(previous) = previous {
            std::env::set_var(STRUCTURAL_PREFLIGHT_BUNDLE_ENV, previous);
        }
    }
}
