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
use super::g27_geometric_fractional_lead_report::materialize_g27_pressure_escape_lead_checked;

const SEARCH_EXPANSION: i32 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27DualUnitAnchorPosture {
    MoserCappedExhaustive,
    UnsupportedDegeneratePair,
    GeometryAuditRequired,
}

impl G27DualUnitAnchorPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MoserCappedExhaustive => "moser_capped_exhaustive",
            Self::UnsupportedDegeneratePair => "unsupported_degenerate_pair",
            Self::GeometryAuditRequired => "geometry_audit_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27DualUnitAnchor {
    coefficients: [i32; 4],
    comparison_profile: Vec<(String, bool)>,
}

impl G27DualUnitAnchor {
    pub fn coefficients(&self) -> [i32; 4] {
        self.coefficients
    }

    pub fn comparison_profile(&self) -> &[(String, bool)] {
        &self.comparison_profile
    }

    fn stable_token(&self) -> String {
        let [a, b, c, d] = self.coefficients;
        let profile = self
            .comparison_profile
            .iter()
            .map(|(vertex, is_unit)| format!("{vertex}:{is_unit}"))
            .collect::<Vec<_>>()
            .join(",");
        format!("{a}:{b}:{c}:{d}:[{profile}]")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27DualUnitAnchorTestReport {
    core: HadwigerArtifactCore,
    left_vertex: String,
    right_vertex: String,
    comparison_vertices: Vec<String>,
    posture: G27DualUnitAnchorPosture,
    anchors: Vec<G27DualUnitAnchor>,
    conclusion: String,
}

impl G27DualUnitAnchorTestReport {
    pub fn left_vertex(&self) -> &str {
        &self.left_vertex
    }

    pub fn right_vertex(&self) -> &str {
        &self.right_vertex
    }

    pub fn comparison_vertices(&self) -> &[String] {
        &self.comparison_vertices
    }

    pub fn posture(&self) -> G27DualUnitAnchorPosture {
        self.posture
    }

    pub fn anchors(&self) -> &[G27DualUnitAnchor] {
        &self.anchors
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn falsifies_outside_moser_dual_anchor(&self) -> bool {
        self.posture == G27DualUnitAnchorPosture::MoserCappedExhaustive
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27DualUnitAnchorTestReport, core);

pub fn test_g27_dual_unit_anchor_pair_checked(
    handle: &HadwigerResearchHandle,
    left_vertex: impl Into<String>,
    right_vertex: impl Into<String>,
    comparison_vertices: impl IntoIterator<Item = impl Into<String>>,
) -> Result<G27DualUnitAnchorTestReport, G27GeometricFractionalError> {
    let source_lead = materialize_g27_pressure_escape_lead_checked(handle)?;
    let coefficients = retained_g27_coefficients()?;
    let left_vertex = left_vertex.into();
    let right_vertex = right_vertex.into();
    let comparison_vertices = comparison_vertices
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();
    require_vertex(&left_vertex, &coefficients)?;
    require_vertex(&right_vertex, &coefficients)?;
    for vertex in &comparison_vertices {
        require_vertex(vertex, &coefficients)?;
    }
    let anchors = find_anchors(
        &coefficients,
        &left_vertex,
        &right_vertex,
        &comparison_vertices,
    )?;
    let posture = match anchors.len() {
        2 => G27DualUnitAnchorPosture::MoserCappedExhaustive,
        0 | 1 => G27DualUnitAnchorPosture::UnsupportedDegeneratePair,
        _ => G27DualUnitAnchorPosture::GeometryAuditRequired,
    };
    let conclusion = conclusion(posture, &left_vertex, &right_vertex);
    let core = artifact_core(
        HadwigerArtifactKind::G27DualUnitAnchorTestReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_dual_unit_anchor_test".to_string(),
        },
        vec![source_lead.reference()],
        payload(
            &left_vertex,
            &right_vertex,
            &comparison_vertices,
            posture,
            &anchors,
            &conclusion,
        ),
    )?;
    Ok(G27DualUnitAnchorTestReport {
        core,
        left_vertex,
        right_vertex,
        comparison_vertices,
        posture,
        anchors,
        conclusion,
    })
}

fn find_anchors(
    coefficients: &[[i32; 4]],
    left_vertex: &str,
    right_vertex: &str,
    comparison_vertices: &[String],
) -> Result<Vec<G27DualUnitAnchor>, G27GeometricFractionalError> {
    let bounds = coefficient_bounds(coefficients);
    let mut anchors = Vec::new();
    for a in bounds[0].0..=bounds[0].1 {
        for b in bounds[1].0..=bounds[1].1 {
            for c in bounds[2].0..=bounds[2].1 {
                for d in bounds[3].0..=bounds[3].1 {
                    let point = [a, b, c, d];
                    if is_unit_to(point, coefficients, left_vertex)?
                        && is_unit_to(point, coefficients, right_vertex)?
                    {
                        anchors.push(anchor(point, coefficients, comparison_vertices)?);
                    }
                }
            }
        }
    }
    anchors.sort_by_key(G27DualUnitAnchor::stable_token);
    anchors.dedup_by_key(|row| row.stable_token());
    Ok(anchors)
}

fn anchor(
    point: [i32; 4],
    coefficients: &[[i32; 4]],
    comparison_vertices: &[String],
) -> Result<G27DualUnitAnchor, G27GeometricFractionalError> {
    let comparison_profile = comparison_vertices
        .iter()
        .map(|vertex| Ok((vertex.clone(), is_unit_to(point, coefficients, vertex)?)))
        .collect::<Result<Vec<_>, G27GeometricFractionalError>>()?;
    Ok(G27DualUnitAnchor {
        coefficients: point,
        comparison_profile,
    })
}

fn is_unit_to(
    point: [i32; 4],
    coefficients: &[[i32; 4]],
    vertex: &str,
) -> Result<bool, G27GeometricFractionalError> {
    let other = coefficients[vertex_index(vertex, coefficients)?];
    Ok(is_retained_g27_moser_unit_difference([
        point[0] - other[0],
        point[1] - other[1],
        point[2] - other[2],
        point[3] - other[3],
    ]))
}

fn require_vertex(
    vertex: &str,
    coefficients: &[[i32; 4]],
) -> Result<(), G27GeometricFractionalError> {
    vertex_index(vertex, coefficients).map(|_| ())
}

fn vertex_index(
    vertex: &str,
    coefficients: &[[i32; 4]],
) -> Result<usize, G27GeometricFractionalError> {
    let index = vertex
        .parse::<usize>()
        .ok()
        .and_then(|value| value.checked_sub(1))
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "g27_vertex_label",
        })?;
    if index < coefficients.len() {
        Ok(index)
    } else {
        Err(G27GeometricFractionalError::MalformedData {
            source: "g27_vertex_label",
        })
    }
}

fn coefficient_bounds(existing: &[[i32; 4]]) -> [(i32, i32); 4] {
    let mut bounds = [(0, 0); 4];
    for index in 0..4 {
        let min = existing.iter().map(|row| row[index]).min().unwrap_or(0);
        let max = existing.iter().map(|row| row[index]).max().unwrap_or(0);
        bounds[index] = (min - SEARCH_EXPANSION, max + SEARCH_EXPANSION);
    }
    bounds
}

fn conclusion(posture: G27DualUnitAnchorPosture, left: &str, right: &str) -> String {
    match posture {
        G27DualUnitAnchorPosture::MoserCappedExhaustive => format!(
            "the two unit-circle intersections for {left}-{right} are already Moser-basis anchors"
        ),
        G27DualUnitAnchorPosture::UnsupportedDegeneratePair => {
            format!("the exact retained search did not find two intersections for {left}-{right}")
        }
        G27DualUnitAnchorPosture::GeometryAuditRequired => {
            format!("the retained search found more than two anchors for {left}-{right}")
        }
    }
}

fn payload(
    left: &str,
    right: &str,
    comparison_vertices: &[String],
    posture: G27DualUnitAnchorPosture,
    anchors: &[G27DualUnitAnchor],
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_dual_anchor.v1"),
        HadwigerArtifactPayloadEntry::text("left_vertex", left),
        HadwigerArtifactPayloadEntry::text("right_vertex", right),
        HadwigerArtifactPayloadEntry::text("comparison_vertices", comparison_vertices.join(",")),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
        HadwigerArtifactPayloadEntry::unsigned("anchor_count", anchors.len() as u128),
    ];
    for anchor in anchors {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "anchor",
            anchor.stable_token(),
        ));
    }
    payload
}
