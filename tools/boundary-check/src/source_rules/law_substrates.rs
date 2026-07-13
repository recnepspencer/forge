//! Production admission for law substrates (universal worth-proof contract).

use crate::config::{LawSubstrateConfig, NamingConfig};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use std::collections::BTreeSet;

const REQUIRED_SUBSTRATE_PACKAGE: &str = "worth-proof";
const REQUIRED_TIERS: &[&str] = &["worth", "worthy"];

/// Fail closed unless config records exactly one universal `worth-proof` row.
pub(super) fn validate_law_substrates(
    naming: &NamingConfig,
    law_substrates: &[LawSubstrateConfig],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen_packages = BTreeSet::new();
    let known_bands: BTreeSet<&str> = naming.bands.iter().map(String::as_str).collect();
    const KNOWN_TIERS: &[&str] = &["worth", "worthy"];

    for substrate in law_substrates {
        if !seen_packages.insert(substrate.package.as_str()) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Bc7002LawSubstrateConfig,
                &substrate.package,
                format!(
                    "duplicate law_substrates package {}; each substrate package may appear once",
                    substrate.package
                ),
            ));
        }
        if substrate.tiers.is_empty() || substrate.bands.is_empty() {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Bc7002LawSubstrateConfig,
                &substrate.package,
                format!(
                    "law substrate {} must declare non-empty tiers and bands",
                    substrate.package
                ),
            ));
        }
        for tier in &substrate.tiers {
            if !KNOWN_TIERS.contains(&tier.as_str()) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc7002LawSubstrateConfig,
                    &substrate.package,
                    format!(
                        "law substrate {} declares unknown tier {tier}; legal tiers: worth, worthy",
                        substrate.package
                    ),
                ));
            }
        }
        for band in &substrate.bands {
            if !known_bands.contains(band.as_str()) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc7002LawSubstrateConfig,
                    &substrate.package,
                    format!(
                        "law substrate {} declares unknown band {band}; legal bands are naming.bands",
                        substrate.package
                    ),
                ));
            }
        }
    }

    diagnostics.extend(require_universal_worth_proof(naming, law_substrates));
    diagnostics
}

fn require_universal_worth_proof(
    naming: &NamingConfig,
    law_substrates: &[LawSubstrateConfig],
) -> Vec<Diagnostic> {
    let matches: Vec<&LawSubstrateConfig> = law_substrates
        .iter()
        .filter(|row| row.package == REQUIRED_SUBSTRATE_PACKAGE)
        .collect();

    if matches.is_empty() {
        return vec![Diagnostic::new(
            DiagnosticCode::Bc7002LawSubstrateConfig,
            REQUIRED_SUBSTRATE_PACKAGE,
            format!(
                "law_substrates must record exactly one `{REQUIRED_SUBSTRATE_PACKAGE}` row legal in \
every tier and band; missing row. Required tiers: {}; required bands: naming.bands exactly.",
                REQUIRED_TIERS.join(", ")
            ),
        )];
    }
    if matches.len() > 1 {
        return vec![Diagnostic::new(
            DiagnosticCode::Bc7002LawSubstrateConfig,
            REQUIRED_SUBSTRATE_PACKAGE,
            format!(
                "law_substrates must record exactly one `{REQUIRED_SUBSTRATE_PACKAGE}` row; found {}",
                matches.len()
            ),
        )];
    }

    let row = matches[0];
    let mut diagnostics = Vec::new();

    let tiers: BTreeSet<&str> = row.tiers.iter().map(String::as_str).collect();
    let required_tiers: BTreeSet<&str> = REQUIRED_TIERS.iter().copied().collect();
    if tiers != required_tiers {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc7002LawSubstrateConfig,
            REQUIRED_SUBSTRATE_PACKAGE,
            format!(
                "`{REQUIRED_SUBSTRATE_PACKAGE}` law substrate tiers must equal exactly {{worth, worthy}}; \
found [{}]",
                join_sorted(&tiers)
            ),
        ));
    }

    let bands: BTreeSet<&str> = row.bands.iter().map(String::as_str).collect();
    let required_bands: BTreeSet<&str> = naming.bands.iter().map(String::as_str).collect();
    if bands != required_bands {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc7002LawSubstrateConfig,
            REQUIRED_SUBSTRATE_PACKAGE,
            format!(
                "`{REQUIRED_SUBSTRATE_PACKAGE}` law substrate bands must equal naming.bands exactly; \
found [{}], required [{}]",
                join_sorted(&bands),
                join_sorted(&required_bands)
            ),
        ));
    }

    diagnostics
}

fn join_sorted(values: &BTreeSet<&str>) -> String {
    values.iter().copied().collect::<Vec<_>>().join(", ")
}

/// True when `dependency` is a configured law substrate legal for the source tier/band.
pub(crate) fn is_legal_law_substrate_edge(
    dependency: &str,
    source_tier: &str,
    source_band: &str,
    law_substrates: &[LawSubstrateConfig],
) -> bool {
    law_substrates.iter().any(|substrate| {
        substrate.package == dependency
            && substrate.tiers.iter().any(|tier| tier == source_tier)
            && substrate.bands.iter().any(|band| band == source_band)
    })
}

/// Known substrate package that is not admitted for this source tier/band.
pub(crate) fn illegal_law_substrate_edge(
    dependency: &str,
    source_tier: &str,
    source_band: &str,
    law_substrates: &[LawSubstrateConfig],
) -> Option<String> {
    let substrate = law_substrates
        .iter()
        .find(|substrate| substrate.package == dependency)?;
    if substrate.tiers.iter().any(|tier| tier == source_tier)
        && substrate.bands.iter().any(|band| band == source_band)
    {
        return None;
    }
    Some(format!(
        "law substrate {} is not admitted for tier {source_tier} band {source_band}; \
admitted tiers: [{}]; admitted bands: [{}]",
        substrate.package,
        substrate.tiers.join(", "),
        substrate.bands.join(", ")
    ))
}
