use super::super::algorithm::{CanonicalDigestInputDomain, CanonicalDigestInputShape};
use crate::canonicalization::CanonicalBasisDomain;

pub(super) fn input_domain_token(domain: CanonicalDigestInputDomain) -> String {
    match domain {
        CanonicalDigestInputDomain::Single(domain) => {
            format!("single:{}", domain_material_token(domain))
        }
        CanonicalDigestInputDomain::DomainBundle => "domain-bundle".to_string(),
        CanonicalDigestInputDomain::ExportBundle => "export-bundle".to_string(),
    }
}

pub(super) fn input_shape_token(shape: CanonicalDigestInputShape) -> &'static str {
    match shape {
        CanonicalDigestInputShape::SingleSequence => "single-sequence",
        CanonicalDigestInputShape::DomainBundle => "domain-bundle",
        CanonicalDigestInputShape::ExportBundle => "export-bundle",
    }
}

pub(crate) fn domain_material_token(domain: CanonicalBasisDomain) -> &'static str {
    match domain {
        CanonicalBasisDomain::Value => "value",
        CanonicalBasisDomain::AspectContract => "aspect-contract",
        CanonicalBasisDomain::AspectMask => "aspect-mask",
        CanonicalBasisDomain::AuthoritativeState => "authoritative-state",
        CanonicalBasisDomain::AuthoritativePatch => "authoritative-patch",
        CanonicalBasisDomain::Identity => "identity",
        CanonicalBasisDomain::Locator => "locator",
        CanonicalBasisDomain::CompatibilityLowering => "compatibility-lowering",
        CanonicalBasisDomain::Future(value) => value,
    }
}
