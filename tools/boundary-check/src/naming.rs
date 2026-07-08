use crate::config::{NamingConfig, ReservedDomainConfig};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::manifest_types::Road1Package;
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub(crate) struct CrateName {
    pub(crate) tier: String,
    pub(crate) band: String,
    pub(crate) domain: String,
}

pub(crate) fn parse_crate_name(raw: &str) -> Result<CrateName, Diagnostic> {
    let parts: Vec<_> = raw.split('-').collect();
    if parts.len() < 3 {
        return Err(Diagnostic::new(
            DiagnosticCode::Bc1001IllegalCrateName,
            raw,
            "crate name must parse as {tier}-{band}-{domain}",
        ));
    }
    Ok(CrateName {
        tier: parts[0].to_owned(),
        band: parts[1].to_owned(),
        domain: parts[2..].join("-"),
    })
}

pub(crate) fn validate_package_names(
    packages: &[Road1Package],
    naming: &NamingConfig,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let reserved = reserved_domain_set(&naming.reserved_domains);
    let bands: BTreeSet<_> = naming.bands.iter().cloned().collect();

    for package in packages {
        match parse_crate_name(&package.name) {
            Ok(parsed) => {
                if !matches!(parsed.tier.as_str(), "worth" | "worthy") {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticCode::Bc1001IllegalCrateName,
                        &package.name,
                        "tier must be worth or worthy",
                    ));
                }
                if !bands.contains(&parsed.band) {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticCode::Bc1001IllegalCrateName,
                        &package.name,
                        "band is not part of the frozen grammar",
                    ));
                }
                if !reserved.contains(&(
                    parsed.tier.clone(),
                    parsed.band.clone(),
                    parsed.domain.clone(),
                )) {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticCode::Bc1002UnreservedDomain,
                        &package.name,
                        "domain is not reserved for this tier/band combination",
                    ));
                }
            }
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }

    diagnostics
}

fn reserved_domain_set(configs: &[ReservedDomainConfig]) -> BTreeSet<(String, String, String)> {
    let mut set = BTreeSet::new();
    for config in configs {
        for domain in &config.domains {
            set.insert((config.tier.clone(), config.band.clone(), domain.clone()));
        }
    }
    set
}
