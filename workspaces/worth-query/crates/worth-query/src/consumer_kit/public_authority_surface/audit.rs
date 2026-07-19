use std::collections::BTreeSet;

use super::registry::worth_query_public_authority_surface_rows;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryPublicAuthoritySurfaceFindingKind {
    UnclassifiedObservedSurface,
    DuplicateObservedSurface,
    MissingObservedManifestSurface,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicAuthoritySurfaceFinding {
    kind: WorthQueryPublicAuthoritySurfaceFindingKind,
    symbol: String,
}

impl WorthQueryPublicAuthoritySurfaceFinding {
    fn new(kind: WorthQueryPublicAuthoritySurfaceFindingKind, symbol: impl Into<String>) -> Self {
        Self {
            kind,
            symbol: symbol.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryPublicAuthoritySurfaceFindingKind {
        self.kind
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicAuthoritySurfaceAudit {
    findings: Vec<WorthQueryPublicAuthoritySurfaceFinding>,
    observed_surface_count: usize,
    classified_surface_count: usize,
}

impl WorthQueryPublicAuthoritySurfaceAudit {
    pub fn findings(&self) -> &[WorthQueryPublicAuthoritySurfaceFinding] {
        &self.findings
    }

    pub fn observed_surface_count(&self) -> usize {
        self.observed_surface_count
    }

    pub fn classified_surface_count(&self) -> usize {
        self.classified_surface_count
    }

    pub fn is_complete(&self) -> bool {
        self.findings.is_empty()
    }
}

pub fn audit_public_authority_surface_symbols(
    observed_symbols: &[&str],
) -> WorthQueryPublicAuthoritySurfaceAudit {
    let expected = worth_query_public_authority_surface_rows()
        .iter()
        .map(|row| row.symbol())
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    let mut findings = Vec::new();

    for symbol in observed_symbols {
        if !seen.insert(*symbol) {
            findings.push(WorthQueryPublicAuthoritySurfaceFinding::new(
                WorthQueryPublicAuthoritySurfaceFindingKind::DuplicateObservedSurface,
                *symbol,
            ));
        }
        if !expected.contains(symbol) {
            findings.push(WorthQueryPublicAuthoritySurfaceFinding::new(
                WorthQueryPublicAuthoritySurfaceFindingKind::UnclassifiedObservedSurface,
                *symbol,
            ));
        }
    }

    for symbol in expected.difference(&seen) {
        findings.push(WorthQueryPublicAuthoritySurfaceFinding::new(
            WorthQueryPublicAuthoritySurfaceFindingKind::MissingObservedManifestSurface,
            *symbol,
        ));
    }

    WorthQueryPublicAuthoritySurfaceAudit {
        findings,
        observed_surface_count: observed_symbols.len(),
        classified_surface_count: seen.intersection(&expected).count(),
    }
}
