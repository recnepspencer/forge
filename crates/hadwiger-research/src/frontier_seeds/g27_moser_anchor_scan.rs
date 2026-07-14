use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::{
    is_retained_g27_moser_unit_difference, retained_g27_coefficients,
};
use super::g27_geometric_fractional_lead_report::{
    materialize_g27_pressure_escape_lead_checked, G27PressureEscapeLeadReport,
};

const DEFAULT_EXPANSION: i32 = 1;
const MAX_RETAINED_CANDIDATES: usize = 12;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MoserAnchorBreakerCandidate {
    coefficients: [i32; 4],
    adjacent_lead_vertices: Vec<String>,
    profile: String,
}

impl G27MoserAnchorBreakerCandidate {
    pub fn coefficients(&self) -> [i32; 4] {
        self.coefficients
    }

    pub fn adjacent_lead_vertices(&self) -> &[String] {
        &self.adjacent_lead_vertices
    }

    pub fn profile(&self) -> &str {
        &self.profile
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:[{}]",
            self.coefficients[0],
            self.coefficients[1],
            self.coefficients[2],
            self.coefficients[3],
            self.profile,
            self.adjacent_lead_vertices.join(",")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MoserAnchorScanReport {
    core: HadwigerArtifactCore,
    source_lead: G27PressureEscapeLeadReport,
    expansion: i32,
    coefficient_points_checked: usize,
    breaker_count: usize,
    retained_breakers: Vec<G27MoserAnchorBreakerCandidate>,
    suppression_reason: String,
}

impl G27MoserAnchorScanReport {
    pub fn source_lead(&self) -> &G27PressureEscapeLeadReport {
        &self.source_lead
    }

    pub fn expansion(&self) -> i32 {
        self.expansion
    }

    pub fn coefficient_points_checked(&self) -> usize {
        self.coefficient_points_checked
    }

    pub fn breaker_count(&self) -> usize {
        self.breaker_count
    }

    pub fn retained_breakers(&self) -> &[G27MoserAnchorBreakerCandidate] {
        &self.retained_breakers
    }

    pub fn suppression_reason(&self) -> &str {
        &self.suppression_reason
    }

    pub fn suppresses_moser_only_breakers(&self) -> bool {
        self.suppression_reason.contains("Moser")
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27MoserAnchorScanReport, core);

pub fn scan_g27_row_685_moser_anchor_breakers_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MoserAnchorScanReport, G27GeometricFractionalError> {
    let source_lead = materialize_g27_pressure_escape_lead_checked(handle)?;
    let coefficients = retained_g27_coefficients()?;
    let row_pairs = source_lead.isometry_detail().mapping_pairs();
    let candidates = scan_breakers(&coefficients, row_pairs, DEFAULT_EXPANSION);
    let retained_breakers = candidates
        .iter()
        .take(MAX_RETAINED_CANDIDATES)
        .cloned()
        .collect::<Vec<_>>();
    let suppression_reason = "Moser-basis anchor breakers are retained as capped suppression evidence; they do not satisfy the outside-Moser escape obligation"
        .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27MoserAnchorScanReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_row_685_moser_anchor_scan".to_string(),
        },
        vec![source_lead.reference()],
        scan_payload(
            DEFAULT_EXPANSION,
            coefficient_box_size(&coefficients, DEFAULT_EXPANSION),
            candidates.len(),
            &retained_breakers,
            &suppression_reason,
        ),
    )?;
    Ok(G27MoserAnchorScanReport {
        core,
        source_lead,
        expansion: DEFAULT_EXPANSION,
        coefficient_points_checked: coefficient_box_size(&coefficients, DEFAULT_EXPANSION),
        breaker_count: candidates.len(),
        retained_breakers,
        suppression_reason,
    })
}

fn scan_breakers(
    existing: &[[i32; 4]],
    row_pairs: &[(String, String)],
    expansion: i32,
) -> Vec<G27MoserAnchorBreakerCandidate> {
    let existing_set = existing
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let bounds = coefficient_bounds(existing, expansion);
    let mut candidates = Vec::new();
    for a in bounds[0].0..=bounds[0].1 {
        for b in bounds[1].0..=bounds[1].1 {
            for c in bounds[2].0..=bounds[2].1 {
                for d in bounds[3].0..=bounds[3].1 {
                    let point = [a, b, c, d];
                    if existing_set.contains(&point) {
                        continue;
                    }
                    if let Some(candidate) = breaker_candidate(point, existing, row_pairs) {
                        candidates.push(candidate);
                    }
                }
            }
        }
    }
    candidates.sort_by_key(G27MoserAnchorBreakerCandidate::stable_token);
    candidates
}

fn breaker_candidate(
    point: [i32; 4],
    existing: &[[i32; 4]],
    row_pairs: &[(String, String)],
) -> Option<G27MoserAnchorBreakerCandidate> {
    let mut adjacent = Vec::new();
    let mut domain_profile = Vec::new();
    let mut image_profile = Vec::new();
    for (source, target) in row_pairs {
        let source_unit = is_unit_to(point, existing, source);
        let target_unit = is_unit_to(point, existing, target);
        domain_profile.push(source_unit);
        image_profile.push(target_unit);
        if source_unit {
            adjacent.push(source.clone());
        }
        if target_unit {
            adjacent.push(target.clone());
        }
    }
    if adjacent.is_empty() || domain_profile == image_profile {
        return None;
    }
    Some(G27MoserAnchorBreakerCandidate {
        coefficients: point,
        adjacent_lead_vertices: adjacent,
        profile: format!("domain{:?}:image{:?}", domain_profile, image_profile),
    })
}

fn is_unit_to(point: [i32; 4], existing: &[[i32; 4]], vertex_label: &str) -> bool {
    let index = vertex_label
        .parse::<usize>()
        .ok()
        .and_then(|value| value.checked_sub(1));
    let Some(index) = index else {
        return false;
    };
    let other = existing[index];
    is_retained_g27_moser_unit_difference([
        point[0] - other[0],
        point[1] - other[1],
        point[2] - other[2],
        point[3] - other[3],
    ])
}

fn coefficient_bounds(existing: &[[i32; 4]], expansion: i32) -> [(i32, i32); 4] {
    let mut bounds = [(0, 0); 4];
    for index in 0..4 {
        let min = existing.iter().map(|row| row[index]).min().unwrap_or(0);
        let max = existing.iter().map(|row| row[index]).max().unwrap_or(0);
        bounds[index] = (min - expansion, max + expansion);
    }
    bounds
}

fn coefficient_box_size(existing: &[[i32; 4]], expansion: i32) -> usize {
    coefficient_bounds(existing, expansion)
        .into_iter()
        .map(|(min, max)| (max - min + 1) as usize)
        .product()
}

fn scan_payload(
    expansion: i32,
    checked: usize,
    breaker_count: usize,
    retained_breakers: &[G27MoserAnchorBreakerCandidate],
    suppression_reason: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_moser_anchor_scan.v1"),
        HadwigerArtifactPayloadEntry::unsigned("expansion", expansion as u128),
        HadwigerArtifactPayloadEntry::unsigned("coefficient_points_checked", checked as u128),
        HadwigerArtifactPayloadEntry::unsigned("breaker_count", breaker_count as u128),
        HadwigerArtifactPayloadEntry::text("suppression_reason", suppression_reason),
    ];
    for breaker in retained_breakers {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "retained_breaker",
            breaker.stable_token(),
        ));
    }
    payload
}
